use tokio::sync::broadcast::{self, Sender};
use tokio::sync::oneshot::{self};
use tokio::time::{self, sleep, Duration};

#[tokio::main]
async fn main() {
    //  broadcase();
    broadcast_lag().await;
    // test_one_shot().await;
}

async fn test_one_shot() {
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

async fn broadcast_lag() {
    tokio::spawn(async {
        let tx: Sender<()> = broadcast::Sender::new(10);


        let mut rx  = tx.subscribe();
        tokio::spawn(async move {
            loop{
                tokio::select! {
                    xxx = rx.recv() => {
                        if let Err(err) = xxx {
                            println!("xxx {err}")
                        }
                    }
                }
                sleep(Duration::from_secs(2)).await;
            }
        });

       
        let new_data_tx = tx.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let _ = new_data_tx.send(());
                    }
                }
            }
        });
        


    });

    
    sleep(Duration::from_millis(100000000)).await;
}
