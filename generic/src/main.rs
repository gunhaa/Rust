fn main() {

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let char_result = largest(&char_list);
    println!("the largest char is {char_result}");

    generic_struct();
}

fn generic_struct(){

    let integer = Point { x: 5, y: 10};
    let float = Point{x: 1.0, y: 4.0};

    let p = Point{x: 5, y: 10};
    println!("p.x = {}", p.x());

    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    struct Point2<T,G> {
        x: T,
        y: G,
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}