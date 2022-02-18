use crate::write_queue::WriteQueue;
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver},
    SinkExt, StreamExt,
};
use notify::{event::CreateKind, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

// this task reads all fs notification events from the watched directory
pub async fn consume_fs_events<P: AsRef<Path>>(
    path: P,
    write_queue: &WriteQueue,
) -> notify::Result<()> {
    // create the fs watcher
    let (mut watcher, mut rx) = fs_event_producer()?;

    // add the path to be watched.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    // read events from the receiving side of the channel
    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => filter_file_create(event, write_queue).await,
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}

// filter file creation events
// and push matching paths to the write queue
async fn filter_file_create(e: Event, write_queue: &WriteQueue) {
    debug!("filter event {:?} ...", e);
    if let EventKind::Create(create_kind) = e.kind {
        if create_kind == CreateKind::File {
            for path in e.paths.iter() {
                if let Err(err) = write_queue.push_file(path.to_str().unwrap()).await {
                    warn!("{}", err);
                }
            }
        }
    }
}

// create fs watcher and return the receiving side of the channel
// see https://github.com/notify-rs/notify/blob/main/examples/async_monitor.rs
fn fs_event_producer(
) -> notify::Result<(RecommendedWatcher, UnboundedReceiver<notify::Result<Event>>)> {
    let (mut tx, rx) = unbounded(); // create an unbounded channel

    let watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap(); // send the event result
                                         // over the channel
        })
    })?;

    Ok((watcher, rx))
}
