use heap_market::market::Market;
use heap_market::model::{AgentId, Order, OrderType};

#[test]
fn test_order_matching_simple() {
    let mut market = Market::new();

    // Agent 1 wants to buy at 100
    market.add_order(Order {
        agent_id: AgentId(1),
        order_type: OrderType::Bid,
        price: 100,
        quantity: 1,
    });

    // Agent 2 wants to sell at 50
    market.add_order(Order {
        agent_id: AgentId(2),
        order_type: OrderType::Ask,
        price: 50,
        quantity: 1,
    });

    let matches = market.match_orders();
    assert_eq!(matches.len(), 1, "Should match 1 order");

    // Check transaction details
    let txn = &matches[0];
    assert_eq!(txn.buyer_id, AgentId(1));
    assert_eq!(txn.seller_id, AgentId(2));
    assert_eq!(txn.price, 75); // (100+50)/2

    // After matching, orders should be removed
    assert!(market.bids.is_empty() || market.asks.is_empty());
}

#[test]
fn test_no_match() {
    let mut market = Market::new();

    // Buy at 10
    market.add_order(Order {
        agent_id: AgentId(1),
        order_type: OrderType::Bid,
        price: 10,
        quantity: 1,
    });

    // Sell at 50
    market.add_order(Order {
        agent_id: AgentId(2),
        order_type: OrderType::Ask,
        price: 50,
        quantity: 1,
    });

    let matches = market.match_orders();
    assert_eq!(matches.len(), 0, "Should not match");
}
