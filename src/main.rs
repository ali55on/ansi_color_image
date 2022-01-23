use ansi_term::Colour::RGB;

use ansi_color_image;

fn main() {
    let width = 80;
    let height = 39;

    // Create Map
    let image_map = ansi_color_image::ImageColorMap::new(
        String::from("/usr/share/pixmaps/neon.png"),
        Some(height),
        Some(width),
        None,
        None,
    );

    // Get pixels
    let pixels_map = image_map.pixels_map();

    // Print
    for line_num in 1..(height + 1) {
        if let Some(image_line) = pixels_map.get(&line_num) {
            for pixel_info in image_line {
                let (c, rgb) = pixel_info;
                print!("{}", RGB(rgb.0, rgb.1, rgb.2).paint(c));
            }
        }
        println!();
    }
}
