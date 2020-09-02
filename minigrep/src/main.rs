use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("处理参数错误：{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("应用程序错误：{}", e);
        process::exit(1);
    }
}
