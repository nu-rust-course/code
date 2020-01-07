fn double_repeat(v: &mut Vec<i32>) {
    let mut w = Vec::new();

    for i in v.iter_mut() {
        *i *= 2;
        w.push(*i);
    }

    v.extend(w);
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    double_repeat(&mut v);
    println!("{:?}", v);
}
