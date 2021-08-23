use objr::bindings::*;
use crate::NSUInteger;

objc_selector_group! {
    trait NSStringSelectors {
        @selector("initWithBytesNoCopy:length:encoding:freeWhenDone:")

    }
    impl NSStringSelectors for Sel {}
}

pub trait NSStringExtension {
    fn from_borrowed_str<'a>(str: &'a str, pool: &ActiveAutoreleasePool) -> StrongLifetimeCell<'a, NSString>;
}
impl NSStringExtension for NSString {
    ///This function creates a string that borrows the argument.
    ///
    ///In Rust, the string is valid for the lifetime of the argument.  'By convention', most objc code
    /// that encounters the NSString will end up copying it, which will copy the data into a new object
    /// with extended lifetime, although this is not guaranteed.
    ///
    /// I'm marking this API safe because, calling such functions are unsafe anyway, although you should be aware of the additional risk.
    fn from_borrowed_str<'a>(str: &'a str, pool: &ActiveAutoreleasePool) -> StrongLifetimeCell<'a,NSString> {
        let bytes = str.as_bytes().as_ptr();
        let len = str.len() as NSUInteger;
        unsafe {
            let alloc = Self::class().alloc(pool);

            let encoding: NSUInteger = 4;
            let raw = Self::perform(alloc, Sel::initWithBytesNoCopy_length_encoding_freeWhenDone(), &pool, (bytes, len,encoding,false ));
            //we assume this will work since it's str already
            Self::assume_nonnil(raw).assume_retained_limited()
        }
    }
}


#[test] fn borrow() {
    let pool = unsafe{ AutoreleasePool::new()};
    let test_str = "test 123".to_owned();

    let str1 = NSString::from_borrowed_str(&test_str,&pool);
    let str2 = str1.copy(&pool);
    println!("{}",str1);
    println!("{}",str2);
    std::mem::drop(str1);
    std::mem::drop(test_str);
    //allocate something else on the stack because we can
    let new_str = "test 456 456".to_owned();
    println!("{}",new_str);

}