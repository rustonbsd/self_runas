use std::{thread::sleep, time::Duration};

use self_runas::is_elevated;

fn main() {
    println!("I have started...");
    if !is_elevated() {
        println!("I am not elevated! Elevating now...");
        let _ = self_runas::admin();
        std::process::exit(0);
    } else {
        println!("I am already elevated");
        sleep(Duration::from_secs(10));
    }
}
