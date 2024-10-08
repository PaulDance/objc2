//! Note: We can't use the `declare_class!` macro for this, it doesn't support
//! such use-cases (yet). Instead, we'll declare the class manually.
#![deny(unsafe_op_in_unsafe_fn)]
use std::cell::Cell;
use std::marker::PhantomData;
use std::sync::Once;

use objc2::msg_send_id;
use objc2::rc::Retained;
use objc2::runtime::{AnyClass, ClassBuilder, NSObject};
use objc2::{ClassType, Encoding, Message, RefEncode};

// The type of the instance variable that we want to store
type Ivar<'a> = &'a Cell<u8>;

/// Struct that represents our custom object.
#[repr(C)]
struct MyObject<'a> {
    // Required to give MyObject the proper layout
    superclass: NSObject,
    // For auto traits and variance
    p: PhantomData<Ivar<'a>>,
}

unsafe impl RefEncode for MyObject<'_> {
    const ENCODING_REF: Encoding = NSObject::ENCODING_REF;
}

unsafe impl Message for MyObject<'_> {}

impl<'a> MyObject<'a> {
    fn new(number: &'a mut u8) -> Retained<Self> {
        // SAFETY: The instance variable is initialized below.
        let this: Retained<Self> = unsafe { msg_send_id![Self::class(), new] };

        // It is generally very hard to use `mut` in Objective-C, so let's use
        // interior mutability instead.
        let number = Cell::from_mut(number);

        let ivar = Self::class().instance_variable("number").unwrap();
        // SAFETY: The ivar is added with the same type below, and the
        // lifetime of the reference is properly bound to the class.
        unsafe { ivar.load_ptr::<Ivar<'_>>(&this.superclass).write(number) };
        this
    }

    fn get(&self) -> u8 {
        let ivar = Self::class().instance_variable("number").unwrap();
        // SAFETY: The ivar is added with the same type below, and is initialized in `new`
        unsafe { ivar.load::<Ivar<'_>>(&self.superclass).get() }
    }

    fn set(&self, number: u8) {
        let ivar = Self::class().instance_variable("number").unwrap();
        // SAFETY: The ivar is added with the same type below, and is initialized in `new`
        unsafe { ivar.load::<Ivar<'_>>(&self.superclass).set(number) };
    }
}

unsafe impl<'a> ClassType for MyObject<'a> {
    type Super = NSObject;
    type ThreadKind = <Self::Super as ClassType>::ThreadKind;
    const NAME: &'static str = "MyObject";

    fn class() -> &'static AnyClass {
        // TODO: Use std::lazy::LazyCell
        static REGISTER_CLASS: Once = Once::new();

        REGISTER_CLASS.call_once(|| {
            let superclass = NSObject::class();
            let mut builder = ClassBuilder::new(Self::NAME, superclass).unwrap();

            builder.add_ivar::<Ivar<'_>>("number");

            let _cls = builder.register();
        });

        AnyClass::get(Self::NAME).unwrap()
    }

    fn as_super(&self) -> &Self::Super {
        &self.superclass
    }
}

fn main() {
    let mut number = 54;

    let obj = MyObject::new(&mut number);
    assert_eq!(obj.get(), 54);

    // We can now mutate the referenced `number`
    obj.set(7);
    assert_eq!(obj.get(), 7);

    // And we can clone the object, since we use interior mutability.
    let clone = obj.clone();
    clone.set(42);
    assert_eq!(obj.get(), 42);
    drop(clone);

    // It is not possible to convert to `Retained<NSObject>`, since that would
    // loose the lifetime information that `MyObject` stores.
    //
    // let obj = Retained::into_super(obj);
    //
    // Neither is it not possible to access `number` any more, since `obj`
    // holds a mutable reference to it.
    //
    // assert_eq!(number, 42);

    drop(obj);
    // And now that we've dropped `obj`, we can access `number` again
    assert_eq!(number, 42);
}
