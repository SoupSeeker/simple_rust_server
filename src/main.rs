use std::fmt::format;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{prelude, Read, Write};



fn main() {
    let listener =
        TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();   //shadowing stream

        println!("Connection established");
        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream){
    let mut in_buffer = [0; 1024];
    let expected_get = b"GET / HTTP/1.1\r\n";
    stream.read(&mut in_buffer).unwrap();

    // HTTP-Version Status-Code Reason-Phrase CRLF [\r\n]
    // headers CRLF
    // message-body

    if in_buffer.starts_with(expected_get){
        let contents = fs::read_to_string("index.html").unwrap();

        let ok_response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);

        stream.write(ok_response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let ok_response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);

        stream.write(ok_response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    //println!(
    //    "Request: {}",
    //    String::from_utf8_lossy(&in_buffer[..])
    //);




}
