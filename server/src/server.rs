use std::sync::Arc;
use std::sync::RwLock;
use crossbeam_channel::{Receiver, Sender};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
// use std::thread;

fn _handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn start(_s: Sender<String>, _r: Receiver<String>, messages: Arc<RwLock<String>>) {

    std::thread::spawn(move || {

        messages.write().unwrap().push_str("helloworld!!");
        println!("{}", messages.read().unwrap());
        /* let listener = TcpListener::bind("0.0.0.0:3333").unwrap();

        // cb(String::from("hello world"));


        // accept connections and process them, spawning a new thread for each one
        println!("Server listening on port 3333");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move|| {
                        // connection succeeded
                        handle_client(stream)
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }
        }
        // close the socket server
        drop(listener);*/
    });
}
