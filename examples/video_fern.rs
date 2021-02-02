use rand::{rngs::ThreadRng, Rng};
use raytracer::canvas::Canvas;
use raytracer::colour::Colour;
use raytracer::descartes::Point3D;
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {
    let width = 640;
    let height = 480;
    let lanes = 4; // RGBA
    let iterations = 10000;
    let mut rng = rand::thread_rng();
    let mut canvas = Canvas::new(width, height);
    let green = Colour::new(0.0, 1.0, 0.0);
    let mut pt: Point3D<f64> = Point3D::new(0.0, 0.0, 0.0);

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
        .arg("test.mp4")
        // stdin, stderr, and stdout are piped
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        // Run the child command
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    let mut buff: Vec<u8> = Vec::with_capacity(width * height * lanes * 10);

    for i in 0..iterations {
        canvas.write_pixel(
            ((pt.x() * 100.0).trunc() + 320.0) as usize,
            (height as f64 - ((pt.y() * 40.0).trunc() + 10.0)) as usize,
            green,
        );
        buff.extend(&canvas.u8_with_alpha(255));
        if i % 10 == 0 {
            stdin.write_all(&buff).unwrap();
            buff.clear();
        }
        pt = next_point(&mut rng, pt);
    }

    stdin.write_all(&buff).unwrap();
    buff.clear();

    child.wait().unwrap();
}

fn next_point(rng: &mut ThreadRng, point: Point3D<f64>) -> Point3D<f64> {
    let rd = rng.gen_range(0, 100);
    match rd {
        0 => Point3D::new(0.0, 0.16 * point.y(), 0.0),
        1..=85 => Point3D::new(
            (0.85 * point.x()) + (0.04 * point.y()),
            (-0.04 * point.x()) + (0.85 * point.y()) + 1.6,
            0.0,
        ),
        86..=92 => Point3D::new(
            (0.2 * point.x()) - (0.26 * point.y()),
            (0.23 * point.x()) + (0.22 * point.y()) + 1.6,
            0.0,
        ),
        93..=99 => Point3D::new(
            (-0.15 * point.x()) + (0.28 * point.y()),
            (0.26 * point.x()) + (0.24 * point.y()) + 0.44,
            0.0,
        ),
        _ => unreachable!(),
    }
}
