// This function takes ownership of the box
fn eat_box(boxed_int: Box<int>) {
    println!("destroying box that contains {}", boxed_int);
}

// This function borrows the box
fn peep_inside_box(borrowed_box: &Box<int>) {
    println!("This box contains {}", borrowed_box);
}

fn main() {
    // A boxed integer
    let boxed_int = box 5;

    // Borrow the box, ownership is not taken
    peep_inside_box(&boxed_int);

    // The box can be borrwed again
    peep_inside_box(&boxed_int);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_int: &int = &*boxed_int;

        // ERROR! Can't destroy boxed_int, while the inner value has been borrowed
        // eat_box(boxed_int);

        // '_ref_to_int' goes out of scope
    }

    // Give up ownership of the box
    eat_box(boxed_int);
}
