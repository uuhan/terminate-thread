#![allow(non_camel_case_types)]
mod api {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[doc = include_str!("../README.md")]
pub struct Thread(ThreadInner);

struct ThreadInner {
    inner: *mut api::terminate_thread_t,
}

unsafe impl Send for Thread {}

impl Thread {
    /// Spawn a terminatable Thread
    #[must_use]
    pub fn spawn<F>(start: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        // Trampoile Function For FnOnce
        unsafe extern "C" fn trampoile(data: *mut std::os::raw::c_void) {
            let callback: Box<Box<dyn FnOnce() + Send + 'static>> = Box::from_raw(data as _);
            callback()
        }

        let cbox: Box<Box<dyn FnOnce() + Send + 'static>> = Box::new(Box::new(start));
        let data = Box::into_raw(cbox);

        unsafe {
            let inner = api::terminate_thread_create(Some(trampoile), data as _);
            Self(ThreadInner { inner })
        }
    }

    /// Stop The Spawned Thread
    pub fn terminate(&self) {
        unsafe {
            api::terminate_thread_terminate(self.0.inner);
        }
    }

    /// Yield Out The Current Thread
    pub fn r#yield(&self) {
        unsafe {
            api::terminate_thread_yield(self.0.inner);
        }
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
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
        let _ = Thread::spawn(move || loop {
            sleep(Duration::from_secs(1));
        });
    }
}
