#[cfg(test)]
mod tests {
    use locus::Topology;

    #[test]
    fn test_normalize_zero_dimensions_safe() {
        let topo = Topology::Torus;
        // This should not panic, but return None
        let res = topo.normalize(10, 10, 0, 0);
        assert_eq!(res, None);
    }
}
