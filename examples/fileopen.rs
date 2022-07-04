use macroquad::{
    prelude::*,
    ui::{root_ui, widgets},
};

use std::sync::{Arc, Mutex};

#[macroquad::main("Open file dialog")]
async fn main() {
    let file_data = Arc::new(Mutex::new(None));

    loop {
        clear_background(WHITE);

        if widgets::Button::new("Open file")
            .size(vec2(400., 50.))
            .ui(&mut *root_ui())
        {
            quad_fileopen::find_file(file_data.clone());
        }

        if let Some(ref file_data) = &*file_data.lock().unwrap() {
            info!("content byte length: {}", file_data.len());

            root_ui().label(
                None,
                &format!("Got a file, content byte length: {}", file_data.len()),
            );
        }

        next_frame().await;
    }
}
