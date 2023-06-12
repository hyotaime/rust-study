// fn main() {
//     // let number = 3;
//     let number = 7;
//
//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// fn main() {
//     let number = 3;
//
//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }

// fn main() {
//     let number = 6;
//
//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main() {
//     let condition = true;
//     let number = if condition {
//         5
//     } else {
//         6
//     };
//
//     println!("The value of number is: {}", number);
// }

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// fn main() {
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{}!", number);
//
//         number -= 1;
//     }
//
//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;
//
//     while index < 5 {
//         println!("the value is: {}", a[index]);
//
//         index = index + 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//
//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }
// }

fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}