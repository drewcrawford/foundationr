use objr::bindings::*;
objc_class! {
    pub struct NSThread {
        @class(NSThread)
    }
}
objc_selector_group! {
    pub trait NSThreadSelectors {
        @selector("currentThread")
        @selector("isMainThread")
    }
    impl NSThreadSelectors for Sel {}
}
#[allow(non_snake_case)]
impl NSThread {
    pub fn currentThread(pool: &ActiveAutoreleasePool) -> StrongCell<NSThread> {
        unsafe {
            let thread = Class::perform_autorelease_to_retain(NSThread::class().assume_nonmut_perform(),Sel::currentThread(), pool, ());
            Self::assume_nonnil(thread).assume_retained()
        }
    }
    pub fn isMainThread(&self, pool: &ActiveAutoreleasePool) -> bool {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::isMainThread(), pool, ())
        }
    }
}

#[test] fn current_thread() {
    let pool = unsafe{ AutoreleasePool::new() };
    let f = NSThread::currentThread(&pool);
    println!("{}",f);
    assert!(f.isMainThread(&pool) == false);
}