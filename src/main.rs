use std::net::TcpListener;
use std::{env, fs, thread};
use std::{io::prelude::*, net::TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                thread::spawn(|| handel_con(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handel_con(mut stream: TcpStream) {
    let mut buff = [0; 1024];
    let n = stream.read(&mut buff).unwrap();
    let request_string = String::from_utf8_lossy(&buff[..n]).into_owned();
    //println!("{:?}",request_string);
    let http_req: Vec<_> = request_string.trim().split("\r\n").collect();
    println!("{:?}", http_req);
    //let http_req : Vec<_> = read_buff.split("").collect()
    let mut res = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
    let file_path = http_req[0].split_whitespace().nth(1).unwrap();
    let mut user_agent = "";
    for i in http_req.iter() {
        if i.starts_with("User-Agent:") {
            user_agent = i.trim_start_matches("User-Agent: ");
        }
    }
    let html = fs::read_to_string("index.html").unwrap();
    if http_req[0].starts_with("GET") {
        if file_path == "/" {
            res = "HTTP/1.1 200 OK\r\n\r\n".to_string();
        }
        if file_path.starts_with("/echo/") {
            let name = file_path.trim_start_matches("/echo/");
            res = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                name.len(),
                name
            );
        }
        if file_path == "/user-agent" {
            let name = user_agent;
            res = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                name.len(),
                name
            );
        }
        if http_req[0].starts_with("GET / ") {
            let name = html;
            res = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                name.len(),
                name
            );
        }
        if file_path.starts_with("/files/") {
            let name = file_path.trim_start_matches("/files/");
            let dir_temp: Vec<_> = env::args().collect();
            let dir = &dir_temp[2];
            let n = fs::read(format!("{}/{}", dir, name));
            match n {
                Ok(n) => {
                    res = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
            n.len(),
            String::from_utf8(n).unwrap()
        );
                }
                Err(e) => println!("{}", e),
            }
        }
    } else if http_req[0].starts_with("POST") {
        let name = file_path.trim_start_matches("/files/");
        let dir_temp: Vec<_> = env::args().collect();
        let dir = &dir_temp[2];
        let data = http_req[http_req.len() - 1];
        let _ = fs::write(format!("{}/{}", dir, name), data);

        res = "HTTP/1.1 201 Created\r\n\r\n".to_string();
    }
    stream.write_all(res.as_bytes()).unwrap();
}
