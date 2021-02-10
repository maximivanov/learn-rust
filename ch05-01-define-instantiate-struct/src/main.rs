struct User {
  email: String,
  active: bool,
}

// basically named tuple
struct Color (i32, i32, i32);

fn main() {
    let user1 = User {
      email: String::from("asda@asdsd.com"),
      active: true,
    };

    let mut user2 = User {
      email: String::from("asd"),
      active: false,
    };
    user2.active = false;

    let user3 = build_user(String::from("asd"), false);

    let user4 = struct_update_syntax(user3, true);

    let black_color = tuple_struct(0, 0, 0);

    struct A {
      name: &str, // FAILS, requires lifetime
    }
}

fn build_user(email: String, active: bool) -> User {
    User {
      // "field init shorthand"
      email,
      active,
    }
}

fn struct_update_syntax(u1: User, active: bool) -> User {
    User {
      active,
      ..u1
    }
}

fn tuple_struct(a: i32, b: i32, c: i32) -> Color {
  Color (a, b, c)
}