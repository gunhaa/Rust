fn main() {
    let mut user1 = mut_user();
    user1.active = false;
    println!("mut user1 : {user1:#?}");

    cont_user(user1);

    tuple_structure();

    rectangles();

    rectangles_associated();
}

fn rectangles_associated(){
    let rect1 = Rectangle {
        width : 30,
        height : 50,
    };

    let rect2 = Rectangle {
        width : 10,
        height : 40,
    };

    let rect3 = Rectangle {
        width : 60,
        height : 45,
    };

    let sq = Rectangle::square(3);
    dbg!(sq);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn rectangles(){
    // let width1= 30;
    // let height1 = 50;
    // let rect1 = (30, 50);
    let rect1 = Rectangle {
        width: 30,
        height:50,
    };

    println!(
        "The area of rectangle is {} square pixels",
        rect1.area()
    );

    // dbg!(&rect1);

    // fn area(width1: u32, height1: u32) -> u32{
    //     width1*height1
    // }

    // fn area(dimensions: (u32, u32))-> u32 {
    //         dimensions.0 * dimensions.1
    // }

    // fn area(rectangle: &Rectangle) -> u32{
    //     rectangle.width * rectangle.height
    // }

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width : size,
            height : size,
        }
    }
}

fn tuple_structure(){
    
    let black = Color(0, 122, 32);

    #[derive(Debug)]
    struct Color(i32, i32, i32);

    println!("this color : {black:#?}")
}

fn mut_user()-> User{
    let mut user1 = User {
        active:true,
        username:String::from("someusername123"),
        email:String::from("someone@example.com"),
        sign_in_count:1,
    };

    user1.email = String::from("wh8299@naver.com");
    println!("user1 : {user1:#?}");

    user1
}

fn cont_user(user1 : User){
    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };
    println!("user2 : {user2:#?}")
}

#[derive(Debug)]
struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}