use std::{error::Error};
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::fs::File;

    fn test_compress() ->  bool{

        let text_file: &str = "test_data";
        let input_path = format!("input/{}.txt",text_file);
        let output_path = format!("compressed/{}.cmprs",text_file);
        let conf = Config{ in_file: input_path.clone(),out_file: output_path.clone()};
        compress(&conf).is_ok()
    }
    fn test_decompress() ->  bool{

        let text_file: &str = "test_data";    
        let input_path = format!("compressed/{}.cmprs",text_file);
        let output_path = format!("decompressed/{}.txt",text_file);
        let conf = Config{ in_file: input_path.clone(),out_file: output_path.clone()};
        decompress(&conf).is_ok()
    }

    #[test]
    fn test_compress_decompress(){
        assert_eq!(test_compress() && test_decompress(),true);
    }
    
    
    
    #[test]
    fn test_eq() {

        let text_file: &str = "test_data_2";
        
        let input_path = format!("input/{}.txt",text_file);
        let output_path = format!("compressed/{}.cmprs",text_file);  

        let conf = Config{ in_file: input_path.clone(),out_file: output_path.clone()};
        compress(&conf).unwrap();


        let input_path = format!("input/{}.txt",text_file);
        let output_path = format!("decompressed/{}.txt",text_file);

        let conf_2 = Config{ in_file: input_path.clone(),out_file: output_path.clone()};
        decompress(&conf_2).unwrap();
        assert_eq!(diff(&input_path,&output_path),true);
    }


    pub fn diff_files(f1: &mut File, f2: &mut File) -> bool {

        let buff1 : &mut [u8] = &mut [0; 1024];
        let buff2 : &mut [u8] = &mut [0; 1024];
        
        loop {
    
            match f1.read(buff1) {
                Err(_) => return false,
                Ok(f1_read_len) => match f2.read(buff2) {
                    Err(_) => return false,
                    Ok(f2_read_len) => {
                        if f1_read_len != f2_read_len {
                            return false;
                        }
                        if f1_read_len == 0 {
                            return true;
                        }
                        if &buff1[0..f1_read_len] != &buff2[0..f2_read_len] {
                            return false;
                        }
                    }
                }
            }
        }
    }
    
    /// Takes two string filepaths and returns true if the two files are identical and exist.
    pub fn diff(f1: &str, f2: &str) -> bool {
        let mut fh1 = File::open(f1);
        let mut fh2 = File::open(f2);
    
        fh1.as_mut().and_then(|file1|
            fh2.as_mut().and_then(|file2|
                Ok(diff_files(file1, file2)))).unwrap_or(false)
    }
}