use core::task;
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut join_set = JoinSet::new();
    join_set.spawn(async move { println!("xxxq") });

    while let Some(res) = join_set.join_next().await {
        match res {
            Ok(_) => println!("A task completed successfully."),
            Err(e) => println!("A task failed: {:?}", e),
        }
    }
}

