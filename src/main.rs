use druid::{Data, Lens};
use rustop::opts;
use std::thread;

mod client;
mod server;
mod shared;
mod ui;
#[derive(Clone, Data, Lens)]
pub struct State {
    ip: String,
    port: String,
    username: String,
    draft: String,
    // #[data(ignore)]
    msgs: String,
}

fn main() {
    let (args, _) = opts! {
        synopsis "A peer-to-peer TCP-based CLI messaging app";
        opt no_server: Option<bool> = Some(false), desc: "";
    }
    .parse_or_exit();

    println!("Args: {:?}", args.no_server);

    if match args.no_server {
        Some(v) => v,
        None => true, // Run server by default if no_server not specified
    } {
        thread::spawn(|| {
            server::launch_server();
        });
    }

    thread::spawn(|| {
        client::run_client();
    });

    ui::main();
}
