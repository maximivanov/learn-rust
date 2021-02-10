// ways to manage memory:
// 1. manual allocate and release (C). error prone
// 2. GC. (Javascript)
// 3. Ownership (Rust). Set of rules to manage memory checked at compile time

// Stack vs Heap
// Stack
//    Faster to write and read.
//    LIFO. Elements must be of fixed size.
// Heap
//    Changeable size. 
//    Program asks OS to get chunk of memory big enough and receives a pointer back.

// Ownership - manage data in heap
//    minimize amount of duplicate data
//    clean up unused data
// Each value in Rust has Owner var (and always one). When var goes out of scope, value is removed

fn main() {
    // stored in stack
    let s1 = "literal string";

    // allocated on the heap
    let s2 = String::from("String type");

    // both are pushed onto the stack, no heap.
    // value is copied since it's cheap (no difference between deep/shallow copy)
    let a = 1;
    let b = a;

    // Heap: "aaa". Stack: (pointer to heap, length, capacity)
    let s1 = String::from("aaa");
    // Heap: no change. Stack: s1 moved to s2. s1 no longer available. So there's ONE OWNER
    let s2 = s1;
    // will error on compile
    // println!("{}", s1);

    let s1 = String::from("aaa");
    // heap copied, both s1 and s2 in the stack.
    let s2 = s1.clone();

    // Stack-only types
    // integers, floats, bool, char, tuple (of simple types)
    // "Copy" trait on a type means it's stack only. Cannot be defined if Drop trait is implemented.

    // functions

    let s1 = String::from("aaa");
    // s1 moved to "s" inside function. s1 no longer valid.
    // Then s from inside function is moved to s2 here.
    let s2 = takes_ownership(s1);

} // at closing } special "drop" function runs. It will return the memory of vars going out of scope

fn takes_ownership(s: String) -> String {
  return s;
}