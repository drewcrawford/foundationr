/*!
A series of generic types that unify various string types.

Often, bindings authors want to accept strings in a way that is both convenient, and fast.

The situation is complicated because Rust has many string types, and objc has even more,
and you can often write fast code for some combination of types, but it's difficult to work generically with them all.

This module defines a MagicString type which can be used as a generic constraint.  This constraint can be exposed to Rust
to write methods that work with any combination of types.
*/
use crate::NSString;
use objr::bindings::{StrongCell, ActiveAutoreleasePool};


///A generic type preferred for accepting a string argument in a binding.
///
///[MagicString] can be converted to [`&NSString`](NSString) via two steps, firstly into an [IntermediateString] and then to [`&NSString`](NSString).
///In cases that can be converted in fewer steps, [IntermediateString] can also be implemented on the type, and implementing the first
/// step with a no-op.
///
/// ```
/// use objr::bindings::*;
/// use foundationr::magic_string::*;
/// fn accepts_string<M:MagicString>(string: M, pool: &ActiveAutoreleasePool) {
///     let intermediate = string.as_intermediate_string(pool);
///     let _as_nsstring = intermediate.as_nsstring();
///     //perform, etc.
/// }
/// let pool = autoreleasepool(|pool| {
///     accepts_string("My owned string".to_owned(),&pool);
///     accepts_string(objc_nsstring!("My static string"),&pool);
/// });
///
///
/// ```
///
///At the moment, an implementation for [&str] is deliberately ommitted.  Usually, you wanted either of these:
/// 1.  if you have `&'static str`, use [objr::foundation::objc_nsstring!] instead which is significantly faster than converting to [NSString] at runtime (which involves
///    heap allocation, etc).
/// 2.  if you have `&str` and you're passing it into objc to do "god knows what" with it (copy it, probably), just go ahead and use an owned
///     string and push the copy back into your caller.  Potentially they can optimize it in some way.
///
/// There is, technically, a way to bridge `&str` directly into `&NSString`, there is an internal implementation in this library.  I may eventually
/// ship a public [MagicString] implementation on that basis but there are a lot of restrictions to using that API correctly, so it isn't
/// something you want to happen automatically.
///
pub trait MagicString: Clone {
    type IntermediateString: IntermediateString;
    fn as_intermediate_string(self, pool: &ActiveAutoreleasePool) ->  Self::IntermediateString;
    fn to_owned(self) -> String;
}

/**
Converts to an [NSString] in one step.

To see why this is needed, consider the case where we convert from an [String], to [StrongCell<NSString>].
In that case, we need to allocate some object.  But we can't return a pointer to that (temporary) object,
we need stack storage for it somewhere.  I mean we could RC it I guess, but it's not ideal.
*/
pub trait IntermediateString {
    fn as_nsstring(&self) -> &NSString;
}

impl IntermediateString for NSString {
    fn as_nsstring(&self) -> &NSString {
        self
    }
}
impl IntermediateString for &NSString {
    fn as_nsstring(&self) -> &NSString {
        self
    }
}

impl IntermediateString for StrongCell<NSString> {
    fn as_nsstring(&self) -> &NSString {
        self
    }
}

impl MagicString for &NSString {
    type IntermediateString = Self;

    fn as_intermediate_string(self, _pool: &ActiveAutoreleasePool) -> Self::IntermediateString {
        self
    }

    fn to_owned(self) -> String {
        self.to_string()
    }
}
impl MagicString for String {
    type IntermediateString = StrongCell<NSString>;

    fn as_intermediate_string(self, pool: &ActiveAutoreleasePool) -> Self::IntermediateString {
        //not sure this is the most efficient implementation but it works well for small strings
        NSString::with_str_copy(&self, pool)
    }

    fn to_owned(self) -> String {
        self
    }
}