use objr::bindings::*;
objc_class! {
    pub struct NSURL {
        @class(NSURL)
    }
}
objc_selector_group! {
    pub trait NSURLSelectors {
        @selector("initWithString:")
    }
    impl NSURLSelectors for Sel {}
}
impl NSURL {
    pub fn from_string(str: &super::NSString, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSURL>> {
        unsafe{
            let uninitialized = Self::class().alloc(pool);
            Self::nullable(Self::perform(uninitialized, Sel::initWithString_(), pool, (str,))).assume_retained()
        }

    }
}

#[test] fn from_string() {
    let pool = AutoreleasePool::new();
    let url = NSURL::from_string(objc_nsstring!("https://sealedabstract.com"), &pool).unwrap();
    assert!(url.description(&pool).to_str(&pool).starts_with("https://sealedabstract.com"));
}