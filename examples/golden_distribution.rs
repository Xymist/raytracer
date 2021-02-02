use raytracer::{canvas::Canvas, colour::Colour, TAU};
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {
    let width = 1000;
    let height = 1000;
    let lanes = 4;
    let points = 5000;
    let exponent = 0.5;
    let turn_fraction = 0.618;
    let mut canvas = Canvas::white(width, height);
    let mut child = Command::new("ffmpeg")
        // Overwrite file if it already exists
        .arg("-y")
        // Interpret the information from stdin as "raw video" ...
        .arg("-f")
        .arg("rawvideo")
        // ... where every four bytes are [r, g, b, a] format
        .arg("-pix_fmt")
        .arg("rgba")
        // The size of the video is 3840x2160
        .arg("-s")
        .arg(&format!("{}x{}", width, height))
        // 60 frames per second
        .arg("-r")
        .arg("120")
        // Don't expect any audio in the stream
        .arg("-an")
        // Get the data from stdin
        .arg("-i")
        .arg("-")
        // encode to h264
        .arg("-c:v")
        .arg("libx264")
        // variable video bitrate
        .arg("-crf")
        .arg("0")
        // Output file
        .arg("golden.mp4")
        // stdin, stderr, and stdout are piped
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        // Run the child command
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    let mut buff: Vec<u8> = Vec::with_capacity(width * height * lanes * 10);

    for i in 0..points {
        let fi = i as f64;
        let base = fi / ((points - 1) as f64);
        let dst = base.powf(exponent);
        let angle = TAU * turn_fraction * fi;
        let mid_width = (width as f64 / 2.0).floor();
        let mid_height = (height as f64 / 2.0).floor();

        let x = ((dst * angle.cos() * mid_width) + mid_width).floor() as isize;
        let y = ((dst * angle.sin() * mid_height) + mid_height).floor() as isize;

        let colour = Colour::hsv_to_rgb(angle, 1.0 - base, 1.0);
        let alpha = 255;

        canvas.write_blob(x, y, colour);

        buff.extend(&canvas.u8_with_alpha(alpha));
        if i % 10 == 0 {
            stdin.write_all(&buff).unwrap();
            buff.clear();
        }
    }

    child.wait().unwrap();
}
