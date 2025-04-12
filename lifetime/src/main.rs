use std::fmt::Display;

fn main() {

    // lifetime은 댕글링 참조를 방지한다
    // 컴파일러에게 생존 기간을 알려주는 것이며, 같은 제네릭의 가장 짧은 lifetime을 lifetime으로 삼는다

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string3 = "urgent";

    longest_with_an_announcement(&string1.as_str(), string2, string3);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T,) -> &'a str where T: Display, {
    println!("Announcement! {}", ann);
    if x.len()> y.len(){
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}