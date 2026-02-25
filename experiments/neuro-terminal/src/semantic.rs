#[cfg(feature = "nova")]
use crate::App;
#[cfg(feature = "nova")]
use tui_shared::semantic::{Action, Entity, Region, Snapshot};

#[cfg(feature = "nova")]
pub fn create_snapshot(app: &App) -> Snapshot {
    let mut snap = Snapshot::new("neuro-terminal")
        .with_metric("steps", app.steps)
        .with_metric("paused", app.paused)
        .with_state(if app.paused { "paused" } else { "running" })
        .with_viewport(100, 100); // Approximate viewport

    if let Some(last_loss) = app.loss_history.last() {
        snap = snap.with_metric("last_loss", *last_loss as f64 / 1000.0);
    }

    // Regions
    snap = snap
        .with_region(Region::new("header", 0, 0, 100, 3).describe("Title and stats"))
        .with_region(
            Region::new("decision_boundary", 0, 3, 50, 50)
                .describe("Visualization of the network's classification"),
        )
        .with_region(Region::new("network_graph", 50, 3, 50, 50).describe("Neurons and weights"))
        .with_region(Region::new("footer", 0, 53, 100, 1).describe("Controls"));

    // Actions
    snap = snap
        .with_action(Action::new("quit").key("q"))
        .with_action(Action::new("pause_resume").key("p"))
        .with_action(Action::new("reset").key("r"))
        .with_action(Action::new("snapshot").key("S"));

    // Entities: Neurons
    for (layer_idx, &neuron_count) in app.network.layers.iter().enumerate() {
        for n in 0..neuron_count {
            let mut entity = Entity::new("neuron")
                .with_id(format!("l{}_n{}", layer_idx, n))
                .with_prop("layer", layer_idx)
                .with_prop("index", n);

            if let Some(layer_data) = app.network.data.get(layer_idx) {
                // If we have activation data
                let activation = layer_data.get(n, 0);
                entity = entity.with_prop("activation", activation);
            }
            snap = snap.with_entity(entity);
        }
    }

    // Entities: Weights
    // Iterate over weights matrices
    for (layer_idx, weights) in app.network.weights.iter().enumerate() {
        for r in 0..weights.rows {
            // To (next layer)
            for c in 0..weights.cols {
                // From (current layer)
                let val = weights.get(r, c);
                let entity = Entity::new("weight")
                    .with_id(format!("w_{}_{}_{}", layer_idx, c, r)) // layer, from, to
                    .with_prop("layer", layer_idx)
                    .with_prop("from_neuron", c)
                    .with_prop("to_neuron", r)
                    .with_prop("value", val);
                snap = snap.with_entity(entity);
            }
        }
    }

    snap
}

#[cfg(all(test, feature = "nova"))]
mod tests {
    use super::*;
    use crate::App;

    #[test]
    fn test_create_snapshot() {
        let mut app = App::new();
        app.steps = 10;
        app.paused = true;

        // Train once to generate data/activations so network.data is populated
        // The original App::new() doesn't populate data, so we need to run update or forward
        // However, App::update() runs random batches.
        // Let's force a forward pass.
        app.network.forward(&[0.5, 0.5]);

        let snap = create_snapshot(&app);

        assert_eq!(snap.app, "neuro-terminal");
        assert_eq!(snap.state.as_deref(), Some("paused"));

        // Check for entities
        let neurons = snap.entities.iter().filter(|e| e.kind == "neuron").count();
        // Default network is [2, 5, 4, 1] -> 2+5+4+1 = 12 neurons
        assert_eq!(neurons, 12);

        let weights = snap.entities.iter().filter(|e| e.kind == "weight").count();
        // Weights: 2*5 + 5*4 + 4*1 = 10 + 20 + 4 = 34 weights
        assert_eq!(weights, 34);
    }
}
