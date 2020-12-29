
const STX:u8 = 0x03 as u8;

fn circ_permute<T: Copy>(input: Vec<T>,distance: usize) -> Vec<T>{
    let mut permuted = input.clone();
    
    for i in 0 .. input.len(){
        if i + distance < input.len(){
           permuted[i+distance] = input[i];
        }
        else{
            permuted[(i + distance) % input.len()] = input[i];
        }
    }
    return permuted; 
}
pub fn burrows_wheeler_slow(input_: Vec<u8>) -> Vec<u8> {
    let mut input = input_.clone();
    input.push(STX);
    let l = input.len();

    let mut permutation_table:Vec<Vec<u8>> = (0..l).map(|x| circ_permute::<u8>(input.clone(),x)).collect();
    permutation_table.sort();
    let bw_output:Vec<u8> = permutation_table.iter().map(|x|x[l-1]).collect();
    return bw_output;
}
pub fn inv_burrows_wheeler_slow(input: Vec<u8>) -> Vec<u8>{

    let l = input.len();
    // println!("inv: {:?}", input) ;
    let mut permutation_table:Vec<Vec<u8>> =(0..l).map(|x| Vec::new()).collect();
    for _ in 0..l+1{
        for i in 0.. permutation_table.len()
        {
            permutation_table[i].insert(0,input[i]);
            
        }
        permutation_table.sort();

    }
    // println!("output: {:?}", permutation_table);
    let bw_output= permutation_table.iter().cloned().filter(|x|x[l-1] == STX).last().unwrap();
    // println!("{:?}", bw_output);
    return bw_output[0..l-1].to_vec();
}
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn bwt(){
        let str = "^BANANA".as_bytes();
        assert_eq!(burrows_wheeler_slow(str.to_vec()),[65, 78, 78, 66, 94, 65, 65, 3].to_vec());
    }
    
    #[test]
    fn ibwt(){
        let str = "^BANANA".as_bytes();
        assert_eq!(inv_burrows_wheeler_slow([65, 78, 78, 66, 94, 65, 65, 3].to_vec()),str.to_vec());
    }
}