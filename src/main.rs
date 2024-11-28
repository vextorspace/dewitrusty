// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    use slint::Model;

    let ui = AppWindow::new()?;

    let items = slint::VecModel::default();

    items.push(ListItem{text: "Item 1".into()});
    items.push(ListItem{text: "Item 2".into()});
    items.push(ListItem{text: "Added Item".into()});

    ui.set_items(std::rc::Rc::new(items).into());

    ui.run()?;

    Ok(())
}

