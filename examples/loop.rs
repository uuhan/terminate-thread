use std::{thread::sleep, time::Duration};
use terminate_thread::Thread;

fn main() {
    let thread = Thread::spawn(|| loop {
        println!("infinite loop");
        sleep(Duration::from_secs(1));
    });

    sleep(Duration::from_secs(5));
    thread.terminate();

    sleep(Duration::from_secs(1));
    println!("exit");
}
