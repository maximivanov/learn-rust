// At any given time, you can have either one mutable reference or any number of 
// immutable references.
// References must always be valid.

fn main() {
    let s1 = String::from("aaa");
    let len = length(&s1);

    let mut s1 = String::from("aaa");
    // pass a mutable reference
    change_string(&mut s1);

    // can create 2nd mutable ref within new scope
    {
      let r2 = &mut s1;
      println!("r2 {}", r2);
    }

    let r1 = &mut s1;
    // This FAILS 
    // can only have 1 mutable reference of a value in a scope at a time. Avoids data races.
    let r2 = &mut s1;
    println!("r1 {}, r2 {}", r1, r2);

    let mut s = String::from("aaa");
    let r1 = &s;
    let r2 = &s;
    // this FAILS because cannot have mutable ref while there's immutable one.
    // Because readers of immutable do not expect it to be changed.
    let r3 = &mut s;
    println!("{}, {}, {}", r1, r2, r3)

    fail_to_create_dangling_reference();
}

// having references in func parameters called "borrowing"
//
// Stack: s holds a pointer to s1 (stack too)
// s1 is (ptr to heap, len, capacity)
// Heap: "aaa"
fn length(s: &String) -> usize {
  // reference is mutable by default, won't work
  // s.push_str("sufffix");

  s.len()
} // s goes out of scope. But because reference does not own the value, it's not dropped.

// accept a mutable reference
fn change_string(s: &mut String) {

}

fn fail_to_create_dangling_reference() -> &String {
    let s = String::from("A");
    
    // This FAILS. Because s will get out of scope and get dropped
    &s
}