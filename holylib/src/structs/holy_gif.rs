#[derive(Debug)]
pub struct HolyGif {
    frames: Vec<crate::HolySprite>,
    delays: Vec<u16>,
    current: usize,
    last_tick: std::time::Instant,
}

impl HolyGif {
    pub fn new(source: &str) -> std::io::Result<crate::HolyImage> {
        let image = std::fs::File::open(source)?;

        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);

        let mut delays: Vec<u16> = Vec::with_capacity(16);
        let mut frames: Vec<crate::HolySprite> = Vec::with_capacity(16);

        let decoder = options.read_info(image).unwrap();

        for frame in decoder {
            let frame = frame.unwrap();

            frames.push(crate::HolySprite::from_frame_buffer(
                frame.width as u32,
                frame.height as u32,
                frame.buffer.into_owned(),
            ));

            delays.push(frame.delay);
        }

        return Ok(crate::HolyImage::Gif(HolyGif {
            current: usize::default(),
            delays,
            frames,
            last_tick: std::time::Instant::now(),
        }));
    }

    fn next_frame(&mut self) {
        let now = std::time::Instant::now();
        let delay_ms = self.delays[self.current] as u64 * 10;
        if now.duration_since(self.last_tick).as_millis() >= delay_ms as u128 {
            self.current = (self.current + 1) % self.frames.len();
            self.last_tick = now;
        }
    }

    pub fn into_frames(self) -> Vec<crate::HolySprite> {
        self.frames
    }

    pub fn current_frame(&mut self) -> &crate::HolySprite {
        self.next_frame();
        &self.frames[self.current]
    }
}
