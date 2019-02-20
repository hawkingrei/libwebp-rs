const MAX_WIDTH: i32 = 8192;
const MAX_HEIGHT: i32 = 8192;

#[derive(Default)]
pub struct ImageContext {
    height: i32,
    width: i32,
    crop: Vec<i32>,
    resize: Vec<i32>,
}

impl ImageContext {
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

    pub fn adapt(&mut self, w: i32, h: i32) {
        let mut adapt_h: i32 = h;
        let mut adapt_w: i32 = w;
        if h > MAX_HEIGHT {
            adapt_w = w * MAX_HEIGHT / h;
            adapt_h = MAX_HEIGHT;
        }
        if w > MAX_WIDTH {
            adapt_h = h * MAX_WIDTH / w;
            adapt_w = MAX_WIDTH;
        }
    }
}
