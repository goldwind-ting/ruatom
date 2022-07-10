use std::{
    cmp::{min, Ordering},
    hash::Hash,
};

use hashbrown::HashMap;

pub(crate) const PRIMES: [usize; 100] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
];

pub(crate) fn prime(n: usize) -> usize {
    if n > 0 {
        PRIMES[n - 1]
    } else {
        1
    }
}

pub(crate) fn rank(x: &mut Vec<usize>, dist: &mut usize) {
    if x.len() < 2 {
        return;
    }
    let mut cp = x.clone();
    cp.sort_unstable();
    let mut hm = HashMap::new();
    let mut ix = 1;
    for i in cp.into_iter() {
        if !hm.contains_key(&i) {
            hm.insert(i, ix);
            ix += 1;
        }
    }
    *dist = hm.len();
    for i in 0..x.len() {
        x[i] = *hm.get(&x[i]).unwrap();
    }
}

pub(crate) fn rank_matrix<T: Hash + Clone + PartialOrd + Eq + Default>(
    matrix: &mut Vec<[T; 3]>,
) -> Vec<usize> {
    let mut rank: Vec<usize> = Vec::new();
    let mut cp = matrix.clone();
    sort_matrix(&mut cp);
    let mut ix = 1;
    let mut hm: HashMap<[T; 3], usize> = HashMap::new();
    for v in cp.into_iter() {
        if !hm.contains_key(&v) {
            hm.insert(v, ix);
            ix += 1;
        };
    }
    for m in matrix.iter() {
        rank.push(*hm.get(m).unwrap());
    }
    rank
}

// desc
pub(crate) fn sort_matrix<T: PartialOrd + Clone>(matrix: &mut Vec<[T; 3]>) {
    matrix.sort_by(|x, y| lexcompare(&x.to_vec(), &y.to_vec()));
}

// 1 - (x < y), 0 - (x = y), -1 - (x > y)
pub(crate) fn lexcompare<T: PartialOrd>(x: &Vec<T>, y: &Vec<T>) -> Ordering {
    if x.len() == 0 && y.len() == 0 {
        return Ordering::Equal;
    }
    let m = min(x.len(), y.len());
    for i in 0..m {
        if x[i] > y[i] {
            return Ordering::Greater;
        } else if x[i] < y[i] {
            return Ordering::Less;
        }
    }
    if x.len() < y.len() {
        return Ordering::Less;
    } else if x.len() > y.len() {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

pub(crate) fn is_unique_array<T: Hash + Eq>(array: &Vec<T>) -> bool {
    let mut hm = HashMap::new();
    for i in array.iter() {
        if hm.contains_key(i) {
            return false;
        } else {
            hm.insert(i, 1);
        }
    }
    return true;
}

#[test]
fn test_lexcompare() {
    let x = vec![2, 2];
    let y = vec![2, 2, 3];
    assert_eq!(lexcompare(&x, &y), Ordering::Less);

    let x: Vec<u8> = vec![];
    let y = vec![];
    assert_eq!(lexcompare(&x, &y), Ordering::Equal);

    let x = vec![1];
    let y = vec![1];
    assert_eq!(lexcompare(&x, &y), Ordering::Equal);

    let x = vec![2, 2];
    let y = vec![1, 2, 3];
    assert_eq!(lexcompare(&x, &y), Ordering::Greater);

    let x = vec![2, 2];
    let y = vec![3, 2, 3];
    assert_eq!(lexcompare(&x, &y), Ordering::Less);
}

#[test]
fn test_sort_matrix() {
    let mut matrix = vec![[2, 3, 4], [1, 2, 3]];
    sort_matrix(&mut matrix);
    assert_eq!(matrix, vec![[1, 2, 3], [2, 3, 4]]);

    let mut matrix = vec![[2, 3, 4], [2, 2, 3]];
    sort_matrix(&mut matrix);
    assert_eq!(matrix, vec![[2, 2, 3], [2, 3, 4]]);
}

#[test]
fn test_rank_matrix() {
    let mut matrix = vec![[2, 3, 4], [1, 2, 3], [1, 1, 2], [1, 1, 2]];
    let rank = rank_matrix(&mut matrix);
    assert_eq!(rank, vec![3, 2, 1, 1])
}

#[test]
fn test_rank() {
    let mut ranks = vec![5, 3, 1];
    let mut dist = 0;
    rank(&mut ranks, &mut dist);
    assert_eq!(ranks, vec![3, 2, 1]);
    assert_eq!(dist, 3);

    let mut ranks = vec![5, 5, 5];
    let mut dist = 0;
    rank(&mut ranks, &mut dist);
    assert_eq!(ranks, vec![1, 1, 1]);
    assert_eq!(dist, 1);
}
