pub fn d2xy(n: u32, d: u32) -> (u32, u32) {
    let mut rx;
    let mut ry;
    let mut s = 1;
    let mut x = 0;
    let mut y = 0;
    let mut t = d;

    // For n=10, loop 10 times.
    for _ in 0..n {
        rx = 1 & (t / 2);
        ry = 1 & (t ^ rx);
        rot(s, &mut x, &mut y, rx, ry);
        x += s * rx;
        y += s * ry;
        t /= 4;
        s *= 2;
    }
    (x, y)
}

fn rot(n: u32, x: &mut u32, y: &mut u32, rx: u32, ry: u32) {
    if ry == 0 {
        if rx == 1 {
            *x = n - 1 - *x;
            *y = n - 1 - *y;
        }
        // Swap x and y
        let temp = *x;
        *x = *y;
        *y = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d2xy() {
        // Known values for Hilbert curve order 1
        // d=0 -> 0,0
        // d=1 -> 0,1
        // d=2 -> 1,1
        // d=3 -> 1,0
        assert_eq!(d2xy(1, 0), (0, 0));
        assert_eq!(d2xy(1, 1), (0, 1));
        assert_eq!(d2xy(1, 2), (1, 1));
        assert_eq!(d2xy(1, 3), (1, 0));
    }
}
