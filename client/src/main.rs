use std::io::{stdin, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    // TODO read input ip? or maybe cli arg

    // Connect to server
    let mut socket = TcpStream::connect(LOCAL)
        .unwrap_or_else(|_| panic!("Failed to connect to {}", LOCAL));

    socket
        .set_nonblocking(true)
        .expect("Failed to set TcpStream not blocking");

    // Create inter-thread channel
    let (sender, receiver) = mpsc::channel::<String>();

    thread::spawn(move || loop {

        // Read from TCP stream
        let mut buf = vec![0; MSG_SIZE];
        match socket.read_exact(&mut buf) {
            Ok(_) => {
                let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                println!("{}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection closed.");
                break;
            }
        }

        // Check for message to send from stdin
        match receiver.try_recv() {
            Ok(msg) => {
                let mut buf = msg.clone().into_bytes();
                buf.resize(MSG_SIZE, 0);
                socket.write_all(&buf).expect("Writing to socket failed");
                // println!("Message sent!");
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

        let msg = buf.trim().to_string();

        if msg == ":quit" || sender.send(msg).is_err() {
            break;
        }
    }
}