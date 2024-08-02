use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use tokio::sync::oneshot::{self, Sender};

#[tokio::main]
async fn main() {
  //  broadcase();
  broadcast_non_receiver().await;
 // test_one_shot().await;
}

async  fn test_one_shot() {
    let (tx, rx) = oneshot::channel::<i32>();

     match rx.await {
         Ok(v) => println!("got = {:?}", v),
         Err(_) => println!("the sender dropped"),
     }

     println!("exit")
}

async fn broadcast() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), 10);
        assert_eq!(rx1.recv().await.unwrap(), 20);
    });

    tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), 10);
        assert_eq!(rx2.recv().await.unwrap(), 20);
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();
    
    sleep(Duration::from_millis(100000000)).await;
    println!("100 ms have elapsed");
}

async fn broadcast_non_receiver() {
    let tx = broadcast::Sender::new(128);
    
    if let Err(e) = tx.send(10) {
        
        println!("send data {}", e.to_string());
        return;
    }
    print!("send success")
}