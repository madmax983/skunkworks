pub fn compute_sdf(width: usize, height: usize, grid: &[bool]) -> Vec<f32> {
    // Distance to nearest background (false) -> Inside distance
    let dist_to_bg = compute_edt(width, height, grid, false);

    // Distance to nearest foreground (true) -> Outside distance
    let dist_to_fg = compute_edt(width, height, grid, true);

    let mut sdf = vec![0.0; width * height];
    for i in 0..sdf.len() {
        let d_in = dist_to_bg[i].sqrt();
        let d_out = dist_to_fg[i].sqrt();

        // If I am INSIDE (grid=true): d_out = 0. SDF = d_in.
        // If I am OUTSIDE (grid=false): d_in = 0. SDF = -d_out.
        sdf[i] = d_in - d_out;
    }
    sdf
}

// Compute Squared Euclidean Distance Transform to nearest pixel with value `target_val`
fn compute_edt(width: usize, height: usize, grid: &[bool], target_val: bool) -> Vec<f32> {
    let size = width * height;
    let mut g = vec![f32::INFINITY; size];

    // Initialize g with 0 for target pixels
    for i in 0..size {
        if grid[i] == target_val {
            g[i] = 0.0;
        }
    }

    // Pass 1: Columns (transform along y)
    let mut f = vec![0.0; height];
    let mut d = vec![0.0; height];
    let mut v = vec![0; height];
    let mut z = vec![0.0; height + 1];

    for x in 0..width {
        for y in 0..height {
            f[y] = g[y * width + x];
        }

        dt_1d(&f, height, &mut d, &mut v, &mut z);

        for y in 0..height {
            g[y * width + x] = d[y];
        }
    }

    // Pass 2: Rows (transform along x)
    let mut f_row = vec![0.0; width];
    let mut d_row = vec![0.0; width];
    let mut v_row = vec![0; width];
    let mut z_row = vec![0.0; width + 1];

    for y in 0..height {
        for x in 0..width {
            f_row[x] = g[y * width + x];
        }

        dt_1d(&f_row, width, &mut d_row, &mut v_row, &mut z_row);

        for x in 0..width {
            g[y * width + x] = d_row[x];
        }
    }

    g
}

// Felzenszwalb & Huttenlocher 1D squared distance transform
fn dt_1d(f: &[f32], n: usize, d: &mut [f32], v: &mut [usize], z: &mut [f32]) {
    // 1. Find the first finite value to initialize the lower envelope
    let mut k = 0;
    let mut start_q = 0;
    let mut found = false;

    for q in 0..n {
        if f[q].is_finite() {
            v[0] = q;
            start_q = q + 1;
            found = true;
            break;
        }
    }

    // If no finite values, distance is all infinity
    if !found {
        for q in 0..n {
            d[q] = f32::INFINITY;
        }
        return;
    }

    z[0] = -f32::INFINITY;
    z[1] = f32::INFINITY;

    // 2. Build the lower envelope
    for q in start_q..n {
        if !f[q].is_finite() {
            continue;
        }

        // Compute intersection s between parabola at v[k] and q
        // s = ((f[q] + q^2) - (f[v[k]] + v[k]^2)) / (2q - 2v[k])
        let q2 = (q as f32).powi(2);
        let vk2 = (v[k] as f32).powi(2);
        let num = (f[q] + q2) - (f[v[k]] + vk2);
        let den = 2.0 * (q as f32) - 2.0 * (v[k] as f32);
        let mut s = num / den;

        while s <= z[k] {
            if k == 0 {
                // Should not happen if z[0] is -INF and s is finite
                // But just in case s is -INF?
                break;
            }
            k -= 1;
            let vk2_new = (v[k] as f32).powi(2);
            let num_new = (f[q] + q2) - (f[v[k]] + vk2_new);
            let den_new = 2.0 * (q as f32) - 2.0 * (v[k] as f32);
            s = num_new / den_new;
        }

        k += 1;
        v[k] = q;
        z[k] = s;
        z[k + 1] = f32::INFINITY;
    }

    // 3. Fill in values
    k = 0;
    for q in 0..n {
        while z[k + 1] < (q as f32) {
            k += 1;
        }
        let dx = (q as f32) - (v[k] as f32);
        d[q] = dx.powi(2) + f[v[k]];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_sdf_simple() {
        let width = 3;
        let height = 3;
        // Center pixel is true
        // F F F
        // F T F
        // F F F
        let grid = vec![false, false, false, false, true, false, false, false, false];

        let sdf = compute_sdf(width, height, &grid);

        // Center (1,1) should have positive SDF. Distance to boundary (false).
        // Nearest false is (0,1), (2,1), (1,0), (1,2). dist=1. SDF=1.
        // Wait, distance to nearest False.
        // If (1,1) is True. Neighbors are False.
        // dist squared = 1. dist = 1.
        // SDF = 1.
        assert_eq!(sdf[4], 1.0);

        // (0,0) is False. Nearest True is (1,1).
        // dist squared = 1^2 + 1^2 = 2.
        // dist = sqrt(2) = 1.414.
        // SDF = -1.414.
        assert!((sdf[0] + 1.4142).abs() < 0.001);

        // (0,1) is False. Nearest True is (1,1).
        // dist squared = 1.
        // dist = 1.
        // SDF = -1.
        assert_eq!(sdf[3], -1.0);
    }

    #[test]
    fn test_compute_edt_1d() {
        // Test simple 1D DT
        // 0 1 0
        // nearest 1 is at index 1.
        // dist sq at 0: (0-1)^2 = 1.
        // dist sq at 1: 0.
        // dist sq at 2: (2-1)^2 = 1.

        let f = vec![f32::INFINITY, 0.0, f32::INFINITY];
        let n = 3;
        let mut d = vec![0.0; n];
        let mut v = vec![0; n];
        let mut z = vec![0.0; n + 1];

        dt_1d(&f, n, &mut d, &mut v, &mut z);

        assert_eq!(d[0], 1.0);
        assert_eq!(d[1], 0.0);
        assert_eq!(d[2], 1.0);
    }
}
