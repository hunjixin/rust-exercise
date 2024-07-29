use tokio_stream::{StreamExt, StreamMap, Stream};
use tokio::sync::mpsc;
use tokio::select;

#[tokio::main]
async fn main() {
    let mut map = StreamMap::new();
    {
        let (tx, mut rx) = mpsc::channel::<usize>(10);
        tx.
        map.insert("A".to_string(), tx);
        tokio::spawn(async move {
            loop {
               select! {
                data_batch = rx.recv() => {
                  println!("A receive {}", data_batch.unwrap())
                },
               }
           }
          });
    }

    {
        let (tx, mut rx) = mpsc::channel::<usize>(10);
        map.insert("B".to_string(), tx);
        tokio::spawn(async move {
            loop {
               select! {
                data_batch = rx.recv() => {
                  println!("V receive {}", data_batch.unwrap())
                },
               }
           }
          });
    }



}
