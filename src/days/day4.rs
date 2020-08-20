use std::ops::Range;

const RANGE_MIN: i32 = 245318;
const RANGE_MAX: i32 = 765747;
const RANGE: Range<i32> = RANGE_MIN..RANGE_MAX;

pub fn main() {
    let valid = find_valid();
    let valid2 = find_valid2();
    println!("");
    println!("___Day 4___");
    println!("Part 1: {}", valid.len());
    println!("Part 2: {}", valid2.len());
}

fn has_adj(nstr: &String) -> bool {
    let mut n1: &str;
    let mut n2: &str;
    for i in 0..(nstr.len() - 1) {
        n1 = &nstr[i..i + 1];
        n2 = &nstr[i + 1..i + 2];
        if n1 == n2 {
            return true;
        }
    }
    return false;
}

fn has_2_adj(nstr: &String) -> bool {
    let mut n1: &str;
    let mut n2: &str;
    let mut n3: &str;
    let mut n4: &str;
    let l = nstr.len();
    if nstr[0..1] == nstr[1..2] && nstr[1..2] != nstr[2..3] {
        return true
    }
    if nstr[l-1..l] == nstr[l-2..l-1] && nstr[l-3..l-2] != nstr[l-2..l-1] {
        return true
    }
    for i in 0..(nstr.len() - 3) {
        n1 = &nstr[i..i + 1];
        n2 = &nstr[i + 1..i + 2];
        n3 = &nstr[i + 2..i + 3];
        n4 = &nstr[i + 3..i + 4];
        if n1 != n2 && n2 == n3 && n3 != n4 {
            return true;
        }
    }
    return false;
}

fn is_increasing(nstr: &String) -> bool {
    let mut n1: &str;
    let mut n2: &str;
    for i in 0..(nstr.len() - 1) {
        n1 = &nstr[i..i + 1];
        n2 = &nstr[i + 1..i + 2];
        let n1_num: i32 = n1.parse().unwrap();
        let n2_num: i32 = n2.parse().unwrap();
        if n1_num > n2_num {
            return false;
        }
    }
    return true;
}

fn find_valid() -> Vec<i32> {
    let mut valid = Vec::<i32>::new();
    for i in RANGE {
        let nstr = i.to_string();
        if has_adj(&nstr) && is_increasing(&nstr) {
            valid.push(i);
        }
    }
    valid
}

fn find_valid2() -> Vec<i32> {
    let mut valid = Vec::<i32>::new();
    for i in RANGE {
        let nstr = i.to_string();
        if is_increasing(&nstr) && has_2_adj(&nstr) {
            valid.push(i);
        }
    }
    valid
}
