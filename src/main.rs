use ansi_term::Colour::RGB;

use ansi_color_image as a_c_img;

fn main() {
    // Map settings
    let url_img = String::from("/usr/share/pixmaps/neon.png");
    let height = Some(20);
    let width = Some(40);  // It's twice the height, so it doesn't get stretched
    let contrast = Some(20.0);
    let brightness = Some(-15);

    // Create instance
    let mut aci = a_c_img::ImageColorMap::new(url_img, height, width, contrast, brightness);

    // Print
    for image_line in aci.build_pixel_map() {
        for pixel_info in image_line {
            let (c, rgb) = pixel_info;
            print!("{}", RGB(rgb.0, rgb.1, rgb.2).paint(c));
        }
        println!();
    }
}
