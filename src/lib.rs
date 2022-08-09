extern crate core;

/**
# Drew's Foundation bindings for Rust

This library binds (some subset of) Apple Foundation to Rust.  It may be compared to [objc-foundation](https://crates.io/crates/objc-foundation/0.1.1/dependencies)
and [cocoa-foundation](https://crates.io/crates/cocoa-foundation).

Part of the [objr expanded universe](https://github.com/drewcrawford/objr#objr-expanded-universe), distinctive features
of foundationr:

* Zero-cost abstractions.  Calling this library should perform identically to calling Foundation from Swift/ObjC applications.
    * Most of the magic happens in [objr](https://github.com/drewcrawford/objr) or [blocksr](https://github.com/drewcrawford/blocksr)
      which provide cutting-edge high-performance primitives which are used here extensively.
* Safe APIs.  Where possible APIs are designed with safe abstractions to provide familiar guarantees to Rust developers
* Low-level.  These bindings assume familiarity with Foundation APIs and are not documented separately.
* Modular.  Foundation is a big library; individual features are gated with feature flags
* Free for noncommercial or "small commercial" use.

# Table of features

The scope of this project is to implement "all of foundation", however, in practice, I mostly have time
to implement APIs I actually use in other projects.

The following cargo features are currently supported, and provide at least some APIs on each type:

## Core types

* `nsstring`
* `nsdata`
* `nsdictionary`
* `nsarray`
* `nsvalue`
* `nsrange`
* `nsdate`

## Runtime

* `nscopying`

## System
* `nsthread`
* `nsnotification`
* `nsrunloop`


## HTTP and URLs
* `nsurl`
* `nsurlresponse`
* `nsurlrequest`
* `nsurlsession`

## Other
* `all` - enables all other features
*/
#[cfg(feature="nsurlsession")]
mod nsurlsession;
#[cfg(feature="nsurlrequest")]
mod nsurlrequest;
#[cfg(feature="nsurl")]
mod nsurl;
#[cfg(feature="nsdata")]
mod nsdata;
mod types;
#[cfg(feature="nsurlresponse")]
mod nsurlresponse;
mod nsstring;
#[cfg(feature="nsthread")]
mod nsthread;
#[cfg(feature="nsdictionary")]
mod nsdictionary;
#[cfg(feature="nscopying")]
mod nscopying;
#[cfg(feature="nsvalue")]
mod nsvalue;
#[cfg(feature="nsnotification")]
mod nsnotification;
#[cfg(feature="nsrange")]
mod nsrange;
#[cfg(feature="nsrunloop")]
mod nsrunloop;
#[cfg(feature="nsdate")]
mod nsdate;
#[cfg(feature="nsarray")]
mod nsarray;
#[cfg(feature="nsenumerator")]
mod nsenumerator;


pub use objr::foundation::*;
pub use types::{NSUInteger,NSInteger};
pub use nsstring::NSStringExtension;

#[cfg(feature="nsurl")]
pub use nsurl::NSURL;
#[cfg(feature="nsdata")]
pub use nsdata::NSData;
#[cfg(feature="nsurlresponse")]
pub use nsurlresponse::NSURLResponse;
#[cfg(feature="nsurlrequest")]
pub use nsurlrequest::NSURLRequest;
#[cfg(feature="nsurlrequest")]
pub use nsurlrequest::NSMutableURLRequest;
#[cfg(feature="nsurlsession")]
pub use nsurlsession::{NSURLSession,NSURLSessionDownloadTask,NSURLSessionDataTask,DataTaskResult};
#[cfg(feature="nsthread")]
pub use nsthread::NSThread;
#[cfg(feature="nsdictionary")]
pub use nsdictionary::{NSDictionary,NSDictionaryRaw};
#[cfg(feature="nscopying")]
pub use nscopying::NSCopying;
#[cfg(feature="nsvalue")]
pub use nsvalue::NSNumber;
#[cfg(feature="nsnotification")]
pub use nsnotification::{NSNotification,NSNotificationName};
#[cfg(feature="nsrange")]
pub use nsrange::NSRange;
#[cfg(feature="nsrunloop")]
pub use nsrunloop::NSRunLoop;
#[cfg(feature="nsdate")]
pub use nsdate::NSDate;