#![allow(non_camel_case_types)]
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
mod api {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
mod ext;

#[doc = include_str!("../README.md")]
#[derive(Clone)]
pub struct Thread(pub(crate) Arc<ThreadInner>);

struct ThreadInner {
    /// hold the Thread instance
    inner: *mut api::terminate_thread_t,
    /// the call is over
    over: Arc<AtomicBool>,
    /// thread safty
    guard: Mutex<()>,
}

unsafe impl Send for Thread {}

impl Thread {
    /// Spawn a terminatable Thread
    pub fn spawn<F>(start: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        // Trampoile Function For Fn
        unsafe extern "C" fn trampoile(xarg: *mut std::os::raw::c_void) {
            let pair: Box<(Box<dyn FnOnce() + Send + 'static>, Arc<AtomicBool>)> =
                Box::from_raw(xarg as _);

            let (call, over) = *pair;

            call();

            over.swap(true, Relaxed);

            // NB: xarg pointer freed after
        }

        let over = Arc::new(AtomicBool::new(false));

        let cbox: Box<(Box<dyn FnOnce() + Send + 'static>, Arc<AtomicBool>)> =
            Box::new((Box::new(start), over.clone()));
        let xarg = Box::into_raw(cbox);

        unsafe {
            // create a thread here
            let inner = api::terminate_thread_create(Some(trampoile), xarg as _);

            let guard = Mutex::new(());
            Self(Arc::new(ThreadInner { inner, over, guard }))
        }
    }

    /// Stop The Spawned Thread
    pub fn terminate(&self) {
        let _guard = self.0.guard.lock().unwrap();

        if self.0.over.load(Relaxed) {
            // Call is already over
            return;
        }

        // Terminate the call
        unsafe {
            api::terminate_thread_terminate(self.0.inner);
        }
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
            if !self.0.over.load(Relaxed) {
                // Call is not over, terminate it
                api::terminate_thread_terminate(self.0.inner);
                return;
            }

            api::terminate_thread_drop(self.0.inner);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_terminate() {
        let thread = Thread::spawn(move || loop {
            sleep(Duration::from_secs(1));
        });

        thread.terminate();
    }

    #[test]
    fn test_drop_immediately() {
        Thread::spawn(move || loop {
            sleep(Duration::from_secs(1));
        });
    }

    #[test]
    fn test_thread_send() {
        let thread = Thread::spawn(move || loop {
            sleep(Duration::from_secs(1));
        });

        std::thread::spawn(move || thread.terminate())
            .join()
            .expect("terminate in other thread failed.");
    }
}
