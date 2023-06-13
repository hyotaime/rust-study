#![allow(unused)]
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }
//
// fn main() {
//     let mut s = String::from("hello world");
//
//     let word = first_word(&s); // word는 5를 갖게 될 것입니다.
//
//     println!("{}", word);
//
//     s.clear(); // 이 코드는 String을 비워서 ""로 만들게 됩니다.
//     // word는 여기서 여전히 5를 갖고 있지만, 5라는 값을 의미있게 쓸 수 있는 스트링은 이제 없습니다.
// } // word는 이제 완전 유효하지 않습니다!

// String Slice
// fn main() {
//     let s = String::from("hello world");
//
//     let hello = &s[0..5];
//     let world = &s[6..11];
//     println!("{}", hello);
//     println!("{}", world);
//
//     let s = String::from("hello");
//
//     let slice = &s[0..2];
//     println!("{}", slice);
//     let slice = &s[..2];
//     println!("{}", slice);
//
// }

// fn main() {
//     fn first_word(s: &String) -> &str {
//         let bytes = s.as_bytes();
//
//         for (i, &item) in bytes.iter().enumerate() {
//             if item == b' ' {
//                 return &s[0..i];
//             }
//         }
//
//         &s[..]
//     }
//     fn second_word(s: &String) -> &str {
//         let bytes = s.as_bytes();
//
//         for (i, &item) in bytes.iter().enumerate() {
//             if item == b' ' {
//                 return &s[i+1..];
//             }
//         }
//
//         &s[0..0]
//     }
//     let mut s = String::from("hello world");
//
//     let first_word = first_word(&s);
//     let second_word = second_word(&s);
//
//     // s.clear(); // Error!
//
//     println!("the first word is: {}", first_word);
//     println!("the first word is: {}", second_word);
// }

fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    for i in slice {
        println!("{}", i);
    }
}