const MAX_WIDTH: i32 = 8192;
const MAX_HEIGHT: i32 = 8192;
const height_LIMIT: i32 = 8092;
const WIDTH_LIMIT: i32 = 8092;

use std::default::Default;
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

#[derive(Default, Copy, Clone, PartialEq)]
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
    pub fn new() -> Self {
        return Default::default();
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn edge(&self) -> i32 {
        self.edge
    }

    pub fn set_height(mut self, height: i32) -> Self {
        self.height = height;
        self
    }

    pub fn set_width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }

    pub fn set_edge(mut self, edge: i32) -> Self {
        self.edge = edge;
        self
    }

    pub fn set_longside(mut self, longside: i32) -> Self {
        self.LongSide = longside;
        self
    }

    pub fn set_region_crop(mut self, rc: Option<RegionCrop>) -> Self {
        self.regionCrop = rc;
        self
    }

    pub fn set_resize(mut self, resize: Option<Resize>) -> Self {
        self.resize = resize;
        self
    }

    pub fn set_crop(mut self, crop: Option<Crop>) -> Self {
        self.crop = crop;
        self
    }

    pub fn set_proportion(mut self, p: i32) -> Self {
        self.p = p;
        self
    }

    pub fn set_auto_crop(mut self, ac: bool) -> Self {
        if ac {
            self.C = 1;
        } else {
            self.C = 0;
        }
        self
    }

    pub fn adapt(&mut self) -> ParamResult<ImageHandler> {
        let mut result: ImageHandler = Default::default();
        result.resize = match &self.resize {
            Some(r) => Some(r.clone()),
            None => Some(Resize {
                width: 0,
                height: 0,
            }),
        };

        let mut crop = self.crop.clone();
        let mut resize = self.resize.clone();
        let mut region_crop = self.regionCrop.clone();

        let mut oriH: i32 = self.height;
        let mut oriW: i32 = self.width;

        let mut adapt_h: i32 = self.height;
        let mut adapt_w: i32 = self.width;

        let mut caluate: bool = false;

        if self.width > MAX_WIDTH {
            result.height = (self.height as f32 * MAX_WIDTH as f32 / self.width as f32) as i32;
            result.width = MAX_WIDTH;
        } else {
            result.width = self.width;
        }

        if self.height > MAX_HEIGHT {
            result.width = (self.width as f32 * MAX_HEIGHT as f32 / self.height as f32) as i32;
            result.height = MAX_HEIGHT;
        } else {
            result.height = self.height;
        }
        let mut fh;
        let mut fw;
        match self.resize {
            Some(mut resize_param) => {
                if resize_param.height > MAX_HEIGHT {
                    resize_param.width = (resize_param.width as f32 * MAX_HEIGHT as f32
                        / resize_param.height as f32)
                        as i32;
                    resize_param.height = MAX_HEIGHT;
                }
                if resize_param.width > MAX_WIDTH {
                    resize_param.height = (resize_param.height as f32 * MAX_WIDTH as f32
                        / resize_param.width as f32)
                        as i32;
                    resize_param.width = MAX_WIDTH;
                }
                if resize_param.height == 0 {
                    result.height = self.height;
                }
                if resize_param.width == 0 {
                    result.width = self.width;
                }

                fh = resize_param.height;
                fw = resize_param.width;
            }
            None => {
                fh = 0;
                fw = 0;
                result.height = self.height;
                result.width = self.width;
            }
        }

        if fh <= 0 && fw > 0 {
            fh = (self.height as f32 * fw as f32 / self.width as f32) as i32;
        }
        if fw <= 0 && fh > 0 {
            fw = (self.width as f32 * fh as f32 / self.height as f32) as i32;
        }

        if (fh > 0 && fw > 0) || self.p > 1 {
            let mut refh_refw_longside: (i32, i32, i32);
            refh_refw_longside = CaluatSize(self.height, self.width, fh, fw, self.edge, self.p);
            result.height = refh_refw_longside.0;
            result.width = refh_refw_longside.1;
            result.LongSide = refh_refw_longside.2;
            result.resize = Some(Resize {
                width: refh_refw_longside.1,
                height: refh_refw_longside.0,
            });
            println!(
                "result height {:?} width {:?} longside {:?}",
                result.height, result.width, refh_refw_longside.2
            );
            caluate = true;
        }
        if caluate
            && (result.height() * result.width() > height_LIMIT * WIDTH_LIMIT
                || result.height() >= height_LIMIT * 4
                || result.width() >= WIDTH_LIMIT)
        {
            return Err(ParamError::ErrResizeParams);
        }

        if result.height() > 0
            && result.width() > 0
            && (result.height() != oriH && result.width() != oriW)
        {
            if result.width() > MAX_HEIGHT {
                result.width =
                    (result.width() as f32 * MAX_HEIGHT as f32 / result.height() as f32) as i32;
                result.height = (MAX_HEIGHT);
            }
            if result.width() > MAX_WIDTH {
                result.height =
                    (result.height() as f32 * MAX_WIDTH as f32 / result.width() as f32) as i32;
                result.width = (MAX_WIDTH);
            }
            result.resize = Some(Resize {
                width: result.width(),
                height: result.height(),
            });
        }
        if !crop.is_none() {
            match crop {
                Some(mut crop) => {
                    if crop.x > result.width() || crop.x < 0 {
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
            }
            if self.C == 1 && self.edge == 1 || (result.C == 1 && result.edge == 1) {
                match result.LongSide {
                    1 => {
                        let cropW = fw;
                        let cropH = result.height();
                        let cropPosX = (result.width() - cropW) / 2;
                        let cropPosY = 0;
                        result.crop = Some(Crop {
                            x: cropPosX,
                            y: cropPosY,
                            height: cropH,
                            width: cropW,
                        });
                    }
                    2 => {
                        let cropW = result.width();
                        let cropH = fh;

                        let cropPosX = 0;
                        let cropPosY = (result.height() - cropH) / 2;
                        result.crop = Some(Crop {
                            x: cropPosX,
                            y: cropPosY,
                            height: cropH,
                            width: cropW,
                        });
                    }
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
