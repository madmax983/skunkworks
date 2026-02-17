use memetic_market::sim::{Market, Topic, Agent, Strategy, MarketPhase};

#[test]
fn test_market_initialization() {
    let topics = vec![
        Topic::new("RUST", 100.0),
        Topic::new("AI", 200.0),
    ];
    let agents = vec![
        Agent::new("Alice", Strategy::TrendFollower, 1000.0),
    ];
    let market = Market::new(topics, agents);

    assert_eq!(market.topics.len(), 2);
    assert_eq!(market.agents.len(), 1);
    assert_eq!(market.tick, 0);
    assert!(market.get_topic("RUST").is_some());
    assert_eq!(market.get_topic("RUST").unwrap().intrinsic_value, 100.0);
}

#[test]
fn test_price_discovery() {
    // 1 Topic, 1 Agent.
    let mut topic = Topic::new("MEME", 50.0);
    topic.market_price = 1.0; // Reset to known state

    let mut agent = Agent::new("Bot", Strategy::TrendFollower, 100.0);

    // Agent allocates 100.0 attention to "MEME".
    // Price should go UP.
    agent.allocate("MEME", 100.0);

    let mut market = Market::new(vec![topic], vec![agent]);

    // Force an update
    market.update();

    let updated_topic = market.get_topic("MEME").unwrap();
    // Assuming simple Price = Total Attention / Constant or similar logic.
    // Or Price = Base + Demand.
    // If agent put 100 in, price should reflect that.
    // Let's assert it increased from 1.0.
    assert!(updated_topic.market_price > 1.0, "Price should increase when attention is allocated");
}

#[test]
fn test_bubble_formation() {
    // Intrinsic Value is low (10.0).
    // Agent has huge cash (1000.0).
    // Agent allocates everything.
    let topic = Topic::new("VAPORWARE", 10.0);
    let mut agent = Agent::new("Whale", Strategy::HypeBeast, 1000.0);
    agent.allocate("VAPORWARE", 1000.0);

    let mut market = Market::new(vec![topic], vec![agent]);
    market.update();

    let price = market.get_topic("VAPORWARE").unwrap().market_price;
    let intrinsic = market.get_topic("VAPORWARE").unwrap().intrinsic_value;

    // A bubble is defined as Price >> Intrinsic.
    assert!(price > intrinsic * 2.0, "Price {} should be > 2x Intrinsic {} (Bubble)", price, intrinsic);
}
