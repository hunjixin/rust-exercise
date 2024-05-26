#![feature(linked_list_cursors)]

use std::collections::{HashMap, LinkedList};
struct LRUCache {
    hash: HashMap<i32, i32>,
    list: LinkedList<i32>,
    capacity: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            hash: HashMap::new(),
            list: LinkedList::new(),
            capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let result = self.hash.get(&key);
        if result.is_none() {
            return -1;
        }
        let value = *result.unwrap();

        self.move_front(key);
        value
    }

    fn put(&mut self, key: i32, value: i32) {
        let insert_result = self.hash.insert(key, value);
        if insert_result.is_none() {
            self.list.cursor_back_mut().push_front(key);
            if self.hash.len() as i32 > self.capacity {
                if let Some(back_key) = self.list.cursor_back_mut().remove_current() {
                    self.hash.remove(&back_key);
                }
            }
        } else {
            self.move_front(key);
        }
    }
    fn move_front(&mut self, key: i32) {
        let mut curser = self.list.cursor_front_mut();
        loop {
            let current = curser.current();
            if current.is_none() {
                break;
            }
            if *current.unwrap() == key {
                curser.remove_current();
                break;
            }
            curser.move_next();
        }
        curser.push_front(key);
    }
}

fn main() {
    let mut lru_cache = LRUCache::new(2);
    lru_cache.put(1, 1); // 缓存是 {1=1}
    lru_cache.put(2, 2); // 缓存是 {1=1, 2=2}
    lru_cache.get(1); // 返回 1
    lru_cache.put(3, 3); // 该操作会使得关键字 2 作废，缓存是 {1=1, 3=3}
    lru_cache.get(2); // 返回 -1 (未找到)
    lru_cache.put(4, 4); // 该操作会使得关键字 1 作废，缓存是 {4=4, 3=3}
    lru_cache.get(1); // 返回 -1 (未找到)
    lru_cache.get(3); // 返回 3
    lru_cache.get(4); // 返回 4
}
