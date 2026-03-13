#[cfg(test)]
mod tests {
    use crate::tree::Tree;
    use crate::monitor::ProcessStats;
    use macroquad::prelude::Vec2;

    #[test]
    #[should_panic]
    fn test_nan_cpu_usage_panic() {
        let stats1 = ProcessStats { pid: 1.into(), name: "a".to_string(), cpu_usage: std::f32::NAN, memory: 0 };
        let t1 = Tree::new(stats1, Vec2::new(0.0, 0.0));

        let stats2 = ProcessStats { pid: 2.into(), name: "b".to_string(), cpu_usage: 0.0, memory: 0 };
        let t2 = Tree::new(stats2, Vec2::new(0.0, 0.0));

        let trees = vec![t1, t2];
        let _ = trees.iter().max_by(|a, b| a.stats.cpu_usage.partial_cmp(&b.stats.cpu_usage).unwrap());
    }
}
