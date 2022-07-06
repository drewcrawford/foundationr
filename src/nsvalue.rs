
use objr::bindings::*;
use std::os::raw::{c_int, c_ulong};
use crate::NSUInteger;
objc_class! {
    pub struct NSNumber {
        @class(NSNumber)
    }
}

objc_selector_group! {
    trait NSNumberSelectors {
        @selector("initWithInt:")
        @selector("initWithUnsignedLong:")
        @selector("unsignedIntegerValue")
        @selector("initWithBool:")
    }
    impl NSNumberSelectors for Sel {}
}

#[allow(non_snake_case)]
impl NSNumber {
    pub fn with_int(int: c_int, pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        unsafe {
            let s = Self::class().alloc(pool);
            let s = Self::perform(s,Sel::initWithInt_(), pool, (int,));
            Self::assume_nonnil(s).assume_retained()
        }
    }
    pub fn with_ulong(v: c_ulong, pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        unsafe {
            let s = Self::class().alloc(pool);
            let s = Self::perform(s,Sel::initWithUnsignedLong_(), pool, (v,));
            Self::assume_nonnil(s).assume_retained()
        }
    }
    pub fn with_bool(b: bool, pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        unsafe {
            let s = Self::class().alloc(pool);
            let s = Self::perform(s,Sel::initWithBool_(), pool, (b,));
            Self::assume_nonnil(s).assume_retained()
        }
    }
    pub fn unsignedIntegerValue(&self, pool: &ActiveAutoreleasePool) -> NSUInteger {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::unsignedIntegerValue(), pool, ())
        }
    }
}

#[test] fn test() {
    autoreleasepool(|pool| {
        let s = NSNumber::with_int(5, pool);
        assert_eq!(s.unsignedIntegerValue(pool),5);

        let s = NSNumber::with_ulong(5,pool);
        assert_eq!(s.unsignedIntegerValue(pool),5);

        let s = NSNumber::with_bool(true, pool);
        assert_eq!(s.unsignedIntegerValue(pool), 1);
    })
}
