use core::time;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
    thread::sleep,
};

mod http;
use crate::http::{Request, Response};

mod thread_pool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
    println!("Listening on port 4000...");
    let pool = thread_pool::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }

    println!("Shutting down.");
}

fn handle_connection(stream: TcpStream) {
    let buf = BufReader::new(&stream).lines().next().unwrap().unwrap();
    let request = Request::new(buf.split(" ").collect::<Vec<&str>>());

    println!("{:?}", request);

    let (status_line, content): Response = match (&request.method[..], &request.path[..]) {
        ("GET", "/sleep") => {
            sleep(time::Duration::from_secs(5));
            (
                "HTTP/1.1 200 OK",
                Some(String::from("Responded after 5 seconds")),
            )
        }
        ("GET", "/") => ("HTTP/1.1 200 OK", read_file("index.html")),
        _ => ("HTTP/1.1 404 NOT FOUND", read_file("404.html")),
    };

    if content.is_some() {
        request.end(&stream, status_line, content);
    } else {
        request.end(&stream, status_line, None);
    }
}

fn read_file(file_name: &str) -> Option<String> {
    let file = fs::read_to_string(Path::new(file_name));

    match file {
        Ok(file) => {
            return Some(file);
        }
        _ => return None,
    };
}
