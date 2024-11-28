// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use dewitrusty::ui::initializer::Initializer;
use dewitrusty::ui::slint_exports::{AppWindow, ListItem};
use slint::ComponentHandle;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    Initializer::default().run(&ui);

    let items = slint::VecModel::default();

    items.push(ListItem{text: "Item 1".into()});
    items.push(ListItem{text: "Item 2".into()});
    items.push(ListItem{text: "Added Item".into()});

    ui.set_items(std::rc::Rc::new(items).into());

    ui.run()?;

    Ok(())
}

