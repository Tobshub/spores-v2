use std::{io::Write, net::TcpStream};

pub type Response<'a> = (&'a str, Option<String>);

#[derive(Debug)]
pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
    _version: &'a str,
}

impl<'a> Request<'a> {
    pub fn new(request_string: Vec<&'a str>) -> Self {
        return Self {
            method: request_string[0],
            path: request_string[1],
            _version: request_string[2],
        };
    }

    pub fn end(self, mut stream: &TcpStream, status_line: &'a str, content: Option<String>) {
        let response;
        if content.is_some() {
            let content = content.unwrap();
            response = format!(
                "{status_line}\r\nContent-Length: {}\r\n\r\n{}",
                content.len(),
                content
            );
        } else {
            response = format!("{status_line}\r\n\r\n");
        }
        return stream.write_all(response.as_bytes()).unwrap();
    }
}
