#![allow(non_camel_case_types)]
mod api {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[doc = include_str!("../README.md")]
pub struct Thread(ThreadInner);

struct ThreadInner {
    inner: *mut api::stoppable_thread_t,
}

unsafe impl Send for Thread {}

impl Thread {
    /// Spawn A Stoppable Thread
    pub fn spawn<F>(start: F) -> Self
    where
        F: FnOnce() + Send + Sync,
    {
        // Trampoile Function For FnOnce
        unsafe extern "C" fn trampoile(data: *mut std::os::raw::c_void) {
            let callback: Box<Box<dyn FnOnce() + Send + Sync>> = Box::from_raw(data as _);
            callback()
        }

        let cbox: Box<Box<dyn FnOnce() + Send + Sync>> = Box::new(Box::new(start));
        let data = Box::into_raw(cbox);

        unsafe {
            let inner = api::stoppable_thread_create(Some(trampoile), data as _);
            Self(ThreadInner { inner })
        }
    }

    /// Stop The Spawned Thread
    pub fn stop(&self) {
        unsafe {
            api::stoppable_thread_terminate(self.0.inner);
        }
    }

    /// Yield Out The Current Thread
    pub fn r#yield(&self) {
        unsafe {
            api::stoppable_thread_yield(self.0.inner);
        }
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
            api::stoppable_thread_drop(self.0.inner);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        atomic::{AtomicU8, Ordering::Relaxed},
        Arc,
    };
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_thread() {
        let count = Arc::new(AtomicU8::new(0));
        let count_in_thread = count.clone();
        std::thread::spawn(move || loop {
            let count = count_in_thread.fetch_add(1, Relaxed);
            if count >= 5 {
                break;
            }
            println!("[{count}] in std::thread loop");
            sleep(Duration::from_secs(1));
        });

        // stop it after 4s, count may be 4 or 5
        sleep(Duration::from_secs(4));
        let count = count.load(Relaxed);
        assert!(count > 3);

        let count = Arc::new(AtomicU8::new(0));
        let count_in_thread = count.clone();
        let thread = Thread::spawn(move || loop {
            let count = count_in_thread.fetch_add(1, Relaxed);
            println!("[{count}] in stoppable_thread loop");
            sleep(Duration::from_secs(1));
        });

        sleep(Duration::from_secs(2));
        // stop it after 2s, count may be 2 or 3
        thread.stop();
        sleep(Duration::from_secs(2));

        let count = count.load(Relaxed);
        assert!(count <= 3);
    }
}
