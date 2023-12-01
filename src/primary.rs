use benchmark_protocol::table::Table;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Table {
//     rows: [[i8; 8]; 2],
// }

// fn main() {
//     let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
//     // accept connections and process them, spawning a new thread for each one
//     println!("Server listening on port 8000");
//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 println!("New connection: {}", stream.peer_addr().unwrap());
//                 thread::spawn(move || {
//                     // connection succeeded
//                     handle_client(stream)
//                 });
//             }
//             Err(e) => {
//                 println!("Error: {}", e);
//                 /* connection failed */
//             }
//         }
//     }
//     // close the socket server
//     drop(listener);
// }

// fn handle_client(mut stream: TcpStream) {
//     let mut data = [0 as u8; 1024]; // using 50 byte buffer

//     while match stream.read(&mut data) {
//         Ok(size) => {
//             if size > 0 {
//                 println!("data {}", std::str::from_utf8(&data[0..size]).unwrap());
//             }

//             true
//         }
//         Err(_) => {
//             println!(
//                 "An error occurred, terminating connection with {}",
//                 stream.peer_addr().unwrap()
//             );
//             stream.shutdown(Shutdown::Both).unwrap();
//             false
//         }
//     } {}
// }

use mysql::prelude::*;
use rand::Rng;
// use rusqlite::{Connection, Result, NO_PARAMS};
// use std::hash::BuildHasher;
// use std::io::{BufWriter, Read, Write};
// use std::net::{SocketAddr, TcpListener, TcpStream};
// // use mysql::{OptsBuilder, Pool};
// use mysql::*;
// use std::thread;

pub struct LineItem {
    id: i32,
    column_value: i32,
}

// // Print the random table

// // Helper function to generate a random row

fn generate_random_table(distance: i8) -> Vec<Vec<i8>> {
    let mut rng = rand::thread_rng();
    let random_table = (0..2)
        .map(|_| (0..8).map(|_| rng.gen_range(1..100)).collect())
        .collect();
    return random_table;
}

pub fn generate_truth_table(number: i32, distance: i8) -> (Vec<Vec<i8>>, Vec<Vec<i8>>) {
    let binary_number: &str = &format!("{number:08b}");
    println!("binary:{:?}", binary_number);

    let mut p2_table = generate_random_table(distance);

    let mut p3_table = p2_table.clone();
    let my_variable: i8 = 1;

    for (index1, bit) in binary_number.chars().enumerate() {
        if bit == '0' {
            let row = p3_table.get_mut(1);

            if let Some(element) = row.expect("REASON").get_mut(index1) {
                *element = *element - my_variable;
                // *element = *element.wrapping_sub(my_variable);
            }
        }
        if bit == '1' {
            let row = p3_table.get_mut(0);
            if let Some(element) = row.expect("REASON").get_mut(index1) {
                *element = *element - my_variable;
            }
        }
    }
    return (p2_table, p3_table);
}
// fn raw_value(
//     pool: &Pool,
//     user_number: i32,
//     column_name: &str,
// ) -> Result<Vec<LineItem>, mysql::Error> {
//     let mut conn = pool.get_conn().unwrap();

//     //connect to database and then subtract the value
//     //     let qu:String="SELECT  id ,".to_owned()+&column_name;
//     //   let query = qu+"from p1test_share";
//     let query = "SELECT id, order_key from p1test_share";
//     let mut stmt = conn.query_map(query, |(id, column_value)| LineItem { id, column_value });

//     return stmt;
// }

fn process_row(id: i32, order_key: i32, user_number: i32) -> (i32, i32) {
    return (id, (order_key - user_number.abs()));
}

fn send_shared_truthtable_to_parties(address: &str, table: &Table) {
    let mut stream = TcpStream::connect(address).unwrap();

    // stream
    //     .write_all(&id.to_be_bytes())
    //     .expect("Failed to write to stream row id");

    // let json_string = serde_json::to_string(&table).unwrap();
    // let json_row = serde_json::to_string(&id).unwrap();

    // let serialized_table: Vec<u8> = serialize(&table).expect("Failed to serialize table");
    // let json_string: String = serde_json::to_string(&table)?;

    let bytes = serde_json::to_string(&table).unwrap();

    stream
        .write_all(&bytes.as_bytes())
        .expect("Failed to write table to stream");
    // stream
    //     .write_all(&json_row.as_bytes())
    //     .expect("Failed to write row id to stream");
    // writer.flush().expect("Failed to flush stream");
}

fn start_p1(server_address: &str) {
    let listener = TcpListener::bind(server_address).expect("Failed to bind");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            // Spawn a new thread to handle each client
            thread::spawn(move || handle_client_connection(stream));
        }
    }
}

fn handle_client_connection(mut stream: TcpStream) {
    // let mut buffer = [0; 4]; // Adjust buffer size as needed
    // let url = "mysql://root:123456789@localhost:3306/testdb";
    // let pool = Pool::new(url).unwrap();

    // let distance = 8;
    // let column_name = "order_key";
    let p2_address = "127.0.0.1:9092";
    // let p3_address = "127.0.0.1:8083";

    // let mut server_identifier = [0; 2];

    // let mut buffer_integer = [0; 4]; // 4 bytes for 32 bits
    // stream
    //     .read_exact(&mut buffer_integer)
    //     .expect("Failed to read table data");
    // let user_number = i32::from_be_bytes(buffer);

    let mut buffer = String::new();
    if let Ok(bytes_read) = stream.take(1024).read_to_string(&mut buffer) {
        if bytes_read > 0 {
            let smt = generate_random_table(8);
            let table: Table = Table {
                row_id: 1,
                rows: smt,
            };

            send_shared_truthtable_to_parties(p2_address, &table);
        }
    }

    // let smt = raw_value(&pool, user_number, column_name);
    let smt = generate_random_table(8);
    let table: Table = Table {
        row_id: 1,
        rows: smt,
    };

    send_shared_truthtable_to_parties(p2_address, &table);

    // for row in smt.iter().flatten() {
    //     let new_value = (row.column_value.wrapping_sub(user_number)).abs();

    //     let (p2_share_table, p3_share_table) = generate_truth_table(new_value, distance);

    //     println!(
    //         "p2 truth table {:?} p3share truth table:{:?} for row id:{:?}, and new value {:?}",
    //         p2_share_table, p3_share_table, row.id, new_value
    //     );
    //     send_shared_truthtable_to_parties(p2_address, p2_share_table, row.id);
    //     send_shared_truthtable_to_parties(p3_address, p3_share_table, row.id);
    // }
}

fn main() {
    // let url = "mysql://root:123456789@localhost:3306/testdb";
    // let pool = Pool::new(url).unwrap();

    let table_name = "p1test_share";
    // let table_name="line_item_1m";

    // let column_name: &str="order_key";
    // let distance: i32=8;
    let p1_address = "127.0.0.1:8081";

    start_p1(p1_address);
}