use nannou::prelude::*;
use nannou_audio;

mod audio_processing;
mod oscillator;
mod midi;

use audio_processing::AudioModel;
use audio_processing::audio;

const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 500;

struct Model {
    stream: nannou_audio::Stream<AudioModel>,
}

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

    let audio_host = nannou_audio::Host::new();

    let audio_model = AudioModel::new();

    let stream = audio_host.new_output_stream(audio_model)
                        .render(audio)
                        .build()
                        .unwrap();

    Model {
        stream
    }
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    println!("Pressed: {:?}", key);

    if key == Key::A {
        model.stream.send(move |audio_model| {
            audio_model.activate_note(0).unwrap();
        }).unwrap();
    };
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
    println!("Released: {:?}", key);

    if key == Key::A {
        model.stream.send(move |audio_model| {
            audio_model.deactivate_note(0).unwrap();
        }).unwrap();
    };
}