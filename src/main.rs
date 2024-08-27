use std::{
    io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time::Duration
};

use rust_basic_web_server::{
    threadpool::ThreadPool, httpresponse::HTTPResponse
};


fn main() {
    
    let listener = TcpListener::bind(String::from("127.0.0.1:7878")).unwrap();
    let pool = ThreadPool::build(4).unwrap();
    
    for stream in listener.incoming() {
        
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });

    }
    
}

fn handle_connection(mut stream: TcpStream) {
    
    let buff_reader = BufReader::new(&mut stream);
    
    let request_line = buff_reader.lines().next().unwrap().unwrap();
    
    let http_response = match &request_line[..] {
        
        "GET / HTTP/1.1" => HTTPResponse::build(
            
            "HTTP/1.1 200 OK".to_string(),
            
            "hello.html".to_string()
            
        ),
  
        "GET /sleep HTTP/1.1" => {
            
            thread::sleep(Duration::from_secs(5));
            
            HTTPResponse::build(
            
            "HTTP/1.1 200 OK".to_string(),
            
            "hello.html".to_string()
            
            )
        },
        
        _ => HTTPResponse::build(
            
            "HTTP/1.1 404 NOT FOUND".to_string(),
            
            "404.html".to_string()
            
        ),
        
    };
    
    let http_response = http_response.unwrap();
    
    stream.write_all(http_response.to_string().as_bytes()).unwrap();  

}
