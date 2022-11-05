
use std::{
    thread,
    io::{self, Read, Write},
    net::TcpStream,
    time::Duration,
};

fn main() {

	let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Connection faild");
	let mut input_stream = stream.try_clone().unwrap();
    let duration = Duration::from_millis(50);
    
	thread::spawn(move || {
        let mut client_buffer = [0u8; 1024];
        // input_stream.set_read_timeout(Some(duration));

		loop {
			match input_stream.read(&mut client_buffer) {
				Ok(n) => {

						io::stdout().write(&client_buffer).unwrap();
						io::stdout().flush().unwrap();
                        thread::sleep(duration);
				},
				Err(error) => {
                    println!("Connection Failed {}", error);
                    break;
                },
			}
            TcpStream::connect_timeout(&"127.0.0.1:7878".parse().unwrap(), duration).unwrap();
		}
	});

    let output_stream = &mut stream;
    let mut user_buffer = String::new();

    loop {
        io::stdin().read_line(&mut user_buffer).unwrap();

        output_stream.write(user_buffer.as_bytes()).unwrap();
        output_stream.flush().unwrap();
    }
}