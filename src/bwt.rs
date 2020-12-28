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
// fn burrows_wheeler_fast(input_: Vec<u8>) -> Vec<u8>{
//     let mut input = input_.clone();
//     let tab = SuffixTable::new(String::from_utf8_lossy(&input));
//     let mut bw_output:Vec<u8> = input.clone();
//     bw_output.push(0 as u8);
//     // println!("suff {:?}",tab.table());
//     // println!("text {:?}",tab.text());
//     for i in 0.. l
//     {    
        
//         let index = tab.table()[i];
//         if tab.table()[i] > 0{
//             bw_output[i] = input[index as usize  - 1];
//         }
//         else
//         {
//             bw_output[i] = 255 as u8;
//         }
//     return bw_output;
// }
fn burrows_wheeler_slow(input_: Vec<u8>) -> Vec<u8> {
    let mut input = input_.clone();
    input.push(255 as u8);
    let l = input.len();

    let mut permutation_table:Vec<Vec<u8>> = (0..l).map(|x| circ_permute::<u8>(input.clone(),x)).collect();
    permutation_table.sort();
    let bw_output:Vec<u8> = permutation_table.iter().map(|x|x[l-1]).collect();
    // println!("output: {:?}", String::from_utf8_lossy(&bw_output));


    return bw_output;
}
fn inv_burrows_wheeler_slow(input: Vec<u8>) -> Vec<u8>{

    let l = input.len();

    let mut permutation_table:Vec<Vec<u8>> =(0..l).map(|x| Vec::new()).collect();

    for _ in 0..l{
        for i in 0.. permutation_table.len()
        {
            permutation_table[i].insert(0,input[i]);
            
        }

        // println!("output: {:?}", permutation_table);
        permutation_table.sort();

    }

    let bw_output= permutation_table.iter().filter(|&x|x[l-1] == 255 as u8).last().unwrap();
    // println!("{:?}", bw_output);
    return bw_output[0..l-1].to_vec();
}
