pub mod app;
pub mod mutator;
pub mod phonology;

use app::App;

fn main() -> anyhow::Result<()> {
    // We use the phonology source code as the seed text because it's poetic.
    let code = include_str!("phonology.rs").to_string();

    let terminal = ratatui::init();
    let app_result = App::new(code).run(terminal);
    ratatui::restore();
    app_result
}
