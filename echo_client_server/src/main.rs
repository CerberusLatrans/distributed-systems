use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::{env, io};
use std::str::from_utf8;

const BUFSIZE: usize = 1024;
const SERVER_PORT: u16 = 5000;
const INADDR_ANY: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        usage();
    } else {
        if args[1]=="-s" {
            server();
        } else if args[1]=="-c" {
            client();
        } else {
            usage();
        }
    }
}

fn server() {
    let server_addr = SocketAddr::new(LOCALHOST, SERVER_PORT);
    let socket = UdpSocket::bind(server_addr).expect("Server coudn't bind to address");
    println!("SERVER STARTED");
    loop {
        let mut buf = [0; BUFSIZE];
        let (amt, src) = socket.recv_from(&mut buf).expect("Server didn't receive data");
        let filled_buf = &mut buf[..amt];
        println!("Server received: {} from {}", from_utf8(filled_buf).unwrap(), src);
        filled_buf.reverse();
        socket.send_to(filled_buf, &src).expect("Server could not send data");
    }
}

fn client() {
    let client_addr = SocketAddr::new(INADDR_ANY, 0);
    let socket = UdpSocket::bind(client_addr).expect("Client coudn't bind to address");
    println!("CLIENT STARTED");
    let server_addr = SocketAddr::new(LOCALHOST, SERVER_PORT);
    loop {
        let mut input_buf = [0; BUFSIZE];
        io::stdin().read(&mut input_buf).expect("Could not read client input");
        socket.send_to(&input_buf, &server_addr).expect("Client could not send data");

        let mut buf = [0; BUFSIZE];
        let (amt, src) = socket.recv_from(&mut buf).expect("Client didn't receive data");
        let filled_buf = &mut buf[..amt];
        println!("Client received: {} from {}", from_utf8(filled_buf).unwrap(), src);
    }
}

fn usage() {
    println!("Server: -s \nClient: -c");
}