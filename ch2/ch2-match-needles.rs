fn main() {
    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in haystack {
        let result = match item {
            42 | 132 => "hit", // 匹配42或132
            _ => "miss", // 匹配所有值
        };

        if result == "hit" {
            println!("{}: {}", item, result);
        }
    }
}