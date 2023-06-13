#![allow(unused)]
// fn main() {
//     let s1 = String::from("hello");
//
//     let len = calculate_length(&s1);
//
//     println!("The length of '{}' is {}.", s1, len);
// }
//
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let s = String::from("hello");
//
//     change(& s);
//
//     println!("{}", s);
// }
//
// fn change(some_string: &String) {
//     // some_string.push_str(", world!"); // 참조한 것은 변경할 수 없음
// }

// // 가변 참조자
// // 가변 참조자는 스코프 내에 하나만 만들 수 있다!!
// fn main() {
//     let mut s = String::from("hello");
//
//     change(&mut s);
//
//     println!("{}", s);
// }
//
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");
//
//     {
//         let r1 = &mut s;
//
//     } // 여기서 r1은 스코프 밖으로 벗어났으므로, 우리는 아무 문제 없이 새로운 참조자를 만들 수 있습니다.
//
//     let r2 = &mut s;
// }

// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = &s; // 문제 없음
//     let r2 = &s; // 문제 없음
//     let r3 = &mut s; // 큰 문제
//
//     println!("{}", r1);
//     println!("{}", r2);
//     println!("{}", r3);
// }

// Dangling Reference
fn main() {
    // let reference_to_nothing = dangle();
    let no_dangle = no_dangle();
    println!("{}", no_dangle);
}
// fn dangle() -> &String { // dangle은 String의 참조자를 반환합니다
//
//     let s = String::from("hello"); // s는 새로운 String입니다
//
//     &s // 우리는 String s의 참조자를 반환합니다.
// } // 여기서 s는 스코프를 벗어나고 버려집니다. 이것의 메모리는 사라집니다.
// 위험하군요!
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}