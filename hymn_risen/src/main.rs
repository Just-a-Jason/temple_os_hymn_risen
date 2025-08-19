use holylib::{HolyGif, HolyParticle, HolyVector2, HolyWindow};
use std::time::Duration;

// Perfect resolution, chosen by god
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const MAX_PARTICLES: u8 = 50;

fn main() {
    unsafe {
        let mut window = HolyWindow::new(WIDTH, HEIGHT, "Temple Os Hymn Risen", false);

        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let mixer = stream_handle.mixer();

        let file = std::fs::File::open("resources/music/hymn.mp3").unwrap();
        let sink = rodio::play(mixer, std::io::BufReader::new(file)).unwrap();

        let mut terry = HolyGif::new("resources/terry.gif").unwrap();
        const TERY_POS: HolyVector2 = HolyVector2::new(350, 250);

        let mut particle_src = HolyGif::new("resources/particle.gif").unwrap();
        let mut particles = init_particles(WIDTH, HEIGHT, MAX_PARTICLES);

        loop {
            if !window.update() {
                break;
            }

            window.clear();

            if particles.len() < MAX_PARTICLES as usize {
                particles.push(HolyParticle::new(WIDTH, HEIGHT));
            }

            for particle in particles.iter_mut() {
                particle.update();
                window.draw_image_at(&mut particle_src, particle.position);
            }

            window.draw_image_at(&mut terry, TERY_POS);
            window.render_frame();

            particles.retain(|particle| {
                !particle.is_dead(
                    WIDTH + particle_src.sprite().width(),
                    HEIGHT + particle_src.sprite().height(),
                )
            });

            // 60 FPS
            std::thread::sleep(Duration::from_millis(16));
        }
    }
}

fn init_particles(width: u32, height: u32, num_particles: u8) -> Vec<crate::HolyParticle> {
    let mut particles = Vec::new();

    for _ in 0..num_particles {
        particles.push(HolyParticle::new(width, height));
    }

    particles
}
