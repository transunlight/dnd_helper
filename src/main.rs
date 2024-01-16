use color_eyre::eyre::Result;
use flexi_logger::{FileSpec, Logger};

use dnd_helper::app::App;
use dnd_helper::tui::Tui;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    Logger::try_with_str("info")?
        .log_to_file(FileSpec::default())
        .start()?;

    let mut tui = Tui::new(1)?;
    tui.enter()?;

    let mut app = App::new();

    while !app.should_quit {
        tui.draw(&app)?;
        app.update(tui.events.next().await?);
    }

    tui.exit()?;
    Ok(())
}
