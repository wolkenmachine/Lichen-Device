use super::Color;

#[derive(Clone)]
pub struct Raster {
    width: usize,
    height: usize,
    color: Vec<u32>,
}

#[derive(Debug)]
pub struct RasterPoint {
    x: isize,
    y: isize,
}

impl RasterPoint {
    pub fn new(x: isize, y: isize) -> RasterPoint {
        return RasterPoint { x, y };
    }
}

macro_rules! rp {
    ($x:expr, $y:expr) => {
        RasterPoint::new($x, $y)
    };
}
pub(crate) use rp;

impl Raster {
    pub fn new(width: usize, height: usize) -> Raster {
        Raster {
            width,
            height,
            color: vec![0; width * height],
        }
    }

    pub fn fill(&mut self, color: &Color) {
        for i in 0..self.color.len() {
            self.color[i] = color.to_u32();
        }
    }

    pub fn size(&self) -> (usize, usize) {
        return (self.width, self.height);
    }

    pub fn put_pixel_unsafe(&mut self, x: usize, y: usize, color: &Color) {
        let offset = (y * self.width + x);
        self.color[offset] = color.to_u32();
    }

    pub fn put_pixel(&mut self, x: isize, y: isize, color: &Color) {
        if x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
            self.put_pixel_unsafe(x as usize, y as usize, color);
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        return (self.width, self.height);
    }

    pub fn draw_line(&mut self, a: &RasterPoint, b: &RasterPoint, color: &Color) {
        let dx = (b.x - a.x).abs();
        let dy = (b.y - a.y).abs();
        let sx = if a.x < b.x { 1 } else { -1 };
        let sy = if a.y < b.y { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = a.x;
        let mut y = a.y;

        loop {
            self.put_pixel(x, y, color);
            // self.put_pixel(x - 1, y, color);
            // self.put_pixel(x + 1, y, color);

            // self.put_pixel(x, y - 1, color);
            // self.put_pixel(x, y + 1, color);

            // self.put_pixel(x + 1, y + 1, color);
            // self.put_pixel(x + 1, y - 1, color);

            // self.put_pixel(x - 1, y + 1, color);
            // self.put_pixel(x - 1, y - 1, color);

            if x == b.x && y == b.y {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_polyline(&mut self, points: &Vec<RasterPoint>, color: &Color, closed: bool) {
        for i in 0..points.len() - 1 {
            let a = &points[i];
            let b = &points[i + 1];
            self.draw_line(a, b, color);
        }
        if closed {
            let a = &points[0];
            let b = &points[points.len() - 1];
            self.draw_line(a, b, color);
        }
    }

    pub fn fill_rect(&mut self, a: RasterPoint, b: RasterPoint, color: &Color) {
        for x in a.x..b.x {
            for y in a.y..b.y {
                self.put_pixel(x, y, color);
            }
        }
    }

    pub fn fill_polygon(&mut self, points: Vec<RasterPoint>, color: &Color) {
        let x_coords = points.iter().map(|p| p.x).collect::<Vec<isize>>();
        let y_coords = points.iter().map(|p| p.y).collect::<Vec<isize>>();

        // Find the bounding box of the polygon
        let min_y = *y_coords.iter().min().unwrap();
        let max_y = *y_coords.iter().max().unwrap();

        // Loop through each scanline within the bounding box
        for y in min_y..=max_y {
            let mut intersections = Vec::new();

            // Find intersections of the polygon edges with the scanline
            let len = x_coords.len();
            for i in 0..len {
                let j = (i + 1) % len;
                let x1 = x_coords[i];
                let y1 = y_coords[i];
                let x2 = x_coords[j];
                let y2 = y_coords[j];

                if (y1 <= y && y2 > y) || (y2 <= y && y1 > y) {
                    let intersect_x = (y - y1) * (x2 - x1) / (y2 - y1) + x1;
                    intersections.push(intersect_x);
                }
            }

            // Sort the intersection points in ascending order
            intersections.sort();

            // Fill the pixels between pairs of intersection points
            for range in intersections.chunks(2) {
                for x in range[0]..range[1] {
                    self.put_pixel(x, y, color);
                }
            }
        }
    }

    pub fn borrow_buffer(&self) -> &Vec<u32> {
        &self.color
    }
}
