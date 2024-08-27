use std::{fs, io};

#[derive(Debug)]
pub struct HTTPResponse {
    status_line: String,
    contents: String,
    length: usize
}

impl HTTPResponse {
    
    pub fn build(status_line: String, html_file_path: String) -> Result<HTTPResponse, io::Error> {

        let contents = fs::read_to_string(html_file_path)?;
        let length = contents.len();
        
        Ok(HTTPResponse {
            status_line,
            contents,
            length
        })
        
    }

}

impl ToString for HTTPResponse {
    fn to_string(&self) -> String {
        format!("{}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_line, self.length, self.contents
        )
    }
}