fn zero_ptr(p: &mut i32) {
    *p = 0;
}

fn zero_optional_ptr(op: Option<&mut i32>) {
    if let Some(p) = op {
        *p = 0;
    }
}

fn main() {
    let mut x: i32 = 8;
    zero_ptr(&mut x);
    zero_optional_ptr(Some(&mut x));
    zero_optional_ptr(None);
}