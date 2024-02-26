use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // env::args 读取并分析传入的命令行参数，最终通过 collect 方法将其转换为一个集合类型 Vector
   let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config){
        println!("Application error: {}", e);
        process::exit(1);
    };
}








