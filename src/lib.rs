//! ansi_color_image
//! 
//! Lib to create figures with text characters and ANSI colors from an image file.
//! 
//! [github](https://github.com/w-a-gomes/ansi_color_image 'github')
//! 
//! ```
//!use ansi_color_image as aci;
//!
//!fn main() {
//!    let mut img = aci::ImageColorMap::new("examples/data/neon.png");
//!    img.dimensions(40, 20);                   // Width and height.
//!    img.filter(20.0, -15);                    // Contrast and brightness.
//!                                              //
//!    for pixel_line in img.build_pixel_map() { // pixel_line = [pixel, pixel, pixel]
//!        for pixel in pixel_line {             // pixel = ("\x1b[38;2;0;0;0m", "*")
//!            let (ansi_code, txt) = pixel;     //
//!            print!("{}{}", ansi_code, txt);   // Print without newline.
//!        }                                     //
//!        img.reset_terminal_color();           // Prevent colored cursor when finished.
//!        println!();                           // New line.
//!    }
//!}
//! ```

use image::{
    open,  // GenericImageView, Rgba, DynamicImage
    imageops::{FilterType, resize},
};

#[derive(Debug)]
pub struct ImageColorMap {
    url_image: String,
    height: u32,
    width: u32,
    contrast: f32,
    brightness: i32,
    background_color: bool,
}

impl ImageColorMap {
    pub fn new(
        url_image: &str,
    ) -> ImageColorMap {
        let url_image = String::from(url_image);

        ImageColorMap {
            url_image,
            height: 20,
            width: 40,
            contrast: 0.0,
            brightness: 0,
            background_color: false,
        }
    }

    pub fn dimensions(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn filter(&mut self, contrast: f32, brightness: i32) {
        self.contrast = contrast;
        self.brightness = brightness;
    }

    pub fn background_color(&mut self, background_color: bool) {
        self.background_color = background_color;
    }

    pub fn build_pixel_map(&mut self) -> Vec<Vec<(String, String)>> {
        // Image
        let img = open(&self.url_image).unwrap();

        // Resize
        let mut new_img = resize(
            &img, self.width, self.height, FilterType::Triangle);
        
        // Contrast
        if self.contrast != 0.0 {
            new_img = image::imageops::contrast(&new_img, self.contrast);
        }

        // Brightness
        if self.brightness != 0 {
            new_img = image::imageops::colorops::brighten(&new_img, self.brightness);
        }

        // Return Map
        // [
        //      [("\x1b[38;2;0;0;0m", "*"), ("\x1b[38;2;0;0;0m", "#")],
        //      [("\x1b[38;2;0;0;0m", "@"), ("\x1b[38;2;0;0;0m", ";")],
        // ]
        let mut pixels_map: Vec<Vec<(String, String)>> = Vec::new();
        let mut count = 1;
        let mut line = 0;

        for &pixel in new_img.pixels() {
            let rgba = pixel;  // Rgba([3, 15, 19, 14])
            let [r, g, b, _a] = rgba.0;

            let map_ascii_chars = [
                " ", " ", ":", "i", "/", "n", "k", "m", "0", "@", "#"];
            
            let pixel_brightness = (  // github.com/EbonJaeger/asciifyer/blob/main/src/lib.rs
                (0.2126 * r as f64) + (0.7152 * g as f64) + (0.0722 * b as f64)) as f64;

            let ascii_chars_index = (
                (pixel_brightness / 255.0) * (map_ascii_chars.len() - 1) as f64).round() as usize;

            // Update map
            if count == 1 {
                pixels_map.push(
                    vec![(
                        self.rgb_to_ansi(r, g, b, self.background_color),
                        String::from(map_ascii_chars[ascii_chars_index])
                    )]
                );
            } else {
                pixels_map[line].push((
                    self.rgb_to_ansi(r, g, b, self.background_color),
                    String::from(map_ascii_chars[ascii_chars_index])
                ));
            }

            // Update loop config
            if count == self.width as usize {
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

    fn rgb_to_ansi(&self, r: u8, g: u8, b: u8, bg: bool) -> String {
        let use_fg = 38;
        let use_bg = 48;

        let mut bg_or_fg = &use_fg;
        if bg {bg_or_fg = &use_bg;}

        format!("\x1b[{};2;{};{};{}m", bg_or_fg, r, g, b)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
