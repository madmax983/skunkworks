#[cfg(test)]
mod tests {
    use quipu::{Quipu, Cord, Knot};

    #[test]
    fn test_knot_symbol() {
        assert_eq!(Knot::Simple.symbol(), "●");
        assert_eq!(Knot::Long(5).symbol(), "≡5");
        assert_eq!(Knot::FigureEight.symbol(), "∞");
    }

    #[test]
    fn test_quipu_add_and_display() {
        let mut q = Quipu::new();
        q.add_cord(Cord::from(15_u64));
        q.add_cord(Cord::from(0_u64));

        let display_str = format!("{}", q);
        assert!(display_str.contains("Quipu with 2 cords:"));
        assert!(display_str.contains("Cord 0:"));
        assert!(display_str.contains("Cord 1:"));
    }

    #[test]
    fn test_cord_fmt_branches() {
        let empty_cord = Cord::new();
        assert_eq!(format!("{}", empty_cord), "(empty)");

        // Make a cord with an empty cluster
        let mut gap_cord = Cord::new();
        gap_cord.clusters.push(vec![]); // Units cluster is empty
        gap_cord.clusters.push(vec![Knot::Simple]); // Tens cluster has 1

        let gap_str = format!("{}", gap_cord);
        assert!(gap_str.contains("●"));
        assert!(gap_str.contains("  |  "));

        // Make a cord with subsidiaries to trigger the subsidiary loop
        let mut sub_cord = Cord::from(5_u64);
        let sub1 = Cord::from(3_u64);
        let mut sub2 = Cord::new();
        sub2.clusters.push(vec![Knot::FigureEight]);

        sub_cord.subsidiaries.push(sub1);
        sub_cord.subsidiaries.push(sub2);

        let sub_str = format!("{}", sub_cord);
        assert!(sub_str.contains("≡5"));
        assert!(sub_str.contains("≡3"));
        assert!(sub_str.contains("∞"));
    }

    #[test]
    fn test_multiple_knots_in_cluster_spacing() {
        let mut cord = Cord::new();
        cord.clusters.push(vec![Knot::Simple, Knot::Simple]);
        let s = format!("{}", cord);
        assert!(s.contains("● ●"));
    }
}
