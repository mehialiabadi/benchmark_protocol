use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::option;
use rand::Rng;
use rusqlite::{Connection, Result, NO_PARAMS};
use mysql::prelude::*;
// use mysql::{OptsBuilder, Pool};
use mysql::*;
use num::Signed;
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use std::thread;
use byteorder::{LittleEndian, ReadBytesExt};
use bincode::{serialize, deserialize};

// pub struct LineItem {
//     id:i32,
//     column_value: i32,
  
// }
#[derive(Serialize, Deserialize, Debug)]
struct Table {
    rows: Vec<Vec<u8>>,
}
pub fn p2_process(pool:&Pool,truth_table: &mut Vec<Vec<u8>>,row_id:i32)->(i8,i8){
    let column_name="order_key";

    let mut conn = pool.get_conn().unwrap();
    let query="select id, order_key from p1test_share where id=id";
    // let qu1: String="SELECT  id ,".to_owned()+&column_name;
// let mut  qu2:&str=qu1.to_owned().as_str();
// let mut qu3= ("from p1test_share where id=".to_owned()+&row_id.to_string()).to_owned().as_str();
// let mut query=qu2.to_owned()+qu3;
    // let query = qu2+"from p1test_share where id=".to_owned()+&row_id.to_string();
    // let query = qu+"from p1test_share where";

    let result: Option<(i32, i32)> = conn.exec_first(query, params! { "id" => row_id })
    .map(|row| row.map(|(id, column_value)| (id, column_value)))
    .expect("Failed to execute query");


    if let Some((id, column_value)) = result
     {
        println!("ID: {}, Name: {}", id, column_value);
        // let bin=format!("l as binary is: {column_value:#032b}");
        let bin: String = format!("{:08b}", column_value);

        let (s2,r2)= p2_computaion(truth_table,&bin);
        return  (s2,r2);
    } else {
        println!("No matching row found");
        return (0,0);
    }
  }
     

     fn p2_computaion(truth_table: &mut Vec<Vec<u8>>, binary_p2number: &str) -> (i8,i8){
        #![feature(int_roundings)]
    
    let mut capital_s2:i32=0;
        let mut small_s2: i8=0;
        let mut r2=0;
    
                for (index, character) in binary_p2number.chars().enumerate() {
                    if character == '0' {
                
                    capital_s2+=truth_table[1][index] as i32;
        
                    
                    }
        
                    else{
                  
                    capital_s2+=truth_table[0][index] as i32;
                }
            }
            small_s2=(capital_s2/8) as i8;
            r2=capital_s2 % 8;
                return  ((small_s2 as f32).floor() as i8,r2 as i8);
        }
fn send_result_to_parties(mut stream_p: &TcpStream, result: i8,row_id:i32) {
    let serialized_result = bincode::serialize(&result).unwrap();
    stream_p.write_all(&serialized_result).unwrap();
    stream_p.write_all(&row_id.to_be_bytes()).unwrap();

}
fn handle_p1_connection(mut stream: TcpStream) {
    // println!("data from p1");
    let mut server_identifier = [0; 2];
    stream.read_exact(&mut server_identifier).expect("Failed to read from p1 identifier");
     let server_identifier:&str= &String::from_utf8_lossy(&server_identifier).to_string();

    match &server_identifier {
        &"p1" => {
            // let mut buffer_new = Vec::new();

            //  let mut buffer_table = [0; 8]; // 1 bytes for each element in a 2x8 table
            // stream.read_exact(&mut buffer_table).expect("Failed to read table data");
            // let table_data = String::from_utf8_lossy(&buffer_table);

            // // Data is an integer from Server 1
            // let table: serde_json::Value = serde_json::from_str(&table_data.trim()).expect("Failed to deserialize table");
//             stream.read_exact(&mut buffer_table).unwrap();
//              let  mut received_table: Vec<Vec<u8>> = bincode::deserialize(&buffer_table).unwrap();
// println!("table:{:?}",received_table);
let mut buffer_table = Vec::new();
    stream.read_to_end(&mut buffer_table).expect("Failed to read from stream");

    // Deserialize the received binary data into a Table using bincode
    let deserialized_table: Table = deserialize(&buffer_table).expect("Failed to deserialize table");
println!("table:{:?}",deserialized_table);
            // Process the deserialized table (replace this with your actual processing logic)
        }
        _ => {
            println!("Unexpected server identifier expect p1: {}", server_identifier);
            // Handle the unexpected identifier
        }
    }
    // stream.read_exact(&mut buffer_table).expect("Failed to read from  p1");;
    // // let  mut received_table: Vec<Vec<i32>> = bincode::deserialize(&buffer_table).unwrap();
    // let deserialized_table:Vec<Vec<i32>> = serde_json::from_slice(&buffer_table).expect("Failed to deserialize table");

    // println!("table :{:?}",deserialized_table);

    // match stream.read_exact(&mut buffer_table) {
    //     Ok(bytes_read) => {
    //         // if bytes_read == 0 {
    //         //     println!("Received an empty buffer, the connection may have been closed.");
    //         //     return;
    //         // }

    //         // Deserialize the JSON table
    //        let deserialized_table:Vec<Vec<i32>> = serde_json::from_slice(&buffer_table).expect("Failed to deserialize table");


        //     // Process the deserialized table (replace this with your actual processing logic)
        //     println!("Received table: {:?}", deserialized_table);
        // }
        // Err(err) => {
        //     eprintln!("Error reading from the server: {}", err);
        //     // Handle the error appropriately
        // }
    }

fn handle_client_connection(mut stream: TcpStream) {

    let mut server_identifier = [0; 2];
    stream.read_exact(&mut server_identifier).expect("Failed to read server identifier");
     let server_identifier:&str= &String::from_utf8_lossy(&server_identifier).to_string();

    match &server_identifier {
        &"c1" => {
            
    let mut integer_buffer = [0; 4]; // Assuming a 32-bit integer
    stream.read_exact(&mut integer_buffer).expect("Failed to read from  cleint");

    let integer_value= i32::from_be_bytes(integer_buffer);
    println!("p2 received integer from client: {:?}", integer_value);
        }
        _ => {
            println!("Unexpected server identifier expect c1: {}", server_identifier);
            // Handle the unexpected identifier
        }
    }


}


fn start_p2(server_address: &str) {
    let listener = TcpListener::bind(server_address).expect("Failed to bind");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            // Clone the stream for each thread
            let client_stream = stream.try_clone().expect("Failed to clone stream for cleint");
            let p2_stream = stream.try_clone().expect("Failed to clone stream for  p1");

            // Spawn threads to handle each server connection
            thread::spawn(|| handle_client_connection(client_stream));
            thread::spawn(|| handle_p1_connection(p2_stream));
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 4]; // Adjust buffer size as needed

    // match stream.read(&mut buffer) {
    //     Ok(bytes_read) => {
            // let received_data = &buffer[..bytes_read];
            // println!("p2 2 received: {:?}", received_data);
            stream.read_exact(&mut buffer).unwrap();

            let integer_value= i32::from_be_bytes(buffer);
            println!("p2 received in integer format: {:?}", integer_value);



                let mut buffer_table = [0; 16]; // 4 bytes for each element in a 2x8 table
                stream.read_exact(&mut buffer_table).unwrap();
                let  mut received_table: Vec<Vec<i32>> = bincode::deserialize(&buffer_table).unwrap();
                println!("table p2:{:?}",received_table);
                
            // let mut buffer = [0; 4 * 32 * 2]; // 4 bytes for each element in a 2x8 table
            //                 stream.read_exact(&mut buffer).unwrap();
            //                 let  mut received_table: Vec<Vec<i32>> = bincode::deserialize(&buffer).unwrap();

            // // Simulate processing the data (you can replace this with your actual logic)
            // let processed_data = process_data(received_data);

            // // Send a response back to the client
            // if let Err(err) = stream.write_all(&processed_data) {
            //     eprintln!("Error sending response: {}", err);
            // }

        }
        // Err(err) => {
        //     eprintln!("Error reading from client: {}", err);
        // }
//     }
// }
fn main() {
    let url = "mysql://root:123456789@localhost:3306/testdb";
    let pool = Pool::new(url).unwrap();
    // let table_name="line_item_1m";
    let table_name="p2share_test";

    let column_name="order_key";
    let distance=32;

    let client_address = "127.0.0.1:8080"; // Assuming p2 is listening on port 8082

    let p2_address = "127.0.0.1:9092"; // Assuming p2 is listening on port 8082
    let p4_address = "127.0.0.1:8084"; // Assuming p4 is listening on port 8084

    // let listener = TcpListener::bind(p2_address).unwrap();
    // let  stream_p4 = TcpStream::connect(p4_address).unwrap();
    // let mut stream_client = TcpStream::connect(client_address).unwrap();

    // let mut stream_p4_clone = stream_p4.try_clone().unwrap();
start_p2(p2_address);
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
     
