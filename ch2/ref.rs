fn main() {
    let a = 42;
    let r = &a; // 对a的引用
    let b = a + *r; // 通过解除引用

    println!("a + a = {}", b);
}