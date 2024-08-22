fn another_func() {
  println!("from another function");
}

fn func_with_params(i: i8, s: String) {
  println!("i = {i}, s = {s}");
}

fn func_with_return(a: i8, b: i8) -> i8 {
  a + b
}

fn main() {

  // variables mutability
  let mut x = 6;
  println!("x = {x}");
  x = 5;
  println!("x = {x}");

  // constants
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("{THREE_HOURS_IN_SECONDS}");

  // shadowing
  let a = 5;
  let a = a + 2;

  {
    let a = a * 3;
    println!("a inside scope: {a}");
  }
  println!("a outside scope: {a}");

  // compound data types: group multiple values into one type
  let tup: (i32, u8, f32, String) = (-23, 3, 3.14, String::from("hello"));
  let (_, x, _, _) = tup; // destructuring of tuple
  println!("0th element: {}, 1st element: {x}", tup.0); // access tuple elements with index

  // unit tuple
  let _unit_tuple: () = ();

  // arrays - all elements same type (fixed size). Variable size - vector
  let arr = [1, 2, 3, 4];
  let a = [3; 5]; // array of 5 elements all with value 3
  println!("arr[0]: {}, a[3]: {}", arr[0], a[3]);

  // calling function
  another_func();
  func_with_params(23, String::from("hello world"));

  // scoped expressions
  let y = {
    let x = 3;
    x + 10
  };
  println!("y = {y}");

  // call a function that returns a value
  let sum = func_with_return(2, 4);
  println!("sum: {sum}");

  // if-else
  let num = 3;
  if num < 3 {
    println!("less than 3");
  } else if num == 3 {
    println!("equal to 3");
  } else {
    println!("greater than 3");
  }

  let tmp = if num < 3 { "less than 3" } else if num == 3 { "equal to 3" } else { "greater than three" };
  println!("tmp: {tmp}");

  // loops
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter > 10 {
      break counter;
    }
  };
  println!("result: {result}");

  // nested loops and labels
  let mut counter = 0;
  'counting_loop: loop {
    println!("counter: {counter}");
    let mut remaining = 0;
    loop {
      println!("remaining: {remaining}");
      remaining += 1;

      if remaining > 5 {
        if counter == 2 {
          break 'counting_loop
        }
        break
      }
    }
    counter += 1
  }

  // while loop
  let mut number = 3;
  while number != 0 {
    println!("{number}!");
    number -= 1;
  }
  println!("number: {number}");

  // for loop
  let a = [1,2,3,4,5];
  for el in a {
    println!("element: {el}");
  }
}
