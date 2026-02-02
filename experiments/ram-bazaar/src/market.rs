use crate::model::{Agent, AgentId, Bid, MarketState};
use std::collections::HashMap;

/// Resolves the market auction.
pub fn resolve_market(agents: &mut [Agent], state: &mut MarketState, mut bids: Vec<Bid>) {
    // 1. Sort bids by price (descending)
    // We use partial_cmp because f64 doesn't implement Ord.
    // Handling NaN by treating it as -infinity (pushing to end).
    bids.sort_by(|a, b| {
        b.price
            .partial_cmp(&a.price)
            .unwrap_or(std::cmp::Ordering::Less)
    });

    // 2. Determine winners (Top N bids, where N is total pages)
    let total_pages = state.total_pages();
    let winning_bids = if bids.len() > total_pages {
        &bids[0..total_pages]
    } else {
        &bids[..]
    };

    // 3. Reset Agent ownership counts
    // We map agent_id back to index in the agents slice for quick update
    let mut agent_map: HashMap<AgentId, usize> = HashMap::new();
    for (idx, agent) in agents.iter_mut().enumerate() {
        agent.owned_pages = 0;
        agent_map.insert(agent.id, idx);
    }

    // 4. Assign Pages
    // For MVP, we just overwrite the pages array from 0..N with the winners.
    // This creates a "defragmented" look where high bidders are at the top-left.
    // To make it look "fragmented" (more realistic), we should map winners to their existing pages.

    // Improved Spatial Allocation:
    // a. Create a demand map: Agent -> Count of winning bids
    let mut demand: HashMap<AgentId, usize> = HashMap::new();
    for bid in winning_bids {
        *demand.entry(bid.agent_id).or_insert(0) += 1;
    }

    // b. Identify existing valid ownership
    // Mark pages as "kept" if the owner is in the demand map and has demand remaining.
    let mut claimed_pages = vec![false; total_pages];

    // First pass: Keep existing pages if possible
    for (page_idx, page) in state.pages.iter_mut().enumerate() {
        if let Some(owner_id) = page.owner
            && let Some(count) = demand.get_mut(&owner_id)
            && *count > 0
        {
            // Agent keeps this page
            *count -= 1;
            claimed_pages[page_idx] = true;
        }
    }

    // Second pass: Fill gaps with remaining demand
    // Flatten remaining demand into a list of AgentIds
    let mut remaining_demand: Vec<AgentId> = Vec::new();
    for (agent_id, count) in demand.iter() {
        for _ in 0..*count {
            remaining_demand.push(*agent_id);
        }
    }

    let mut demand_idx = 0;
    for (page_idx, claimed) in claimed_pages.iter().enumerate() {
        if !*claimed {
            // This page is free or was evicted. Assign to next waiting winner.
            if demand_idx < remaining_demand.len() {
                let new_owner = remaining_demand[demand_idx];
                state.pages[page_idx].owner = Some(new_owner);
                state.pages[page_idx].rent = 0.0; // Todo: set price
                demand_idx += 1;
            } else {
                // No more demand, page is empty
                state.pages[page_idx].owner = None;
                state.pages[page_idx].rent = 0.0;
            }
        }
    }

    // 5. Update Rent and Agent Stats
    // Calculate global clearing price (lowest winning bid)
    let clearing_price = if let Some(last_winner) = winning_bids.last() {
        last_winner.price
    } else {
        0.0
    };

    state.current_price = clearing_price;
    state.price_history.push(clearing_price);
    if state.price_history.len() > 100 {
        state.price_history.remove(0);
    }

    // Apply rent to all owned pages
    for page in state.pages.iter_mut() {
        if let Some(owner) = page.owner {
            page.rent = clearing_price;
            if let Some(idx) = agent_map.get(&owner) {
                agents[*idx].owned_pages += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Agent, Strategy};

    #[test]
    fn test_highest_bidder_wins() {
        let mut state = MarketState::new(1, 1); // 1x1 grid = 1 page
        let agent1 = Agent::new(0, Strategy::Greedy, 100.0, 1);
        let agent2 = Agent::new(1, Strategy::Greedy, 100.0, 1);

        // Agent 2 bids higher
        let bids = vec![
            Bid {
                agent_id: 0,
                price: 10.0,
            },
            Bid {
                agent_id: 1,
                price: 20.0,
            },
        ];

        let mut agents = vec![agent1.clone(), agent2.clone()];

        resolve_market(&mut agents, &mut state, bids);

        // Assert Agent 2 (id 1) owns the page
        assert_eq!(
            state.pages[0].owner,
            Some(1),
            "Highest bidder should own the page"
        );
        // With Single Price Auction logic, the price is the lowest *winning* bid.
        // Since only Agent 2 wins, the clearing price is 20.0.
        assert_eq!(
            state.pages[0].rent, 20.0,
            "Rent should match the clearing price"
        );

        // Verify agent stats
        assert_eq!(agents[1].owned_pages, 1);
        assert_eq!(agents[0].owned_pages, 0);
    }
}
