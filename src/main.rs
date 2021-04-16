use std::thread;
use std::env;

mod server;
mod client;


fn main() {
    let mut run_server = true;

    let args: Vec<String> = env::args().collect();
    for arg in args.iter() {
        if arg.trim() == "--no-server" {
            run_server = false;
        }
    }
    println!("{:?}", args);

    
    if run_server {
        thread::spawn(|| {
            server::launch_server();
        });
    }

    client::run_client();
}
