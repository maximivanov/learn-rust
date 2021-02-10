// Slice. Does not have ownership. Contiguous sequence of collection elements.

fn main() {
    // (ptr, len, capacity), heap
    let s = String::from("hello world");
    let first_word_index = get_first_word_last_letter_index(&s);

    println!("first word index: {}", first_word_index);

    // slice of string. (ptr, len). ptr to the first element in heap.
    let hello = &s[0..5];
    let hello = &s[..5];
    let world = &s[6..];

    // string slice index must occur at valid utf8 boundary. 
    // Will error at runtime if index is in the middle of char byte sequence.

    let first_word = get_first_word(&s);
    println!("First word: {}", first_word);

    // this will FAIL because it tries to get mutable reference. 
    // But we already have an immutable reference (passed to get_first_word)
    s.clear();

    // s is of type &str. it is slice pointing to specific byte of the binary (code).
    let s_immutable = "immutable string";

    let first_word = get_first_word_improved(&s[..]);
    let first_word = get_first_word_improved(&s_immutable[..]);
    // same as previous because s_immutable is already a slice
    let first_word = get_first_word_improved(s_immutable);

    let arr = [1, 2, 3, 4, 5];
    // of type &[i32]
    let arr_slice = &arr[1..3];
}

fn get_first_word_last_letter_index(s: &String) -> usize {
    // convert to an array of bytes
    let bytes = s.as_bytes();

    // iter() returns elements one by one
    // enumerate wraps it and returns tuples (index, &element)
    // for (i, &item) - destructures tuple. index and single byte here
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// String slice is represented by &str type
fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        //return &s[..i];
      }
    }

    return &s[..]
}

// Improved to accept string slices. That way it's easier to use for String and string slice.
fn get_first_word_improved(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      //return &s[..i];
    }
  }

  return &s[..]
}
