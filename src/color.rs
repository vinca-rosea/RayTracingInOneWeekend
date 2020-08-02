use super::*;
use std::io::Write;

pub fn write_color(
    out: &mut std::io::BufWriter<std::io::StdoutLock>,
    pixel_color: Color,
    samples_per_pixel: i64,
) -> std::io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Replace NaN components with zero. See explanation in Ray Tracing: The Rest of Your Life.
    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    // Write the translated [0,255] value of each color component.
    writeln!(
        out,
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as i64,
        (256.0 * clamp(g, 0.0, 0.999)) as i64,
        (256.0 * clamp(b, 0.0, 0.999)) as i64,
    )?;
    Ok(())
}

pub fn get_color(
    pixel_color: Color,
    samples_per_pixel: i64,
) -> (i64,i64,i64,) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Replace NaN components with zero. See explanation in Ray Tracing: The Rest of Your Life.
    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    // Write the translated [0,255] value of each color component.
    (
        (256.0 * clamp(r, 0.0, 0.999)) as i64,
        (256.0 * clamp(g, 0.0, 0.999)) as i64,
        (256.0 * clamp(b, 0.0, 0.999)) as i64,
    )
}