// SERVER

use std::io;
use std::time;
use std ::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::thread;

// Handle access stream
// Create a struct to hold the stream's state
// Perform I/O operations

fn handle_sender(mut stream: TcpStream) -> io::Result<()>{
    // Handle multiple access stream
    let mut buf = [0 ; 512];
    for _ in 0..1000 {
        // Let the receiver get a message from a sender
        let bytes_read = stream.read(&mut buf)?;
        // Sender stream in a mutable variable
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        // Print acceptance message
        // Read, print message sent
        println!("from the sender: {}", String::from_utf8_lossy(&buf));
        // Sleep this connection with the connected sender 
        thread::sleep(time::Duration::from_secs(1));
    }
    
    // Success value
    Ok(())
}

fn main() -> io::Result<()>{
    //println!("Hello, world!");
    
    // Enable port 7878 binding
    let receiver_listener = TcpListener::bind("127.0.0.1:7878").expect("Failed and bind with the sender");
    
    // Getting handle of underlying thread 
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    
    // Listen to incoming connections message and bind them to a server socket address
    for stream in receiver_listener.incoming() {
        let stream = stream.expect("failed");

        //Let receiver connect with the sender
        let handle = thread::spawn(move || {
            // Receive failed to read from the stream
            handle_sender(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
        });

        // Push messages in the order they are sent
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        // Return each single value output contained in the heap
        handle.join().unwrap();
    }

    Ok(())
}

/* PROVIDED CODE 
proto
syntax = "proto3";
package test;

service Test {
    rpc DepositMoney (DepositMoneyRequest) returns (DepositMoneyResponse) ;
}

message DepositMoneyRequest {
    uint user_id = 1;
    uint amount = 2;
    string description = 3;
}

message DepositMoneyResponse {
    uint total_balance = 1;
}


DepositMoneyRequest {
        user_id: 123,
        amount: 40,
        description: "First transacation!"
}

DepositMoneyRequest {
        user_id: 123,
        amount: 5,
        description: "Second transacation :)" */
