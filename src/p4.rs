use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::{option, thread};
use rand::Rng;
use rusqlite::{Connection, Result, NO_PARAMS};
use mysql::prelude::*;
// use mysql::{OptsBuilder, Pool};
use mysql::*;
use num::Signed;
use std::net::TcpListener;

fn handle_p2(mut stream: TcpStream) -> i8 {
    let mut r2_bytes = [0; 1];

    // Receive r2 from P2
    stream.read_exact(&mut r2_bytes).unwrap();
    let r2 = i8::from_ne_bytes(r2_bytes);
    r2
}

fn handle_p3(mut stream: TcpStream) -> i8 {
    let mut r3_bytes = [0; 1];

    // Receive r3 from P3
    stream.read_exact(&mut r3_bytes).unwrap();
    let r3 = i8::from_ne_bytes(r3_bytes);
    r3
}

fn main() {

    let client_address = "127.0.0.1:8080"; // Assuming p2 is listening on port 8082

    let p2_address = "127.0.0.1:80822"; // Assuming p2 is listening on port 8082
    let p3_address = "127.0.0.1:8083"; // Assuming p2 is listening on port 8082
    let p4_address = "127.0.0.1:8084"; // Assuming p4 is listening on port 8084

    let listener_p4 = TcpListener::bind(p4_address).unwrap();
    let mut client_stream = TcpStream::connect(client_address).unwrap();

    for stream in listener_p4.incoming() {
        match stream {
            Ok(stream) => {
                // Clone the stream for each thread
                let p2_stream = stream.try_clone().unwrap();
                let p3_stream = stream.try_clone().unwrap();

                // Spawn separate threads for P2 and P3
                let r2_handle = thread::spawn(move || {
                    handle_p2(p2_stream)
                });

                let r3_handle = thread::spawn(move || {
                    handle_p3(p3_stream)
                });

                // Wait for both threads to finish and get their results
                let r2 = r2_handle.join().unwrap();
                let r3 = r3_handle.join().unwrap();
println!("recieve r2,r3 from p2 and p3 : {:?},{:?}",r2,r3);
                // Perform computation on r2 and r3
                let result = (r2 != 0 && r2 / r3 >= 0) as bool;
                
                let byte_value: u8 = if result { 1 } else { 0 };

                // Send the result back to the client
                client_stream.write(&[byte_value]).unwrap();

                // println!("Received r2 from P2: {}, r3 from P3: {}", r2, r3);
                println!("Result of computation: {}", result);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
