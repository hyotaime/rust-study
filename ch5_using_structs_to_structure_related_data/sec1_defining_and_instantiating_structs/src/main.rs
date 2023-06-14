// fn main() {
//     struct User {
//         username: String,
//         email: String,
//         sign_in_count: u64,
//         active: bool,
//     }
//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };
//     println!("{} {} {} {}", user1.email, user1.username, user1.active, user1.sign_in_count);
//
//     user1.email = String::from("anotheremail@example.com");
//     println!("{} {} {} {}", user1.email, user1.username, user1.active, user1.sign_in_count);
//
//     fn build_user(email: String, username: String) -> User {
//         User {
//             email, // email: email <= 생략가능
//             username, // username: username <= 생략가능
//             active: true,
//             sign_in_count: 1,
//         }
//     }
//
//     let user2 = User {
//         email: String::from("user2@example.com"),
//         ..user1
//     };
//     println!("{} {} {} {}", user2.email, user2.username, user2.active, user2.sign_in_count);
//
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);
//
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//
//
//
// }

struct User {
    // username: &str,
    // email: &str,
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}