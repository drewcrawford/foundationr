use objr::bindings::*;
use crate::NSUInteger;
objc_class! {
    pub struct NSData {
        @class(NSData)
    }
}
unsafe impl Send for NSData {}
unsafe impl Sync for NSData {}
objc_selector_group! {
    trait NSDataSelectors {
        @selector("initWithBytesNoCopy:length:freeWhenDone:")
        @selector("initWithBytesNoCopy:length:deallocator:")
        @selector("length")
        @selector("bytes")
        @selector("writeToFile:atomically:")
    }
    impl NSDataSelectors for Sel {}
}

blocksr::once_escaping!(Deallocator(ptr: *const std::ffi::c_void, length: NSUInteger) -> ());
unsafe impl Arguable for &Deallocator {}
#[allow(non_snake_case)]
impl NSData {
    ///This creates a NSData by borrowing the data argument.
    ///
    /// # Safety:
    /// You must check
    /// * That all objc APIs which end up seeing this instance will either only access it for the lifetime specified,
    /// or will take some other step (usually, copying) the object into a longer lifetime.
    pub unsafe fn from_borrowed_bytes<'a>(data: &'a [u8], pool: &ActiveAutoreleasePool) -> StrongLifetimeCell<'a,NSData> {
        let uninitialized = NSData::class().alloc(pool);
        let init = Self::perform(uninitialized, Sel::initWithBytesNoCopy_length_freeWhenDone(), pool, (data.as_ptr().assume_nonmut_perform(), data.len() as NSUInteger, false));
        Self::assume_nonnil(init).assume_retained_limited()
    }

    pub fn from_boxed_bytes(data: Box<[u8]>, pool: &ActiveAutoreleasePool) -> StrongCell<NSData> {
        unsafe {
            let uninitialized = NSData::class().alloc(pool);
            let ptr = data.as_ptr();
            let len = data.len() as NSUInteger;
            //todo: I think this could be rewritten as a global block by reconstructing the box via length and pointer
            let block = Deallocator::new(move |_ptr,_length| {
                std::mem::drop(data);
            });
            let r = Self::perform(uninitialized, Sel::initWithBytesNoCopy_length_deallocator(), pool, (ptr.assume_nonmut_perform(), len, &block));
            NSData::assume_nonnil(r).assume_retained()
        }

    }

    pub fn length(&self, pool: &ActiveAutoreleasePool) -> NSUInteger {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::length(), &pool, ())
        }
    }
    pub fn as_slice(&self, pool: &ActiveAutoreleasePool) -> &[u8] {
        unsafe {
            let ptr: *const u8 = Self::perform_primitive(self.assume_nonmut_perform(), Sel::bytes(), &pool, ());
            let length = self.length(&pool);
            std::slice::from_raw_parts(ptr,length as usize)
        }
    }
    pub fn writeToFileAtomically(&self, path: &NSString, atomically: bool, pool: &ActiveAutoreleasePool) -> bool {
        unsafe {
            let r: bool = Self::perform_primitive(self.assume_nonmut_perform(), Sel::writeToFile_atomically(), &pool, (path.assume_nonmut_perform(), atomically));
            r
        }
    }
}

#[test] fn with_borrowed_data() {
    let s = "My test string".to_owned();
    let pool = unsafe{ AutoreleasePool::new() };
    unsafe{
        let data = NSData::from_borrowed_bytes(s.as_bytes(),&pool);
        println!("data {}",data);
        assert_eq!(data.length(&pool),14);
        let new_slice = data.as_slice(&pool);
        let old_slice = s.as_bytes();
        assert_eq!(new_slice,old_slice);
    }
}

#[test] fn with_owned_data() {
    let data = "My test string".to_owned().into_boxed_str().into_boxed_bytes();
    let pool = unsafe{ AutoreleasePool::new() };
    let data = NSData::from_boxed_bytes(data, &pool);
    println!("data {}",data);
}

#[test] fn write_to_file() {
    let data = "My test string".to_owned().into_boxed_str().into_boxed_bytes();
    let pool = unsafe{ AutoreleasePool::new() };
    let data = NSData::from_boxed_bytes(data, &pool);
    let path = NSString::with_str_copy("test.txt", &pool);
    data.writeToFileAtomically(&path, true, &pool);
}