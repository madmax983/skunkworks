pub mod model;

#[cfg(test)]
mod tests {
    use super::model::*;
    use ratatui::style::Color;

    #[test]
    fn test_agent_bids_and_wins() {
        let mut market = Market::new(10, 10);

        let agent = Agent {
            id: 0,
            wealth: 100.0,
            color: Color::Red,
            desired_blocks: 5,
            owned_blocks: vec![],
        };

        market.agents.push(agent);

        // Before update, grid is empty
        assert_eq!(market.grid[0].owner, None);

        // Update should trigger bidding
        market.update();

        // Agent should own at least one block because they have money and desire blocks
        let owned_count = market.grid.iter().filter(|b| b.owner == Some(0)).count();
        assert!(owned_count > 0, "Agent failed to acquire blocks");
    }
}
