use mysql::prelude::*;
use num::complex::ComplexFloat;
use rand::Rng;
use rusqlite::{Connection, Result, NO_PARAMS};
use serde_json::json;
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use std::ops::ShlAssign;
use std::option;
use std::ptr::null;
// use mysql::{OptsBuilder, Pool};
use bincode::{deserialize, serialize};
use byteorder::{LittleEndian, ReadBytesExt};
use mysql::*;
use num::Signed;
use serde::{Deserialize, Serialize};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::thread;

use benchmark_protocol::table::{Partyr, Table};

enum Payload {
    Int(usize),
    Table(Table),
}

#[derive(Debug)]
enum Message {
    Data(String),
    NoData,
}
pub struct LineItem {
    id: i32,
    column_value: i32,
}

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

// struct Table {
//     rows: [[i8; 8]; 2],
// }
// s// enum MessageType {
//     Number(i32),
//     Table(Vec<Vec<i8>>),
// }

// fn receive_message(stream: &mut TcpStream) -> io::Result<MessageType> {
//     let mut buffer = String::new();
//     stream.read_to_string(&mut buffer)?;

//     serde_json::from_str::<MessageType>(&buffer)
//         .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
// }

pub fn process_table(
    pool: &Pool,
    truth_table: &mut Vec<Vec<i8>>,
    row_id: &usize,
    shared: usize,
) -> (i32, i32) {
    let column_name = "order_key";

    let mut conn = pool.get_conn().unwrap();
    // let query = "select id, order_key from p1test_share where id=:id";
    // let qu1: String="SELECT  id ,".to_owned()+&column_name;
    // let mut  qu2:&str=qu1.to_owned().as_str();
    // let mut qu3= ("from p1test_share where id=".to_owned()+&row_id.to_string()).to_owned().as_str();
    // let mut query=qu2.to_owned()+qu3;
    // let query = qu2+"from p1test_share where id=".to_owned()+&row_id.to_string();
    // let query = qu+"from p1test_share where";

    let query = format!("SELECT id, order_key FROM line_item_1m_testp3  WHERE id = :id",);

    // // Execute the query with parameters
    // let result: Option<(i32, i32)> = conn.query_first(
    //     &query,
    //     params! {
    //         "id" => row_id,
    //     },
    // )?;

    let result: Option<(i32, i32)> = conn
        .exec_first(&query, params! { "id" => row_id })
        .map(|row| row.map(|(id, column_value)| (id, column_value)))
        .expect("Failed to execute query");

    if let Some((id, column_value)) = result {
        // println!("id: {}, value: {}", id, column_value);
        let bin: String = format!(
            "{:08b}",
            (column_value.wrapping_sub(shared.try_into().unwrap())).abs()
        );

        let (s2, r2) = p2_computaion(truth_table, &bin);
        return (s2, r2);
    } else {
        // println!("No matching row found");
        return (0, 0);
    }
    fn p2_computaion(truth_table: &mut Vec<Vec<i8>>, binary_p2number: &str) -> (i32, i32) {
        #![feature(int_roundings)]

        let mut capital_s2: i32 = 0;
        let mut small_s2: i32;
        let mut r2 = 0;
        // println!("binary:{:?}", binary_p2number);
        let modulue = 32;
        for (index, character) in binary_p2number.chars().enumerate() {
            if character == '0' {
                capital_s2 += truth_table[1][index] as i32;
                // println!("element:{:?}", truth_table[1][index]);
            } else {
                capital_s2 += truth_table[0][index] as i32;
                // println!("element:{:?}", truth_table[0][index]);
            }
        }
        small_s2 = (capital_s2 / 8) as i32;
        r2 = capital_s2 % 8;
        // println!("capital S2:{:?}", capital_s2);
        return ((small_s2 as f32).floor() as i32, r2 as i32);
    }
}

fn p2_prepare(data: &Message) -> Option<Payload> {
    match data {
        Message::Data(data) => {
            if let Ok(i) = data.parse::<usize>() {
                return Some(Payload::Int(i));
            }

            let r: Result<Table, serde_json::Error> = serde_json::from_str(data);
            match r {
                Ok(table) => Some(Payload::Table(table)),
                _ => None,
            }
        }
        Message::NoData => None,
    }
    // // let nn = data.last_char();
    // // let row_id = nn.unwrap() as i32;
    // let arrays_part = &data.to_string();
    // let x2 = arrays_part.trim_start_matches('[').trim_end_matches(']');
    // let arrays: Vec<&str> = x2.split("],[").collect();

    // // Convert each array string to Vec<i8>
    // let result: Vec<Vec<i8>> = arrays
    //     .iter()
    //     .map(|array_str| {
    //         array_str
    //             .split(',')
    //             .filter_map(|num_str| num_str.parse::<i8>().ok())
    //             .collect()
    //     })
    //     .collect();
    // return result;
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
// fn pt_computation(arr: Vec<Vec<i8>>, row_id: i32, share_value: i32, pool: &Pool) -> (i8, i8, i32) {
//     let mut capital_s2: i32 = 0;
//     let mut small_s2: i8 = 0;
//     let mut r2 = 0;

//             for (index, character) in new_value.chars().enumerate() {
//                 if character == '0' {
//                     let row = arr.get_mut(1);

//                     if let Some(element) = row.expect("REASON").get_mut(index) {
//                         capital_s2 += *element as i32;
//                     }
//                 } else {
//                     let row = arr.get_mut(0);

//                     if let Some(element) = row.expect("REASON").get_mut(index) {
//                         capital_s2 += *element as i32;
//                     }
//                 }
//             }
//             small_s2 = (capital_s2 / 8) as i8;
//             r2 = capital_s2 % 8;
//             return ((small_s2 as f32).floor() as i8, r2 as i8, row_id);

//     } else {
//         return (0, 0, row_id);
//     }
// }

///
/// fn read_table_from_stream<T: Read>(stream: &mut T) -> Result<Table> {
// Read bytes from the stream
/// ///

fn handle_p1_connection(mut stream: TcpStream, sender: Sender<String>) {
    // println!("data from p1");
    // let mut server_identifier = [0; 2];
    // stream
    //     .read_exact(&mut server_identifier)
    //     .expect("Failed to read from p1 identifier");
    // let server_identifier: &str = &String::from_utf8_lossy(&server_identifier).to_string();

    // match &server_identifier {
    //     &"p1" => {
    // let mut buffer_row_id = [0; 4];
    // stream
    //     .read_exact(&mut buffer_row_id)
    //     .expect("Failed to read from p1 identifier");
    // let row_id = i32::from_be_bytes(buffer_row_id);
    // println!("row id:{:?}", row_id);
    // let mut buffer_table = [0, 16];

    // stream
    //     .read_exact(&mut buffer_table)
    //     .expect("Failed to read from stream");
    let mut buffer = String::new();

    stream.read_to_string(&mut buffer);
    if buffer.trim().is_empty() {
        println!("Received an empty JSON string");
    }
    // if let Ok(table) = serde_json::from_str(&buffer_table) {
    //     sender
    //         .send(table)
    //         .expect("Failed to send result to main thread");
    // }
    // let mut buff_integer = [0; 4];
    // stream.read_exact(&mut buff_integer).expect("...");
    // let row_id = i32::from_be_bytes(buff_integer);
    // println!("row id{:?}", row_id);
    //  sender.send(row_id).expect("Failed to send result to main thread");

    //  println!("table:{:?}",&deserialized_table);
    match serde_json::from_str::<String>(&buffer) {
        Ok(random_table) => {
            sender
                .send(random_table)
                .expect("Failed to send result to main thread");
        }

        Err(err) => {
            eprintln!("Failed to deserialize JSON into Random Table: {}", err);
        }
    }
}

fn handle_client_connection(mut stream: TcpStream, sender: Sender<String>) {
    let mut integer_buffer = [0; 4]; // Assuming a 32-bit integer
    stream
        .read_exact(&mut integer_buffer)
        .expect("Failed to read from  cleint");

    let integer_value = i32::from_be_bytes(integer_buffer);
    sender
        .send(integer_value.to_string())
        .expect("Failed to send result to main thread");
}
fn set_share(inp: i32) -> i32 {
    return inp;
}
fn send_result_to_parties(server_addr: &str, data: &String) {
    // let serialized_result = bincode::serialize(&result).unwrap();

    let mut stream = TcpStream::connect(server_addr).unwrap();
    // let json_user = serde_json::to_string(&data).unwrap();
    let bytes = serde_json::to_string(&data).unwrap();

    stream
        .write_all(&bytes.as_bytes())
        .expect("Failed to write table to stream");
}

fn start_p2(server_address: &str) {
    let url = "mysql://root:123456789@localhost:3306/benchdb";
    let pool = Pool::new(url).unwrap();
    // let mut row_id = 0;
    let listener = TcpListener::bind(server_address).expect("Failed to bind");

    let (sender1, receiver1) = channel();
    // let (sender2, receiver2) = channel();
    let client_address = "127.0.0.1:8080"; // Assuming p2 is listening on port 8082

    let p4_address = "127.0.0.1:8084"; // Assuming p4 is listening on port 8084
    let mut cleint_share = 0;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            // Clone the stream for each thread
            let sender1_clone = sender1.clone();
            // let rec2_clone = sender2.clone();
            // let (thread_sender, thread_receiver) = channel();

            // Spawn threads to handle each server connection
            let handle1 = thread::spawn(|| handle_client(stream, sender1_clone));
            let mut data_rec = receiver1.recv().expect("Failed handle1 thread 1");

            if let Some(res) = p2_prepare(&data_rec) {
                match res {
                    Payload::Int(i) => {
                        // println!("integer received {}", i);
                        cleint_share = i;
                    }
                    Payload::Table(mut t) => {
                        // println!("Table received {:?}", t);
                        //create own table and send the result to p4 and client
                        let (s2, r2) = process_table(&pool, &mut t.rows, &t.row_id, cleint_share);
                        // println!("s2:{:?}, r2:{:?}, row:{:?}", s2, r2, t.row_id);
                        //send result to p4 and client

                        let res_to_p4: Partyr = Partyr {
                            row_id: (t.row_id),
                            comput: (r2),
                        };
                        let res_to_client: Partyr = Partyr {
                            row_id: (t.row_id),
                            comput: (s2),
                        };

                        // send_result_to_parties(client_address, &res_to_client.to_string());
                        send_result_to_parties(p4_address, &res_to_p4.to_string());
                        //
                    }
                }
            }

            // println!(
            //     "check if the first recod or not:{:?}, ---len {:?}",
            //     res,
            //     res.len()
            // );
            // if (res.len() == 1) {
            //     share_value = res[0][0];
            //     println!("shared{:?}", share_value);
            // } else {
            //     println!("raw data:{:?}", user_num);
            //     let (mut arr, row_id) = p2_prepar_enext(&user_num);
            //     println!("arrr:{:?}--row{:?}, shared{:?}", arr, &row_id, share_value);
            //     // let (s2, r2, row_num) = p2_process(&pool, &mut arr, &row_id, share_value);
            //     // let stream_p4 = TcpStream::connect(p4_address).unwrap();
            //     // let mut stream_client = TcpStream::connect(client_address).unwrap();
            //     // send_result_to_parties(&stream_client, s2, row_num);
            //     // send_result_to_parties(&stream_p4, r2, row_num);
            // }

            // if (flag == true) {
            //     let nn = user_num.last_char();
            //     let row_id = nn.unwrap();
            //     flag = false;
            // }

            // Use the last_char method to get the last character
            // if let Some(last_char) = user_num.last_char() {
            //     println!("las:{:?}", last_char);
            // } else {
            //     println!("String is empty");
            // }

            // let parsed_number = user_num.parse::<i32>().unwrap_or_else(|err| {
            //     eprintln!("Failed to parse as i32: {}", err);
            //     // You can handle the error case here, for example, returning a default value
            //     0
            // });
            // println!("user_num:{:?}", user_num.);

            //             println!("this is the integer value from client:{:?}",user_num);
            // let handle2 = thread::spawn(|| handle_client(p1_stream, rec2_clone));
            // let mut tab = receiver2
            //     .recv()
            //     .expect("Failed to receive table from thread 1");
            // println!("table:{:?}", tab);
            // p2_computaion(&mut tab,&user_num.to_string() );
            // handle1.join().unwrap();
            // handle2.join().unwrap();
        }
    }
    // match receiver1.recv().unwrap() {
    //     Message::Data(data) => println!("Received data from Server 1: {}", data),
    //     Message::NoData => println!("No data received from Server 1"),
    // }

    // match receiver2.recv().unwrap() {
    //     Message::Data(data) => println!("Received data from Server 2: {}", data),
    //     Message::NoData => println!("No data received from Server 2"),
    // }
}

// fn handle_client(mut stream: TcpStream) {
//     let mut buffer = [0; 4]; // Adjust buffer size as needed

//     stream.read_exact(&mut buffer).unwrap();

//     let integer_value = i32::from_be_bytes(buffer);
//     println!("p2 received in integer format: {:?}", integer_value);

//     let mut buffer_table = [0; 16]; // 4 bytes for each element in a 2x8 table
//     stream.read_exact(&mut buffer_table).unwrap();
//     let mut received_table: Table = bincode::deserialize(&buffer_table).unwrap();
//     println!("table p2:{:?}", received_table);
// }

fn main() {
    // let url = "mysql://root@localhost:3306/testdb";
    // let pool = Pool::new(url).unwrap();
    // let table_name="line_item_1m";
    // let table_name = "p2share_test";
    // let column_name = "order_key";
    // let distance = 32;

    let p3_address = "127.0.0.1:8083"; // Assuming p2 is listening on port 8082

    // let listener = TcpListener::bind(p2_address).unwrap();
    // let  stream_p4 = TcpStream::connect(p4_address).unwrap();
    // let mut stream_client = TcpStream::connect(client_address).unwrap();

    // let mut stream_p4_clone = stream_p4.try_clone().unwrap();
    start_p2(p3_address);
    //     for stream in listener.incoming() {
    //         match stream {
    //             Ok(mut stream) => {
    //                 let mut buffer = [0; 4]; // Assuming 4 bytes for a 32-bit integer
    //                 stream.read_exact(&mut buffer);
    //                 let user_number = i32::from_be_bytes(buffer);
    //                 println!("p2 receives share from client for keyword to seach {:?}",user_number);

    //                 let mut buffer = [0; 4 * 32 * 2]; // 4 bytes for each element in a 2x8 table
    //                 stream.read_exact(&mut buffer).unwrap();
    //                 let  mut received_table: Vec<Vec<i32>> = bincode::deserialize(&buffer).unwrap();

    //                 let row_id:i32 = bincode::deserialize(&buffer).unwrap();
    //                 println!("p2 received truth table from p1 for keyword to seach for row id:{:?} {:?}",received_table,row_id);

    // //                 // Process the received table
    // //                 let (s2,r2) = p2_process(&pool,&mut received_table,row_id);
    // // println!("p2 computes s2:{:?} , r2:{:?} ",s2,r2);
    // //                 // Send the result to p4
    // //                 send_result_to_parties(&stream_p4_clone, r2,row_id);
    // //                 send_result_to_parties(&stream_client, s2,row_id);

    //             }
    //             Err(e) => {
    //                 eprintln!("Error: {}", e);
    //             }
    //         }
    //     }
}
