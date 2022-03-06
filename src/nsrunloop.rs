use objr::bindings::*;
use super::NSDate;
objc_class! {
    pub struct NSRunLoop {
        @class(NSRunLoop)
    }
}
objc_selector_group! {
    pub trait Selectors {
        @selector("mainRunLoop")
        @selector("runUntilDate:")
    }
    impl Selectors for Sel {}
}
#[allow(non_snake_case)]
impl NSRunLoop {
    pub fn mainRunLoop(pool: &ActiveAutoreleasePool) -> StrongCell<NSRunLoop> {
        unsafe {
            let raw = objr::bindings::Class::<NSRunLoop>::perform_autorelease_to_retain(Self::class().assume_nonmut_perform(), Sel::mainRunLoop(), pool, ());
            Self::assume_nonnil(raw).assume_retained()
        }
    }
    pub fn runUntilDate(&self, date: &NSDate, pool: &ActiveAutoreleasePool) {
        unsafe{Self::perform_primitive(self.assume_nonmut_perform(), Sel::runUntilDate_(), pool, (date,))}
    }
}

#[test] fn runloop() {
    autoreleasepool(|pool| {
        let runloop = NSRunLoop::mainRunLoop(pool);
        let date = NSDate::class().alloc_init(pool);
        runloop.runUntilDate(&date, pool);
    })
}