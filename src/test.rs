#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::fs::File;

    const TEXT_FILE: &str = "test_data";
    fn test_compress() ->  bool{
        let input_path = format!("input/{}.txt",TEXT_FILE);
        let output_path = format!("compressed/{}.cmprs",TEXT_FILE);
        let conf = Config{ in_file: input_path.clone(),out_file: output_path.clone()};
        compress(&conf).is_ok()
    }
    fn test_decompress() ->  bool{
        let input_path = format!("compressed/{}.cmprs",TEXT_FILE);
        let output_path = format!("decompressed/{}.txt",TEXT_FILE);
        let conf = Config{ in_file: input_path.clone(),out_file: output_path.clone()};
        decompress(&conf).is_ok()
    }

    #[test]
    fn test_compress_decompress(){
        assert_eq!(test_compress() && test_decompress(),true);
    }
    
    
    
    #[test]
    fn test_eq() {
        test_compress();
        test_decompress();
        let input_path = format!("input/{}.txt",TEXT_FILE);
        let output_path = format!("decompressed/{}.txt",TEXT_FILE);

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