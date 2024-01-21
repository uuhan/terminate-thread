## What is this?

It's just a simple terminatable thread implement with *posix thread* for rust

*LIMITATIONS*: `std::panic::catch_unwind` does not work in foreign language on Linux,

**BE CAREFUL** with the panics in `Thread::spawn`.

It works well in macOS. But, I can not figure out how to solve it on Linux.

## But Why?

Sometimes, I need to terminate a blocked thread. There is no way to 

do it with the standard `std::thread` without putting into some `Sync` thing.

## How to use it?

```toml
[dependencies]
terminate-thread = "0.3"
```

### Spawn your thread

```rust
use terminate_thread::Thread;
Thread::spawn(|| {}).join(); // ← spawn & join (>= 0.3.0) your thread
```

### Manually terminate your thread

```rust
use terminate_thread::Thread;
let thr = Thread::spawn(|| loop {
    // infinite loop in this thread
    println!("loop run");
    std::thread::sleep(std::time::Duration::from_secs(1));
});
std::thread::sleep(std::time::Duration::from_secs(1));
thr.terminate() // ← the thread is terminated manually!
```

### Auto terminate your thread

```rust
use terminate_thread::Thread;
{
    let _thread = Thread::spawn(|| loop {}); // ← the thread will be terminated when thread is dropped
}
```

### Panic tolerant (v0.3.1, macOS only)

```rust
use terminate_thread::Thread;
Thread::spawn(|| panic!()); // ← this is fine
let thread = Thread::spawn(|| panic!("your message")).join(); // ← thread stores the panic info
assert!(thread.over() && thread.panics()); // ← it's over and panics
let info = thread.panic_info().lock().unwrap().take().unwrap(); // ← take out the panic info
assert_eq!(info.downcast_ref::<&str>().unwrap(), &"your message"); // ← get your panic info
```

## Not a good idea!

Terminate a running thread is *ALWAYS A BAD IDEA*!

The better way is to use something like `std::sync::atomic::AtomicBool`,

to give your thread a chance to return.

## Tested Platform

- [x] Linux
- [x] macOS

It should work in any platform support *pthread*,

but the real world is sophisticated to make any promise.

## To-do 

- [ ] Terminate the job which panics. >= v0.3.0 
    - [x] macOS, >= v0.3.0
    - [ ] Linux 🚧

```rust
use terminate_thread::Thread;
Thread::spawn(|| panic!()); // ← this is fine

let thread = Thread::spawn(|| panic!()).join(); // ← thread stores the panic info
assert!(thread.over() && thread.panics()); // ← it's over and panics
```

## Issue

- [x] Terminate the thread too quick panics. >= v0.2.0

```rust
use terminate_thread::Thread;
Thread::spawn(|| {}); // ← bus error
```

- [ ] `std::panic::AssertUnwindSafe()` does not work in linux. == v0.3.0

```rust
use terminate_thread::Thread;
Thread::spawn(|| panic!()); // ← FATAL: exception not rethrown
```
