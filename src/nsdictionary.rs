use objr::bindings::*;
use crate::nscopying::NSCopying;
use crate::NSUInteger;
use std::fmt::{Debug};

objc_class! {
    pub struct NSDictionaryRaw {
        @class(NSDictionary)
    }
}

objc_class_newtype! {
    pub struct NSDictionary <Key,Value>: NSDictionaryRaw;
}

objc_selector_group! {
    trait NSDictionarySelectors {
        @selector("initWithObjects:forKeys:count:")
        @selector("objectForKey:")
    }
    impl NSDictionarySelectors for Sel {}
}

#[allow(non_snake_case)]
impl<Key: NSCopying + 'static + Debug,Value: ObjcInstance + 'static> NSDictionary<Key,Value> {
    pub fn withObjectsForKeys(objects:&[&Value],keys:&[&Key], pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        assert_eq!(objects.len(),keys.len());
        unsafe {
            let s = Self::class().alloc(pool);
            let key_ptr: *const Key = std::mem::transmute(keys.as_ptr());
            let value_ptr: *const Value = std::mem::transmute(objects.as_ptr());
            let s = Self::perform(s,Sel::initWithObjects_forKeys_count(), pool, (value_ptr.assume_nonmut_perform(), key_ptr.assume_nonmut_perform(), objects.len() as NSUInteger));
            Self::assume_nonnil(s).assume_retained()
        }
    }
    pub fn objectForKey(&self, key: &Key, pool: &ActiveAutoreleasePool) -> Option<StrongCell<Value>> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::objectForKey_(), pool, (key.assume_nonmut_perform(),));
            Value::nullable(ptr).assume_retained()
        }
    }
}


#[test] fn test() {
    autoreleasepool(|pool| {
        let key = objc_nsstring!("Key");
        let value = objc_nsstring!("Value");
        let n = NSDictionary::withObjectsForKeys(&[value],&[key], pool);
        let description = n.description(pool);
        let description_str = description.to_str(pool);
        assert_eq!(description_str, "{\n    Key = Value;\n}");

        let object_for_key = n.objectForKey(key, pool);
        object_for_key.unwrap();

        let nil_for_key = n.objectForKey(objc_nsstring!("Not a key"), pool);
        assert!(nil_for_key.is_none());
    })
}