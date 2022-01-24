use ansi_term::Colour::RGB;

use ansi_color_image;

fn main() {
    let width = 40;
    let height = 20;

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
    // for line_num in 1..(height + 1) {
    for image_line in pixels_map {
        for pixel_info in image_line {
            let (c, rgb) = pixel_info;
            print!("{}", RGB(rgb.0, rgb.1, rgb.2).paint(c));
        }
        println!();
    }
    
    
    
}
