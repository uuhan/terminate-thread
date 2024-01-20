use std::time::Duration;
use terminate_thread::Thread;

fn main() {
    Thread::spawn(|| loop {
        println!("loop in main");
        std::thread::sleep(Duration::from_secs(1));
    });

    std::thread::park();
}
