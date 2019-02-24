const MAX_WIDTH: i32 = 8192;
const MAX_HEIGHT: i32 = 8192;
const HEIGHT_LIMIT: i32 = 8092;
const WIDTH_LIMIT: i32 = 8092;

use std::result::Result;

#[derive(Debug)]
pub enum ParamError {
    ErrCropParams,
    ErrResizeParams,
}

pub type ParamResult<T> = Result<T, ParamError>;

#[derive(Default, Clone, PartialEq)]
pub struct Crop {
    pub x: i32,
    pub y: i32,
    pub height: i32,
    pub width: i32,
}

#[derive(Default, Clone, PartialEq)]
pub struct RegionCrop {
    pub height: i32,
    pub width: i32,
    pub Region: i32,
}

#[derive(Default, Clone, PartialEq)]
pub struct Resize {
    pub height: i32,
    pub width: i32,
}

#[derive(Default)]
pub struct ImageHandler {
    pub height: i32,
    pub width: i32,
    pub crop: Option<Crop>,
    pub resize: Option<Resize>,
    pub regionCrop: Option<RegionCrop>,

    /**
      e: 图片缩放, 缩放尺寸比例与原图比例不同时的优先缩放边, 格式[edge]e, 默认为0表示长边优先, 1表示短边优先, 2表示强制缩放(改变比例), 4表示短边缩略并且用指定颜色填充剩余区域
      缩放限制: w * h <= 4096 * 4096 && w <= 4096 * 4 && h <= 4096 * 4
      长边/短边: 长边指原图相对于缩放尺寸看起来更长的那条边, 也即 原长度/缩放长度 更大的那条边
      长边优先意味着长边满足缩放尺寸, 那么短边必然不足, 也即缩放后图片尺寸小于指定缩放尺寸, 部分需要填充颜色; 短边优先则相反, 尺寸大于缩放尺寸, 需要根据l决定是否截取
    */
    edge: i32,

    /**
    p: 图片缩放, 等比例缩放比例, 格式[ratio]p, [1-1000], 与w/h一起存在时将合并直接得到新w/h
    */
    pub p: i32, // proportional scaling
    /**
    c: 图片裁剪, 是否进行自动裁剪, 自动裁剪表示图片先按短边缩略，然后从缩略的目标图片裁剪出中间部分得到对应指定高度和宽度的目标缩略图, 格式[value]c, 1表示进行自动裁剪
    */
    pub C: i8,
    pub LongSide: i32,
}

impl ImageHandler {
    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn edge(&self) -> i32 {
        self.edge
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width
    }

    pub fn set_edge(&mut self, edge: i32) {
        self.edge = edge
    }

    pub fn adapt(&mut self, w: i32, h: i32) -> ParamResult<ImageHandler> {
        let mut result: ImageHandler = Default::default();

        let mut crop = self.crop.clone();
        let mut resize = self.resize.clone();
        let mut region_crop = self.regionCrop.clone();

        let mut oriH: i32 = h;
        let mut oriW: i32 = w;

        let mut adapt_h: i32 = h;
        let mut adapt_w: i32 = w;

        let mut caluate: bool = false;

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
            result.set_width((self.width as f32 * MAX_HEIGHT as f32 / self.height as f32) as i32);
            result.set_height(MAX_HEIGHT);
        } else {
            result.set_height(self.height)
        }

        let mut fh = result.height();
        let mut fw = result.width();
        if fh <= 0 && fw > 0 {
            fh = (h as f32 * fw as f32 / w as f32) as i32;
        }
        if fw <= 0 && fh > 0 {
            fw = (w as f32 * fh as f32 / h as f32) as i32;
        }
        if result.height() == 0 {
            result.set_height(h)
        }
        if result.width() == 0 {
            result.set_width(w)
        }
        if (fh > 0 && fw > 0) || self.p > 1 {
            let mut refh_refw_longside: (i32, i32, i32);
            refh_refw_longside = CaluatSize(h, w, fh, fw, self.edge, self.p);
            result.set_height(refh_refw_longside.0);
            result.set_width(refh_refw_longside.1);
            result.LongSide = refh_refw_longside.2;
            caluate = true;
        }
        if caluate
            && (result.height() * result.width() > HEIGHT_LIMIT * WIDTH_LIMIT
                || result.height() >= HEIGHT_LIMIT * 4
                || result.width() >= WIDTH_LIMIT)
        {
            return Err(ParamError::ErrResizeParams);
        }

        if result.height() > 0
            && result.width() > 0
            && (result.height() != oriH && result.width() != oriW)
        {
            if result.width() > MAX_HEIGHT {
                result.set_width(
                    (result.width() as f32 * MAX_HEIGHT as f32 / result.height() as f32) as i32,
                );
                result.set_height(MAX_HEIGHT);
            }
            if result.width() > MAX_WIDTH {
                result.set_height(
                    (result.height() as f32 * MAX_WIDTH as f32 / result.width() as f32) as i32,
                );
                result.set_width(MAX_WIDTH);
            }
            result.resize = Some(Resize {
                width: result.width(),
                height: result.height(),
            });
        }
        if !crop.is_none() {
            match crop {
                Some(mut crop) => {
                    if crop.x > self.width() || crop.x < 0 {
                        crop.x = 0;
                    }
                    if crop.y > result.height() || crop.y < 0 {
                        crop.y = 0
                    }
                    if crop.height < 0 || crop.height > result.height() {
                        crop.height = result.height()
                    }
                    if crop.width < 0 || crop.width > result.width() {
                        crop.width = result.width()
                    }

                    if crop.width == 0 || crop.width > (result.width() - crop.x) {
                        crop.width = result.width() - crop.x
                    }
                    if crop.height == 0 || crop.height > (result.height() - crop.y) {
                        crop.height = result.height() - crop.y
                    }
                    result.crop = Some(crop.clone());
                }
                _ => {}
            }
        } else {
            if !region_crop.is_none() {
                match region_crop {
                    Some(mut regionc) => {
                        let mut rcW = regionc.width;
                        let mut rcH = regionc.height;

                        if rcW < 0 || rcH < 0 {
                            return Err(ParamError::ErrCropParams);
                        }
                        if rcH == 0 {
                            rcH = self.height();
                        }
                        if rcW == 0 {
                            rcW = self.width();
                        }
                        let mut regionH = self.height() / 3;
                        let mut regionW = self.width() / 3;

                        if rcH > regionH {
                            rcH = regionH;
                        }
                        let mut rcXSt = (regionc.Region - 1) % 3;
                        let mut rcYSt = (regionc.Region - 1) / 3;
                        let mut cropPosX = 0;
                        let mut cropPosY = 0;

                        match rcXSt {
                            0 => cropPosX = 0,
                            1 => cropPosX = (rcXSt + regionW) + ((regionW - rcW) / 2),
                            2 => cropPosX = (rcXSt * regionW) + (regionW - rcW),
                            _ => {}
                        }

                        match rcYSt {
                            0 => cropPosY = 0,
                            1 => cropPosY = (rcYSt * regionH) + ((regionH - rcH) / 2),
                            2 => cropPosY = (rcYSt * regionH) + (regionH - rcH),
                            _ => {}
                        }
                        result.crop = Some(Crop {
                            x: cropPosX,
                            y: cropPosY,
                            height: rcH,
                            width: rcW,
                        });
                    }
                    _ => {}
                }
            } else if self.C == 1 && self.edge == 1 {
                match self.LongSide {
                    1 => {}
                    2 => {}
                    _ => {}
                }
            }
        }

        Ok(result)
    }
}

fn CaluatSize(oriH: i32, oriW: i32, h: i32, w: i32, e: i32, p: i32) -> (i32, i32, i32) {
    let mut ratioH: f64;
    let mut ratioW: f64;
    let mut ratio: f64;

    let mut refH = oriH;
    let mut refW = oriW;

    let mut longside: i32 = 0;

    if p > 1 {
        refH = oriH * p / 100;
        refW = oriW * p / 100;
    }
    if e == 2 {
        return (h, w, 0);
    }
    if e >= 0 && h > 0 && w > 0 {
        ratioH = oriH as f64 / h as f64;
        // 1.222493888
        ratioW = oriW as f64 / w as f64;

        longside = 1;
        ratio = ratioH;
        if ratioW > ratioH {
            ratio = ratioW;
            longside = 2;
        }
        if e == 1 {
            ratio = ratioH;
            longside = 1;
            if ratioW < ratioH {
                ratio = ratioW;
                longside = 2;
            }
        }
        refH = (oriH as f64 / ratio) as i32;
        refW = (oriW as f64 / ratio) as i32;
    }
    return (refH, refW, longside);
}
