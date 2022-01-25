use ansi_color_image as aci;
use std::process::Command;

fn main() {
    println!("-------\nNeon logo");
    neon_logo_example();

    println!("\n-------\nPoe poem:\n  Cut the poem line to fit the terminal.");
    poe_poem_example();

    // Using on README
    // println!("\n-------\nPoe poem:\n  It doesn't cut the poem line to fit the terminal.");
    // simple_poe_poem_example();
}

#[allow(dead_code)]
fn neon_logo_example() {
    let url = "examples/data/neon.png";
    let mut img = aci::ImageColorMap::new(url, Some(20), Some(40), Some(20.0), Some(-15), false);
    //                                   image height    width     contrast    brightness bg_color

    for pixel_line in img.build_pixel_map() { // pixel_line = [pixel, pixel, pixel]
        for pixel in pixel_line {            //  pixel = ("\x1b[38;2;0;0;0m", "*")
            let (ansi_code, txt) = pixel;
            print!("{}{}", ansi_code, txt);  // Print without newline
        }
        img.reset_terminal_color();  // Prevent colored cursor when finished
        println!();  // New line
    }
}

#[allow(dead_code)]
fn simple_poe_poem_example() {
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
    let url = "examples/data/poe.png";
    let mut img = aci::ImageColorMap::new(url, Some(20), Some(40), None, None, true);
    //                                   image height    width                 bg_color
    for (pixel_line, poem_line) in img.build_pixel_map().iter().zip(poem.split("\n")) {
        // IMAGE:
        for pixel in pixel_line {
            let (ansi_code, _txt) = pixel;
            print!("{} ", ansi_code);  // shows only colors without text character
        }
        // POEM:
        img.reset_terminal_color();
        println!(" {}", poem_line);
    }
}

#[allow(dead_code)]
fn poe_poem_example() {
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
    
    let image_width = 40;
    let mut img = aci::ImageColorMap::new(
        "examples/data/poe.png", Some(20), Some(image_width), None, None, true);
    for (pixel_line, poem_line) in img.build_pixel_map().iter().zip(poem.split("\n")) {
        // IMAGE:
        for pixel in pixel_line {
            let (ansi_code, _txt) = pixel;
            print!("{} ", ansi_code);
        }
        // POEM:
        img.reset_terminal_color();

        // Terminal size line
        let tput_command = Command::new("tput").arg("cols").output().expect("tput error");
        let term_size: u32 = String::from_utf8_lossy(&tput_command.stdout).trim().parse().unwrap();

        // FIX: not implement Slice -> line[..term_size]
        let (mut line, mut count) = (String::new(), 1);
        for c in poem_line.chars() {
            line.push(c);  //                      1: the space on println!
            if count == (term_size - image_width - 1) {break;} else {count += 1;}
        }
        println!(" {}", line);
    }
}