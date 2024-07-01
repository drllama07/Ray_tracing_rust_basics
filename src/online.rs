use std::{
    io,
    io::Error,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};



pub fn server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("started to listen for connections in 127.0.0.1:7878");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut number = [0;4];
        stream.read_exact(&mut number).unwrap();
        let number: i32 = i32::from_be_bytes(number);
        println!("{number}");
        stream.write_all(&[number as u8; 4]).unwrap();
    }
}


pub fn client() -> Result<(), Error> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to server at 127.0.0.1:7878!");

    let number: i32 = 123; // The integer to send
    let mut buffer = number.to_be_bytes(); // Convert integer to bytes (big-endian)
    stream.write_all(&buffer)?;

    println!("Sent number: {}", number);

    let mut response = [0; 4]; // Buffer to hold the response
    stream.read_exact(&mut response)?;

    let response: i32 = i32::from_be_bytes(response); // Convert bytes to integer
    println!("Received response: {}", response);

    Ok(())
}


pub fn connection() {
    let mut input = String::new();
    print!("Are you $SERVER(1) or $CLIENT(2)? type 1 or 2: ");
    io::stdout().flush().unwrap(); 
    io::stdin().read_line(&mut input).unwrap();
    let number: usize = input.trim().parse().expect("parsing error");
    if  number == 1{
        server();
    }
    else if  number == 2{
        let _ = client();
    } else {
        println!("Invalid response please type 1 or 2 -> {number}")
    }
}