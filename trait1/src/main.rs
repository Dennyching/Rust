trait Movable {
    fn move1(&self);
  }
  
struct Human;
  
impl Movable for Human {
    fn move1(&self) {
        println!("Human walk");
    }
}
  
struct Rabbit;
  
impl Movable for Rabbit {
    fn move1(&self) {
      println!("Rabbit jump");
    }
}
trait Greeter {
  fn greet(&self) {
    println!("{}", self.message());
  }

  fn greet_to(&self, name: &str) {
    println!("{} {}", self.message(), name);
  }

  fn message(&self) -> &'static str;
}

struct Someone;

impl Greeter for Someone {
  // 提供必要的方法
  fn message(&self) -> &'static str {
    "Hello"
  }

  // 覆寫 (override) 預設實作
  fn greet_to(&self, name: &str) {
    println!("Yo {}", name);
  }
}
trait HasName: Greeter {
    fn name(&self) -> &'static str;

    fn greet_with_name(&self) {
        println!("{} my name is {}", self.message(), self.name());
    }
}
struct Point(i32, i32);

// 當然這邊你可以先用 use std::fmt::Display; 這樣這邊就只需要使用 Display
impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}
use std::{ops::Deref, fmt};

#[derive(Copy, Clone)]
struct Num(i32);

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 直接呼叫被包裝的 i32 所實作的 fmt::Display
        fmt::Display::fmt(&self.0, f)
    }
}

impl Num {
  fn inc_one(self) -> Self {
    Num(self.0 + 1)
  }
}

impl Deref for Num {
  type Target = i32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

fn main() {
    let  user1 = Human;
    let  user2 = Rabbit;
    user1.move1();
    user2.move1();
    let user3 = Someone;
    user3.message();
    let point = Point(1,1);
    println!("{}",point);
    let n = Num(42);
    println!("{}", n.inc_one()); // n 可以有新定義的方法
    println!("{}", n.abs()); // n 也可以有原本定義的方法
    //println!("Hello, world!");
}
