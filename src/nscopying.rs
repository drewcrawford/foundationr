use objr::bindings::ObjcInstance;
use crate::NSString;

pub trait NSCopying: ObjcInstance {
}

impl NSCopying for NSString {

}