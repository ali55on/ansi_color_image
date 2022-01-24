//! ansi_color_image
//! 
//! Lib to create figures with text characters and ANSI colors from an image file.
//! 
//! [github](https://github.com/w-a-gomes/ansi_color_image 'github')
//! 
//! ```
//! use ansi_color_image as aci;
//! 
//! fn main() {
//!     let url = "/usr/share/pixmaps/neon.png";
//!     let mut img = aci::ImageColorMap::new(url, Some(20), Some(40), Some(20.0), Some(-15));
//!     //                                   image height    width     contrast    brightness
//! 
//!     for pixel_line in img.build_pixel_map() {  // pixel_line = [pixel, pixel, pixel]
//!         for pixel in pixel_line {  //  pixel = ("*", (255, 255, 255), "\x1b[38;2;0;0;0m")
//!             let (txt, _convenient_rgb, ansi_code) = pixel;
//!             print!("{}{}", ansi_code, txt);  // Print without newline
//!         }
//!         img.reset_terminal_color();  // Prevent colored cursor when finished
//!         println!();  // New line
//!     }
//! }
//! ```

use image::{
    GenericImageView, open,  // Rgba, DynamicImage
    imageops::{FilterType, resize},
};

#[derive(Debug)]
pub struct ImageColorMap {
    url_image: String,
    height: Option<u32>,
    width: Option<u32>,
    contrast: Option<f32>,
    brightness: Option<i32>
}

impl ImageColorMap {
    pub fn new(
        url_image: &str, 
        height: Option<u32>,
        width: Option<u32>,
        contrast: Option<f32>,
        brightness: Option<i32>
    ) -> ImageColorMap {
        let url_image = String::from(url_image);

        ImageColorMap {
            url_image,
            height,
            width,
            contrast,
            brightness,
        }
    }

    pub fn build_pixel_map(&mut self) -> Vec<Vec<(String, (u8, u8, u8), String)>> {
        // Image
        let img = open(&self.url_image).unwrap();

        // Original size
        let (original_width, original_height) = img.dimensions();

        // Height
        if let Some(w) = self.height {
            self.height = Some(w);
        } else {
            self.height = Some(original_height);
        }

        // Width
        if let Some(w) = self.width {
            self.width = Some(w);
        } else {
            self.width = Some(original_width);
        }

        // Resize
        let mut new_img = resize(
            &img, self.width.unwrap(), self.height.unwrap(), FilterType::Triangle);
        
        // Contrast
        if let Some(c) = self.contrast {
            self.contrast = Some(c);
        } else {
            self.contrast = None;
        }

        if let Some(c) = self.contrast {
            new_img = image::imageops::contrast(&new_img, c);
        }

        // Brightness
        if let Some(b) = self.brightness {
            self.brightness = Some(b);
        } else {
            self.brightness = None;
        }

        if let Some(b) = self.brightness {
            new_img = image::imageops::colorops::brighten(&new_img, b);
        }

        // Return Map
        // [
        //      [("*", (1, 2, 3)), ("*", (1, 2, 3)), ("*", (1, 2, 3))],
        //      [("*", (1, 2, 3)), ("*", (1, 2, 3)), ("*", (1, 2, 3))],
        // ]
        let mut pixels_map: Vec<Vec<(String, (u8, u8, u8), String)>> = Vec::new();
        let mut count = 1;
        let mut line = 0;

        for &pixel in new_img.pixels() {
            let rgba = pixel;  // Rgba([3, 15, 19, 14])
            let [r, g, b, _a] = rgba.0;
            
            // https://github.com/EbonJaeger/asciifyer/blob/main/src/lib.rs
            let brightness_ = (
                (0.2126 * r as f64) + (0.7152 * g as f64) + (0.0722 * b as f64)) as f64;

            let ascii_chars = [" ", " ", ":", "i", "/", "n", "k", "m", "0", "@", "#"];
            // let ascii_chars = ["#", "@", "0", "m", "k", "n", "/", "i", ":", " ", " "];
            let ascii_chars_index =(
                (brightness_ / 255.0) * (ascii_chars.len() - 1) as f64).round() as usize;

            // Update map
            if count == 1 {
                pixels_map.push(
                    vec![(
                        String::from(ascii_chars[ascii_chars_index]),
                        (r, g, b),
                        self.rgb_to_ansi(r, g, b)
                    )]
                );
            } else {
                pixels_map[line].push((
                    String::from(ascii_chars[ascii_chars_index]),
                    (r, g, b),
                    self.rgb_to_ansi(r, g, b)
                ));
            }

            // Update loop config
            if count == self.width.unwrap() as usize {
                line += 1;
                count = 1;
            } else {
                count += 1;
            }
        }

        // Return
        pixels_map
    }

    pub fn reset_terminal_color(&self){
        print!("\x1B[0m")
    }

    fn rgb_to_ansi(&self, r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }
}

// fn rgb_to_ansi(r: u8, g: u8, b: u8, text: &String) -> String {
//     format!("\x1b[38;2;{};{};{}m{}\x1B[0m", r, g, b, text)
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut icm = ImageColorMap::new(
            "/usr/share/pixmaps/neon.png",
            Some(20),
            Some(40),
            None,
            None,
        );
        let _bpm = icm.build_pixel_map();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
