#[cfg(all(test, feature = "biophysics"))]
mod tests {
    use crate::tui::{AppState, ViewMode};

    #[test]
    fn test_cortex_view_initialization() {
        let mut app_state = AppState::new(None);

        // Default is Genome
        assert_eq!(app_state.view_mode, ViewMode::Genome);

        // Can switch to Cortex
        app_state.view_mode = ViewMode::Cortex;
        assert_eq!(app_state.view_mode, ViewMode::Cortex);

        // Verify fields exist
        assert!(app_state.voltage_history.is_empty());
        assert!(app_state.selected_neuron_coords.is_none());

        // Verify we can push to history
        app_state.voltage_history.push(123);
        assert_eq!(app_state.voltage_history.len(), 1);
        assert_eq!(app_state.voltage_history[0], 123);
    }
}
