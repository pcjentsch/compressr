use std::error::Error;
use std::io::prelude::*;
use std::fs;
pub struct Config {
    pub in_file: String,
    pub out_file: String,
}

pub fn compress(config: &Config)-> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.in_file.clone())?;
    let binary_contents = contents.as_bytes();

    let mut pos = 0;
    let mut buffer = fs::File::create(config.out_file.clone())?;

    while pos < contents.len() {
        let bytes_written = buffer.write(&binary_contents[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}
pub fn decompress(config: &Config)-> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.in_file.clone())?;
    let binary_contents = contents.as_bytes();

    let mut pos = 0;
    let mut buffer = fs::File::create(config.out_file.clone())?;

    while pos < contents.len() {
        let bytes_written = buffer.write(&binary_contents[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}


fn burrows_wheeler(input: &[u8]) -> &[u8] {
    
    return input
}




impl Config {
    pub fn new(args: &[String]) ->  Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let in_file = args[1].clone();
        let out_file = args[2].clone();

        Ok(Config { in_file, out_file })
    }
}


