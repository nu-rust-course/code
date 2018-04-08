fn identity(x: i32, y: i32) -> i32 {
    x * y / y
}

fn print_identity(x: i32, y: i32) {
    println!("identity({}, {}) == {}", x, y, identity(x, y));
}

fn main() {
    print_identity(7, 5);
    print_identity(7, 0);
}