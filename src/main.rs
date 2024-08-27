use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time::Duration
};

struct HTTPResponse {
    status_line: String,
    contents: String,
    length: usize
}

impl HTTPResponse {
    
    fn new( status_line: String, html_file_path: String) -> Self {

        let contents = fs::read_to_string(html_file_path).unwrap();
        let length = contents.len();
        
        HTTPResponse {
            status_line,
            contents,
            length
        }
        
    }

}

impl ToString for HTTPResponse {
    fn to_string(&self) -> String {
        format!("{}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_line, self.length, self.contents
        )
    }
}

fn main() {
    
    let listener = TcpListener::bind(String::from("127.0.0.1:7878")).unwrap();
    
    for stream in listener.incoming() {
        
        let stream = stream.unwrap();
        
        handle_connection(stream);
        
    }
    
}

fn handle_connection(mut stream: TcpStream) {
    
    let buff_reader = BufReader::new(&mut stream);
    
    let request_line = buff_reader.lines().next().unwrap().unwrap();
    
    let http_response = match &request_line[..] {
        
        "GET / HTTP/1.1" => HTTPResponse::new(
            
            "HTTP/1.1 200 OK".to_string(),
            
            "hello.html".to_string()
            
        ),
  
        "GET /sleep HTTP/1.1" => {
            
            thread::sleep(Duration::from_secs(5));
            
            HTTPResponse::new(
            
            "HTTP/1.1 200 OK".to_string(),
            
            "hello.html".to_string()
            
            )
        },
        
        _ => HTTPResponse::new(
            
            "HTTP/1.1 404 NOT FOUND".to_string(),
            
            "404.html".to_string()
            
        ),
        
    };
    
    stream.write_all(http_response.to_string().as_bytes()).unwrap();

}
