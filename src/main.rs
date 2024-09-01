mod cpal;
mod math;

use nannou::prelude::*;

const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 500;

struct Model {}

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
                    .title(app.exe_name().unwrap())
                    .size(WINDOW_WIDTH, WINDOW_HEIGHT)
                    .view(view)
                    .key_pressed(key_pressed)
                    .key_released(key_released)
                    .build()
                    .unwrap();

    Model {}
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn key_pressed(_app: &App, _model: &mut Model, key: Key) {
    println!("Pressed: {:?}", key);
}

fn key_released(_app: &App, _model: &mut Model, key: Key) {
    println!("Released: {:?}", key);
}