use std::{
    io::{self, Read, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Connection faild");
    let mut input_stream = stream.try_clone().unwrap();
    let duration = Duration::from_millis(100);

    thread::spawn(move || {
        let mut client_buffer = [0u8; 1024];

        loop {
            match input_stream.read(&mut client_buffer) {
                Ok(_n) => {
                    io::stdout().write(&client_buffer).unwrap();
                    io::stdout().flush().unwrap();
                }
                Err(error) => {
                    println!("Connection Failed {}", error);
                    break;
                }
            }
            thread::sleep(duration);
        }
    });

    let mut user_buffer = String::new();

    loop {
        io::stdin().read_line(&mut user_buffer).unwrap();
        stream.write(user_buffer.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
