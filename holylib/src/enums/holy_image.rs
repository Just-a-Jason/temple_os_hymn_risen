#[derive(Debug)]
pub enum HolyImage {
    Gif(crate::HolyGif),
    Static(crate::HolySprite),
}

impl HolyImage {
    pub fn sprite(&mut self) -> &crate::HolySprite {
        match self {
            HolyImage::Gif(gif) => gif.current_frame(),
            HolyImage::Static(sprite) => sprite,
        }
    }
}
