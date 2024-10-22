use std::io::{Read, Write};
use std::{fs, str};
use std::path::PathBuf;
use std::net::TcpStream;
use clap::Parser;
use serde_json::Value;
//use rustls::RootCertStore;

mod message;
use message::{
    ByeMessage,
    ErrorMessage,
    GuessMessage,
    HelloMessage,
    RetryMessage,
    Score,
    StartMessage
};

mod guess;
use guess::make_guess;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'p')]
    port: Option<u16>,

    #[arg(short = 's', default_value_t = false)]
    tls: bool,

    hostname: String,
    username: String,
}

const DEFAULT_PORT: u16 = 27993;
const DEFAULT_TLS_PORT: u16 = 27994;
const WORD_LIST_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/", "src", "/", "words.txt");

fn parse_args() -> (String, u16, String) {
    let args = Args::parse();
    let port = match args.port {
        Some(p) => p,
        _ => if args.tls {DEFAULT_TLS_PORT} else {DEFAULT_PORT}
    };
    let hostname = args.hostname;
    let username = args.username;
    (hostname, port, username)
}

fn handshake(hostname: String, port: u16) -> TcpStream {
    if let Ok(s) = TcpStream::connect((hostname.clone(), port)) {
        s
    } else {
        panic!("Failed to connect to server {} on port {}", hostname, port);
    }
}

// write hello message to stream
fn greet_server(username: String, mut stream: &TcpStream) {
    let hello = HelloMessage::new(&username);
    let mut hello_json = serde_json::to_string(&hello).unwrap();
    hello_json.push_str("\n");
    let _ = stream.write(&hello_json.as_bytes());
}

fn send_guess(mut stream: &TcpStream, id: String, history: Vec<Score>, mut word_list: &Vec<&str>) {
    let guess_word= make_guess(history, &word_list);
    let guess = GuessMessage::new(guess_word, id.to_owned());
    let mut guess_json = serde_json::to_string(&guess).unwrap();
    guess_json.push_str("\n");
    let _ = stream.write(&guess_json.as_bytes());
    stream.flush().unwrap();
}

fn get_msg_type(server_msg: &Value) -> &str {
    let server_msg_serde_type = &server_msg["type"];
    if let Value::String(s) = server_msg_serde_type {
        s
    } else {
        panic!("Server message type not a string");
    }
}

fn get_server_msg(buffer: &mut [u8], n: usize) -> Value {
    //let server_msg_pre = serde_json::from_slice(trunc_buffer);
    let mut msg_str = str::from_utf8(buffer).unwrap();
    println!("{}", msg_str);
    msg_str = msg_str.trim();
    println!("{}", msg_str);
    let server_msg_pre = serde_json::from_str(&msg_str[..msg_str.len()-1]);
    let server_msg: Value = match server_msg_pre {
        Ok(x) => {
            println!("Read {} bytes", n);
            println!(
                "GOOD: {}\nFULL: {}",
                str::from_utf8(&buffer[..n]).unwrap(),
                str::from_utf8(&buffer).unwrap()
            );
            x
        },
        Err(e) => {
            println!("{:?}, read {} bytes", e, n);
            //println!("LAST BYTE: {:?}", &client_buffer[n-3..n+3]);
            panic!(
                "ERROR: {}",
                msg_str);
        }
    };
    println!("LAST BYTE: {}", &buffer[n]);
    buffer.fill(0);
    //client_buffer.clear();
    server_msg
}

fn main() {
    let (hostname, port, username) = parse_args();
    //Connect to the server
    let mut stream = handshake(hostname, port);
    greet_server(username, &stream);

    let mut client_buffer = [0u8; 1024*128];
    //let mut client_buffer = Vec::new();
    let mut id: Option<String> = None;
    
    // read in words.txt into word list
    let word_list_text = fs::read_to_string(PathBuf::from(WORD_LIST_PATH)).unwrap();
    let word_list = word_list_text.split_ascii_whitespace().collect();
    loop {
        match stream.read(&mut client_buffer) { //ERROR: n is cut off message: the whole message has id which from index n to n+j is missing some characters
            Ok(n) => {
                let server_msg = get_server_msg(&mut client_buffer, n);
                let mut history: Vec<Score> = Vec::new();
                let server_msg_type = get_msg_type(&server_msg);
                match server_msg_type {
                    "start" => {
                        let start: StartMessage = serde_json::from_value(server_msg).unwrap();
                        println!("START");
                        id = Some(start.id);
                    },
                    "retry" => {
                        let retry: RetryMessage = serde_json::from_value(server_msg).unwrap();
                        println!("RETRY");
                        history = retry.guesses;
                    },
                    "error" => {
                        let error: ErrorMessage = serde_json::from_value(server_msg).unwrap();
                        panic!("Server message error: {}", error.message);
                    }
                    "bye" => {
                        let bye: ByeMessage = serde_json::from_value(server_msg).unwrap();
                        println!("{}", bye.flag);
                        return
                    },
                    _ => {
                        panic!("Server message unknown type: {}", server_msg_type);
                    },
                }
                // write a guess if the id has been given
                if let Some(identification) = &id {
                    send_guess(&stream, identification.to_owned(), history, &word_list);
                }
            },
            Err(error) => {
                panic!("{}", error);
            }    
        }
    }
}