const MAX_WIDTH: i32 = 8192;
const MAX_HEIGHT: i32 = 8192;

#[derive(Default)]
pub struct ImageHandler {
    height: i32,
    width: i32,
    crop: Vec<i32>,
    resize: Vec<i32>,
}

impl ImageHandler {
    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width
    }

    pub fn adapt(&mut self, w: i32, h: i32) -> ImageHandler {
        let mut result: ImageHandler;

        let mut adapt_h: i32 = h;
        let mut adapt_w: i32 = w;
        if h > MAX_HEIGHT {
            adapt_w = (w as f32 * MAX_HEIGHT as f32 / h as f32) as i32;
            adapt_h = MAX_HEIGHT;
        }
        if w > MAX_WIDTH {
            adapt_h = (h as f32 * MAX_WIDTH as f32 / w as f32) as i32;
            adapt_w = MAX_WIDTH;
        }

        if self.width > MAX_WIDTH {
            result.set_height((self.height as f32 * MAX_WIDTH as f32 / self.width as f32) as i32);
            result.set_width(MAX_WIDTH)
        } else {
            result.set_width(self.width)
        }

        if self.height > MAX_HEIGHT {
            result.set_width(self.width as f32 * MAX_HEIGHT as f32 / self.height as f32) as i32);
            result.set_height(MAX_HEIGHT);
        } else {
            result.set_height(self.height)
        }
    }
}
