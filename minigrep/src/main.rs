use std::env;
use std::fs;
use std::process;

fn main() {
    // env::args 读取并分析传入的命令行参数，最终通过 collect 方法将其转换为一个集合类型 Vector
   let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config{
     query: String,
     file_path: String,

}

impl Config{
  fn build(args: &[String]) -> Result<Config,&'static str>{
    if args.len() < 3 {
        return Err("not enough arguments");
    }

    let query = args[1].clone();
    let file_path = args[2].clone();
    Ok(Config{query,file_path})
  }
}


