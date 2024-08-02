use std::any::Any;

fn print_type_of<T: Any>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn get_type_of<T: Any>(_: &T) -> Box<dyn Any> {
    let type_name = std::any::type_name::<T>();
    return Box::new(type_name);
}

fn main() {
    let x = 5;
    let y = "hello";
    print_type_of(&x); // Outputs: i32
    print_type_of(&y); // Outputs: &str

    let b = get_type_of(&y);
    let c: b = "banana";
    println!("{}", c);
}
