use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<K, V> {
  key: K,
  val: V,
  prev: Option<NonNull<Node<K, V>>>,
  next: Option<NonNull<Node<K, V>>>,
}

struct KeyRef<K, V>(NonNull<Node<K, V>>);

impl<K: Hash, V> Hash for KeyRef<K, V> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    unsafe { self.0.as_ref().key.hash(state) }
  }
}

impl<K: Eq, V> PartialEq for KeyRef<K, V> {
  fn eq(&self, other: &Self) -> bool {
    unsafe { self.0.as_ref() }
      .key
      .eq(&unsafe { other.0.as_ref() }.key)
  }
}

impl<K: Eq, V> Eq for KeyRef<K, V> {}

impl<K, V> Borrow<K> for KeyRef<K, V> {
  fn borrow(&self) -> &K {
    unsafe { &self.0.as_ref().key }
  }
}

pub struct Lru<K, V> {
  cap: usize,
  head: Option<NonNull<Node<K, V>>>,
  tail: Option<NonNull<Node<K, V>>>,
  container: HashSet<KeyRef<K, V>>,
  marker: PhantomData<Node<K, V>>,
}

impl<K: Eq + PartialEq + Hash, V> Lru<K, V> {
  pub fn new(cap: usize) -> Self {
    Lru {
      cap,
      head: None,
      tail: None,
      container: HashSet::new(),
      marker: PhantomData,
    }
  }

  fn detach(&mut self, node: &mut NonNull<Node<K, V>>) {
    let node_ref = unsafe { node.as_mut() };
    if let Some(mut pre) = node_ref.prev {
      unsafe { pre.as_mut() }.next = node_ref.next;
      // node is head
    } else {
      self.head = node_ref.next;
    }
    if let Some(mut next) = node_ref.next {
      unsafe { next.as_mut() }.prev = node_ref.prev;
      // node is tail
    } else {
      self.tail = node_ref.prev;
    }
    node_ref.next = None;
    node_ref.prev = None;
  }

  fn attach(&mut self, node: &mut NonNull<Node<K, V>>) {
    let node_ref = unsafe { node.as_mut() };
    if let Some(mut old_head) = self.head {
      unsafe { old_head.as_mut() }.prev = Some(*node);
      node_ref.next = self.head;
      self.head = Some(*node)
    } else {
      self.head = Some(*node);
      self.tail = Some(*node);
    }
  }

  fn remove_tail(&mut self) -> Option<NonNull<Node<K, V>>> {
    self.tail.map(|mut old_tail| {
      let node_ref = unsafe { old_tail.as_mut() };
      if let Some(mut pre) = node_ref.prev {
        unsafe { pre.as_mut() }.next = None;
        node_ref.prev = None;
        self.tail = Some(pre);
      } else {
        self.tail = None;
        self.head = None;
      }
      old_tail
    })
  }

  pub fn insert(&mut self, key: K, val: V) {
    if let Some(KeyRef(mut node)) = self.container.take(&key) {
      self.detach(&mut node);
    } else if self.container.len() >= self.cap {
      if let Some(removed_key) = self.remove_tail() {
        self.container.remove(&KeyRef(removed_key));
      }
    }
    let mut new_node = unsafe {
      NonNull::new_unchecked(Box::into_raw(Box::new(Node {
        key,
        val,
        prev: None,
        next: None,
      })))
    };
    self.attach(&mut new_node);
    self.container.insert(KeyRef(new_node));
  }

  pub fn get(&mut self, key: &K) -> Option<&V> {
    if let Some(node) = self.container.get(key) {
      let mut node = node.0;
      self.detach(&mut node);
      self.attach(&mut node);
      Some(unsafe { &node.as_ref().val })
    } else {
      None
    }
  }
}

struct LRUCache {
  cache: Lru<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
*/
impl LRUCache {
  fn new(capacity: i32) -> Self {
    Self {
      cache: Lru::new(capacity as usize),
    }
  }

  fn get(&mut self, key: i32) -> i32 {
    if let Some(v) = self.cache.get(&key) {
      *v
    } else {
      -1
    }
  }

  fn put(&mut self, key: i32, value: i32) {
    self.cache.insert(key, value);
  }
}

/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
*/
