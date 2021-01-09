extern crate lodepng;
extern crate rgb;
extern crate argparse;

mod ffmpeg;
mod transition;

use std::process::exit;

use argparse::{ArgumentParser, Store, List, Print};

use transition as ts;
use ffmpeg::Render;


fn main() {
    // transition values
    let mut transitions: Vec<f32> = vec![0.0, 1.0, 1.0, 0.0];
    // loadable images
    let mut images: Vec<String> = Vec::new();
    // output file name
    let mut output = String::from("render.mp4");
    // using transtion effect
    let mut method = String::from("alpha");
    // video duration in secods
    let mut transition_time = 10;
    // output video fps
    let mut fps = 25;

    // lets parse application arguments
    {
        let mut ap = ArgumentParser::new();

        ap.set_description("Make special transition effect from two images");
        ap.refer(&mut transition_time)
          .add_option(&["-d", "--duration"], Store, "video duration in seconds");
        ap.refer(&mut fps)
          .add_option(&["-f", "--fps"], Store, "video fps");
        ap.refer(&mut transitions)
          .add_option(&["-t", "--transition"], List, "transition parameters");
        ap.refer(&mut images)
          .add_option(&["-i", "--images"], List, "input images");
        ap.refer(&mut output)
          .add_option(&["-o", "--output"], Store, "output file name");
        ap.refer(&mut method)
          .add_option(&["-m", "--method"], Store, "transition method (alpha|vertical)");
        ap.add_option(&["-v", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()), "show version");

        ap.parse_args_or_exit();
    }

    // validate values count
    if transitions.len() % 2 != 0 {
        println!("[error]: transitions parameters must be a multiple by two");
        exit(-1);
    }

    // validate images count
    if images.len() != 2 {
        println!("[error]: input images count != 2");
        exit(-1);
    }

    // choice transition function
    let func: &dyn ffmpeg::TransitionFunc = match method.as_ref() {
        "alpha" => &ts::AlphaBlend,
        "vertical" => &ts::Vertical,
        method => {
            println!("[error]: unknown method {}", method);
            exit(-1);
        }
    };

    // calculate transtition step
    let step = (transitions.len() >> 1) as f32 / (fps as f32 * transition_time as f32);

    // prepare render
    let i = Render::default()
        .first_image(&images[0])
        .second_image(&images[1])
        .transition_series(transitions, step)
        .set_output_file(&output);

    // and render
    i.render(func, fps);
}
