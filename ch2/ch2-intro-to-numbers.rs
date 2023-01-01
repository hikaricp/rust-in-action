fn main() {
    let twenty = 20; // 自动类型推断
    let twenty_one: i32 = 21; // 显示添加类型注解
    let twenty_two = 22i32; // 添加类型后缀
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000; // 使用下换线增强可读性
    println!("{}", one_million.pow(2)); // 数字本身可以执行方法调用

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    println!("{:02}", forty_twos[0]); // 使用索引获取数组元素


}
