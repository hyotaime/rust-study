// #![allow(unused)]
// fn main() {
//     // {                      // s는 유효하지 않습니다. 아직 선언이 안됐거든요.
//     //     let s = "hello";   // s는 이 지점부터 유효합니다.
//     //
//     //     // s를 가지고 뭔가 합니다.
//     // }                      // 이 스코프는 이제 끝이므로, s는 더이상 유효하지 않습니다.
//
//     // let s = String::from("hello");
//
//     // let mut s = String::from("hello");
//     //
//     // s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.
//     //
//     // println!("{}", s); // 이 부분이 `hello, world!`를 출력할 겁니다.
//
//     // let s1 = String::from("hello");
//     // let s2 = s1;
//     //
//     // println!("{}, world!", s1); // ERROR
//
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//
//     println!("s1 = {}, s2 = {}", s1, s2);
//
//     // 정수형은 컴파일 타임에 크기가 결정되어 스택에 저장되기 때문에 무효화 할 필요가 없음.
//     let x = 5;
//     let y = x;
//
//     println!("x = {}, y = {}", x, y);
// }

// fn main() {
//     let s = String::from("hello");  // s가 스코프 안으로 들어왔습니다.
//
//     takes_ownership(s);             // s의 값이 함수 안으로 이동했습니다...
//     // ... 그리고 이제 더이상 유효하지 않습니다.
//     // println!("{}", s); // ERROR!
//
//     let x = 5;                      // x가 스코프 안으로 들어왔습니다.
//
//     makes_copy(x);                  // x가 함수 안으로 이동했습니다만,
//     // i32는 Copy가 되므로, x를 이후에 계속
//     // 사용해도 됩니다.
//     println!("{}", x);
//
// } // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나갑니다. 하지만 s는 이미 이동되었으므로,
// // 별다른 일이 발생하지 않습니다.
//
// fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔습니다.
//     println!("{}", some_string);
// } // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
// // 해제되었습니다.
//
// fn makes_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어왔습니다.
//     println!("{}", some_integer);
// } // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.

// fn main() {
//     let s1 = gives_ownership();         // gives_ownership은 반환값을 s1에게
//     // 이동시킵니다.
//
//     let s2 = String::from("hello");     // s2가 스코프 안에 들어왔습니다.
//
//     let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back 안으로
//     // 이동되었고, 이 함수가 반환값을 s3으로도
//     // 이동시켰습니다.
//
//     println!("{}", s1);
//     // println!("{}", s2); // ERROR!
//     println!("{}", s3);
//
// } // 여기서 s3는 스코프 밖으로 벗어났으며 drop이 호출됩니다. s2는 스코프 밖으로
// // 벗어났지만 이동되었으므로 아무 일도 일어나지 않습니다. s1은 스코프 밖으로
// // 벗어나서 drop이 호출됩니다.
//
// fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을
//     // 호출한 쪽으로 이동시킵니다.
//
//     let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.
//
//     some_string                              // some_string이 반환되고, 호출한 쪽의
//     // 함수로 이동됩니다.
// }
//
// // takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
// fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프
//     // 안으로 들어왔습니다.
//
//     a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
// }

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()함수는 문자열의 길이를 반환합니다.

    (s, length)
}
// 튜플은 투머치다 => 레퍼런스 사용
