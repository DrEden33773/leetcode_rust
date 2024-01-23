#[derive(Default)]
struct Trie([Option<Box<Trie>>; 26], bool);

impl Trie {
  fn new() -> Self {
    Default::default()
  }

  fn insert(&mut self, word: String) {
    let mut node = self;
    for byte in word.bytes() {
      let i = (byte - b'a') as usize;
      node = node.0[i].get_or_insert_with(|| Trie::new().into());
    }
    node.1 = true;
  }

  fn search(&self, word: String) -> bool {
    let mut node = self;
    for byte in word.bytes() {
      let i = (byte - b'a') as usize;
      if let Some(n) = node.0[i].as_ref() {
        node = n;
      } else {
        return false;
      }
    }
    node.1
  }

  fn starts_with(&self, prefix: String) -> bool {
    let mut node = self;
    for c in prefix.chars() {
      let idx = (c as u8 - b'a') as usize;
      if let Some(n) = node.0[idx].as_ref() {
        node = n;
      } else {
        return false;
      }
    }
    true
  }
}
