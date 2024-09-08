#[derive(Debug)]
struct Rectangle {
  width: i32,
  height: i32,
}

impl Rectangle {
  fn area(&self) -> i32 {
    return self.height * self.width;
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let rect = Rectangle {
    width: 4,
    height: 5
  };

  println!("area of rectangle: {}", rect.area());
  println!("can hold: {}", rect.can_hold(&Rectangle { width: 2, height: 1}))
}
