use rocket::{fs::FileServer, get, launch, routes};
use ws::{WebSocket, Stream};

#[get("/echo?channel")]
fn echo_channel(ws: ws::WebSocket) -> ws::Channel<'static> {
    use rocket::futures::{SinkExt, StreamExt};

    ws.channel(move |mut stream| Box::pin(async move {
        while let Some(message) = stream.next().await {
            let _ = stream.send(message?).await;
        }

        Ok(())
    }))
}

#[get("/echo?stream")]
fn echo_stream(ws: ws::WebSocket) -> ws::Stream!['static] {
    ws::Stream! { ws =>
        for await message in ws {
            yield message?;
        }
    }
}

#[get("/echo?compose")]
fn echo_compose(ws: ws::WebSocket) -> ws::Stream!['static] {
    ws.stream(|io| io)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
         // serve files from `/www/static` at path `/public`
        .mount("/", FileServer::from("rserver/static"))
}
