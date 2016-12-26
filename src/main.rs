// Attempt to make a silly Lycos-style hit counter using `image` and `Rocket`

extern crate image;

use std::path::Path;
use std::fs::File;

use image::{
    GenericImage,
    ImageBuffer,
    DynamicImage
};

const DIGIT_HEIGHT: u32 = 32;
const DIGIT_WIDTH: u32 = 32;
const IMAGE_DIGITS : u32 = 10;
const IMAGE_HEIGHT: u32 = DIGIT_HEIGHT;
const IMAGE_WIDTH: u32 = DIGIT_WIDTH * IMAGE_DIGITS;

const PATH_SPRITES: &'static str = "./resources/digits.png";
const PATH_OUT: &'static str = "out.png";

const NUMBER: u32 = 3423428978;

fn main() {
    let image_count = gen_image(NUMBER);

    // Save out the buffer
    let ref mut out_file = File::create(&Path::new(PATH_OUT)).unwrap();
    image_count.save(out_file, image::PNG).unwrap()
}

fn gen_image(count: u32) -> DynamicImage {
    let mut img = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let sprites = image::open(&Path::new(PATH_SPRITES)).unwrap();
    let mut digits = Vec::new();

    // Create a lookup table for digits. Ideally I could just blit from one image to the other,
    // but there are no good calls to do that
    for i in 0 .. 10 {
        let digit = sprites.clone().crop(0, DIGIT_HEIGHT * i, DIGIT_WIDTH, DIGIT_HEIGHT);
        digits.push(digit);
    }

    let x: u32 = 10;
    for i in 0 .. IMAGE_DIGITS {
        let value = (count / x.pow(IMAGE_DIGITS - 1 - i)) % x;
        if let Some(digit) = digits.get(value as usize) {
            img.copy_from(digit, i * DIGIT_WIDTH, 0);
        }
    }

    image::ImageRgba8(img)
}
