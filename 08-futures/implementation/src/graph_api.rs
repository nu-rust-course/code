
fn double<F>(n: usize, mut f: F) -> usize
where
    F: FnMut(usize) -> usize {

    f(f(n))
}

fn add_any(a: usize, b: usize) -> usize {
    double(a, |x| x + b)
}

fn double_translated(n: usize, mut f: AddAnyLambda0) -> usize {
    f.apply(f.apply(n))
}

fn add_any_translated(a: usize, b: usize) -> usize {
    double(a, AddAnyLambda0 { b: &b })
}

struct AddAnyLambda0<'a> {
    b: &'a usize,
}

impl FnMut(usize) -> usize for AddAnyLambda0 {
    fn apply(&mut self, x: usize) {
        x + *self.b
    }
}

struct AddAnyLambda2 {
    b: usize,
}

fn add_any2(a: usize, b: usize2) -> usize {
    let br = &b;
    double(a, move |x| x + *br)
}



[=x, &y, k=std::move(o)](auto a, auto b){ return a + b; }
