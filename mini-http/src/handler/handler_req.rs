use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const CRLF: &str = "\r\n";

pub async fn handle_request(mut stream: TcpStream) {
    let mut buf = [0; 4096];

    stream.read(&mut buf).await.unwrap();

    let write = |(content, status)| write(stream, content, status);

    if matched(&buf, "/index") {
        write(handle_index().await).await;
    } else {
        write(handler_404()).await;
    }
}

fn handler_404() -> (String, String) {
    (String::from("404"), status(200, "OK"))
}

async fn write(mut stream: TcpStream, contents: String, status: String) {
    let content_type = format!("Content-Type: text/html;charset=utf-8{}", CRLF);
    let server = format!("Server: Rust{}", CRLF);
    let content_length = format!("Content-Length: {}{}", contents.as_bytes().len(), CRLF);

    let response = format!(
        "{0}{1}{2}{3}{4}{5}",
        status, server, content_type, content_length, CRLF, contents
    );

    stream.write_all(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

fn matched(buf: &[u8; 4096], route: &str) -> bool {
    let s = format!("GET {} HTTP/1.1{}", route, CRLF);

    buf.starts_with(s.as_bytes())
}

async fn handle_index() -> (String, String) {
    let mut file = File::open("index.html").await.unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).await.unwrap();

    (file_content, status(200, "OK"))
}

fn status(code: i32, text: &str) -> String {
    format!("HTTP/1.1 {} {}{}", code, text, CRLF)
}
