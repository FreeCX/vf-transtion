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
    // значения для генерации переходов
    let mut transitions: Vec<f32> = vec![0.0, 1.0, 1.0, 0.0];
    // загружаемые изображения
    let mut images: Vec<String> = Vec::new();
    // имя выходного файла
    let mut output = String::from("render.mp4");
    // используемый метод
    let mut method = String::from("alpha");
    // время ролика в секундах
    let mut transition_time = 10;
    // количество кадров в секунду у выходного видео
    let mut fps = 25;

    // парсинг аргрументов
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

    // проверки на валидность количества значений
    if transitions.len() % 2 != 0 {
        println!("[error]: transitions parameters must be a multiple by two");
        exit(-1);
    }

    // проверки на валидность количества входных фоток
    if images.len() != 2 {
        println!("[error]: input images count != 2");
        exit(-1);
    }

    // функция генерирующая переход
    let func: Box<dyn ffmpeg::TransitionFunc> = match method.as_ref() {
        "alpha" => Box::new(ts::AlphaBlend),
        "vertical" => Box::new(ts::Vertical),
        method => {
            println!("[error]: unknown method {}", method);
            exit(-1);
        }
    };

    // шаг для создания перехода на заданное время и fps
    let step = (transitions.len() / 2) as f32 / (fps * transition_time) as f32;

    // тут должно быть всё понятно
    let i = Render::default()
        .first_image(&images[0])
        .second_image(&images[1])
        .transition_series(transitions, step)
        .set_output_file(&output);

    // рендерим
    i.render(func, fps);
}
