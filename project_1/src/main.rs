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
const WORD_LIST_PATH: &str = "/Users/olivertoh/Documents/Classes/Fall-2024/Distributed Systems/project_1/src/words.txt";

fn main() {
    println!("{:?}", std::env::current_dir().unwrap());
    let args = Args::parse();
    let port = match args.port {
        Some(p) => p,
        _ => if args.tls {DEFAULT_TLS_PORT} else {DEFAULT_PORT}
    };
    let hostname = args.hostname;
    let username = args.username;

    //Connect to the server
    let mut stream: TcpStream;
    if let Ok(s) = TcpStream::connect((hostname.clone(), port)) {
        stream = s;
    } else {
        println!("Failed to connect to server {} on port {}", hostname, port);
        return;
    }
    
    let hello = HelloMessage::new(&username);
    // write hello message to stream
    let mut hello_json = serde_json::to_string(&hello).unwrap();
    hello_json.push_str("\n");
    let _ = stream.write(&hello_json.as_bytes());

    //let mut client_buffer = [0u8; 1024*256*2];
    let mut client_buffer = Vec::new();
    let mut id: Option<String> = None;
    
    // read in words.txt into word list
    let word_list_text = fs::read_to_string(PathBuf::from(WORD_LIST_PATH)).unwrap();
    let word_list = word_list_text.split_ascii_whitespace().collect();
    loop {
        println!("ITER");
        match stream.read_to_end(&mut client_buffer) { //ERROR: n is cut off message: the whole message has id which from index n to n+j is missing some characters
            Ok(n) => {
                let server_msg_pre = serde_json::from_slice(&client_buffer[..n]);
                let server_msg: Value = match server_msg_pre {
                    Ok(x) => {println!("GOOD: {}\nFULL: {}", str::from_utf8(&client_buffer[..n]).unwrap(), str::from_utf8(&client_buffer).unwrap()); x},
                    Err(e) => {println!("{:?}, read {} bytes", e, n); println!("LAST BYTE: {:?}", &client_buffer[n-3..n+3]);println!("ERROR: {}\nFULL: {}", str::from_utf8(&client_buffer[..n]).unwrap(), str::from_utf8(&client_buffer).unwrap()); continue}
                };
                println!("LAST BYTE: {}", &client_buffer[n]);
                client_buffer.fill(0);
                let server_msg_serde_type = &server_msg["type"];
                let server_msg_type: &str;
                if let Value::String(s) = server_msg_serde_type {
                    server_msg_type = &s;
                } else {
                    println!("Server message type not a string");
                    return
                }
                let mut history: Vec<Score> = Vec::new();
                match server_msg_type {
                    "start" => {
                        let start: StartMessage = serde_json::from_value(server_msg).unwrap();
                        println!("START");
                        id = Some(start.id);
                    },
                    "retry" => {
                        let retry: RetryMessage = serde_json::from_value(server_msg).unwrap();
                        println!("RETRY");
                        history = retry.guesses;//7vSjmGo0LJMcn+2oC3V/7ijU8X7QAMuNPiVgvUqXDCcSWN
                    },
                    "error" => {
                        let error: ErrorMessage = serde_json::from_value(server_msg).unwrap();
                        println!("Server message error: {}", error.message);
                        break;
                    }
                    "bye" => {
                        let bye: ByeMessage = serde_json::from_value(server_msg).unwrap();
                        println!("{}", bye.flag);
                        break;
                    },
                    _ => {
                        println!("Server message unknown type: {}", server_msg_type);
                        break;
                    },

                }
                if let Some(identification) = &id {
                    println!("SENDING ID: {:?}", id);
                    let guess_word= make_guess(history, &word_list);
                    let guess = GuessMessage::new(guess_word, identification.to_owned());
                    let mut guess_json = serde_json::to_string(&guess).unwrap();
                    guess_json.push_str("\n");
                    println!("GUESS");
                    let _ = stream.write(&guess_json.as_bytes());
                }
            },
            Err(error) => {
                println!("{}", error);
            }    
        }
    }
}