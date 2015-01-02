fn main() {
  let an_integer = 1u;
  let a_boolean = true;
  let unit = ();

  let copied_integer = an_integer;

  println!("An integer: {}", copied_integer);
  println!("A boolean: {}", a_boolean);
  println!("Meet the unit value: {}", unit);

  // the compiler should warn about one of these (hint: underscore has powers)
  let _unused_variable = 3u;
  let noisy_unused_variable = 2u;
}
