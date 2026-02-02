use crate::model::{Core, ThreadAgent, MarketState, AgentState};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Bid {
    pub agent_id: usize,
    pub price: f64,
}

pub fn resolve_auction(
    agents: &mut [ThreadAgent],
    cores: &mut [Core],
    state: &mut MarketState,
    mut bids: Vec<Bid>,
) {
    // Reset all running agents to Active (if not finished/killed)
    for agent in agents.iter_mut() {
        if agent.state == AgentState::Running {
            agent.state = AgentState::Active;
        }
    }

    // Sort bids descending
    bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(Ordering::Equal));

    // Reset cores
    for core in cores.iter_mut() {
        core.current_agent_id = None;
    }

    let core_count = cores.len();
    let winning_bids = if bids.len() > core_count {
        &bids[0..core_count]
    } else {
        &bids[..]
    };

    // Determine clearing price (k+1 th price, or lowest winning price if not enough bids)
    let clearing_price = if bids.len() > core_count {
        bids[core_count].price
    } else if !bids.is_empty() {
        bids.last().unwrap().price
    } else {
        0.0 // No demand
    };

    // Prevent zero price if there is demand
    let clearing_price = clearing_price.max(0.01);

    state.current_price = clearing_price;
    state.price_history.push(clearing_price);
    if state.price_history.len() > 100 {
        state.price_history.remove(0);
    }

    // Assign winners
    for (i, bid) in winning_bids.iter().enumerate() {
        if let Some(agent) = agents.iter_mut().find(|a| a.id == bid.agent_id) {
            // Ensure agent can afford it (though they should have bid responsibly)
            // If they can't afford, we technically should skip them, but let's assume debt or strict bidding

            agent.state = AgentState::Running;
            agent.credits -= clearing_price;
            agent.work_remaining = (agent.work_remaining - 1.0).max(0.0);
            agent.executed_ticks += 1;

            // Assign to core
            if i < cores.len() {
                cores[i].current_agent_id = Some(agent.id);
                cores[i].utilization = (cores[i].utilization * 0.9) + 0.1; // Simple smoothing
            }
        }
    }

    // Decay utilization for idle cores
    for core in cores.iter_mut() {
        if core.current_agent_id.is_none() {
            core.utilization *= 0.9;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Strategy;

    #[test]
    fn test_auction_clearing() {
        let mut agents = vec![
            ThreadAgent::new(0, Strategy::Value, 100.0, 10.0, 100),
            ThreadAgent::new(1, Strategy::Value, 100.0, 10.0, 100),
            ThreadAgent::new(2, Strategy::Value, 100.0, 10.0, 100),
        ];
        let mut cores = vec![Core { id: 0, current_agent_id: None, utilization: 0.0 }];
        let mut state = MarketState::new();

        let bids = vec![
            Bid { agent_id: 0, price: 10.0 },
            Bid { agent_id: 1, price: 5.0 },
            Bid { agent_id: 2, price: 1.0 },
        ];

        resolve_auction(&mut agents, &mut cores, &mut state, bids);

        // Core count is 1. Top bid (10.0) wins.
        // Logic: k=1. bids[k] = bids[1] = 5.0.

        assert_eq!(state.current_price, 5.0);
        assert_eq!(cores[0].current_agent_id, Some(0));
        assert_eq!(agents[0].executed_ticks, 1);
        // Credits should decrease by clearing price (5.0)
        assert!((agents[0].credits - 95.0).abs() < 0.001);
    }
}
