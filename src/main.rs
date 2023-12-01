use std::process::{exit, Command};

fn main() {
    // Replace these strings with the actual names or paths of your server and client binaries
    let primary = "../target/debug/primary";
    let p2 = "../target/debug/p2";
    let p3 = "../target/debug/p3";
    let p4 = "../target/debug/p4";

    // let client_binary = "../target/debug/client";

    // Spawn three server processes concurrently
    let server1 = Command::new(primary).spawn();
    let server2 = Command::new(p2).spawn();
    let server3 = Command::new(p3).spawn();
    let server4 = Command::new(p4).spawn();

    // Check if server processes were spawned successfully
    if let Err(err) = server1 {
        eprintln!("Error spawning server1: {}", err);
        exit(1);
    }
    if let Err(err) = server2 {
        eprintln!("Error spawning server2: {}", err);
        exit(1);
    }
    if let Err(err) = server3 {
        eprintln!("Error spawning server3: {}", err);
        exit(1);
    }

    if let Err(err) = server4 {
        eprintln!("Error spawning server4: {}", err);
        exit(1);
    }
    // Spawn the client process
    // let client = Command::new(client_binary).spawn();

    // Check if the client process was spawned successfully
    // if let Err(err) = client {
    //     eprintln!("Error spawning client: {}", err);
    //     exit(1);
    // }

    // Keep the main thread alive
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
