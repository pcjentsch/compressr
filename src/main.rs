use std::env;
use std::process;
use comprssr::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // if let Err(e) =  comprssr::compress(&file_config) {
    //     println!("Application error: {}", e);

    //     process::exit(1);
    // }
    // if let Err(e) =  comprssr::decompress(&file_config) {
    //     println!("Application error: {}", e);

    //     process::exit(1);
    // }
}
