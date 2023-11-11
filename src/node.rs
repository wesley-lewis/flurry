use std::cell::UnsafeCell;
use crossbeam::epoch::{Atomic, Guard, Shared};

#[allow(dead_code)]
pub(crate) enum BinEntry<K,V> {
    Node(Node<K,V>),
}

impl<K,V> BinEntry<K,V> 
    where K: Eq, 
    {
        pub(crate) fn find<'g>(&self, hash: u64, key: &K, guard: &'g Guard) -> Option<Shared<'g, &Node<K,V >>>  {
        match *self {
            BinEntry::Node(ref n) => {
                if n.hash == hash && &n.key == key {
                    return Some(n);
                }

                if n.next.load() {}
            },
        }
    }
}

#[allow(dead_code)]
pub(crate) struct Node<K,V> {
    pub(crate) hash: u64,
    pub(crate) key: K, 
    pub(crate) value: UnsafeCell<V>,
    pub(crate) next: Atomic<Node<K,V>>,
}
