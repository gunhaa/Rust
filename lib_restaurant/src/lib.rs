// cargo new --lib restaurant



mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
    
        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}

        fn serve_order(){}

        fn take_payment(){}
    }
}

fn deliver_order(){}

mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();

        // super는 부모모듈, 즉 root를 의미한다
        super::deliver_order();
    }

    fn cook_order(){}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast:String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant(){
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();

    // Rye toast를 곁들인 여름철 조식 주문
    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    // 먹고 싶은 빵 바꾸기
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast plz", meal.toast);

    // 컴파일 불가
    // meal.seasonal_fruit = String:;from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
