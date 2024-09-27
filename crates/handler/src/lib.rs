use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::{self, JoinHandle};

use router::Router;
use server::request::RequestMessage;
use server::response::ResponseMessage;

pub struct Handler<T> {
    stream: TcpStream,
    thread_handles: Vec<JoinHandle<T>>,
    active_threads: usize,
    thread_limit: usize,
}

impl<T> Handler<T> {
    pub fn init(stream: TcpStream, thread_limit: usize) -> Self {
        Self {
            stream,
            thread_handles: vec![],
            active_threads: 0,
            thread_limit,
        }
    }

    pub fn handle(&mut self, stream: &mut TcpStream, router: &Router) {
        thread::spawn(move || {
            let mut buffer = [0 as u8; 1000];
            stream.read(&mut buffer);
            let request = RequestMessage::parse_request(&buffer).expect("Failure parsing request");
            let response = ResponseMessage::build_response(request, router)
                .expect("Failure building response");
            stream
                .write_all(response.to_string().as_bytes())
                .expect("Failure writing response");
        });
    }
}
