fn main() {
  if_then_else();

  iterate_with_loop();

  iterate_with_while();

  iterate_with_for();
}

fn if_then_else() {
  let a = 1;

  // fails, no automatic conversion to bool
  // if a { ... }
  // works
  if a != 0 {
    // number is non-zero
  }

  // multiple if else (3 "arms")
  if a == 1 {
    //
  } else if a == 2 {
    //
  } else {
    //
  }

  // if is an expression so can use it with let
  let is_positive = if a > 0 { true } else { false };
}

fn iterate_with_loop() {
    // simple loop
    loop {
      println!("infinite iteration");
      break;
    }

    // loop with return value
    let mut i = 0;
    
    let result = loop {
      i = i + 1;
      if i == 10 {
        break i * 2;
      }
    };

    println!("result it {}", result);
}

fn iterate_with_while() {
  let mut i = 1;

  while i < 3 {
    i += 1;
    println!("while iteration...");
  }

  println!("i is {}", i);
}

fn iterate_with_for() {
  let arr = [1, 2, 3];

  for elem in arr.iter() {
    println!("arr elem {}", elem);
  }

  // (1..5) returns value of std Range type, which generates sequence of numbers
  for elem in (1..5).rev() {
    println!("arr elem 2 {}", elem);
  }
}