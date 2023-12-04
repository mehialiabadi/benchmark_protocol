use benchmark_protocol::table::ClientShare;
use rand::Rng;
use std::io::Read;
use std::io::{self, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::{Duration, Instant};

fn generate_additive_shares(user_number: i32) -> (i32, i32) {
    let mut rng = rand::thread_rng();
    let share1 = rng.gen_range(1..=user_number);
    let share2 = user_number - share1;
    return (share1, share2);
}

fn send_number_to_parties(mut stream: TcpStream, number: i32) {
    stream.write_all(&number.to_be_bytes()).unwrap();
    // stream.flush();
    println!("write the number to {:?}", number);
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

fn send_data_to_p123(server_address: &str, data: ClientShare) -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect(server_address)?;
    let bytes = serde_json::to_string(&data).unwrap();
    println!("size:{:?}", bytes.as_bytes());
    stream
        .write_all(bytes.as_bytes())
        .expect("Failed to write table to stream");
    // stream.write_all(b"\n");

    Ok(())
}
fn send_data_to_p2(server_address: &str, data: String) -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect(server_address)?;

    stream
        .write_all(&data.as_bytes())
        .expect("Failed to write table to stream");

    Ok(())
}
fn handle_client_connection(mut stream: TcpStream) {}
fn main() {
    let client_address = "127.0.0.1:8080";
    let p1_address = "127.0.0.1:8081";
    let p2_address = "127.0.0.1:8082";
    let p3_address = "127.0.0.1:8083";
    let listener_client = TcpListener::bind(client_address).unwrap();

    let column_name = "order_key";

    println!("Enter a number to search:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let user_number: i32 = input.trim().parse().expect("Invalid number entered");

    let (share1, share2) = generate_additive_shares(user_number);
    println!("{:?},{:?}", share1, share2);
    let shareStruct1: ClientShare = ClientShare { share1: (share1) };
    let shareStruct23: ClientShare = ClientShare { share1: (share2) };
    let share3: ClientShare = ClientShare { share1: (share2) };
    let handle1 = thread::spawn(move || send_data_to_p123(p1_address, shareStruct1));
    // let handle2 = thread::spawn(move || send_data_to_p123(p2_address, shareStruct23));
    // let handle3 = thread::spawn(move || send_data_to_p123(p3_address, share3));

    // Wait for three threads to finish
    handle1.join().unwrap().expect("Error in thread 1");
    // handle2.join().unwrap().expect("Error in thread 1");
    // handle3.join().unwrap().expect("Error in thread 1");

    // Todo : client recieves some results from p2, p3 and primary

    // while let Ok((stream, _)) = listener.accept() {
    //     // let stream_p2_clone = stream_p2.try_clone().unwrap();

    //     Thread::spawn(handle_client_connection(stream));
    // }

    if let Ok((stream, _)) = listener_client.accept() {
        handle_client_connection(stream);
    }
}
