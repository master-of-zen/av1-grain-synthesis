use clap::{AppSettings, Parser};
use rand::distributions::{Distribution, Normal};
use std::path::PathBuf;
use v_frame::plane::Plane;

#[derive(Parser, Debug)]
#[clap(name = "av1-grain-synth", setting = AppSettings::DeriveDisplayOrder)]
struct Args {
    mean: f64,
    std_dev: f64,
}

fn main() {
    //let parsed = argparse::parse_args();
    //println!("{:?}", parsed);

    let normal = Normal::new(0.5, 0.3);

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
