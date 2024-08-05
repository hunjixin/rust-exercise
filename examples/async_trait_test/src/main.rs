#![feature(trait_alias)]

use anyhow::Result;
use std::collections::HashMap;
use std::hash::Hash;

pub trait GID = Eq + Clone + Default + Hash;

trait ISay {
    async fn say(&self);
}

trait IPeopleQueue<ID>
where
    ID: GID,
{
    async fn people_say<'a>(&'a self, name: &'a ID) -> Result<&'a impl ISay>;
}

struct Hello {
    id: String,
}
impl ISay for Hello {
    async fn say(&self) {
        println!("isay {}", self.id);
    }
}

struct PeopleQueue<ID> {
    peoples: HashMap<ID, Hello>,
}

impl<ID> IPeopleQueue<ID> for PeopleQueue<ID>
where
    ID: GID,
{
    async fn people_say<'a>(&'a self, name: &'a ID) -> Result<&'a impl ISay> {
        Ok(self.peoples.get(name).unwrap())
    }
}

#[tokio::main]
async fn main() {
    let mut pq = PeopleQueue {
        peoples: HashMap::new(),
    };
    pq.peoples.insert(
        "123".to_string(),
        Hello {
            id: "123".to_string(),
        },
    );

    let id = "123".to_string();
    let data = pq.people_say(&id).await.unwrap();
    data.say().await;
}
