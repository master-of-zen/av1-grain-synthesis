use clap::{AppSettings, Arg, Parser};
use rand::distributions::{Distribution, Normal};
use std::path::PathBuf;
use v_frame::plane::Plane;

#[derive(Parser, Debug)]
#[clap(name = "av1-grain-synth", setting = AppSettings::DeriveDisplayOrder)]
struct Args {
    #[clap(short, long, default_value_t = 0.5)]
    mean: f64,

    #[clap(short, long, default_value_t = 0.3)]
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
    image::GrayImage::from_vec(plane.cfg.width as u32, plane.cfg.height as u32, buf)
        .unwrap()
        .save(PathBuf::from("image").with_extension("png"))
        .unwrap();
}

fn get_block_mean_low_bd(block: Plane<u8>) -> f64 {
    unimplemented!()
}

fn get_block_mean_high_bd(block: Plane<u16>) -> f64 {
    unimplemented!()
}

/// Should be run on residual of denoised and original source
fn get_noise_variance() -> f64 {
    unimplemented!()
}

fn get_block_variance() -> f64 {
    unimplemented!()
}
