use objr::bindings::*;
use super::NSString;
objc_class! {
    pub struct NSURL {
        @class(NSURL)
    }
}
objc_selector_group! {
    pub trait NSURLSelectors {
        @selector("initWithString:")
        @selector("absoluteString")
        @selector("path")
    }
    impl NSURLSelectors for Sel {}
}
#[allow(non_snake_case)]
impl NSURL {
    pub fn from_string(str: &super::NSString, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSURL>> {
        unsafe{
            let uninitialized = Self::class().alloc(pool);
            Self::nullable(Self::perform(uninitialized, Sel::initWithString_(), pool, (str,))).assume_retained()
        }

    }
    pub fn absoluteString(self: &NSURL, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSString>> {
        unsafe {
            let r = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::absoluteString(), pool, ());
            NSString::nullable(r).assume_retained()
        }
    }
    pub fn path(self: &NSURL, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSString>> {
        unsafe {
            let r = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::path(), pool, ());
            NSString::nullable(r).assume_retained()
        }
    }
}

#[test] fn from_string() {
    let pool = unsafe{ AutoreleasePool::new() };
    let url = NSURL::from_string(objc_nsstring!("https://sealedabstract.com"), &pool).unwrap();
    assert!(url.description(&pool).to_str(&pool).starts_with("https://sealedabstract.com"));
}