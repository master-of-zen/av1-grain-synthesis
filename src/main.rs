use clap::{AppSettings, Parser};
use rand::distributions::{Distribution, Normal};
use std::path::PathBuf;
use v_frame::pixel::Pixel;
use v_frame::plane::Plane;

static LAG: u8 = 4;

#[derive(Parser, Debug)]
#[clap(name = "av1-grain-synth", setting = AppSettings::DeriveDisplayOrder)]
struct Args {
    #[clap(short, long, default_value_t = 0.5)]
    mean: f64,

    #[clap(short, long, default_value_t = 0.1)]
    std_dev: f64,
}

fn main() {
    let args = Args::parse();

    let normal = Normal::new(args.mean, args.std_dev);

    let vals = (0..4096)
        .into_iter()
        .map(|_f| {
            let sample = normal.sample(&mut rand::thread_rng());
            let new = 255.0 * sample;
            new as u8
        })
        .collect::<Vec<u8>>();

    let plane = Plane::from_slice(&vals, 64);

    println!("{:#?}", plane);

    let buf: Vec<_> = plane.iter().map(|p| p as u8).collect();
    // image::GrayImage::from_vec(plane.cfg.width as u32, plane.cfg.height as u32, buf)
    //    .unwrap()
    //    .save(PathBuf::from("image").with_extension("png"))
    //    .unwrap();

    let avg = get_block_mean(&plane);
    let noise_mean = get_noise_variance(&plane);

    dbg!(avg);
    dbg!(noise_mean);
}

fn get_block_mean(block: &Plane<u8>) -> u64 {
    let mut sum = 0u64;
    let total_pixels: u64 = (block.cfg.width * block.cfg.height) as u64;

    block.data.iter().for_each(|x| sum += *x as u64);

    (sum / total_pixels) as u64
}

/// Should be run on residual of denoised and original source
fn get_noise_variance(block: &Plane<u8>) -> f64 {
    let mut sum = 0u64;

    block.data.iter().for_each(|x| sum += *x as u64 * *x as u64);

    let mean = get_block_mean(&block);

    return sum as f64 / (block.cfg.width * block.cfg.height) as f64 - (mean * mean) as f64;
}

fn get_block_variance() -> f64 {
    unimplemented!()
}
