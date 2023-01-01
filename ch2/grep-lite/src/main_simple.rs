use clap::{App, Arg};
use regex::Regex;

fn main() {
    // 构建命令行参数解析器
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // 提取 pattern 参数
    let pattern = args.value_of("pattern").unwrap();

    let re = Regex::new(pattern).unwrap();

    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for (i, line) in quote.lines().enumerate() {
        let contains_substring = re.find(line);
        let line_num = i + 1;
        match contains_substring {
            Some(_) => println!("{}: {}", line_num, line),
            None => (),
        };
    }
}
