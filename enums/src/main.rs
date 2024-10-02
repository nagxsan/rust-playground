// Enum without values
#[derive(Debug)]
enum Direction {
  North,
  South,
  East,
  West
}

// Enum with values
#[derive(Debug)]
enum Shape {
  Circle(f64), // f64 radius of circle
  Square(f64), // f64 side of square
  Rectangle(f64, f64) // f64 length and width of the rectangle
}

fn move_around(dir: &Direction) -> Direction {
  Direction::East
}

fn calc_area(shape: &Shape) -> f64 {
  match shape {
    Shape::Circle(r) => 3.14 * r * r,
    Shape::Square(s) => s * s,
    Shape::Rectangle(l, w) => l * w
  }
}

fn main() {
  let curr_dir = Direction::North;
  let mut inc_dir = Direction::North;
  // inc_dir = "temp" won't work since inc_dir has type Direction

  let new_dir = move_around(&curr_dir);
  println!("Old direction: {:?}", curr_dir);
  println!("Old direction: {:?}", new_dir);

  let circle = Shape::Circle(5.0);
  let square: Shape = Shape::Square(4.5);
  let rectangle: Shape = Shape::Rectangle(3.0, 4.0);

  println!("Area of circle: {}", calc_area(&circle));
  println!("Area of circle: {}", calc_area(&square));
  println!("Area of circle: {}", calc_area(&rectangle));
}
