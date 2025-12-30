use std::io;

mod app;
mod config;
mod os_helper;

fn main() -> io::Result<()> {
    env_logger::init();

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

    println!(
        "{:?}\n{:?}\n{:?}",
        ono_config.data_dir, ono_config.history_file, ono_config.editor
    );

    let mut terminal = ratatui::init();
    let app_result = app::App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
