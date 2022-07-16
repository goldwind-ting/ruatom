use std::{
    cmp::{min, Ordering},
    hash::Hash,
};

use hashbrown::HashMap;

pub(crate) const PRIMES: [u128; 300] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039,
    1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153,
    1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279,
    1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409,
    1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499,
    1511, 1523, 1531, 1543, 1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613,
    1619, 1621, 1627, 1637, 1657, 1663, 1667, 1669, 1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741,
    1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873,
    1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987,
];

pub(crate) fn prime(n: u128) -> u128 {
    if n > 0 {
        PRIMES[n as usize - 1]
    } else {
        1
    }
}

pub(crate) fn rank(x: &mut Vec<u128>, dist: &mut usize) {
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
) -> Vec<u128> {
    let mut cp = matrix.clone();
    sort_matrix(&mut cp);
    let mut ix = 1;
    let mut hm: HashMap<[T; 3], u128> = HashMap::new();
    for v in cp.into_iter() {
        if !hm.contains_key(&v) {
            hm.insert(v, ix);
            ix += 1;
        };
    }
    let mut rank: Vec<u128> = Vec::with_capacity(matrix.len());
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
