extern crate lodepng;
extern crate rgb;

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::rgb::ComponentBytes;

#[derive(Debug, Default, PartialEq, Eq)]
struct Size {
    width: usize,
    height: usize,
}

#[derive(Debug, Default)]
struct Render {
    transition: Vec<f32>,
    image1: Vec<u8>,
    image2: Vec<u8>,
    output: String,
    size: Size,
}

impl Size {
    fn new(width: usize, height: usize) -> Size {
        Size { width, height }
    }
}

impl Render {
    pub fn first_image<P: AsRef<Path>>(mut self, filename: P) -> Render {
        let (image, size) = load_image(filename).unwrap();
        self.image1 = image;
        self.size = size;
        self
    }

    pub fn second_image<P: AsRef<Path>>(mut self, filename: P) -> Render {
        let (image, size) = load_image(filename).unwrap();
        // никаких переходов в случае картинок разного размера
        if self.size != size {
            panic!("image size mismatch");
        }
        self.image2 = image;
        self
    }

    pub fn add_transition(mut self, start: f32, stop: f32, step: f32) -> Render {
        // эти два варианта мы игнорируем, т.к. они расходятся
        let bad_condition = (start > stop && step > 0.0) || (start < stop && step < 0.0);
        // для всех остальных считаем количество шагов
        let count = if !bad_condition {
            ((start - stop) / step).abs() as u32 + 1
        } else {
            0
        };
        // и генерируем переход
        let mut transition: Vec<f32> = (0..count).map(|x| start + x as f32 * step).collect();
        self.transition.append(&mut transition);
        self
    }

    pub fn set_output_file(mut self, output: &str) -> Render {
        self.output = output.to_owned();
        self
    }

    pub fn render(self, fps: u8) {
        // аргументы для ffmpeg
        #[rustfmt::skip]
        let arguments = [
            "-f", "rawvideo", "-pix_fmt", "rgb24", "-video_size", &format!("{}x{}", self.size.width, self.size.height),
            "-r", &format!("{}", fps), "-i", "-", "-c:v", "libx264", "-preset", "slow", "-profile:v", "high",
            "-crf", "18", "-coder", "1", "-pix_fmt", "yuv420p", "-vf", "scale=iw:-2", "-movflags", "+faststart",
            "-g", "30", "-bf", "2", "-y", &self.output,
        ];
        // создаём процесс
        let mut process = match Command::new("ffmpeg")
            .args(&arguments)
            .stdin(Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("couldn't spawn ffmpeg: {}", why),
            Ok(process) => process,
        };
        {
            // заимствуем stdin
            let stdin = process.stdin.as_mut().unwrap();
            // и фигачим в него наши картиночки
            for alpha in &self.transition {
                let img = self.blend(*alpha);
                match stdin.write_all(&img) {
                    Err(why) => panic!("couldn't write to ffmpeg stdin: {}", why),
                    Ok(_) => (),
                };
            }
        }
        // ожидание завершения ffmpeg
        let _result = process.wait().unwrap();
    }

    fn blend(&self, alpha: f32) -> Vec<u8> {
        let mut r = vec![0; self.image1.len()];
        for (d, (a, b)) in r.iter_mut().zip(self.image1.iter().zip(self.image2.iter())) {
            *d = (*a as f32 * alpha + *b as f32 * (1.0 - alpha)).round() as u8;
        }
        r
    }
}

fn load_image<P: AsRef<Path>>(filename: P) -> Result<(Vec<u8>, Size), lodepng::Error> {
    let image = lodepng::decode24_file(filename)?;
    let size = Size::new(image.width, image.height);
    Ok((image.buffer.as_bytes().to_vec(), size))
}

fn main() {
    // время ролика в секундах
    let transition_time = 5;
    // количество кадров в секунду у выходного видео
    let fps = 25;
    // шаг для создания перехода на заданное время и fps
    let step = 2.0 / (fps * transition_time) as f32;
    // тут должно быть всё понятно
    let i = Render::default()
        .first_image("./demo/01.png")
        .second_image("./demo/02.png")
        .add_transition(0.0, 1.0, step)
        .add_transition(1.0, 0.0, -step)
        .set_output_file("render.mp4");
    i.render(fps);
}
