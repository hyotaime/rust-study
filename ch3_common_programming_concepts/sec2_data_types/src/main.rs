fn main() {
    // let guess: u32 = "42".parse().expect("Not a number!");

    // 스칼라 타입들

    let x = 2.0; // f64
    println!("{}", x);

    let y: f32 = 3.0; // f32
    println!("{}", y);

    // 수학적 연산들

    // addition
    let sum = 5 + 10;
    println!("{}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{}", difference);

    // multiplication
    let product = 4 * 30;
    println!("{}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("{}", remainder);

    // boolean 타입

    let t = true;
    println!("{}", t);

    let f: bool = false; // with explicit type annotation
    println!("{}", f);

    // 문자 타입

    let c = 'z';
    println!("{}", c);
    let z = 'ℤ';
    println!("{}", z);
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    // 복합 타입들

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x, y, z is: {}, {}, {}", x, y, z);
    println!("The value of tup is: {}, {}, {}", tup.0, tup.1, tup.2);

    // 배열

    let a = [1, 2, 3, 4, 5];
    let index = 4;  // 5이상은 outOfBounds ERROR

    let element = a[index];

    println!("The value of element is: {}", element);

}
