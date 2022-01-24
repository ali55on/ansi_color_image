## ansi_color_image

Lib to create figures with text characters and ANSI colors from an image file.

https://github.com/w-a-gomes/ansi_color_image

Example:

```rust
use ansi_color_image as aci;

fn main() {
    let url = "/usr/share/pixmaps/neon.png";
    let mut img = aci::ImageColorMap::new(url, Some(20), Some(40), Some(20.0), Some(-15));
    //                                   image height    width     contrast    brightness

    for pixel_line in img.build_pixel_map() { // pixel_line = [pixel, pixel, pixel]
        for pixel in pixel_line {            //  pixel = ("*", (255, 255, 255), "\x1b[38;2;0;0;0m")
            let (txt, _convenient_rgb, ansi_code) = pixel;
            print!("{}{}", ansi_code, txt);  // Print without newline
        }
        img.reset_terminal_color();  // Prevent colored cursor when finished
        println!();  // New line
    }
}
```
![Image](data/screenshot_01.png "screenshot")

Each vector represents a line, making it easier to place parallel information.

```rust
let poem = "
The Raven

...
 Then this ebony bird beguiling my sad fancy into smiling,
By the grave and stern decorum of the countenance it wore,
“Though thy crest be shorn and shaven, thou,” I said, “art sure no craven,
Ghastly grim and ancient Raven wandering from the Nightly shore—
Tell me what thy lordly name is on the Night’s Plutonian shore!”
            Quoth the Raven “Nevermore.”
...
But the Raven still beguiling all my fancy into smiling,
Straight I wheeled a cushioned seat in front of bird, and bust and door;
    Then, upon the velvet sinking, I betook myself to linking
    Fancy unto fancy, thinking what this ominous bird of yore—
What this grim, ungainly, ghastly, gaunt, and ominous bird of yore
            Meant in croaking “Nevermore.”
...

(By Edgar Allan Poe)";

    let mut img = aci::ImageColorMap::new("poe.png", Some(20), Some(40), None, None);
    for (pixel_line, poem_line) in img.build_pixel_map().iter().zip(poem.split("\n")) {

        for pixel in pixel_line {
            let (txt, _, ansi_code) = pixel;
            print!("{}{}", ansi_code, txt);
        }
        img.reset_terminal_color();
        println!(" {}", poem_line);
    }
```
![Image](data/screenshot_02.png "screenshot")
