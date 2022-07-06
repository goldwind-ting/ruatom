use std::cmp::{min, Ordering};

pub(crate) const PRIMES: [i32; 100] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
];

pub(crate) fn prime(n: usize) -> i32 {
    if n > 0 {
        PRIMES[n]
    } else {
        1
    }
}

pub(crate) fn rank(x: &mut Vec<u8>, highest_rank: &mut u8) {
    if x.len() < 2 {
        return;
    }
    x.sort_unstable();
    let mut tmp = vec![x[0]];
    for i in 1..x.len() {
        if x[i] > tmp[tmp.len() - 1] {
            tmp.push(x[i]);
        }
    }
    *highest_rank = tmp.len() as u8;
    for i in x.iter_mut() {
        *i = tmp.binary_search(i).unwrap() as u8;
    }
}


pub(crate) fn rank_matrix(matrix: &mut Vec<[u32; 3]>){
    let mut rank: Vec<usize> = Vec::new();
    let mut cp = matrix.clone();
    sort_matrix(&mut cp);
    cp.dedup();
}


 // desc
pub(crate) fn sort_matrix(matrix: &mut Vec<[u32; 3]>){
    matrix.sort_by(|x, y|lexcompare(&x.to_vec(), &y.to_vec()));

}

// 1 - (x < y), 0 - (x = y), -1 - (x > y)
pub(crate) fn lexcompare(x: &Vec<u32>, y: &Vec<u32>) -> Ordering{
    if x.len() == 0 && y.len() == 0{
        return Ordering::Equal;
    }
    let m = min(x.len(), y.len());
    for i in 0..m{
        if x[i] > y[i]{
            return Ordering::Greater;
        }else if x[i] < y[i]{
            return Ordering::Less;
        }
    }
    if x.len() < y.len(){
        return Ordering::Less;
    }else if x.len() > y.len(){
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

pub(crate) fn binary_search_matrix(a: &Vec<u32>, matrix: &Vec<[u32; 3]>) -> Option<usize>{
    matrix.binary_search_by(|x|lexcompare(&x.to_vec(), a)).ok()
}


#[test]
fn test_lexcompare(){
    let x = vec![2,2];
    let y = vec![2,2,3];
    assert_eq!(lexcompare(&x, &y), Ordering::Less);

    let x = vec![];
    let y = vec![];
    assert_eq!(lexcompare(&x, &y), Ordering::Equal);

    let x = vec![1];
    let y = vec![1];
    assert_eq!(lexcompare(&x, &y), Ordering::Equal);

    let x = vec![2,2];
    let y = vec![1,2,3];
    assert_eq!(lexcompare(&x, &y), Ordering::Greater);


    let x = vec![2,2];
    let y = vec![3,2,3];
    assert_eq!(lexcompare(&x, &y), Ordering::Less);


}


#[test]
fn test_sort_matrix(){
    let mut matrix = vec![[2,3,4], [1,2,3]];
    sort_matrix(&mut matrix);
    assert_eq!(matrix, vec![[1, 2, 3], [2, 3, 4]]);

    let mut matrix = vec![[2,3,4], [2,2,3]];
    sort_matrix(&mut matrix);
    assert_eq!(matrix, vec![[2, 2, 3], [2, 3, 4]]);

}


#[test]
fn test_binary_search_matrix(){
    let matrix = vec![[1,2,3], [4,5,6]];
    assert_eq!(binary_search_matrix(&vec![1,2,3], &matrix), Some(0));
    assert_eq!(binary_search_matrix(&vec![4,5,6], &matrix), Some(1));
    assert_eq!(binary_search_matrix(&vec![1,2,4], &matrix), None);
}