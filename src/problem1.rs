pub fn sum(slice: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for x in slice {
        sum += x;
    }
    sum
}


pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut dups: Vec<i32> = Vec::new();
    for x in 0..vs.len() {
        for j in x + 1..vs.len() {
            if vs[x] == vs[j] && !dups.contains(&vs[x]) {
                dups.push(vs[x])
            }
        }
    }
    dups
}


pub fn filter(vs: &Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
    let mut filtered: Vec<i32> = Vec::new();
    for x in vs {
        if pred(*x) {
            filtered.push(*x)
        }
    }
    filtered
}