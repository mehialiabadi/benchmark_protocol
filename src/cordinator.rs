use benchmark_protocol::table::{PartyCollection, Partyr, Table};
use mysql::prelude::*;
use rand::Rng;
use rusqlite::{Connection, Result, NO_PARAMS};
use serde::de::value::UsizeDeserializer;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::{option, thread};
// use mysql::{OptsBuilder, Pool};
use mysql::*;
use num::{iter, Signed};
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
fn p4_prepare(data: &Message) -> Option<Partyr> {
    match data {
        Message::Data(data) => {
            let r: Result<Partyr, serde_json::Error> = serde_json::from_str(data);
            match r {
                Ok(party) => return Some(party),
                _ => None,
            }
        }
        Message::NoData => None,
    }
}
// fn get_party(data: &Message) -> Partyr {
//     if let Some(res) = p4_prepare(&data) {
//         match res {
//             Partyr => {
//                 return Partyr;
//             }

//         }
//         else {return None;}
//     }
// }
// fn process_data(p2: Partyr) -> i8 {
//     let my_collection:PartyCollection {

//     }
//     return 1;
// }

fn process_data(parrty1: Option<Partyr>) -> i8 {
    return 1;
}
fn start_p4(server_address: &str) {
    let url = "mysql://root:123456789@localhost:3306/testdb";
    let pool = Pool::new(url).unwrap();
    // let mut row_id = 0;
    let listener = TcpListener::bind(server_address).expect("Failed to bind");

    let (sender1, receiver1) = channel();
    // let (sender2, receiver2) = channel();

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
            // println!("raw---{:?}", data_rec);
            // let part1: Partyr = p4_prepare(&data_rec);
            println!("data:{:?}", data_rec);
            // let option_party: Partyr = p4_prepare(&data_rec).unwrap();
            // let data2: Option<Partyr> = p4_prepare(&data_rec);
            // if let (Some(party1)) = (data1) {
            //     // Process the data pair
            //     let w = process_data(party1);
            //     // println!("wwww:{:?}", w);
            // } else {
            //     eprintln!("Failed to deserialize data");
            // }

            // let shared_data = Arc::new(Mutex::new(data_rec));
            // let shared_data_clone = shared_data.clone();

            // let t1 = thread::spawn(move || {
            // let mut data_iter = shared_data.lock().unwrap();

            // while let Some(data1) = data_iter.next() {
            // if let Some(data2) = data_iter.next() {
            // let party1: Option<Partyr> = p4_prepare(&data_iter);
            // let party2: Option<Partyr> = p4_prepare(&data2);

            // Process the data pair
            // process_data(party1);
            // } else {
            // Break if no more data
            // break;
            // }
            // }
            // after locked_user goes out of scope, mutex will be unlocked again,
            // but you can also explicitly unlock it with:
            // drop(locked_user);
            // });

            // Create a vector to hold the thread handles
            // let mut handles: Vec<_> = vec![];

            // Number of threads you want to process the data
            // let num_threads = 2;

            // for _ in 0..num_threads {
            //     // Clone the Arc to share among threads
            //     let shared_data = Arc::clone(&shared_data);

            // Spawn a new thread
            // let handle = thread::spawn(move || {
            // Lock the mutex to get exclusive access to the shared data
            // let mut data_iter: std::sync::MutexGuard<'_, Partyr> = shared_data.lock().unwrap();

            //     // Process the data in pairs
            //     while let Some(data1) = data_iter.into() {
            //         if let Some(data2) = data_iter.into() {
            //             // Deserialize the data into Party structs
            //             let party1: Partyr = serde_json::from_str(&data1).ok()?;
            //             let party2: Partyr = serde_json::from_str(&data2).ok()?;

            //             // Process the data pair
            //             process_data((party1, party2));
            //         } else {
            //             // Break if no more data
            //             break;
            //         }
            //     }
            // });

            // Keep track of the thread handles
            // handles.push(handle);
        }

        // Wait for all threads to finish
        // for handle in handles {
        //     handle.join().unwrap();
        // }
        // match option_party {
        //     Some(party) => {
        //         // Process the Party
        //         println!("Processing party: {:?}", party);
        //     }
        //     None => {
        //         // Handle the None case
        //         println!("Received None");
        //     }
        // }

        // let mut deserializer = Deserializer::from_reader(valu.as_bytes());

        // while let Ok(party1) = Partyr::deserialize(&mut deserializer) {
        //     if let Ok(party2) = Partyr::deserialize(&mut deserializer) {
        //         // Process the data pair
        //         println!("pair:{:?}{:?}", party1, party2);
        //         // process_data((party1, party2));
        //     } else {
        //         // Handle any remaining single item or odd-sized chunk
        //         eprintln!("Unexpected data format");
        //         break;
        //     }
        // }
        // let party_collection = PartyCollection {
        //     parties: [party1, party2],
        // };
        // let part1: get_party(&data_rec);

        // process_data(data_recieved);
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
