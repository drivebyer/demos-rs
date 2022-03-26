use anyhow::Result;
use futures::SinkExt;
use futures::StreamExt;
use kv::pb::{Request, Response};
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

#[tokio::main]
async fn main() -> Result<()> {
    let stream = TcpStream::connect("localhost:8888").await?;

    let mut stream = LengthDelimitedCodec::builder()
        .length_field_length(2)
        .new_framed(stream);

    let req = Request::new_put("hello", b"world");
    stream.send(req.into()).await?;

    let req = Request::new_get("hello");
    stream.send(req.into()).await?;

    while let Some(Ok(buf)) = stream.next().await {
        let resp = Response::try_from(buf)?;

        println!("Got message {:?}", resp);
    }

    Ok(())
}
