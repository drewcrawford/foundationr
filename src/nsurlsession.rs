//! <Foundation/NSURLSession.h>
use objr::bindings::*;
use super::{NSData,NSURLResponse,NSURLRequest};


objc_class! {
    pub struct NSURLSession {
        @class(NSURLSession)
    }
}
objc_selector_group! {
    trait NSURLSessionSelectors {
        @selector("sharedSession")
        @selector("dataTaskWithRequest:completionHandler:")
    }
    impl NSURLSessionSelectors for Sel {}
}

type DataTaskResult = Result<(StrongCell<NSData>,StrongCell<NSURLResponse>),(StrongCell<NSError>,Option<StrongCell<NSURLResponse>>)>;



blocksr::once_escaping!(DataTaskCompletionHandler(data: *const NSData, response: *const NSURLResponse, error: *const NSError) -> ());
unsafe impl Arguable for &DataTaskCompletionHandler {}
#[allow(non_snake_case)]
impl NSURLSession {
    pub fn shared(pool: &ActiveAutoreleasePool) -> StrongCell<NSURLSession> {
        unsafe {
            let raw = Class::perform_autorelease_to_retain(Self::class().assume_nonmut_perform(), Sel::sharedSession(),pool, () );
            Self::assume_nonnil(raw).assume_retained()
        }
    }
    pub fn dataTaskWithRequestCompletionHandler<C: FnOnce(DataTaskResult) + Send + 'static>(&self,request: &NSURLRequest, pool: &ActiveAutoreleasePool, completion_handler: C) -> StrongMutCell<NSURLSessionDataTask> {
        let block = unsafe{ DataTaskCompletionHandler::new(|data,response,error| {
           let completion_arg = if error.is_null() {
               let data = NSData::assume_nonnil(data).retain();
               let response = NSURLResponse::assume_nonnil(response).retain();
               Ok((data,response))
           }
            else {
                let error = NSError::assume_nonnil(error).retain();
                let response = NSURLResponse::nullable(response).retain();
                Err((error,response))
            };
            completion_handler(completion_arg)
        })};
        unsafe {
            let task = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(),Sel::dataTaskWithRequest_completionHandler(), pool, (request, &block));
            NSURLSessionDataTask::assume_nonnil(task).assume_retained().assume_mut()
        }
    }
    #[cfg(feature="async")]
    pub async fn dataTaskWithRequest(&self,request: &NSURLRequest, pool: &ActiveAutoreleasePool) -> DataTaskResult {
        use blocksr::continuation::Continuation;
        let (mut completion, completer) = Continuation::new();
        let mut foundation_task = self.dataTaskWithRequestCompletionHandler(request, pool, |arg| {
            completer.complete(arg);
        });
        foundation_task.resume(pool);
        completion.accept(TaskDropper(foundation_task));
        completion.await
    }
}

objc_class! {
    pub struct NSURLSessionDataTask {
        @class(NSURLSessionDataTask)
    }
}
objc_selector_group! {
    pub trait NSURLSessionDataTaskSelectors {
        @selector("resume")
        @selector("cancel")
    }
    impl NSURLSessionDataTaskSelectors for Sel {}
}
impl NSURLSessionDataTask {
    pub fn resume(&mut self, pool: &ActiveAutoreleasePool) {
        unsafe{
            let _:() = Self::perform_primitive(self, Sel::resume(), pool, ());
        }
    }
    pub fn cancel(&mut self, pool: &ActiveAutoreleasePool) {
        unsafe{
            let _:() = Self::perform_primitive(self, Sel::cancel(), pool, ());
        }
    }
}

struct TaskDropper (StrongMutCell<NSURLSessionDataTask>);
impl Drop for TaskDropper {
    fn drop(&mut self) {
        let pool = AutoreleasePool::new();
        self.0.cancel(&pool)
    }
}


#[test] fn test_session() {
    let pool = AutoreleasePool::new();
    let a = NSURLSession::shared(&pool);
    println!("{}",a);
}

#[cfg(feature="async")]
#[test] fn test_request() {
    use super::{NSMutableURLRequest,NSURL};

    let pool = AutoreleasePool::new();
    let session = NSURLSession::shared(&pool);
    let request = NSMutableURLRequest::with_url(&NSURL::from_string(objc_nsstring!("https://sealedabstract.com"),&pool).unwrap(),&pool);
    let immutable_request = request.as_immutable();
    let task = session.dataTaskWithRequest(&immutable_request,&pool);
    let r = kiruna::test::test_await(task, std::time::Duration::from_secs(10));
    println!("{}",r.unwrap().0);
}