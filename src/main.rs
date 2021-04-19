use rustop::opts;
use std::thread;

mod client;
mod server;
mod shared;

fn main() {
    let (args, _) = opts! {
        synopsis "A peer-to-peer TCP-based CLI messaging app";
        opt no_server: Option<bool> = Some(false), desc: "";
        opt debug: Option<bool> = Some(false), desc: "";
        param server_ip: Option<String>, desc: "IP of server to connect to";
    }
    .parse_or_exit();

    println!(
        "Args: {:?} {:?} {:?}",
        args.no_server, args.debug, args.server_ip
    );

    if match args.no_server {
        Some(v) => v,
        None => true, // Run server by default if no_server not specified
    } {
        thread::spawn(|| {
            server::launch_server();
        });
    }

    // Run client on main thread
    client::run_client(args.server_ip);
}
