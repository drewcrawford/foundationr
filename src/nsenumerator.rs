use std::convert::TryInto;
use std::marker::PhantomData;
use std::os::raw::{c_ulong, c_void, c_long};
use objr::bindings::{ActiveAutoreleasePool, Arguable};
use crate::NSUInteger;
#[derive(Debug)]
#[repr(C)]
pub struct NSFastEnumerationState {
    state: c_ulong,
    items_ptr: *mut *const c_void,
    mutations_ptr: *mut c_long,
    extra: [c_ulong; 5],
}
unsafe impl Arguable for &mut NSFastEnumerationState {}
/**
A trait to indicate conformance to NSFastEnumeration.

Note that unlike many traits, this accepts parameters of type `self`.  Chances are you want to implement it on some particular
reference type.
*/
#[allow(non_snake_case)]
pub trait NSFastEnumeration {
    type Element;
    fn countByEnumeratingWithStateObjectsCount(self, state: &mut NSFastEnumerationState, objects: *mut *const Self::Element, count: NSUInteger, pool: &ActiveAutoreleasePool)-> NSUInteger;
}

pub struct FastEnumerator<'e, Enumerated> {
    enumerated: Enumerated ,
    state: NSFastEnumerationState,
    stack_buf: [*const c_void; 16],
    //although stack_buf would fit into u8,
    //there is some question about whether the object being enumerated
    //can return an internal buffer rather than use the provided storage.
    //in practice, this seems to be the approach taken by NSArray in many cases.
    stack_head: usize,
    stack_tail: usize,
    mutations_ptr_value: c_long,
    dummy: PhantomData<&'e ()>,
    pool: &'e ActiveAutoreleasePool,
}
impl<'e, Enumerated> FastEnumerator<'e, Enumerated> {
    pub fn new(enumerated: Enumerated, pool: &'e ActiveAutoreleasePool) -> Self {
        Self {
            enumerated: enumerated,
            state: NSFastEnumerationState {
                state: 0,
                items_ptr: std::ptr::null_mut(),
                mutations_ptr: std::ptr::null_mut(),
                extra: [0,0,0,0,0],
            },
            stack_buf: [std::ptr::null(); 16],
            stack_head: usize::MAX,
            stack_tail: usize::MAX,
            mutations_ptr_value: 0,
            dummy: PhantomData,
            pool,
        }
    }
}

enum EnumerationResult<Item> {
    Item(Item),
    NeedsSlowPath,
    Done,
}

impl<'e, Enumerated: NSFastEnumeration> FastEnumerator<'e, Enumerated> {
    fn fast_path(&mut self) -> EnumerationResult<&'e Enumerated::Element> {
        if self.stack_head < self.stack_tail {
            let p = unsafe{* self.state.items_ptr.add(self.stack_head)};
            //safety: API guarantee
            self.stack_head += 1;
            EnumerationResult::Item(unsafe{std::mem::transmute(p)})
        }
        else if self.stack_tail  < self.stack_buf.len()  {
            EnumerationResult::Done
        }
        else {
            EnumerationResult::NeedsSlowPath
        }
    }
}

impl<'e, Enumerated: NSFastEnumeration + Copy> Iterator for FastEnumerator<'e, Enumerated> where Enumerated::Element: 'e {
    type Item = &'e Enumerated::Element;

    fn next(&mut self) -> Option<Self::Item> {
        match self.fast_path() {
            EnumerationResult::Item(i) => Some(i),
            EnumerationResult::Done => None,
            EnumerationResult::NeedsSlowPath => {
                let typecast = self.stack_buf.as_mut_ptr() as *mut *const _ as *mut *const _;
                self.stack_tail = self.enumerated.countByEnumeratingWithStateObjectsCount(&mut self.state, typecast, self.stack_buf.len() as u64,self.pool ).try_into().unwrap();
                //safety: API guarantee
                if self.stack_head != usize::MAX && self.mutations_ptr_value != unsafe{*self.state.mutations_ptr} {
                    panic!("Mutation!")
                }
                self.stack_head = 0;

                //safety: API guarantee
                self.mutations_ptr_value = unsafe{*self.state.mutations_ptr};
                match self.fast_path() {
                    EnumerationResult::Item(i) => {
                        Option::Some(i)
                    },
                    EnumerationResult::Done => None,
                    EnumerationResult::NeedsSlowPath => panic!(),
                }
            }
        }
    }
}