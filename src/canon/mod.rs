
const PRIMES: [i32; 100] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263,
269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367,
373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463,
467, 479, 487, 491, 499, 503, 509, 521, 523, 541];


pub fn prime(n: usize) ->  i32{
    if n > 0{
        PRIMES[n]
    }else{
        1
    }
    
}

pub fn rank(x: &mut Vec<u8>, highest_rank: &mut u8){
    if x.len() < 2{
        return;
    }
    x.sort_unstable();
    let mut tmp = vec![x[0]];
    for i in 1..x.len(){
        if x[i] > tmp[tmp.len()-1]{
            tmp.push(x[i]);
        }
    }
    *highest_rank = tmp.len() as u8;
    for i in x.iter_mut(){
        *i = tmp.binary_search(i).unwrap() as u8;
    }
}




mod tests{

    #[test]
    fn test_rank(){
        use super::rank;
        let mut x = vec![5, 4, 3,1,3];
        let mut hr = 0;
        
        rank(&mut x, &mut hr);
        println!("{:?}", x);
    }
}