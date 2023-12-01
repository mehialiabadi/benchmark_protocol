use std::net::TcpStream;
use rand::Rng;
use std::io::Read;
use std::io::{self, Write};
use std::net::TcpListener;
use std::time::{Duration, Instant};
use std::thread;


fn generate_additive_shares(user_number:i32) -> (i32,i32 ){
    let mut rng = rand::thread_rng();
    let share1=rng.gen_range(1..=user_number);
    let share2 = user_number-share1;
    return (share1,share2);
}

fn send_number_to_parties(mut stream: TcpStream, number: i32) {
    stream.write_all(&number.to_be_bytes()).unwrap();
    // stream.flush();
    println!("write the number to {:?}",number);


}
fn handle_p2(mut stream: TcpStream) -> i8 {
    let mut s2_bytes = [0; 1];

    // Receive s2 from P2
    stream.read_exact(&mut s2_bytes).unwrap();
    let s2 = i8::from_ne_bytes(s2_bytes);
    s2
}

fn handle_p3(mut stream: TcpStream) -> i8 {
    let mut r3_bytes = [0; 1];

    // Receive s3 from P3
    stream.read_exact(&mut r3_bytes).unwrap();
    let s3 = i8::from_ne_bytes(r3_bytes);
    s3
}
fn handle_p4(mut stream: TcpStream) -> i8 {
    let mut r_bytes = [0; 1];

    // Receive s3 from P3
    stream.read_exact(&mut r_bytes).unwrap();
    let r = i8::from_ne_bytes(r_bytes);
    r
}

fn write_to_server(server_address: &str, data: i32) -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect(server_address)?;

    // Send data to the server
    stream.write_all(&data.to_be_bytes())?;
    println!("Client sent to {}: {}", server_address, data);

    // Read and print the response from the server (optional)
    let mut response = [0; 1024];
    let bytes_read = stream.read(&mut response)?;
    println!("Client received from {}: {:?}", server_address, &response[..bytes_read]);

    Ok(())
}

fn send_data_to_server(server_address: &str, data: &i32) -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect(server_address)?;
    stream.write_all(b"c1").expect("Failed to send server identifier");

    // Send data to the server
    stream.write_all(&data.to_be_bytes())?;
        stream.flush();

    println!("Client sent to {}: {:?}", server_address, &data.to_be_bytes());

    // Read and print the response from the server (optional)
    // let mut response = [0; 1024];
    // let bytes_read = stream.read(&mut response)?;
    // println!("Client received from {}: {:?}", server_address, &response[..bytes_read]);

    Ok(())
}



fn main() {

    // let table_name="line_item_1m";
    // let table_name="user";

    let client_address="127.0.0.1:8080";
    let p1_address = "127.0.0.1:8081"; 
    let p2_address = "127.0.0.1:9092"; 
    let p3_address = "127.0.0.1:8083";  
    let listener_client = TcpListener::bind(client_address).unwrap();
    println!("client is listening  on port 8080");

    let column_name="order_key";

    println!("Enter a number to search:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let user_number: i32 = input.trim().parse().expect("Invalid number entered");

    let start_time = Instant::now();

    // Generate the additive shares and send them to p1 and p2 and p3
    let (p1_share, p23_share) = generate_additive_shares(user_number);
    println!("{:?},{:?}",&p1_share,&p23_share);
   
    let handle1 = thread::spawn(move || send_data_to_server(p1_address, &p1_share));
    let handle2 = thread::spawn(move || send_data_to_server(p2_address,&p23_share));
    let handle3 = thread::spawn(move || send_data_to_server(p3_address,&p23_share));
  
 

    // Wait for three threads to finish
    handle1.join().unwrap().expect("Error in thread 1");
    handle2.join().unwrap().expect("Error in thread 1");
    handle3.join().unwrap().expect("Error in thread 1");
    //////
    // println!("if data send to clients");

    //recieve the protocol's result from p2,p3 ,p4 

//     for stream in listener_client.incoming() {
//          match stream {
//             Ok(stream) => {
//         let p4_stream = stream.try_clone().unwrap();
//         let p2_stream = stream.try_clone().unwrap();
//         let p3_stream = stream.try_clone().unwrap();


//         // Spawn separate threads for P2 and P3 and P4
//         let s2_handle = thread::spawn(move || {
//             handle_p2(p2_stream)
//         });

//         let s3_handle = thread::spawn(move || {
//             handle_p3(p3_stream)
//         });
//         let r_handle = thread::spawn(move || {
//             handle_p3(p4_stream)
//         });

//         // Wait for three threads to finish and get their results
//         let s2 = s2_handle.join().unwrap();
//         let s3 = s3_handle.join().unwrap();
//         let r = r_handle.join().unwrap();
//     }
//         Err(e) => {
//             eprintln!("Error: {}", e);
//         }
//     }
// }
let elapsed_time = start_time.elapsed();
//  println!("select query time complexity for- {:?} and table - {:?}:::{:?} is ",column_name,table_name,elapsed_time );


}

        

    

