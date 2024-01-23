crate::sln!();

use std::collections::HashSet;

impl Solution {
  pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    struct Direction {
      all: [(i32, i32); 4],
      curr: i32,
    }
    impl Direction {
      fn get(&self) -> (i32, i32) {
        self.all[self.curr as usize]
      }
      fn update(&mut self, cmd: i32) {
        match cmd {
          -2 => {
            self.curr += 3;
            self.curr %= 4;
          }
          -1 => {
            self.curr += 1;
            self.curr %= 4;
          }
          _ => {}
        }
      }
    }
    impl Default for Direction {
      fn default() -> Self {
        Self {
          all: [(0, 1), (1, 0), (0, -1), (-1, 0)],
          curr: 0,
        }
      }
    }
    let mut dir = Direction::default();
    let mut pos = (0, 0);
    let mut ans = 0;
    let obstacles = obstacles
      .into_iter()
      .map(|v| (v[0], v[1]))
      .collect::<HashSet<_>>();
    commands.into_iter().for_each(|cmd| {
      if cmd < 1 {
        dir.update(cmd);
      } else {
        let (x, y) = dir.get();
        for _ in 0..cmd {
          pos.0 += x;
          pos.1 += y;
          if obstacles.contains(&pos) {
            pos.0 -= x;
            pos.1 -= y;
            break;
          }
          let dist = pos.0 * pos.0 + pos.1 * pos.1;
          ans = ans.max(dist);
        }
      }
    });
    ans
  }
}
