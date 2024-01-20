## What is this?

It's just a simple stoppable thread impl with *pthread*

## But Why?

Some times, I need to terminate a blocked thread. There is no way to 

do it with the standard `std::thread` without putting into some `Sync` thing.

## How to use it?

```rust
use stoppable_thread::Thread;

let thr = Thread::spawn(|| loop {
    // infinite loop the this thread
    println!("loop run");
    std::thread::sleep(std::time::Duration::from_secs(1));
});

std::thread::sleep(std::time::Duration::from_secs(2));
// Just stop it
thr.stop()
```

## Not a good idea!

Terminate a running thread is *ALWAYS A BAD IDEA*!

The better way is to use somethink like `std::sync::atomic::AtomicBool`,

to give your thread a chance to return.

## Tested Platform

- [x] linux
- [x] macos

It should work in any platform support *pthread*,

but the real world is sophisticated to make any promise.

