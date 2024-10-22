use std::io::{stdout, BufRead, BufReader, Read, Write};
use std::sync::Arc;
use std::{fs, str};
use std::path::PathBuf;
use std::net::TcpStream;
use clap::Parser;
use rustls::pki_types::ServerName;
use native_tls::TlsConnector;
use serde_json::Value;
use rustls::{ClientConnection, RootCertStore, Stream};

mod message;
use message::{ ByeMessage, ErrorMessage, GuessMessage, HelloMessage, RetryMessage, Score, StartMessage
};

mod guess;
use guess::filter_words;

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

fn parse_args() -> (String, u16, String, bool) {
    let args = Args::parse();
    let port = match args.port {
        Some(p) => p,
        _ => if args.tls {DEFAULT_TLS_PORT} else {DEFAULT_PORT}
    };
    let hostname = args.hostname;
    let username = args.username;
    (hostname, port, username, args.tls)
}

fn handshake(hostname: String, port: u16, tls: bool) -> TcpStream {
    if let Ok(s) = TcpStream::connect((hostname.clone(), port)) {
        if tls {
            //let connector = TlsConnector::new().unwrap();
            //let mut stream = connector.connect(&hostname, s).unwrap();
            //stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
            //stream
            s
        } else {
            s
        }
    } else {
        panic!("Failed to connect to server {} on port {}", hostname, port);
    }
}

//fn enable_tls(host: String, mut sock: &TcpStream) -> Stream<'_,ClientConnection, &TcpStream>  {
//    let root_store = RootCertStore {
//        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
//    };
//    let mut config = rustls::ClientConfig::builder()
//        .with_root_certificates(root_store)
//        .with_no_client_auth();
//
//    // Allow using SSLKEYLOGFILE.
//    config.key_log = Arc::new(rustls::KeyLogFile::new());
//
//    let server_name: ServerName = host.clone().try_into().unwrap();
//    let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();
//    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
//    tls.write_all(
//        format!(
//        concat!(
//            "GET / HTTP/1.1\r\n",
//            "Host: {}\r\n",
//            "Connection: close\r\n",
//            "Accept-Encoding: identity\r\n",
//            "\r\n"
//        ), host)
//        .as_bytes(),
//    )
//    .unwrap();
//    tls
//}

// write hello message to stream
fn greet_server(username: String, mut stream: &TcpStream) {
    let hello = HelloMessage::new(&username);
    let mut hello_json = serde_json::to_string(&hello).unwrap();
    hello_json.push_str("\n");
    let _ = stream.write(&hello_json.as_bytes());
}

fn send_guess(
    mut stream: &TcpStream,
    id: String,
    guess_word: String) {
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

fn get_server_msg(buffer: &mut [u8]) -> Value {
    let mut msg_str = str::from_utf8(buffer).unwrap();
    msg_str = msg_str.trim_matches(char::from(0)).trim();
    let server_msg_pre = serde_json::from_str(&msg_str);
    let server_msg: Value = match server_msg_pre {
        Ok(x) => x,
        Err(e) => {panic!("ERROR: {} trying to parse {}", e, msg_str);}
    };
    buffer.fill(0);
    server_msg
}

fn main() {
    let (hostname, port, username, tls) = parse_args();
    //Connect to the server
    let stream = handshake(hostname, port, tls);
    greet_server(username, &stream);

    //let mut client_buffer = [0u8; BUFFER_SIZE];
    let mut client_buffer = Vec::new();
    let mut id: Option<String> = None;
    
    // read in words.txt into word list
    let word_list_text = fs::read_to_string(PathBuf::from(WORD_LIST_PATH)).unwrap();
    let mut word_list: Vec<&str> = word_list_text.split_ascii_whitespace().collect();
    let mut packet = BufReader::new(&stream);
    loop {
        match packet.read_until(b'\n', &mut client_buffer) {
            Ok(n) => {
                let server_msg = get_server_msg(&mut client_buffer);
                let mut history: Vec<Score> = Vec::new();
                let server_msg_type = get_msg_type(&server_msg);
                match server_msg_type {
                    "start" => {
                        let start: StartMessage = serde_json::from_value(server_msg).unwrap();
                        id = Some(start.id);
                    },
                    "retry" => {
                        let retry: RetryMessage = serde_json::from_value(server_msg).unwrap();
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
                    word_list = filter_words(history, word_list);
                    if word_list.len()>0 {
                        send_guess(&stream, identification.to_owned(), String::from(word_list[0]));
                    } else {
                        panic!("Guessed all possible words!");
                    }             
                }
            },
            Err(error) => {
                panic!("{}", error);
            }    
        }
    }
}