fn main() {
    move_string();
    move_scalar();

    ownership_fn();
    ownership_return();

    ownership_reference();
    ownership_mut_reference();
    ownership_slice_type();
}

fn move_string(){
    let s1 = String::from("hello");

    // 다른 언어라면 해당 코드가 얕은 복사를 해 포인터, 길이, 용량 값만을 복사하지만
    // Rust에서는 얕은 복사가 아닌 move가 일어난다.
    // move가 일어나게 된다면 s1은 사라지게 된다(메모리 이중 해제 해결)
    let _s2 = s1;

    println!("rust에서 해당 참조는 허용하지 않는다 : s1, world");
}

fn move_scalar(){
    let x = 5;
    let y = x;
    println!("rust에서 스칼라 값은 이동해도 문제가 없다 즉, Stack이 아닌 메모리 해제가 필요한 Heap에 저장되는 값만 move에 문제가 생긴다: x= {x}, y= {y}");
}

fn ownership_fn(){
    let s = String::from("hello"); // s가 스코프 안으로 들어옴

    takes_ownership(s); // s의 값이 함수로 이동되며, s는 이곳에서 유효하지 않아진다
    println!("can't use s : s");

    let x = 5; // x가 스코프 안으로 들어온다

    makes_copy(x); // x가 함수 안으로 이동되지만, i32는 Copy가 구현되어 있어 앞으로도 x를 사용 할 수 있다.
    println!("can use x : {x}");

    fn takes_ownership(some_string : String){
        println!("{}", some_string);
    } // some_string이 스코프 밖으로 벗어나고 drop이 호출되어 메모리가 해제된다
    
    fn makes_copy(some_integer: i32){
        println!("{}", some_integer);
    } // some_integer이 스코프 밖으로 벗어난다, 별다른일이 생기지 않는다
}

fn ownership_return(){
    let s1 = gives_ownership(); // 반환 값을 s1으로 이동시키낟
    println!("ownership move : {s1}");

    let s2 = String::from("hello"); // s2가 스코프로 들어온다
    println!("ownership generate : {s2}");

    let s3 = takes_and_gives_back(s2); // s2는 이동되며, 자신의 반환값을 s3로 이동 시킨다
    println!("ownership move : {s3}");

    fn gives_ownership() -> String{
        let some_string = String::from("yours");
        some_string
    }

    fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로 들어온다

        a_string // a_string이 반환되고 호출자 함수 쪽으로 이동한다.
    }
} // s1, s3는 스코프 밖으로 벗어나며 버려진다. 이미 이동된 s2는 아무일도 일어나지 않는다

fn ownership_reference(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of {} is {} : 참조자를 넘기면 소유권 이전이 일어나지 않고 참조(대여) 할 수 있도록 해준다.", s1, len);

    fn calculate_length(s:&String) -> usize{
        s.len()
    }
}

fn ownership_mut_reference(){
    let mut s = String::from("hello");

    change(&mut s);

    println!("mut reference를 넘겨야 값을 바꿀 수 있다");
    println!("mut ref는 한개만 생성가능하며, 두개는 생성이 불가하다. 이 특성으로 인해 race condition을 방지한다.");

    fn change(some_string: &mut String){
        some_string.push_str(", world");
    }
}

fn ownership_slice_type(){

    let mut s = String::from("hello world");

    let word = first_word(&s);

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for(i, &item) in bytes.iter().enumerate() {
            if item == b' '{
                return &s[0..i];
            }
        }

        &s[..]
    }

    println!("참조 방식을 반환해여 해당 문자열을 다뤄도 값을 유지시킬수 있다(컴파일 타임 안전 보장) : {word}")
}