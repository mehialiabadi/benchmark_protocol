use benchmark_protocol::table::{P4paylod, Partyr, Table};
use mysql::prelude::*;
use rand::Rng;
use rusqlite::{Connection, Result, NO_PARAMS};
use serde::de::value::UsizeDeserializer;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::{option, thread};
// use mysql::{OptsBuilder, Pool};
use mysql::*;
use num::Signed;
use std::net::TcpListener;

#[derive(Debug)]
enum Message {
    Data(String),
    NoData,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub enum P4data {
//     ID(usize),
//     Smallr(i32),
// }
// fn handle_p2(mut stream: TcpStream) -> i8 {
//     // let mut r2_bytes: [u8; 1] = [0; 1];

//     // Receive r2 from P2
//     stream.read_exact(&mut r2_bytes).unwrap();
//     let r2 = i8::from_ne_bytes(r2_bytes);
//     r2
// }
fn handle_p2(stream: TcpStream, sender: Sender<Message>) {
    let mut buffer = String::new();
    if let Ok(bytes_read) = stream.take(1024).read_to_string(&mut buffer) {
        if bytes_read > 0 {
            sender.send(Message::Data(buffer)).unwrap();
        } else {
            sender.send(Message::NoData).unwrap();
        }
    } else {
        sender.send(Message::NoData).unwrap();
    }
}
fn handle_p3(stream: TcpStream, sender: Sender<Message>) {
    let mut buffer = String::new();
    if let Ok(bytes_read) = stream.take(1024).read_to_string(&mut buffer) {
        if bytes_read > 0 {
            sender.send(Message::Data(buffer)).unwrap();
        } else {
            sender.send(Message::NoData).unwrap();
        }
    } else {
        sender.send(Message::NoData).unwrap();
    }
}

// fn handle_p3(mut stream: TcpStream) -> i8 {
//     let mut r3_bytes = [0; 1];

//     // Receive r3 from P3
//     stream.read_exact(&mut r3_bytes).unwrap();
//     let r3 = i8::from_ne_bytes(r3_bytes);
//     r3
// }
// enum Payload {
//     Int(usize),
//     Table(Table),
// }

impl Message {
    fn to_string(&self) -> String {
        match self {
            Message::Data(text) => text.clone(),
            Message::NoData => todo!(),
            // Handle other variants if needed
        }
    }
}
impl Message {
    fn last_char(&self) -> Option<char> {
        match self {
            Message::Data(data) => data.chars().last(),
            Message::NoData => todo!(),
            // Handle other variants if needed
        }
    }
}
fn handle_client(stream: TcpStream, sender: Sender<Message>) {
    let mut buffer = String::new();
    if let Ok(bytes_read) = stream.take(1024).read_to_string(&mut buffer) {
        if bytes_read > 0 {
            sender.send(Message::Data(buffer)).unwrap();
        } else {
            sender.send(Message::NoData).unwrap();
        }
    } else {
        sender.send(Message::NoData).unwrap();
    }
}
// fn p4_prepare(data: &Message) -> P4paylod {
//     match data {
//         Message::Data(data) => {
//             // if let Ok(i) = data.parse::<usize>() {
//             //     return Some(P4paylod::Int(i));
//             let r: Result<Partyr, serde_json::Error> = serde_json::from_str(data);

//             }

//             // match r {
//             //     Ok(table) => Some(Payload::Table(table)),
//             //     _ => None,

//         }
//         Message::NoData => None,
//     }
// }
// fn p4_prepare(data: &Message) -> Option<P4data> {
// match data {
//     Message::Data(data) => {
//         //     if let Ok(i) = data.parse::<usize>() {
//         //         return Some(P4data::ID(i));
//         //     }
//         let r: Result<P4data, serde_json::Error> = serde_json::from_str(data);
//         match r {
//             Ok(r3) => {
//                 return Some(P4data);
//             }
//             _ => None,
//         }

// let r: Result<i32, serde_json::Error> = serde_json::from_str(data);
// match r {
//     Ok(r) => Some(P4data::Smallr(r)),
//     _ => None,
//             // }
//         }
//         Message::NoData => None,
//     }
// }

fn start_p4(server_address: &str) {
    let url = "mysql://root:123456789@localhost:3306/testdb";
    let pool = Pool::new(url).unwrap();
    // let mut row_id = 0;
    let listener = TcpListener::bind(server_address).expect("Failed to bind");

    let (sender1, receiver1) = channel();
    // let (sender2, receiver2) = channel();
    let client_address = "127.0.0.1:8080"; // Assuming p2 is listening on port 8082

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            // Clone the stream for each thread
            let sender1_clone = sender1.clone();

            // let handle1 = thread::spawn(|| handle_client(stream, sender1_clone));
            // let mut data_rec = receiver1.recv().expect("Failed handle1 thread 1");

            // println!("recieved to p4:{:?}", data_rec);

            let p2_stream = stream.try_clone().unwrap();

            //             // Spawn separate threads for P2 and P3
            let r2_handle = thread::spawn(move || handle_p2(p2_stream, sender1_clone));
            let mut data_rec = receiver1.recv().expect("Failed handle1 thread 1");
            println!("raw data is here-{:?}", data_rec);
            // let r: Result<Partyr, serde_json::Error> = serde_json::from_str(&data_rec.to_string());
            let my_struct: P4paylod = serde_json::from_str(&data_rec.to_string()).unwrap();
            for party in &my_struct.p4data {
                println!("Row ID: {}, Comput: {}", party.row_id, party.comput);
            }

            // let party_instance: Partyr = match serde_json::from_str(&data_rec.to_string()) {
            //     Ok(party) => party,
            //     Err(e) => {
            //         eprintln!("Error deserializing Partyr: {}", e);
            //         return;
            //     }
            // };

            // println!("----{:?}-----{:?}", r.row_id, r.comput);
            // if let Some(res) = p4_prepare(&data_rec) {
            //     match res {
            //         P4data::ID(i) => {
            //             println!("row_id  {:?} ", i);
            //         }
            //         P4data::Smallr(i) => {
            //             println!(" r {:?} ", i);
            //         } // send_result_to_parties(client_address, &res_to_client.to_string());
            //           // send_result_to_parties(p4_address, &res_to_p4.to_string());
            //           //
            //     }
            // }
        }
    }
}
fn main() {
    // let client_address = "127.0.0.1:8080"; // Assuming p2 is listening on port 8082

    // let p2_address = "127.0.0.1:8082"; // Assuming p2 is listening on port 8082
    // let p3_address = "127.0.0.1:8083"; // Assuming p2 is listening on port 8082
    let p4_address = "127.0.0.1:8084"; // Assuming p4 is listening on port 8084

    start_p4(p4_address);
    // let listener_p4 = TcpListener::bind(p4_address).unwrap();
    // let mut client_stream = TcpStream::connect(client_address).unwrap();

    // for stream in listener_p4.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             // Clone the stream for each thread
    //             let p2_stream = stream.try_clone().unwrap();
    //             let p3_stream = stream.try_clone().unwrap();

    //             // Spawn separate threads for P2 and P3
    //             let r2_handle = thread::spawn(move || handle_p2(p2_stream));

    //             let r3_handle = thread::spawn(move || handle_p3(p3_stream));

    //             // Wait for both threads to finish and get their results
    //             let r2 = r2_handle.join().unwrap();
    //             let r3 = r3_handle.join().unwrap();
    //             println!("recieve r2,r3 from p2 and p3 : {:?},{:?}", r2, r3);
    //             // Perform computation on r2 and r3
    //             let result = (r2 != 0 && r2 / r3 >= 0) as bool;

    //             let byte_value: u8 = if result { 1 } else { 0 };

    //             // Send the result back to the client
    //             client_stream.write(&[byte_value]).unwrap();

    //             // println!("Received r2 from P2: {}, r3 from P3: {}", r2, r3);
    //             println!("Result of computation: {}", result);
    //         }
    //         Err(e) => {
    //             eprintln!("Error: {}", e);
    //         }
    //     }
    // }
}
