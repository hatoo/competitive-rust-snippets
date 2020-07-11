#![allow(unused_attributes)]
#![cargo_snippet::snippet("template")]

///   _           _                    __
///  | |         | |                  / /
///  | |__   __ _| |_ ___   ___      / /
///  | '_ \ / _` | __/ _ \ / _ \    / /
///  | | | | (_| | || (_) | (_) |  / /   _ _   _                                 _                    _                  _
///  |_| |_|\__,_|\__\___/ \___/  /_/ | (_) | (_)                               | |                  (_)                | |
///    ___ ___  _ __ ___  _ __   ___| |_ _| |_ ___   _____ ______ _ __ _   _ ___| |_ ______ ___ _ __  _ _ __  _ __   ___| |_ ___
///   / __/ _ \| '_ ` _ \| '_ \ / _ \ __| | __| \ \ / / _ \______| '__| | | / __| __|______/ __| '_ \| | '_ \| '_ \ / _ \ __/ __|
///  | (_| (_) | | | | | | |_) |  __/ |_| | |_| |\ V /  __/      | |  | |_| \__ \ |_       \__ \ | | | | |_) | |_) |  __/ |_\__ \
///   \___\___/|_| |_| |_| .__/ \___|\__|_|\__|_| \_/ \___|      |_|   \__,_|___/\__|      |___/_| |_|_| .__/| .__/ \___|\__|___/
///                      | |                                                                           | |   | |
///                      |_|                                                                           |_|   |_|
/// https://github.com/hatoo/competitive-rust-snippets

#[allow(unused_imports)]
use std::cmp::{max, min, Ordering, Reverse};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};
#[allow(unused_imports)]
use std::iter::FromIterator;

mod util {
    use std::fmt::Debug;
    use std::io::{stdin, stdout, BufWriter, StdoutLock};
    use std::str::FromStr;

    #[allow(dead_code)]
    pub fn line() -> String {
        let mut line: String = String::new();
        stdin().read_line(&mut line).unwrap();
        line.trim().to_string()
    }

    #[allow(dead_code)]
    pub fn chars() -> Vec<char> {
        line().chars().collect()
    }

    #[allow(dead_code)]
    pub fn gets<T: FromStr>() -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let mut line: String = String::new();
        stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect()
    }

    #[allow(dead_code)]
    pub fn with_bufwriter<F: FnOnce(BufWriter<StdoutLock>)>(f: F) {
        let out = stdout();
        let writer = BufWriter::new(out.lock());
        f(writer)
    }
}

#[allow(unused_macros)]
macro_rules! get {
      ([$t:ty]) => {
          {
              let mut line: String = String::new();
              stdin().read_line(&mut line).unwrap();
              line.split_whitespace()
                  .map(|t| t.parse::<$t>().unwrap())
                  .collect::<Vec<_>>()
          }
      };
      ([$t:ty]; $n:expr) => {
          (0..$n).map(|_| get!([$t])).collect::<Vec<_>>()
      };
      ($t:ty) => {
          {
              let mut line: String = String::new();
              stdin().read_line(&mut line).unwrap();
              line.trim().parse::<$t>().unwrap()
          }
      };
      ($($t:ty),*) => {
          {
              let mut line: String = String::new();
              stdin().read_line(&mut line).unwrap();
              let mut iter = line.split_whitespace();
              (
                  $(iter.next().unwrap().parse::<$t>().unwrap(),)*
              )
          }
      };
      ($t:ty; $n:expr) => {
          (0..$n).map(|_|
              get!($t)
          ).collect::<Vec<_>>()
      };
      ($($t:ty),*; $n:expr) => {
          (0..$n).map(|_|
              get!($($t),*)
          ).collect::<Vec<_>>()
      };
  }

const BIG_STACK_SIZE: bool = true;

#[allow(dead_code)]
fn main() {
    use std::thread;
    if BIG_STACK_SIZE {
        thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .name("solve".into())
            .spawn(solve)
            .unwrap()
            .join()
            .unwrap();
    } else {
        solve();
    }
}

fn solve() {}
