fn main() {
    let three = 0b11; // 0b 前缀表示二进制数字
    let thirty = 0o36; // 0o 前缀表示八进制数字
    let three_hundred = 0x12C; // 0x 前缀表示十六进制数字

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
