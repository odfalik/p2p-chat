use std::{io::{ErrorKind, Read, Write}, net::SocketAddr};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const LOCAL: &str = "localhost:6000";
const MSG_SIZE: usize = 64;

#[derive(Debug, Clone)]
struct ServerEvent {
    client_addr: SocketAddr,
    msg: String
}

pub fn launch_server() {
    println!("Launching server on {}", LOCAL);

    // Open up TCP listener
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("Failed to set TcpListener non-blocking");

    let mut clients = vec![];

    // Create inter-thread channel
    // TODO make ServerEvent struct
    let (tx, rx) = mpsc::channel::<ServerEvent>();

    loop {
        // Listen for client connections
        if let Ok((mut socket, addr)) = server.accept() {

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client socket"));

            // Spawn client-handling thread
            thread::spawn(move || loop {
                let mut buf = vec![0; MSG_SIZE];

                // Receive messages from thread's client
                match socket.read_exact(&mut buf) {
                    Ok(_) => {
                        let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                        let event = ServerEvent {
                            client_addr: addr,
                            msg: msg
                        };

                        // Send formatted message to be published (on main thread)
                        tx.send(event)
                            .unwrap_or_else(|_| panic!("Failed to send message to {}!", addr));
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("{} disconnected", addr);
                        // TODO remove client from clients vec?
                        break; // Kill this thread
                    }
                }

                sleep();
            });
        }

        // Check for new msg to send to all clients
        if let Ok(e) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let event = e.clone();
                    let mut buf = format!("{}: {}", event.client_addr, event.msg).into_bytes();

                    buf.resize(MSG_SIZE, 0);
                    client.write_all(&buf).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }

        sleep();
    }
}

fn sleep() {
    thread::sleep(Duration::from_millis(50));
}
