use objr::bindings::*;
use crate::NSDictionary;
use crate::NSCopying;
objc_class_newtype! {
    pub struct NSAttributedStringKey: NSString;
}
impl NSCopying for NSAttributedStringKey {}
extern "C" {
    static NSInlinePresentationIntentAttributeName: &'static NSAttributedStringKey;
    static NSAlternateDescriptionAttributeName: &'static NSAttributedStringKey;
    static NSImageURLAttributeName: &'static NSAttributedStringKey;
    static NSLanguageIdentifierAttributeName: &'static NSAttributedStringKey;
}
impl NSAttributedStringKey {
    pub fn inline_presentation_intent() -> &'static Self {
        unsafe { &NSInlinePresentationIntentAttributeName }
    }
    pub fn alternate_description() -> &'static Self {
        unsafe { &NSAlternateDescriptionAttributeName }
    }
    pub fn image_url() -> &'static Self {
        unsafe { &NSImageURLAttributeName }
    }
    pub fn language_identifier() -> &'static Self {
        unsafe { &NSLanguageIdentifierAttributeName }
    }
}

objc_class! {
    pub struct NSAttributedString {
        @class(NSAttributedString)
    }
}
#[allow(non_snake_case)]
impl NSAttributedString {
    ///```objc
    /// - (instancetype)initWithString:(NSString *)str attributes:(nullable NSDictionary<NSAttributedStringKey, id> *)attrs;
    /// ```
    pub fn withStringAttributes(string: &NSString, attributes: Option<&NSDictionary<NSAttributedStringKey, NSObject>>, pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        unsafe {
            let alloc = Self::class().alloc(pool);
            let raw: *const Self = Self::perform_autorelease_to_retain(alloc, Sel::initWithString_attributes(), pool,(string.assume_nonmut_perform(), attributes.assume_nonmut_perform()));
            Self::assume_nonnil(raw).assume_retained()
        }
    }
}

objc_selector_group! {
    trait Selectors {
        @selector("initWithString:attributes:")
        @selector("appendAttributedString:")
    }
    impl Selectors for Sel {}
}

objc_class! {
    pub struct NSMutableAttributedString {
        @class(NSMutableAttributedString)
    }
}
#[allow(non_snake_case)]
impl NSMutableAttributedString {
    ///```objc
    /// - (instancetype)initWithString:(NSString *)str attributes:(nullable NSDictionary<NSAttributedStringKey, id> *)attrs;
    /// ```
    pub fn withStringAttributes(string: &NSString, attributes: Option<&NSDictionary<NSAttributedStringKey, NSObject>>, pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        unsafe {
            let alloc = Self::class().alloc(pool);
            let raw: *const Self = Self::perform_autorelease_to_retain(alloc, Sel::initWithString_attributes(), pool,(string.assume_nonmut_perform(), attributes.assume_nonmut_perform()));
            Self::assume_nonnil(raw).assume_retained()
        }
    }
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe {
           Self::class().alloc_init(pool).assume_mut()
        }
    }
    pub fn appendAttributedString(&mut self, string: &NSAttributedString, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self,Sel::appendAttributedString_(), pool, (string.assume_nonmut_perform(),))
        }
    }
}
objc_cast!(NSMutableAttributedString, unsafe NSAttributedString, as_immutable, as_immutable_mut);

#[test] fn smoke() {
    autoreleasepool(|pool| {
        let string = NSString::with_str_copy("Hello, world!", pool);
        let attributes = NSDictionary::withObjectsForKeys(&[], &[], pool);
        let attributed_string = NSAttributedString::withStringAttributes(&string, Some(&attributes), pool);
        println!("{}", attributed_string);

        let mut mut_string = NSMutableAttributedString::new(pool);
        mut_string.appendAttributedString(&attributed_string, pool);
        mut_string.appendAttributedString(&attributed_string, pool);
        println!("{}", mut_string);
    })
}