#![allow(non_camel_case_types)]
use std::sync::{Arc, Condvar, Mutex};

#[allow(dead_code)]
mod api {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
mod ext;

pub type ThreadPanicInfo = Arc<Mutex<Option<Box<dyn std::any::Any>>>>;
pub type ThreadGuard = Arc<(Condvar, Mutex<bool>)>;

#[doc = include_str!("../README.md")]
#[derive(Clone)]
pub struct Thread(pub(crate) Arc<ThreadInner>);

struct ThreadInner {
    /// hold the Thread instance
    inner: *mut api::terminate_thread_t,
    /// condvar to wait for call finish
    guard: ThreadGuard,
    /// store the panic info
    panic_info: ThreadPanicInfo,
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
            let pair: Box<(
                Box<dyn FnOnce() + Send + 'static>,
                ThreadPanicInfo,
                ThreadGuard,
            )> = Box::from_raw(xarg as _);

            let (call, panic_info, guard) = *pair;

            let call = std::panic::AssertUnwindSafe(call);

            // Store the panic info
            if let Err(info) = std::panic::catch_unwind(call) {
                panic_info.lock().unwrap().replace(info);
            }

            let mut over = guard.1.lock().unwrap();
            *over = true;
            guard.0.notify_all();

            // NB: xarg pointer freed after
        }

        let panic_info = Arc::new(Mutex::new(None));
        let guard = Arc::new((Condvar::new(), Mutex::new(false)));

        let cbox: Box<(
            Box<dyn FnOnce() + Send + 'static>,
            ThreadPanicInfo,
            ThreadGuard,
        )> = Box::new((Box::new(start), panic_info.clone(), guard.clone()));
        let xarg = Box::into_raw(cbox);

        unsafe {
            // create a thread here
            let inner = api::terminate_thread_create(Some(trampoile), xarg as _);

            Self(Arc::new(ThreadInner {
                inner,
                guard,
                panic_info,
            }))
        }
    }

    /// The thread guard
    fn guard(&self) -> ThreadGuard {
        self.0.guard.clone()
    }

    /// Stop The Spawned Thread
    pub fn terminate(&self) {
        let over = self.0.guard.1.lock().unwrap();

        if *over {
            // Call is already over
            return;
        }

        // Terminate the call
        unsafe {
            api::terminate_thread_terminate(self.0.inner);
        }
    }

    /// The call finishes
    pub fn over(&self) -> bool {
        *self.guard().1.lock().unwrap()
    }

    /// The call panics
    pub fn panics(&self) -> bool {
        // if panic occurs
        self.0.panic_info.lock().unwrap().is_some()
    }

    /// Get the thread panic info
    pub fn panic_info(&self) -> ThreadPanicInfo {
        self.0.panic_info.clone()
    }

    /// Wait the call finishes
    pub fn join(&self) -> Self {
        let (cdv, over_mtx) = &*self.0.guard;
        let mut over = over_mtx.lock().unwrap();
        while !*over {
            over = cdv.wait(over).unwrap();
        }

        self.clone()
    }
}

impl Drop for ThreadInner {
    fn drop(&mut self) {
        unsafe {
            if !*self.guard.1.lock().unwrap() {
                // Call is not over, terminate it
                api::terminate_thread_terminate(self.inner);
                return;
            }

            api::terminate_thread_drop(self.inner);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_thread_send() {
        let thread = Thread::spawn(move || loop {
            sleep(Duration::from_secs(1));
        });

        std::thread::spawn(move || thread.terminate())
            .join()
            .expect("terminate in other thread failed.");

        sleep(Duration::from_millis(500));
    }
}
