use std::{
    io::{prelude::*},
    net::{TcpListener, TcpStream},
    thread,
};

use tokio::{
    io::{self, AsyncWriteExt, AsyncReadExt},
    net::{TcpListener, TcpStream},
    // sync::broadcast::{self, Receiver},
    runtime::Runtime
};
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        print!("Connection Established");
        tokio::spawn(async move {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream:TcpStream) {
    loop{
        let mut buffer = [1; 500];
        let len = stream.read(&mut buffer).unwrap();
        stream.write(&buffer[..len]).unwrap();
    }
}