use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    println!("[+] Handle Client.");
    loop {
        let mut input = String::new();

        let _ = std::io::stdin().read_line(&mut input);
        stream.write_all(input.as_bytes()).unwrap();

        let mut data = [0 as u8; 10024];
        stream.read(&mut data).unwrap();
        let string = String::from_utf8_lossy(&data);

        println!("{}", string);
    }
}

fn main() {
    let _args: Vec<String> = std::env::args().collect();

    let port = 3000;
    let complete = format!("0.0.0.0:{}", port);
    let listener2 = TcpListener::bind(&complete).unwrap();

    println!("[+] Welcome \n");
    println!("[+] Server listening port {}", &port);

    for stream2 in listener2.incoming() {
        match stream2 {
            Ok(stream2) => {
                println!("[+] Join: {}", stream2.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream2)
                });
            }
            Err(e) => {
                println!("[+] Error: {}", e);
            }
        }
    }
}
