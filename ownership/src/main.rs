fn main() {

  // Variables live on the stack frame defined within the function scope
  let a = 5;

  // When an expression reads a variable, it copies the value from the stack frame for usage
  let mut b = a;
  b += 1;

  println!("a = {a}, b = {b}"); // a = 5, b = 6

  // A pointer is a value that describes a location in memory.
  // The value that a pointer points-to is called its pointee.
  // One common way to make a pointer is to allocate memory in the heap.
  // Data can live indefinitely in the heap.
  // Heap data is NOT TIED to a specific stack frame.

  let _a = Box::new([0; 1_000_000]);
  // _a is a pointer that points to the 1 million long array which is stored in the heap
  // Box is used to create data on the heap
  
  // A stack frame is automatically deallocated when function ends.
  // How is heap data deallocated? If manually deallocation is allowed, then there
  // may be dangling pointers issue. The pointer would point to an invalid
  // memory location. Therefore, Rust does not allow this
  // When a heap-allocated variable goes out of scope, the heap data
  // corresponding to that variable is deallocated.

  let _x = 4;
  {
    let _y = Box::new(5);
    // inside this scope, the pointer _y points to the location that holds the value 5
  }
  // outside the scope, since _y does not exist, Rust frees up the location that held 5

  // Now consider this code:
  let a = Box::new([2,3,4]);
  let b = a;
  
  // Here, let's say that we want to free the memory space, then it would free
  // for both a and b making it a double clear.
  // So to prevent this, Rust introduces the concept of ownership.
  // First `a` has the ownership to the Box.
  // When we assign the pointer `a` to the variable `b`, we render `a` moot.
  // In doing so if `a` is used again in the code, it would throw error
  // since the ownership of `a`'s data has transferred to `b`

  //  If a variable owns a box, when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.
  // Now, `b` owns the Box so ONLY WHEN scope of `b` ends will the heap data be
  // deallocated. If the ownership is transferred, then the heap data would still
  // remain, now bound to the new variable

  let a = Box::new(6);
  {
    let _b = a;
  }

  // Below code would not compile since the ownership of a has been moved so `a` is
  // now undefined.
  // println!("a = {}", a);

  let first = String::from("Hello");
  let greeting = make_greeting(first);
  println!("{greeting}");

  // In the above example:
  // first points to the heap location where "Hello" is stored.
  // When the `make_greeting` function is called, the ownership of `first` is
  // transferred to `name` rendering `first` moot. Now `name` appends John to the
  // string and gives the ownership to `greeting`. Since `name` exists within the scope of
  // `make_greeting`, when the function returns, the variable scope for `name`
  // ends and it is removed. Now `greeting` owns the heap data. `Hello John`.

  // Remember: it's not a problem that first points to deallocated memory.
  // It's a problem that we tried to use first after it became invalid.

  // Moved heap data principle: if a variable x moves ownership of heap
  // data to another variable y, then x cannot be used after the move.

  let b = Box::new(0);
  let b2 = b;
  // println!("{}", b); Incorrect since b's ownership has been transferred
  move_a_box(b2);

  // Ownership is primarily a discipline of heap management:
  // - All heap data must be owned by exactly one variable.
  // - Rust deallocates heap data once its owner goes out of scope.
  // - Ownership can be transferred by moves, which happen on assignments and function calls.
  // - Heap data can only be accessed through its current owner, not a previous owner.

  // References: a way in Rust to share pointer in different places without changing
  // actual ownership of the data.
  let m1 = String::from("Hello");
  let m2 = String::from("World");
  greet(&m1, &m2); // immutable references to m1 and m2 respectively
  println!("m1: {m1}, m2: {m2}");

  // References are non-owning pointers, because they do not own the data they point to.

  // Dereferencing a pointer allows us to access its data.
  // Note, if we are dereferencing a "reference" then the value obtained is the
  // original reference.
  // There can be multiple chained/hierarchical references (or reference of reference)
  let s1 = String::from("Hello");
  let r1 = &s1; // immutable reference of s1
  let r2 = &r1; // immutable reference of r1 (reference of reference).
  let _r3 = &*r1; // destructures reference first to get String and references that

  println!("s1: {s1}");
  println!("Destructuring once, s1: {}", *r1);
  println!("Destructuring twice, s1: {}", **r2);

  // calling a method with dot operator in some cases means implicit dereference
  let x = Box::new(-1);
  let s1 = String::from("World");
  println!("Abs of x: {}", i32::abs(*x)); // explicit dereference
  println!("Abs of x: {}", x.abs()); // implicit dereference
  println!("Length of s1: {}", str::len(&s1));
  println!("Length of s1: {}", s1.len());

  let x = Box::new(0);
  let y = Box::new(&x);
  println!("{}", *y);

  // Aliasing (referencing) along with mutation is extremely dangerous. Cases:
  // - By deallocating the aliased data, leaving the other variable to point to deallocated memory.
  // - By mutating the aliased data, invalidating runtime properties expected by the other variable.
  // - By concurrently mutating the aliased data, causing a data race with nondeterministic behavior for the other variable.

  let mut v = vec![1,2,3]; // Len:3, Cap: 3
  v.push(4); // Len: 4, Cap: 6
  // When the vector is at capacity and a new element is pushed, then the vector
  // doubles its capacity, copies all the elements, moves them (maybe)
  // to a new location and deallocates original vector in the heap.

  let mut v = vec![1];
  let num = &v[0];
  // v.push(2);
  println!("{}", *num);
  // num is an immutable borrow of vector v. Now if we attempt to perform the
  // `push` operation, that is an implicit mutable borrow of vector v. This is not
  // allowed and the code would not compile.

  // There can be multiple immutable borrows of a variable (given no mutable borrow).
  // There can only be a single mutable borrow of a variable. IF there is a mutable
  // borrow for a variable, then no immutable borrows are allowed.
  // While a borrow is active, ownership for the borrowed variable cannot change

  // Data can be aliased. Data can be mutated. But data cannot be both aliased and mutated

}

// This function won't compile because Rust does not know if the output reference (&String) is
// coming from `strings` or `default`. Therefore Rust would not know which variable
// in the caller function can be moved/written and this is not allowed.
// fn first_or(strings: &Vec<String>, default: &String) -> &String {
//   if strings.len() > 0 {
//       &strings[0]
//   } else {
//       default
//   }
// }
// Use Lifetime Parameters for this

fn greet(g1: &String, g2: &String) { // &String means a reference to a String
  // g1 and g2 are references to m1 and m2 on the stack, m1 and m2 point to actual data
  println!("{g1} {g2}");
}

fn make_greeting(mut name: String) -> String {
  name.push_str("John");
  name
}

fn move_a_box(b: Box<i32>) {
  println!("{}", b);
}