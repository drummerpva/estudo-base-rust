use std::{
  fs,
  io::{Read, Write},
  net::{TcpListener, TcpStream},
};

fn main() {
  //iniciar o servidor
  let address = "127.0.0.1:4001";
  let listener = TcpListener::bind(address).unwrap();
  println!("Server started at: {address}");

  //escutar por conexões
  for stream in listener.incoming() {
    let stream = stream.unwrap();
    handle_connection(stream);
  }
}

//gerenciar as conexões
fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  println!("Stream recebida!");
  // println!("{}", String::from_utf8_lossy(&buffer));

  let get = b"GET / HTTP/1.1";
  if buffer.starts_with(get) {
    send_index(stream);
  } else {
    send_not_found(stream);
  }
}

//reponder ao cliente
fn send_index(stream: TcpStream) {
  let content = fs::read_to_string("index.html").unwrap();

  let response = format!(
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    content.len(),
    content
  );
  send_to_client(stream, response);
}
fn send_not_found(stream: TcpStream) {
  let content = fs::read_to_string("404.html").unwrap();

  let response = format!(
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    content.len(),
    content
  );
  send_to_client(stream, response);
}

fn send_to_client(mut stream: TcpStream, response: String) {
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
