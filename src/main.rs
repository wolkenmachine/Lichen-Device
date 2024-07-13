use minifb::{Key, Window, WindowOptions};

mod rasterize;
use rasterize::{rgb, rp, Color, Raster, RasterPoint};

const WIDTH: usize = 800;
const HEIGHT: usize = 480;

fn main() {
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

    // Raster
    let mut raster = Raster::new(WIDTH, HEIGHT);
    let background = rgb!(16, 18, 33);
    raster.fill(&background);
    raster.fill_rect(rp!(100, 100), rp!(200, 200), &rgb!(255, 0, 0));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(raster.borrow_buffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}
