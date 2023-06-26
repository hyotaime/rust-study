pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

// pub use로 다른 스코프에서도 사용할 수 있다
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // use로 선언
    hosting::add_to_waitlist();

    // 호밀 (Rye) 토스트를 곁들인 여름철 조식 주문하기
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 먹고 싶은 빵 바꾸기
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // 다음 라인의 주석을 해제하면 컴파일되지 않습니다
    // 식사와 함께 제공되는 계절 과일은 조회나 수정이 허용되지 않습니다
    // meal.seasonal_fruit = String::from("blueberries");

    // 반면 열거형 enum public으로 선언하면 모든 배리언트가 public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
mod customer {
    pub fn eat_at_restaurant() {
        // 자식 모듈로 옮긴 후에는 use의 스코프를 벗어남
        // hosting::add_to_waitlist();
    }
}
fn deliver_order() {}

use std::fmt::Result;
// as로 이름을 재정의
use std::io::Result as IoResult;
fn function1() -> Result {
    // --생략--
    Ok(())
}
fn function2() -> IoResult<()> {
    // --생략--
    Ok(())
}

