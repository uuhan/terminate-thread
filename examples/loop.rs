use std::{thread::sleep, time::Duration};
use terminate_thread::Thread;

fn main() {
    static PANIC_MSG: &'static str = "the loop in thread panic...";

    let thread = Thread::spawn(|| loop {
        println!("infinite loop");
        sleep(Duration::from_secs(1));
        panic!("{}", PANIC_MSG);
    });

    sleep(Duration::from_secs(3));
    thread.terminate();

    println!("\nIs thread panic? {}", thread.panics());

    let info = thread.panic_info();

    if let Some(info) = info.lock().unwrap().take() {
        if let Some(msg) = info.downcast_ref::<String>() {
            println!("Panic message is: {}", msg);
        } else {
            println!("{:?}", info);
        }
    }

    sleep(Duration::from_secs(1));
    println!("\nexit");
}
