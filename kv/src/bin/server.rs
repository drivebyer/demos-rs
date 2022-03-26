use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use kv::pb::{request::Command, Request, RequestGet, RequestPut, Response};
use tokio::net::TcpListener;
use tokio_util::codec::LengthDelimitedCodec;
use tracing::info;

#[derive(Debug)]
struct ServerState {
    store: DashMap<String, Vec<u8>>,
}

impl ServerState {
    #[must_use]
    fn new() -> Self {
        Self {
            store: DashMap::new(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    info!("hello, i am a kv server");

    let state = Arc::new(ServerState::new());
    let listener = TcpListener::bind("0.0.0.0:8888").await?;

    info!("listening on {:?}", listener.local_addr());

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        info!("new client {:?}", peer_addr);

        let shared = state.clone();

        tokio::spawn(async move {
            let mut stream = LengthDelimitedCodec::builder()
                .length_field_length(2)
                .new_framed(stream);

            while let Some(Ok(buf)) = stream.next().await {
                let msg = Request::try_from(buf)?;

                info!("received {:?}", msg);

                let response = match msg.command {
                    Some(Command::Get(RequestGet { key })) => match shared.store.get(&key) {
                        Some(v) => Response::new(key, v.value().to_vec()),
                        None => Response::not_found(key),
                    },
                    Some(Command::Put(RequestPut { key, value })) => {
                        shared.store.insert(key.clone(), value.clone());
                        Response::new(key, value)
                    }
                    None => todo!(),
                };

                stream.send(response.into()).await?
            }

            Ok::<(), anyhow::Error>(())
        });
    }
}
