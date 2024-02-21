use std::env;
use std::fs;

fn main() {
    // env::args 读取并分析传入的命令行参数，最终通过 collect 方法将其转换为一个集合类型 Vector
   let args: Vec<String> = env::args().collect();
    // dbg! 宏用于打印调试信息
    dbg!(&args);

    //需要两个变量来存储文件路径和待搜索的字符串
    let query = &args[1];
    let filename = &args[2];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

 println!("With text:\n{}", contents);
}
