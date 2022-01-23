use std::collections::HashMap;

use ansi_term::Colour::RGB;
use image::{
    GenericImageView, open, Rgba,
    imageops::{FilterType, resize},
};

fn main() {
    // Image
    let img = open("/usr/share/pixmaps/neon.png").unwrap();

    // Tamanho original
    let (width, height) = img.dimensions();
    println!("{}, {}", width, height);

    // Redimencionar
    let (width, height) = (40, 20);
    let img = resize(&img, width, height, FilterType::Triangle);

    // Novo tamanho
    let (width, height) = img.dimensions();
    println!("{}, {}", width, height);

    // Mapa com vetores de tuplas com rgb. Ex:
    // {
    //      1: [Rgba, Rgba, Rgba],
    //      2: [Rgba, Rgba, Rgba],
    // }
    // Cada Vetor é uma linha da imagem.
    // Cada Rgba, é uma informação de cor (um pixel).
    let mut map: HashMap<u32, Vec<Rgba<u8>>> = HashMap::new();
    let mut count = 1;
    let mut line = 1;

    // Iterar pixels
    for &pixel in img.pixels() {
        // Se linha existe
        if let Some(_) = map.get(&line) {
            // Atualiza o valor que vai na key da hash
            let mut v = map.get(&line).unwrap().to_vec();
            v.push(pixel);

            // Atualiza a hash
            map.insert(line, v);

        // Se não, insere uma nova key/linha e valor
        } else {
            map.insert(line, vec![pixel]);
        }
        
        if count == width {
            count = 1;
            line += 1;
        } else {
            count += 1;
        }

    }

    // Mapa de retorno
    let mut pixels_map: HashMap<u32, Vec<(String, (u8, u8, u8))>> = HashMap::new();

    for line_num in 1..(width + 1) {
        let mut pixels_line: Vec<(String, (u8, u8, u8))> = Vec::new();

        if let Some(image_line) = map.get(&line_num) {
            for pixel_info in image_line {  // pixel_info = Rgba([1, 2, 3])
                let rgba = pixel_info;
                let [r, g, b, _a] = rgba.0;

                // https://github.com/EbonJaeger/asciifyer/blob/main/src/lib.rs
                let brightness = (
                    (0.2126 * r as f64) + (0.7152 * g as f64) + (0.0722 * b as f64)) as f64;

                let ascii_chars = [" ", " ", ":", "i", "/", "n", "k", "m", "0", "@", "#"];
                let ascii_chars_index =(
                    (brightness / 255.0) * (ascii_chars.len() - 1) as f64).round() as usize;

                // print!("{}", RGB(r, g, b).paint(ascii_chars[ascii_chars_index]));
                pixels_line.push((String::from(ascii_chars[ascii_chars_index]), (r, g, b)));
            }
            // println!();
            pixels_map.insert(line_num, pixels_line);
        }
    }

    // print
    for line_num in 1..(height + 1) {
        if let Some(image_line) = pixels_map.get(&line_num) {
            for pixel_info in image_line {
                let (c, rgb) = pixel_info;
                print!("{}", RGB(rgb.0, rgb.1, rgb.2).paint(c));
            }
        }
        println!();
    }

    // save
    // new_img.save("test.png").unwrap();
    //
    // ansi do rgb
    // println!("{}", RGB(70, 130, 180).paint("Steel blue"));
    //
    // let img = image::imageops::colorops::brighten(&img, -15);
    // let img = image::imageops::contrast(&img, 20.0);
    // let grayscale = image::imageops::colorops::grayscale(&img, 20.0);
}
