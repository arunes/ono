use color_eyre::Result;

use crate::tui::app::App;

mod config;
mod os_helper;
mod store;
mod tui;

fn main() -> Result<()> {
    env_logger::init();
    let _ = color_eyre::install();

    let ono_config = match config::get_config() {
        Ok(cfg) => {
            log::debug!("Config retrieved: {cfg:?}");
            cfg
        }
        Err(err) => {
            log::error!("Error occurred while getting the config: {err:?}");
            panic!("")
        }
    };

    let mut app = App::default();
    if let Some(data_dir) = ono_config.data_dir {
        app.snippets = store::load_snippets(&data_dir)?;
    }

    render_tui(&mut app)
}

fn render_tui(app: &mut App) -> Result<()> {
    let mut terminal = tui::init()?;
    let app_result = app.run(&mut terminal);
    if let Err(err) = tui::restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {err}"
        );
    }
    app_result
}
