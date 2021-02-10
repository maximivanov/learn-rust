fn main() {
  let res = sum(1, 2);
  println!("sum is {}", res);

  // let a = ...; is a statement (does not return value)
  // 1 is an expression
  // { ... } is an expression
  let a = {
    let b = 1;
    // expressions do not include semicolons
    b + 1
  };

  println!("calculated a = {}", a);

  nested_print();
}

fn nested_print() {
  println!("I'm not returning anything");
}

fn sum(a: i32, b: i32) -> i32 {
  // can do return a + b to exit early

  // last expression is an implicit result of function
  a + b
}