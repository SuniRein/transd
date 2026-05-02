use crate::app::Message;
use iced::{
    Subscription,
    futures::{self, SinkExt, StreamExt, channel::mpsc::Sender},
    stream,
};
use zbus::{connection, interface};

const DBUS_NAME: &str = "tech.sunirein.transd";
const DBUS_PATH: &str = "/tech/sunirein/transd";

const DBUS_STREAM_CAPACITY: usize = 8;

pub fn subscription() -> Subscription<Message> {
    Subscription::run(worker)
}

fn worker() -> impl futures::Stream<Item = Message> {
    stream::channel(DBUS_STREAM_CAPACITY, |mut output| async move {
        if let Err(e) = serve(&mut output).await {
            let _ = output
                .send(Message::DbusError(format!("dbus service failed: {e}")))
                .await;
        }
    })
}

struct MainWindow {
    sender: Sender<Message>,
}

#[interface(name = "tech.sunirein.transd.MainWindow")]
impl MainWindow {
    fn translate_selection(&self) {
        // Best-effort: if the GUI is busy or shutting down, ignore.
        let _ = self
            .sender
            .clone()
            .try_send(Message::DbusTranslateSelection);
    }
}

async fn serve(output: &mut Sender<Message>) -> Result<(), zbus::Error> {
    let (sender, mut receiver) = futures::channel::mpsc::channel(DBUS_STREAM_CAPACITY);

    let _conn = connection::Builder::session()?
        .name(DBUS_NAME)?
        .serve_at(DBUS_PATH, MainWindow { sender })?
        .build()
        .await?;

    while let Some(msg) = receiver.next().await {
        if output.send(msg).await.is_err() {
            break;
        }
    }

    Ok(())
}
