fn main() {
    vector();
    string();
}

fn vector(){
    // Rust의 vector는 메모리에서 모든 값을 이웃하게 배치하는 단일 데이터 구조에 하나 이상의 값을 저장할수 있음
    let mut v: Vec<i32> = Vec::new();
    
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);


    let third: &i32 = &v[2];
    println!("The third elemnet is {third}");

    let get_third: Option<&i32> = v.get(2);
    println!("get third element is {get_third:#?}");
    if let Some(res) = get_third {
        println!("get third element is {res}");
    } 

    // 같은 코드지만 rust는 if let을 권장해 관점을 보여주는 것을 권장한다
    match get_third {
        Some(res) => println!("res: {res}"),
        None => {} // 아무 것도 안 할 경우라도 명시해야 함
    }

    let vc = vec![6,7,8];

    for i in &vc {
        println!("iter : {i}");
    }

    let mut vcm = vec![6,7,8];
    for i in &mut vcm {
        *i += 50;
        println!("this i : {i}");
    }
}

fn string(){
    // 문자열은 바이트 컬렉션으로 구현되어있다
    // Rust에는 한가지 문자열 타입만 제공한다 : 참조자 형태인 문자열 슬라이스 &str, +String(가변, 소유권, UTF-8)

    // rust의 String은 바이트 벡터에 보장, 제한, 기능을 추가한 wrapper로 구현되어있다

    let s_literal: &str = "hello"; // 문자열 리터럴 (&str)

    let mut s = String::from("foo");
    s.push_str("bar");

    // 소유권을 가져가지 않는다
    s.push_str(s_literal);

    println!("s_literal is .. {s_literal}");

    // rust의 String은 인덱스를 지원하지 않는다, 그 이유는 바이트로 저장되는데 글자마다 바이트가 다르기 때문에 어떤걸 읽을지 모르기 떄문이다
    // .chars()를 이용한 iteration 혹은 .bytes() 를 이용한 iteration을 통해 명확하게 사용해야 한다
}