// First attempt: no explicit lifetimes
// ERROR! Compiler needs explicit lifetimes
// struct Singleton {
//     one: &mut int,
// }

// Second attempt: Add lifetimes to all the references
struct Pair<'a, 'b> {
    one: &'a mut int,
    two: &'b mut int,
}

fn main() {
    // Let's say that 'one' has lifetime 'o'
    let mut one = 1;

    {
        // And that 'two' has lifetime 't'
        // 'two' has a shorter (and different) lifetime than 'one' (`'t < 'o`)
        let mut two = 2;

        println!("Before: ({}, {})", one, two);

        // 'Pair' gets specialize for `'a = 'o` and `'b = 't`
        let pair = Pair { one: &mut one, two: &mut two };

        *pair.one = 2;
        *pair.two = 1;

        println!("After: ({}, {})", pair.one, pair.two);
    }
}
