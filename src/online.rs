use std::{
    net,
    io::Error,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration

};



pub fn server()-> (usize, usize) {
    thread::sleep(Duration::from_secs(2));
    // this ensures that two cennections dont interrupt each other while changing tasks
    println!("Started to listen-->");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut my_x: usize = 3;
    let mut my_y: usize = 3;
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        // Buffer to hold the received data (16 bytes for two usize values)
        let mut buffer = [0; 16];
        stream.read_exact(&mut buffer).unwrap();

        // Convert bytes to usize values
        my_x = usize::from_be_bytes(buffer[0..8].try_into().unwrap());
        my_y = usize::from_be_bytes(buffer[8..16].try_into().unwrap());
        if my_x <= 2 && my_y <= 2  {
            break;
        }   
    }
    (my_x,my_y)
}

pub fn client(my_x: &usize, my_y: &usize) -> Result<(), Error> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Sending data_>");

    // Convert usize values to bytes
    let my_x_bytes = my_x.to_be_bytes();
    let my_y_bytes = my_y.to_be_bytes();

    // Create a buffer to hold both values
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&my_x_bytes);
    buffer.extend_from_slice(&my_y_bytes);
    // Send the buffer to the server
    stream.write_all(&buffer)?;
    
    stream.shutdown(net::Shutdown::Both).unwrap();

    
    Ok(())
}