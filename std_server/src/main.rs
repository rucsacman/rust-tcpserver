use std::{
    collections::{HashMap, VecDeque},
    str::from_utf8,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,}, borrow::BorrowMut
};
use serde::de::value;
use tokio::{
    io::{ AsyncWriteExt, AsyncReadExt},
    net::{TcpListener, TcpStream},
    // sync::broadcast::{self, Receiver},
    // runtime::Runtime
};
#[tokio::main]
async fn main() {
    let  mut client_object: HashMap<usize, VecDeque<String>> = HashMap::new();
    let client_object_lock = Arc::new(RwLock::new(client_object));

    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    //let pool = ThreadPool::new(4);
    loop{
        let (stream, _addr) = listener.accept().await.unwrap();
        let c_client_object_lock = Arc::clone(&client_object_lock);
        tokio::spawn(async move {
            
            println!("Connection Established");
            handle_connection(stream, &mut c_client_object_lock.clone()).await;
        });
    }
}


async fn handle_connection(mut stream:TcpStream, client_lock: &mut Arc<RwLock<HashMap<usize, VecDeque<String>>>>) {
    let client_id = get_id()-1;
    {
        let boradcast_map = &mut *client_lock.write().unwrap();
        boradcast_map.insert(client_id, VecDeque::new());
        println!("size: {}",boradcast_map.len());
    }

    loop{
        let mut buffer = [1; 500];
        let len = stream.read(&mut buffer).await.unwrap();
        let mut boradcast_vec;

        {
            let client_map = &mut *client_lock.write().unwrap();
            let message = String::from_utf8_lossy(&mut buffer[..len]);
            
            
            for (key, value) in &mut *client_map {
                println!("key: {}", key);
                value.push_back(message.to_string());
            }
            boradcast_vec = client_map.get_mut(&client_id).unwrap().clone();
        }
        {
            let client_map = &mut *client_lock.write().unwrap();
            for (key, value) in &mut *client_map {
                println!("key: {}", key);
                for client_message in value.iter() {
                    println!("loop message: {}", client_message);
                }
                print!("________________________________________________________________");
            }

        }
        let mut borrow_vec = boradcast_vec.borrow_mut();
        loop{
            if borrow_vec.len() == 0 {
                break;
            } 
            let value = borrow_vec.pop_front().unwrap();
            println!("Write: {}", value);
                let _ = stream.write(value.as_bytes()).await.unwrap();
                let _ = stream.flush().await.unwrap();
        }
        // for client_message in boradcast_vec.iter() {
        //     println!("{} message: {}", client_id, client_message);
        // }

       
    }
}


fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}