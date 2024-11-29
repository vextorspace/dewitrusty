// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dewitrusty::data::default_item_loader::DefaultItemLoader;
use dewitrusty::ui::initializer::Initializer;
use dewitrusty::ui::slint_exports::AppWindow;
use slint::ComponentHandle;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    Initializer::default().with_item_loader(
        Box::new(DefaultItemLoader::new())
    ).run(&ui);

    ui.run()?;

    Ok(())
}

