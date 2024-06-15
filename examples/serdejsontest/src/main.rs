#![feature(trait_alias)]

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::hash::Hash;

pub trait GID = Eq + Clone + Copy + Hash + Serialize + for<'de> Deserialize<'de>;

pub trait Unit<ID>
where
    ID: GID,
{
    fn id(&self) -> ID;
    fn name(&self) -> String;
}

// Channel use to definite data transfer channel
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {}

// ComputeUnit used to define logic for data generation, transformer, ouput
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeUnit<ID>
where
    ID: GID,
{
    #[serde(bound(deserialize = ""))]
    pub id: ID,

    pub name: String,
    pub channel: Option<Channel>,

    pub image: String,
    pub cmd: Vec<String>,

    #[serde(bound(deserialize = ""))]
    pub(crate) dependency: Vec<ID>,
}

impl<ID: GID> Unit<ID> for ComputeUnit<ID> {
    fn id(&self) -> ID {
        self.id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    println!("Hello, world!");
}
