use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::rgb::ComponentBytes;

pub trait TransitionFunc {
    fn calc(&self, value: f32, im1: &Vec<u8>, im1: &Vec<u8>, size: &Size) -> Vec<u8>;
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Default)]
pub struct Render {
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
        // panic if image not found
        let (image, size) = load_image(filename).unwrap();
        self.image1 = image;
        self.size = size;
        self
    }

    pub fn second_image<P: AsRef<Path>>(mut self, filename: P) -> Render {
        let (image, size) = load_image(filename).unwrap();
        // panic if images not the same size
        if self.size != size {
            panic!("image size mismatch");
        }
        self.image2 = image;
        self
    }

    pub fn add_transition(mut self, start: f32, stop: f32, step: f32) -> Render {
        // ignore this variants because is divergent
        let bad_condition = (start > stop && step > 0.0) || (start < stop && step < 0.0);
        // count steps for transition
        let count = if !bad_condition {
            ((start - stop) / step).abs() as u32 + 1
        } else {
            0
        };
        // create transition
        let mut transition: Vec<f32> = (0..count).map(|x| start + x as f32 * step).collect();
        self.transition.append(&mut transition);
        self
    }

    pub fn transition_series(mut self, series: Vec<f32>, step: f32) -> Render {
        let mut iterator = series.iter();
        while let Some(f1) = iterator.next() {
            // we sure about second value exist
            let f2 = iterator.next().unwrap();
            if f1 > f2 {
                // transition 0 -> 1
                self = self.add_transition(*f1, *f2, -step);
            } else {
                // transition 1 -> 0
                self = self.add_transition(*f1, *f2, step);
            }
        }
        self
    }

    pub fn set_output_file(mut self, output: &str) -> Render {
        self.output = output.to_owned();
        self
    }

    pub fn render(self, method: &dyn TransitionFunc, fps: u8) {
        // ffmpeg args
        #[rustfmt::skip]
        let arguments = [
            "-f", "rawvideo", "-pix_fmt", "rgb24", "-video_size", &format!("{}x{}", self.size.width, self.size.height),
            "-r", &format!("{}", fps), "-i", "-", "-c:v", "libx264", "-preset", "slow", "-profile:v", "high",
            "-crf", "18", "-coder", "1", "-pix_fmt", "yuv420p", "-vf", "scale=iw:-2", "-movflags", "+faststart",
            "-g", "30", "-bf", "2", "-y", &self.output,
        ];
        // create ffmpeg with pipe
        let mut process = match Command::new("ffmpeg")
            .args(&arguments)
            .stdin(Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("couldn't spawn ffmpeg: {}", why),
            Ok(process) => process,
        };
        {
            // borrow stdin
            let stdin = process.stdin.as_mut().unwrap();
            // and send images into it
            for value in &self.transition {
                // calling transition function
                let img = method.calc(*value, &self.image1, &self.image2, &self.size);
                match stdin.write_all(&img) {
                    Err(why) => panic!("couldn't write to ffmpeg stdin: {}", why),
                    Ok(_) => (),
                };
            }
        }
        // wait ffmpeg
        let _result = process.wait().unwrap();
    }
}

fn load_image<P: AsRef<Path>>(filename: P) -> Result<(Vec<u8>, Size), lodepng::Error> {
    let image = lodepng::decode24_file(filename)?;
    let size = Size::new(image.width, image.height);
    Ok((image.buffer.as_bytes().to_vec(), size))
}