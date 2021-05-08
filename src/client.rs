use std::io::{stdin, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "localhost";
const PORT: &str = ":6000";
const MSG_SIZE: usize = 64;

pub fn run_client(arg_server_ip: Option<String>) {
    std::thread::sleep(Duration::from_secs(1));

    let mut server_ip;

    if arg_server_ip.is_some() {
        server_ip = arg_server_ip.expect("");
    } else {
        println!("Enter an IP to connect to: \t\t\t (press enter to connect to localhost)");
        let mut std_buf = String::new();
        stdin()
            .read_line(&mut std_buf)
            .expect("Reading from stdin failed!");
        server_ip = String::from(std_buf.trim());
    }
    if server_ip.len() == 0 {
        server_ip = String::from(LOCAL);
    }

    server_ip.push_str(PORT);

    // Connect to server
    let mut socket = TcpStream::connect(server_ip.to_string())
        .unwrap_or_else(|_| {
            println!("Failed to connect to {}", server_ip);
            std::process::exit(1);
        });

    println!("Connected to {}", server_ip);

    socket
        .set_nonblocking(true)
        .expect("Failed to set TcpStream non-blocking");

    // Create inter-thread channel
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        // Read from TCP stream
        let mut buf = vec![0; MSG_SIZE];
        match socket.read_exact(&mut buf) {
            Ok(_) => {
                let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                // Print message from server
                println!("{}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection closed.");
                break;
            }
        }

        // Check for message to send from stdin
        match rx.try_recv() {
            Ok(msg) => {
                let mut buf = msg.clone().into_bytes();
                buf.resize(MSG_SIZE, 0);
                socket.write_all(&buf).expect("Writing to socket failed");
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a message: ");

    loop {
        let mut buf = String::new();
        stdin()
            .read_line(&mut buf)
            .expect("Reading from stdin failed!");

        let message = crate::shared::Message {
            text: buf.trim().to_string(),
        };

        // let json = serde_json::to_string_pretty(&message).expect("Couldn't serialize message");

        // Pass message from command line to be sent
        if tx.send(message.text).is_err() {
            break;
        }
    }
}
