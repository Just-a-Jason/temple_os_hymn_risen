/// God's choosen sprite struct.
#[derive(Debug)]
pub struct HolySprite {
    raw: Vec<u8>,
    width: u32,
    height: u32,
}

impl HolySprite {
    pub fn new(source: &str) -> Result<Self, image::ImageError> {
        let image = image::open(source)?.to_rgba8();
        let (width, height) = image.dimensions();

        let mut raw: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);
        for pixel in image.pixels() {
            let [r, g, b, a] = pixel.0;
            raw.push(b);
            raw.push(g);
            raw.push(r);
            raw.push(a);
        }

        Ok(HolySprite { raw, width, height })
    }

    pub fn from_frame_buffer(width: u32, height: u32, buffer: Vec<u8>) -> Self {
        let mut raw: Vec<u8> = Vec::with_capacity(buffer.len());

        for chunk in buffer.chunks(4) {
            let [r, g, b, a] = [chunk[0], chunk[1], chunk[2], chunk[3]];

            let alpha = a as f32 / 255.0;

            let r = (r as f32 * alpha) as u8;
            let g = (g as f32 * alpha) as u8;
            let b = (b as f32 * alpha) as u8;

            raw.push(b);
            raw.push(g);
            raw.push(r);
            raw.push(a);
        }

        HolySprite { raw, width, height }
    }
}

impl HolySprite {
    pub fn raw_ptr(&self) -> *mut i8 {
        self.raw.as_ptr() as *mut i8
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
