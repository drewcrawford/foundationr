use std::convert::TryInto;
use objr::bindings::*;
use crate::NSUInteger;
use crate::nsenumerator::{FastEnumerator,NSFastEnumeration,NSFastEnumerationState};

objc_class! {
    pub struct NSArrayRaw {
        @class(NSArray)
    }
}

objc_class_newtype! {
    pub struct NSArray<Element>: NSArrayRaw;
}

objc_selector_group! {
    trait Selectors {
        @selector("initWithObjects:count:")
        @selector("countByEnumeratingWithState:objects:count:")
        @selector("count")
    }
    impl Selectors for Sel {}
}

#[allow(non_snake_case)]
impl<Element: Arguable + 'static> NSArray<Element> {
    ///# Safety
    /// Consider use of [Self::with_slice] instead.
    /// `objects` must be valid to dereference and of the correct type and alignment
    /// `objects` must contain at least `count` elements
    pub unsafe fn initWithObjectsCount(objects: *const &Element, count: NSUInteger, pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        let alloc = Self::class().alloc(pool);
        //the equivalent of assume_nonmut_perform for this argument
        let transmut: *mut *mut Element = std::mem::transmute(objects);
        let raw = Self::perform(alloc, Sel::initWithObjects_count(), pool, (transmut,count));
        Self::assume_nonnil(raw).assume_retained()
    }

    pub fn with_slice(objects: &[&Element], pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        unsafe {
            Self::initWithObjectsCount(objects.as_ptr(), objects.len().try_into().unwrap(), pool)
        }
    }

    pub fn iter<'a>(&'a self, pool: &'a ActiveAutoreleasePool) -> FastEnumerator<&Self> {
        FastEnumerator::new(self, pool)
    }
    pub fn count(&self, pool: &ActiveAutoreleasePool) -> NSUInteger {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::count(), pool, ())
        }
    }
}



impl<Element: Arguable + 'static> NSFastEnumeration for &NSArray<Element> {
    type Element = Element;
    fn countByEnumeratingWithStateObjectsCount(self, state: &mut NSFastEnumerationState, objects: *mut *const Self::Element, count: NSUInteger, pool: &ActiveAutoreleasePool) -> NSUInteger {
        unsafe {
            //the equivalent of assume_nonmut_perform for this argument
            let transmut: *mut *mut Element = std::mem::transmute(objects);
            NSArray::perform_primitive(self.assume_nonmut_perform(), Sel::countByEnumeratingWithState_objects_count(), pool, (state,transmut, count))
        }
    }
}

#[test] fn smoke() {
    autoreleasepool(|pool| {
        use objr::bindings::objc_nsstring;
        let value1  = objc_nsstring!("hi");
        let value2 = objc_nsstring!("there");
        let arr = [value1,value2, objc_nsstring!("you"), objc_nsstring!("fool")];

        let f = NSArray::with_slice(&arr, pool);
        println!("array description {:?}",f.description(pool).to_str(pool));

        let mut alternate_iterator = f.iter(pool);
        let pop = alternate_iterator.next();
        println!("popped {pop:?}");

        let mut elements = Vec::new();
        for element in f.iter(pool) {
            elements.push(element);
        }
        assert_eq!(elements.len(), 4);
        assert_eq!(f.count(pool), 4);

    })

}

