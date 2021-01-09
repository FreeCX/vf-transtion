use crate::ffmpeg::{ TransitionFunc, Size };

pub struct AlphaBlend;
pub struct Vertical;

impl TransitionFunc for AlphaBlend {
    fn calc(&self, alpha: f32, img1: &Vec<u8>, img2: &Vec<u8>, _size: &Size) -> Vec<u8> {
        // create output vector with image size
        let mut r = vec![0; img1.len()];
        // iterate by images data
        for (d, (a, b)) in r.iter_mut().zip(img1.iter().zip(img2.iter())) {
            // mixing colors
            *d = (*b as f32 * alpha + *a as f32 * (1.0 - alpha)).round() as u8;
        }
        r
    }
}

impl TransitionFunc for Vertical {
    fn calc(&self, percent: f32, img1: &Vec<u8>, img2: &Vec<u8>, size: &Size) -> Vec<u8> {
        let mut r = vec![0; img1.len()];
        // current x position in the picture, 3 -- color channels
        let position = (3.0 * percent * size.width as f32).abs().round() as usize;
        // ... you know ... iterator
        for (i, (d, (a, b))) in r.iter_mut().zip(img1.iter().zip(img2.iter())).enumerate() {
            // x value of current pixel
            let current_x = i % (size.width * 3);
            *d = match current_x > position {
                // right side -- img1
                true => *a,
                // left side -- img2
                false => *b, 
            };
        }
        r
    }
}