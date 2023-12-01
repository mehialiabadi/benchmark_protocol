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
use std::sync::mpsc::{channel, Sender};



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
        println!("id: {}, value: {}", id, column_value);
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
fn handle_p1_connection(mut stream: TcpStream,sender: Sender<Table>) {
    // println!("data from p1");
    let mut server_identifier = [0; 2];
    stream.read_exact(&mut server_identifier).expect("Failed to read from p1 identifier");
     let server_identifier:&str= &String::from_utf8_lossy(&server_identifier).to_string();

    match &server_identifier {
        &"p1" => {
let mut buffer_table = Vec::new();
    stream.read_to_end(&mut buffer_table).expect("Failed to read from stream");

   
 let deserialized_table: Table = deserialize(&buffer_table).expect("Failed to deserialize table");
 sender.send(deserialized_table).expect("Failed to send result to main thread");

//  println!("table:{:?}",&deserialized_table);
        }
        _ => {
            println!("Unexpected server identifier expect p1: {}", server_identifier);
        }
    }

    
    }

fn handle_client_connection(mut stream: TcpStream,sender: Sender<i32>) {

    let mut server_identifier = [0; 2];
    stream.read_exact(&mut server_identifier).expect("Failed to read server identifier");
     let server_identifier:&str= &String::from_utf8_lossy(&server_identifier).to_string();

    match &server_identifier {
        &"c1" => {
            
    let mut integer_buffer = [0; 4]; // Assuming a 32-bit integer
    stream.read_exact(&mut integer_buffer).expect("Failed to read from  cleint");

    let integer_value= i32::from_be_bytes(integer_buffer);
    sender.send(integer_value).expect("Failed to send result to main thread");

    println!("p2 received integer from client: {:?}", integer_value);
        }
        _ => {
            println!("Unexpected server identifier expect c1: {}", server_identifier);
        }
    }


}


fn start_p2(server_address: &str) {
    let listener = TcpListener::bind(server_address).expect("Failed to bind");
   
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            // Clone the stream for each thread
            let client_stream = stream.try_clone().expect("Failed to clone stream for cleint");
            let p1_stream = stream.try_clone().expect("Failed to clone stream for  p1");
            // let (thread_sender, thread_receiver) = channel();
            let (sender1, receiver1) = channel();
            let (sender2, receiver2) = channel();
            // Spawn threads to handle each server connection
         let handle1=   thread::spawn(|| handle_client_connection(client_stream,sender1));
         let res1 = receiver1.recv().expect("Failed to receive integer from thread 1");


         let handle2=   thread::spawn(|| handle_p1_connection(p1_stream,sender2));
         let res2 = receiver2.recv().expect("Failed to receive integer from thread 1");

       
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 4]; // Adjust buffer size as needed

    
            stream.read_exact(&mut buffer).unwrap();

            let integer_value= i32::from_be_bytes(buffer);
            println!("p2 received in integer format: {:?}", integer_value);



                let mut buffer_table = [0; 16]; // 4 bytes for each element in a 2x8 table
                stream.read_exact(&mut buffer_table).unwrap();
                let  mut received_table: Vec<Vec<i32>> = bincode::deserialize(&buffer_table).unwrap();
                println!("table p2:{:?}",received_table);
                
            

        }
      
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
     
