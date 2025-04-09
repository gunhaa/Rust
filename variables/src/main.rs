fn main() {

    // variable
    let mut x = 5;
    let y = 50;
    println!("The value of x, y is : {x}, {y}");
    x = 6;
    println!("The value of x is : {x}");
    let z : i64 = 124124235235925235;
    println!("The value of x is : {z}");

    //constant
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("Three Hours in Seconds : {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let s = 5;
    let s = s + 1;

    {
        let s = s * 2;
        println!("Shadowing value inner scope is : {s}");
    }

    println!("Shadowing value is : {s}");

    data_type();

    floating_point();

    calculate();

    boolean_type();

    char_type();

    tuple();

    array();

    function();

    expression();

    re_turn();

    if_expression();

    loop_syntax();
}

fn data_type(){
    let x : i32 = 2147483647;
    println!("i32는 부호가 있는 32비트 변수이다. -214783648 ~ 214783647 : {x}");

    let y : u32 = 4294967295;
    println!("u32는 부호가 없는 32비트 변수이다. 0 ~ 4294967295 : {y}");
}

fn floating_point(){
    let x : f32 = 2.0;
    println!("rust의 부동소수점은 f32와 f64가 있으며, 비슷한 속도를 내어 기본 타입을 f64로 사용한다: {x}");
    let y : f64 = -1.5;
    println!("rust의 모든 부동소수점 타입은 부호가 있다 : {y}");
}

fn calculate(){
    // 덧셈
    let _sum = 5 + 10;
    let _difference = 95.5-4.3;
    let _product = 4*30;
    
    // 나눗셈
    let _quotient = 56.7/32.2;
    let truncated = -5/3;

    let _remainder = 43 % 5;

    println!("정수 나눗셈의 결과는 -1이 된다, Rust에서는 소수점 이하를 버리고, 정수 부분만 취한다 : {truncated}");
}

fn boolean_type(){
    // 불리언 값은 1바이트 이다
    let t = true;
    let f = false;

    println!("true : {t} , false : {f}")
}

fn char_type(){
    let _c = 'z';
    let _z: char = 'Z';
    let j = 'ウ';
    println!("Rust의 char타입은 4바이트 크기이며, 유니코드 스칼라 값을 표현한다 : {j}")
}

fn tuple(){
    let tup:(i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("tuple은 다양한 타입의 여러 값을 묶어 하나의 복합 타입을 만들 수 있으며, 사용하기 위해선 패턴 매칭을 해 튜플값을 해체해야한다. : {x}, {y}, {z}");

    // let tup:(i32, f64, u8) = (500, 6.4, 1);
    let xx = tup.0;
    let yy = tup.1;
    let zz = tup.2;
    println!("인덱스 방식으로도 접근이 가능하다. x : {xx}, y : {yy}, z: {zz}");

    let unit = ();
    println!("아무 값도 없는 튜플은 unit이라는 특별한 이름을 갖으며, 빈 값이나 비어있는 반환 타입을 나타낸다. : {unit:?}");
}

fn array(){
    let a = [1,2,3,4,5];
    let _b:[u32; 5] = [1,2,3,4,5];
    println!("rust의 array는 고정된 길이를 가지며, 스택에 데이터를 할당하고 싶을 때 사용한다. : {a:?}");

    let c = [3; 5];
    println!("해당 방식의 array 초기화도 가능하다 : {c:?}");

    let d = [1,2,3,4,5];
    let item1 = d[0];
    println!("인덱스를 통한 접근도 가능하다 : {item1}");
}


fn function(){
    another_function(123, 'A');

    println!("rust는 함수 위치를 고려하지 않으며, 호출하는 쪽에서 볼 수 있는 스코프 어딘가에 정의만 되어 있으면 된다.");
}

fn another_function(x: i32, unit: char){
    println!("rust의 함수는 snake_case 방식을 사용한다");
    println!("rust의 함수는 파라미터를 가질 수 있으며 구체적인 값을 argument라고 부른다. : {x} , {unit}");
}

fn expression(){
    let _x = 6;
    let s = "let x = (let y = 6);";
    println!("rust에서는 할당문이 할당된 값을 반환하지 않아, 해당 코드는 컴파일 오류를 낸다 : {s}");   
}

fn re_turn(){
    let x = five();
    println!("리턴 타입은 화살표 뒤에 선언 되어야 한다. : {x}");

    fn five() -> i32{
        5
    }
}

fn if_expression(){
    let number = 3;
    if number < 5 {
        println!("rust는 해당 방식으로 분기하며, 조건은 반드시 bool 이어야 한다 : true {number}");
    } else {
        println!("rust는 해당 방식으로 분기하며, 조건은 반드시 bool 이어야 한다 : false {number}");        
    }

    let condition = true;
    let number = if condition {5} else {6};

    println!("rust는 해당 방식의 판정도 할 수 있으며, 같은 자료형만 사용할 수 있다. : {number}");
}

fn loop_syntax(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("rust의 루프문이며, 결과는 다음과 같다 : {result}")
}