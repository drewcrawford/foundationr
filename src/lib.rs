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
pub mod magic_string;
#[cfg(feature="nsthread")]
mod nsthread;

pub use objr::foundation::*;
pub use types::{NSUInteger,NSInteger};

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
pub use nsurlsession::{NSURLSession,NSURLSessionDownloadTask,NSURLSessionDataTask};
#[cfg(feature="nsthread")]
pub use nsthread::NSThread;