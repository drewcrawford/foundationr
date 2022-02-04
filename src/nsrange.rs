//!nsrange.h

use objr::bindings::Arguable;
use crate::NSUInteger;

#[repr(C)]
#[derive(Debug)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}
unsafe impl Arguable for NSRange {}