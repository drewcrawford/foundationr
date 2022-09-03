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
    fn withStringAttributes(string: &NSString, attributes: &NSDictionary<NSAttributedStringKey, NSObject>, pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
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
    }
    impl Selectors for Sel {}
}

#[test] fn smoke() {
    autoreleasepool(|pool| {
        let string = NSString::with_str_copy("Hello, world!", pool);
        let attributes = NSDictionary::withObjectsForKeys(&[], &[], pool);
        let attributed_string = NSAttributedString::withStringAttributes(&string, &attributes, pool);
        println!("{}", attributed_string);
    })
}