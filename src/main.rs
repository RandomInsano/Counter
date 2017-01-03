// Attempt to make a silly Lycos-style hit counter using `image` and `Rocket`
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;

extern crate rocket;
extern crate image;
extern crate memstream;
extern crate uuid;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use std::sync::RwLock;

use image::{
    GenericImage,
    ImageBuffer,
    DynamicImage,
};

use rocket::response::{
    Stream
};
use rocket::response::content::Content;
use rocket::http::ContentType;
use memstream::MemStream;

const DIGIT_HEIGHT: u32 = 32;
const DIGIT_WIDTH: u32 = 32;
const IMAGE_DIGITS : u32 = 10;
const IMAGE_HEIGHT: u32 = DIGIT_HEIGHT;
const IMAGE_WIDTH: u32 = DIGIT_WIDTH * IMAGE_DIGITS;

const PATH_SPRITES: &'static str = "./resources/digits.png";

lazy_static! {
    static ref COUNTS: CounterDict = CounterDict::new();
    static ref DIGITS: Vec<DynamicImage> = {
        let mut digits = Vec::new();
        let sprites = image::open(&Path::new(PATH_SPRITES)).unwrap();

        // Create a lookup table for digits. Ideally I could just blit from one image to the other,
        // but there are no good calls to do that
        for i in 0 .. 10 {
            let digit = sprites.clone().crop(0, DIGIT_HEIGHT * i, DIGIT_WIDTH, DIGIT_HEIGHT);
            digits.push(digit);
        }

        digits
    };
}

fn main() {
    rocket::ignite().mount("/v1.0/", routes![serve_imge]).launch();
}

#[get("/counter/<id>")]
fn serve_imge(id: String) -> Result<Content<Stream<MemStream>>, &'static str> {
    if id.len() > 64 {
        return Err("Id was too long")
    }

    let mut buffer = MemStream::new();
    let number = COUNTS.get(&id);
    gen_image(number).save(&mut buffer, image::PNG).unwrap();

    Ok(Content(ContentType::PNG, Stream::from(buffer)))
}

fn gen_image(count: u32) -> DynamicImage {
    let mut img = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let x: u32 = 10;
    for i in 0 .. IMAGE_DIGITS {
        let value = (count / x.pow(IMAGE_DIGITS - 1 - i)) % x;
        if let Some(digit) = DIGITS.get(value as usize) {
            img.copy_from(digit, i * DIGIT_WIDTH, 0);
        }
    }

    image::ImageRgba8(img)
}

struct CounterDict {
    counts: RwLock<HashMap<String, Mutex<u32>>>,
}

impl CounterDict {
    fn new() -> CounterDict {
        CounterDict {
            counts: RwLock::new(HashMap::new())
        }
    }

    fn get(&self, key: &str) -> u32 {
        if let Some(x) = self.counts.read().unwrap().get(key) {
            let mut count = x.lock().unwrap();
            *count += 1;

            return *count;
        }

        self.counts.write().unwrap().insert(String::from(key), Mutex::new(1));

        1
    }
}
