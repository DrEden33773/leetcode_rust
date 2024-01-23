#[derive(Default)]
struct Trie(pub [Option<Box<Trie>>; 26], pub bool);

impl Trie {
  #[inline]
  fn new() -> Self {
    Default::default()
  }

  #[inline]
  fn insert(&mut self, word: String) {
    let mut node = self;
    for byte in word.bytes() {
      let i = (byte - b'a') as usize;
      node = node.0[i].get_or_insert_with(|| Trie::new().into());
    }
    node.1 = true;
  }
}

#[derive(Default)]
struct WordDictionary {
  trie: Trie,
}

impl WordDictionary {
  fn new() -> Self {
    Self::default()
  }

  fn add_word(&mut self, word: String) {
    self.trie.insert(word)
  }

  fn dfs(&self, trie: &Trie, left: &[u8]) -> bool {
    if left.is_empty() {
      return trie.1;
    }
    let c = left[0];
    if c == b'.' {
      for n in trie.0.iter().filter_map(|n| n.as_ref()) {
        if self.dfs(n, &left[1..]) {
          return true;
        }
      }
      false
    } else {
      let i = (c - b'a') as usize;
      if let Some(n) = trie.0[i].as_ref() {
        self.dfs(n, &left[1..])
      } else {
        false
      }
    }
  }

  fn search(&self, word: String) -> bool {
    self.dfs(&self.trie, word.as_bytes())
  }
}
