fn main() {
    let rows: usize = usize::MAX - 1;
    let cols: usize = 0;

    let res = rows
        .checked_add(1)
        .and_then(|r| cols.checked_add(1).and_then(|c| r.checked_mul(c)));
    println!("{:?}", res);

    if let Some(c) = res {
        println!("{} <= {} = {}", c, (isize::MAX as usize) / 32, c <= (isize::MAX as usize) / 32);
    }
}
