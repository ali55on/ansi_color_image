// use std::collections::HashMap;

use image::{
    GenericImageView, open, Rgba,  // DynamicImage
    imageops::{FilterType, resize},
};

#[derive(Debug)]
pub struct ImageColorMap {
    pixels_map: Vec<Vec<(String, (u8, u8, u8))>>,
}

impl ImageColorMap {
    pub fn new(
        url_image: String,
        mut height: Option<u32>,
        mut width: Option<u32>,
        mut contrast: Option<f32>,
        mut brightness: Option<i32>
    ) -> ImageColorMap {
        // Image
        let new_image = open(&url_image).unwrap();

        // Original size
        let (original_width, original_height) = new_image.dimensions();

        
        // Height
        if let Some(h) = height {
            height = Some(h);
        } else {
            height = Some(original_height);
        }

        // Width
        if let Some(w) = width {
            width = Some(w);
        } else {
            width = Some(original_width);
        }

        // Resize
        // if width != Some(original_width) && height != Some(original_height) {
        let mut new_image = resize(&new_image, width.unwrap(), height.unwrap(), FilterType::Triangle);
        
        // Contrast
        if let Some(c) = contrast {
            contrast = Some(c);
        } else {
            contrast = None;
        }

        if let Some(c) = contrast {
            new_image = image::imageops::contrast(&new_image, c);
        }

        // Brightness
        if let Some(b) = brightness {
            brightness = Some(b);
        } else {
            brightness = None;
        }

        if let Some(b) = brightness {
            new_image = image::imageops::colorops::brighten(&new_image, b);
        }

        // Criar um 'Mapa' com linhas/vetores de rgb. Ex:
        // [
        //      [Rgba, Rgba, Rgba],
        //      [Rgba, Rgba, Rgba],
        // ]
        // Cada Vetor é uma linha da imagem.
        // Cada Rgba, é uma informação de cor (um pixel).
        // let mut map: HashMap<u32, Vec<Rgba<u8>>> = HashMap::new();
        let mut map: Vec<Vec<Rgba<u8>>> = Vec::new();
        let mut count = 1;
        let mut line = 0;

        // Iterar pixels
        for &pixel in new_image.pixels() {
            if count == 1 {
                map.push(vec![pixel]);
            } else {
                map[line].push(pixel);    
            }

            if count == width.unwrap() as usize {
                count = 1;
                line += 1;
            } else {
                count += 1;
            }
        }
        
        // Mapa de retorno
        let mut pixels_map: Vec<Vec<(String, (u8, u8, u8))>> = Vec::new();
        let mut count = 1;
        let mut line = 0;

        for map_line in map {
            for pixel_info in map_line {
                let rgba = pixel_info;  // Rgba([3, 15, 19, 14])
                let [r, g, b, _a] = rgba.0;
                
                // https://github.com/EbonJaeger/asciifyer/blob/main/src/lib.rs
                let brightness = (
                    (0.2126 * r as f64) + (0.7152 * g as f64) + (0.0722 * b as f64)) as f64;

                let ascii_chars = [" ", " ", ":", "i", "/", "n", "k", "m", "0", "@", "#"];
                let ascii_chars_index =(
                    (brightness / 255.0) * (ascii_chars.len() - 1) as f64).round() as usize;

                if count == 1 {
                    pixels_map.push(vec![(String::from(ascii_chars[ascii_chars_index]), (r, g, b))]);
                } else {
                    pixels_map[line].push((String::from(ascii_chars[ascii_chars_index]), (r, g, b)));
                }

                if count == width.unwrap() as usize {
                    line += 1;
                    count = 1;
                } else {
                    count += 1;
                }
            }
        }

        // Return
        ImageColorMap {
            pixels_map,
        }
    }

    pub fn pixels_map(&self) -> &Vec<Vec<(String, (u8, u8, u8))>> {
        &self.pixels_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let _i = ImageColorMap::new(
            String::from("/usr/share/pixmaps/neon.png"),
            Some(20),
            Some(40),
            None,
            None,
        );
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
