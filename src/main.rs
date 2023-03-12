use std::time::Duration;
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
  Red,
  Blue
}

struct Inventory {
  shirts: Vec<ShirtColor>
}

impl Inventory {
  fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
  }

  fn most_stocked(&self) -> ShirtColor {
    let mut num_red = 0;
    let mut num_blue = 0;

    for color in &self.shirts {
      match color {
        ShirtColor::Red => num_red += 1,
        ShirtColor::Blue => num_blue += 1,
      }
    }
    if num_red > num_blue {
      ShirtColor::Red
    } else {
      ShirtColor::Blue
    }
  }
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let store = Inventory {
    shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
  };

  let user_pref1 = Some(ShirtColor::Red);
  let giveaway1 = store.giveaway(user_pref1);
  println!(
    "the user with preference {:?} gets {:?}",
    user_pref1, giveaway1
  );

  let user_pref2 = None;
  let giveaway2 = store.giveaway(user_pref2);
  println!(
    "the user with preference {:?} gets {:?}",
    user_pref2, giveaway2
  );

  let _expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  };

  // let example_closure = |x| x;
  // let s = example_closure(String::from("hello world"));
  // let n = example_closure(5);

  let list = vec![1, 2, 3, 4, 5];
  println!("before defining closure: {:?}", list);

  let only_borrows = || println!("from closure: {:?}", list);

  println!("before calling closure: {:?}", list);
  only_borrows();
  println!("after calling closure: {:?}", list);

  println!();

  let other_list = vec![1, 2, 3, 4, 5];
  println!("before defining closure: {:?}", other_list);

  thread::spawn(move || println!("from thread: {:?}", other_list))
  .join()
  .unwrap();

  let mut another_list = [
    Rectangle { width: 10, height: 1 },
    Rectangle { width: 3, height: 8 },
    Rectangle { width: 6, height: 4 },
  ];

  another_list.sort_by_key(|r| r.width);
  println!("{:#?}", another_list);

  let mut num_sort_operations = 0;

  another_list.sort_by_key (|r| {
    num_sort_operations += 1;
    r.height
  });

  println!("{:#?}, sorted in {num_sort_operations} operations", another_list);

  // Processing a Series of Items with Iterators
  let v1 = vec![1, 2, 3, 4];
  let v1_iter = v1.iter();

  for val in v1_iter {
    println!("got: {}", val);
  }

  println!();

  let v2 = vec![1, 2, 3, 4];
  let v3: Vec<_> = v2.iter().map(|z| z + 1).collect();
  assert_eq!(v3, vec![2, 3, 4, 5]);

}