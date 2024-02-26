use std::env;
use std::fs;

fn main() {
    // env::args 读取并分析传入的命令行参数，最终通过 collect 方法将其转换为一个集合类型 Vector
   let args: Vec<String> = env::args().collect();
    // dbg! 宏用于打印调试信息
    dbg!(&args);

    //需要两个变量来存储文件路径和待搜索的字符串
    let config = Config::new(&args);

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config{
     query: String,
     file_path: String,

}

impl Config{
  fn new(args: &[String]) -> Config{
    let query = args[1].clone();
    let file_path = &args[2].clone();
    Config{query,file_path}
  }
}


fn parse_config(args: &[String]) -> Config{
  let query = args[1].clone();
  let file_path = &args[2].clone();
  return Config{query,file_path}
}

