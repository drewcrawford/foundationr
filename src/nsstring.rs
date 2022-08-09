use std::ffi::OsStr;
use objr::bindings::*;
use crate::NSUInteger;

objc_selector_group! {
    trait NSStringSelectors {
        @selector("initWithBytesNoCopy:length:encoding:freeWhenDone:")

    }
    impl NSStringSelectors for Sel {}
}

pub trait NSStringExtension {
    ///This function creates a string that borrows the argument.
    ///
    ///In Rust, the string is valid for the lifetime of the argument.  'By convention', most objc code
    /// that encounters the NSString will end up copying it, which will copy the data into a new object
    /// with extended lifetime, although this is not guaranteed.
    ///
    /// I'm marking this API safe because, calling such functions are unsafe anyway, although you should be aware of the additional risk.
    fn from_borrowed_str<'a>(str: &'a str, pool: &ActiveAutoreleasePool) -> StrongLifetimeCell<'a, NSString>;
    /**

    Creates a string that borrows the argument.

    In Rust, the string is valid for the lifetime of the argument.  'By convention', most objc cod
    that encounters the NSString will end up copying it, which will copy the data into a new objec
    with extended lifetime, although this is not guaranteed
      */
    fn from_borrowed_os_str<'a>(str: &'a OsStr, pool: &ActiveAutoreleasePool) -> StrongLifetimeCell<'a, NSString>;
}
impl NSStringExtension for NSString {

    fn from_borrowed_str<'a>(str: &'a str, pool: &ActiveAutoreleasePool) -> StrongLifetimeCell<'a,NSString> {
        let bytes = str.as_bytes().as_ptr();
        let len = str.len() as NSUInteger;
        unsafe {
            let alloc = Self::class().alloc(pool);

            let encoding: NSUInteger = 4;
            let raw = Self::perform(alloc, Sel::initWithBytesNoCopy_length_encoding_freeWhenDone(), &pool, (bytes.assume_nonmut_perform(), len,encoding,false ));
            //we assume this will work since it's str already
            Self::assume_nonnil(raw).assume_retained_limited()
        }
    }

    fn from_borrowed_os_str<'a>(str: &'a OsStr, pool: &ActiveAutoreleasePool) -> StrongLifetimeCell<'a, NSString> {
        use std::os::unix::ffi::OsStrExt;
        let bytes = str.as_bytes();
        let len = bytes.len() as NSUInteger;

        let encoding: NSUInteger = 4;
        unsafe {
            let alloc = Self::class().alloc(pool);

            let raw = Self::perform(alloc, Sel::initWithBytesNoCopy_length_encoding_freeWhenDone(), &pool, (bytes.as_ptr().assume_nonmut_perform(), len,encoding,false ));
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

#[test] fn os_str() {
    use std::path::Path;
    let path = Path::new("test my på†h");
    let os_str = path.as_os_str();
    autoreleasepool(|pool| {
        let nsstring = NSString::from_borrowed_os_str(os_str,pool);
        let description = nsstring.description(pool).to_string();
        assert_eq!(&description, "test my på†h")
    });
}