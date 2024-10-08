//! Various common #defines and enum constants.
#![allow(non_camel_case_types)]

#[cfg(any(doc, target_vendor = "apple"))]
use core::ffi::c_int;

use crate::runtime::{AnyClass, AnyObject};

/// A quick alias for a [`null_mut`][`core::ptr::null_mut`] object / instance.
pub const nil: *mut AnyObject = 0 as *mut _;

/// A quick alias for a [`null_mut`][`core::ptr::null_mut`] class.
pub const Nil: *mut AnyClass = 0 as *mut _;

/// Policies related to associative references.
///
/// These are options to [`objc_setAssociatedObject`].
///
/// [`objc_setAssociatedObject`]: crate::ffi::objc_setAssociatedObject
pub type objc_AssociationPolicy = usize;
/// Specifies a weak reference to the associated object.
///
/// This performs straight assignment, without message sends.
pub const OBJC_ASSOCIATION_ASSIGN: objc_AssociationPolicy = 0;
/// Specifies a strong reference to the associated object.
///
/// The association is not made atomically.
pub const OBJC_ASSOCIATION_RETAIN_NONATOMIC: objc_AssociationPolicy = 1;
/// Specifies that the associated object is copied.
///
/// The association is not made atomically.
pub const OBJC_ASSOCIATION_COPY_NONATOMIC: objc_AssociationPolicy = 3;
/// Specifies a strong reference to the associated object.
///
/// The association is made atomically.
pub const OBJC_ASSOCIATION_RETAIN: objc_AssociationPolicy = 0o1401;
/// Specifies that the associated object is copied.
///
/// The association is made atomically.
pub const OBJC_ASSOCIATION_COPY: objc_AssociationPolicy = 0o1403;

#[cfg(any(doc, target_vendor = "apple"))]
pub const OBJC_SYNC_SUCCESS: c_int = 0;
#[cfg(any(doc, target_vendor = "apple"))]
pub const OBJC_SYNC_NOT_OWNING_THREAD_ERROR: c_int = -1;
/// Only relevant before macOS 10.13
#[cfg(any(doc, target_vendor = "apple"))]
pub const OBJC_SYNC_TIMED_OUT: c_int = -2;
/// Only relevant before macOS 10.13
#[cfg(any(doc, target_vendor = "apple"))]
pub const OBJC_SYNC_NOT_INITIALIZED: c_int = -3;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_association_policy() {
        assert_eq!(OBJC_ASSOCIATION_RETAIN, 769);
        assert_eq!(OBJC_ASSOCIATION_COPY, 771);

        // What the GNUStep headers define
        assert_eq!(OBJC_ASSOCIATION_RETAIN, 0x301);
        assert_eq!(OBJC_ASSOCIATION_COPY, 0x303);
    }
}
