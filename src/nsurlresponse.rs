use objr::bindings::*;
use crate::NSUInteger;
objc_class! {
    pub struct NSURLResponse {
        @class(NSURLResponse)
    }

}
objc_selector_group! {
    trait NSURLResponseSelectors {
        @selector("statusCode")
    }
    impl NSURLResponseSelectors for Sel {}
}
//type is immutable
unsafe impl Send for NSURLResponse {}
unsafe impl Sync for NSURLResponse {}
#[allow(non_snake_case)]
impl NSURLResponse {
    pub fn statusCode(&self, pool: &ActiveAutoreleasePool) -> NSUInteger {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::statusCode(), pool, ())
        }
    }
}