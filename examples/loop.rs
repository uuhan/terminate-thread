use std::{thread::sleep, time::Duration};
use terminate_thread::Thread;

fn main() {
    let thread = Thread::spawn(|| loop {
        println!("loop in main");
        sleep(Duration::from_secs(1));
    });

    sleep(Duration::from_secs(2));
    thread.terminate();
}
