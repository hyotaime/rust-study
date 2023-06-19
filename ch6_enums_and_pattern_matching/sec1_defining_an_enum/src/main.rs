// use std::net::{IpAddr, Ipv6Addr};
//
// fn main() {
//     // enum IpAddrKind {
//     //     V4,
//     //     V6,
//     // }
//     //
//     // struct IpAddr {
//     //     kind: IpAddrKind,
//     //     address: String,
//     // }
//     // let home = IpAddr {
//     //     kind: IpAddrKind::V4,
//     //     address: String::from("127.0.0.1"),
//     // };
//     // let loopback = IpAddr {
//     //     kind: IpAddrKind::V6,
//     //     address: String::from("::1"),
//     // };
//
//     // enum IpAddr {
//     //     V4(String),
//     //     V6(String),
//     // }
//     // let home = IpAddr::V4(String::from("127.0.0.1"));
//     // let loopback = IpAddr::V6(String::from("::1"));
//
//     // enum IpAddr {
//     //     V4(u8, u8, u8, u8),
//     //     V6(String),
//     // }
//     // let home = IpAddr::V4(127, 0, 0, 1);
//     // let loopback = IpAddr::V6(String::from("::1"));
//
//     // struct Ipv4Addr {
//     //     // details elided
//     // }
//     //
//     // struct Ipv6Addr {
//     //     // details elided
//     // }
//     //
//     // enum IpAddr {
//     //     V4(Ipv4Addr),
//     //     V6(Ipv6Addr),
//     // }
//
//     enum Message {
//         Quit,
//         Move {x: i32, y: i32 },
//         Write(String),
//         ChangeColor(i32, i32, i32),
//     }
//     // ==
//     struct QuitMessage;
//     struct MoveMessage {
//         x: i32,
//         y: i32,
//
//     }
//     struct WriteMessage(String);
//     struct ChangeColorMessage(i32, i32, i32);
//
//     impl Message {
//         fn call(&self) {
//             // method
//             println!("Call's Method");
//         }
//     }
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// Option형
fn main() {
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // let some_number = Some(5);
    // let some_string = Some("a string");
    //
    // let absent_number: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    //
    // let sum = x + y; // ERROR!
}