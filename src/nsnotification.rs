use objr::bindings::*;
objc_class! {
    pub struct NSNotification {
        @class(NSNotification)
    }
}

objc_class_newtype! {
    pub struct NSNotificationName: NSString;
}
objc_selector_group! {
    trait NSNotificationSelectors {
        @selector("object")
    }
    impl NSNotificationSelectors for Sel {}
}

impl NSNotification {
    pub fn object(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSObject>> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::object(), pool, ());
            NSObject::nullable(ptr).assume_retained()
        }
    }
}