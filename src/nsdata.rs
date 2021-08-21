use objr::bindings::*;
use crate::NSUInteger;
objc_class! {
    pub struct NSData {
        @class(NSData)
    }
}
objc_selector_group! {
    trait NSDataSelectors {
        @selector("initWithBytesNoCopy:length:freeWhenDone:")
    }
    impl NSDataSelectors for Sel {}
}
impl NSData {
    ///This creates a NSData by borrowing the data argument.
    ///
    /// # Safety:
    /// You must check
    /// * That all objc APIs which end up seeing this instance will either only access it for the lifetime specified,
    /// or will take some other step (usually, copying) the object into a longer lifetime.
    pub unsafe fn from_borrowed_bytes<'a>(data: &'a [u8], pool: &ActiveAutoreleasePool) -> StrongLifetimeCell<'a,NSData> {
        let uninitialized = NSData::class().alloc(pool);
        let init = Self::perform(uninitialized, Sel::initWithBytesNoCopy_length_freeWhenDone(), pool, (data.as_ptr(), data.len() as NSUInteger, false));
        Self::assume_nonnil(init).assume_retained_limited()
    }
}

#[test] fn with_borrowed_data() {
    let s = "My test string".to_owned();
    let pool = AutoreleasePool::new();
    unsafe{
        let data = NSData::from_borrowed_bytes(s.as_bytes(),&pool);
        println!("data {}",data);

    }
}