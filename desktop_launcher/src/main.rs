use std::{env, thread, time::Duration};

fn main() {
    let mut args = env::args().skip(1); // skip program name
    let mut headless = false;
    let mut url = String::new();
    while let Some(arg) = args.next() {
        if arg == "--headless" {
            headless = true;
        } else {
            url = arg;
            break;
        }
    }
    if url.is_empty() {
        eprintln!("No URL provided");
        std::process::exit(1);
    }
    if headless {
        println!("Launching in headless mode: {}", url);
    } else {
        println!("Launching: {}", url);
    }
    thread::sleep(Duration::from_secs(3));
}
