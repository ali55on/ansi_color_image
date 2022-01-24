## ansi_color_image

Lib to create figures with text characters and ANSI colors from an image file.

https://github.com/w-a-gomes/ansi_color_image

Example:

```rust
use ansi_term::Colour::RGB;
use ansi_color_image as aci;

fn main() {
    let url = String::from("/usr/share/pixmaps/neon.png");

    // Create instance:                  image height    width     contrast    brightness
    let mut img = aci::ImageColorMap::new(url, Some(20), Some(40), Some(20.0), Some(-15));

    // Print
    for pixel_line  in img.build_pixel_map() { // pixel_line  =  [pixel, pixel, pixel]
        for pixel in pixel_line {             //  pixel       =  ("*", (255, 255, 255))   
            let (char_, rgb) = pixel;
            print!("{}", RGB(rgb.0, rgb.1, rgb.2).paint(char_));  // Print without newline 
        }
        println!();  // New line
    }
}
```
![Image](data/screenshot_o1.png "screenshot")
