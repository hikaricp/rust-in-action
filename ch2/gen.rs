fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let res = add(10, 20);
    println!("{}", res);
}