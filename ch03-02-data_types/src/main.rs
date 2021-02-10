fn main() {
    // type of a variable is inferred by compiler from the value and (sometimes can't tell from just value) its usage below
    let a = 5;

    // sometimes type cannot be inferred (it won't compile)
    // Here it can be number of any type. we tell compiler explicitly
    let guess: u32 = "11".parse().expect("Error");

    // # 4 primary scalar types

    // ## Integer Types
    
    // prefix "i" - signed
    let a: i8 = 127;
    // prefix "u" - unsigned
    let b: u8 = 255;
    
    // bit sizes: 8, 16, 32, 64, 128
    // i/usize depend on computer architecture (32/64)
    let c: isize = 1;
    let d: usize = 2;

    // ways to write
    let a = 11_111; // decimal
    let a = 11_111u32; // also possible to set suffix
    let a = 0xff; // hex
    let a = 0o77; // octal
    let a = 0b1111_1111; // bin
    let a = b'A'; // byte, u8 only

    // integer overflow errors will panic at runtime when compiled in debug mode
    // they will wrap around in release mode: u8: 255+1 will be 0

    // ## Floating Point Types
    // f32 and f64 (default, on modern CPUs almost same speed as f32 but better precision)

    let x = 2.0; // f64
    let y : f32 = 3.0;

    // ## Boolean

    // 1 byte
    let f : bool = true;
    let f = false;

    // ## Char

    // 4 bytes
    // char literals defined with single quotes unlink string literals
    let c = 'c';
    let emoji: char = '👍';

    // # Compund Types

    // ## Tuple Type

    // fixed size, cannot grow/shrink
    let tup: (i32, f64, bool) = (11, 1.2, true);
    // destructuring
    let (a, b, c) = tup;
    // access individual element by index
    let c = tup.2;

    // ## Array Type

    // fixed size as well (for flexible length use Vector)
    let arr = [1, 2, 3];

    // define type explicitly
    let arr : [i32; 3] = [1, 2, 3];
    let arr = [1; 3]; // "1" 3 times. [1, 1, 1]

    let index = 10;
    // will panic at runtime
    let elem = arr[index];
}
