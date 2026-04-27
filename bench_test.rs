use std::time::Instant;

pub fn contains_ignore_case_old(s: &str, keyword: &str) -> bool {
    s.as_bytes().windows(keyword.len()).any(|window| {
        window
            .iter()
            .zip(keyword.as_bytes())
            .all(|(&c, &k)| c.to_ascii_lowercase() == k)
    })
}

pub fn contains_ignore_case_new(s: &str, keyword: &str) -> bool {
    let s_bytes = s.as_bytes();
    let k_bytes = keyword.as_bytes();
    if k_bytes.len() > s_bytes.len() {
        return false;
    }
    for i in 0..=(s_bytes.len() - k_bytes.len()) {
        let mut match_found = true;
        for j in 0..k_bytes.len() {
            if s_bytes[i + j].to_ascii_lowercase() != k_bytes[j] {
                match_found = false;
                break;
            }
        }
        if match_found {
            return true;
        }
    }
    false
}

pub fn contains_ignore_case_eq(s: &str, keyword: &str) -> bool {
    let s_bytes = s.as_bytes();
    let k_bytes = keyword.as_bytes();
    if k_bytes.len() > s_bytes.len() {
        return false;
    }
    s_bytes.windows(k_bytes.len()).any(|w| w.eq_ignore_ascii_case(k_bytes))
}

fn main() {
    let s = "Success: Data saved";
    let keyword = "success";

    let start = Instant::now();
    for _ in 0..10_000_000 {
        std::hint::black_box(contains_ignore_case_old(s, keyword));
    }
    println!("Old: {:?}", start.elapsed());

    let start = Instant::now();
    for _ in 0..10_000_000 {
        std::hint::black_box(contains_ignore_case_new(s, keyword));
    }
    println!("New (manual loop): {:?}", start.elapsed());

    let start = Instant::now();
    for _ in 0..10_000_000 {
        std::hint::black_box(contains_ignore_case_eq(s, keyword));
    }
    println!("Eq ignore case: {:?}", start.elapsed());
}
