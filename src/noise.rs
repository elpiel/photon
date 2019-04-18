extern crate image;
extern crate rand;
use image::{GenericImage, DynamicImage, GenericImageView};
use rand::Rng;
use image::{Rgb, Rgba, Luma, Pixel, ImageBuffer};

pub type Image<P> = ImageBuffer<P, Vec<<P as Pixel>::Subpixel>>;

/// Add randomized noise to an image. 
/// This function adds a Gaussian Noise Sample to each pixel through incrementing each channel by a randomized offset.
/// This randomized offset is generated by creating a randomized thread pool.
/// 
/// # Arguments
/// * `name` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to increase the Red channel for all pixels by 10:
/// use photon::noise;
/// photon::noise::add_noise_rand(img);
/// ```
pub fn add_noise_rand(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut rng = rand::thread_rng();

    for x in 0..width {
        for y in 0..height {
            let offset = rng.gen_range(0, 150);
            let px = img.get_pixel(x, y).map(|ch| if ch <= 255 - offset { ch + offset } else { 255});
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Add pink-tinted noise to an image. 
/// 
/// # Arguments
/// * `name` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to add pink-tinted noise to an image:
/// use photon::noise;
/// photon::noise::pink_noise(img);
/// ```
pub fn pink_noise(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut rng = rand::thread_rng();

    for x in 0..width {
        for y in 0..height {
            let ran1: f64 = rng.gen(); // generates a float between 0 and 1
            let ran2: f64 = rng.gen();
            let ran3: f64 = rng.gen();

            let ran_color1: f64 = 0.6 + ran1 * 0.6;
            let ran_color2: f64 = 0.6 + ran2 * 0.1;
            let ran_color3: f64 = 0.6 + ran3 * 0.4;

            let mut px = img.get_pixel(x, y);
            px.data[0] = (px.data[0] as f64 * 0.99 * ran_color1) as u8;
            px.data[1] = (px.data[1] as f64 * 0.99 * ran_color2) as u8;
            px.data[2] = (px.data[2] as f64 * 0.99 * ran_color3) as u8;
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

// pub fn noise_gen<I: GenericImage>(img: I, Pixel: P)
// {
//     let (width, height) = img.dimensions();

//     // for x in 0 .. width {
//     //     for y in 0..height {
//     //         let px = img.get_pixel(x, y);
//     //         if px.data[0] < 253 {
//     //             px.data[0] += 1;
//     //         }
//     //         img.put_pixel(x, y, px);
//     //     }
//     // }

//     for pixel in img.pixels() {
//     // Do something with pixel.
//     println!("{:?}", pixel.data[0])
//     }

// }