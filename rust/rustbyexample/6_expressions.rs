fn main() {
  let x = 5u;

  let y = {
    let x_squared = x * x;
    let x_cube = x_squared * x;

    x_cube + x_squared + x
  };

  // Will be assigned () unit because of the semicolon
  let z = {
    2 * x;
  };

  println!("x is {}", x);
  println!("y is {}", y);
  println!("z is {}", z);
}