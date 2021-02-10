// unlike immutable var:
// - can be in global scope
// - must be uppercase
// - type must be annotated
// - can be set to contant expression only, not something known at runtime (function call)
const C: u32 = 2;

// can use underscore in number literals for readability
const D: u32 = 10_000;

fn main() {
    let a = 1;
    // fails to compile - immutable by default
    // a = 2;
    println!("a is {}", a);

    let mut b = 1;
    b = 2;
    println!("b is {}", b);
    // can opt out to mutable if need to perform in-place change on the variable instead of copying/modifying its value to a new one

    println!("C is {}", C);

    // Shadowing (new variable shadows old)
    // Reuse variable name without making it mutable.
    // type can be changed
    let e = 1;
    let e = 2;
    let e = 3;
    println!("e is {}", e);
}
