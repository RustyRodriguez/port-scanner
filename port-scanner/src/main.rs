use std::io::{self, Write};
use std::net::IpAddr;
use tokio::net::TcpStream;
use tokio::task;
use std::sync::mpsc::{channel, Sender};

const MAX_PORT_NUMBER: u16 = 65535;

async fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, start_port)).await {
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(start_port).unwrap();
        }
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    let addr: IpAddr = "127.0.0.1".parse().expect("Error");

    let start_port = 1;
    let end_port = MAX_PORT_NUMBER;

    let (tx, rx) = channel();
    for port in start_port..end_port {
        let tx = tx.clone();
        
        task::spawn(async move { scan(tx, port, addr).await });
    }

    let mut output = vec![];

    drop(tx);

    for port in rx {
        output.push(port);
    }

    println!("");

    output.sort();

    for port in output {
        println!("{} is an open port", port);
    }
}