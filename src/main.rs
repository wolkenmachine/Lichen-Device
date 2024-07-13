use minifb::{Key, Window, WindowOptions};

mod audio;
use audio::AudioSystem;

mod rasterize;
use rasterize::{rgb, rp, Color, Raster, RasterPoint};
use std::sync::{Arc, Mutex};

mod sequencer;
use sequencer::Sequencer;

const WIDTH: usize = 800;
const HEIGHT: usize = 480;

fn main() {
    // Setup window
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);
    window.set_cursor_visibility(false);

    // Audio
    let sequencer = Arc::new(Mutex::new(Sequencer::new(vec![0.25, 0.5, 0.75, 1.0])));
    let audio = AudioSystem::new(sequencer.clone());

    // Raster
    let mut raster = Raster::new(WIDTH, HEIGHT);
    let background = rgb!(16, 18, 33);

    let mut x: isize = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        x += 1;
        if x > WIDTH as isize {
            x = 0;
        }

        raster.fill(&background);
        raster.fill_rect(rp!(0, 100), rp!(x, 200), &rgb!(255, 0, 0));

        window
            .update_with_buffer(raster.borrow_buffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}
