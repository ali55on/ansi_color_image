use ansi_color_image as aci;
use std::process::Command;

fn main() {
    println!("-------\nNeon logo");
    neon_logo_example();

    println!("\n-------\nPoe poem:\n  Cut the poem line to fit the terminal.");
    poe_poem_example();

    println!("\n-------\nLinux logo");
    linux_logo_example();

    // Using on README
    // println!("\n-------\nPoe poem:\n  It doesn't cut the poem line to fit the terminal.");
    // simple_poe_poem_example();

}

fn linux_logo_example() {
    let mut img = aci::ImageColorMap::new("examples/data/linux.png");
    img.dimensions(50, 25); // Width and height
    img.filter(20.0, -15);  // Contrast and brightness

    for line in img.build_pixel_map() {
        println!("{}", line);
    }
}

#[allow(dead_code)]
fn neon_logo_example() {
    let mut img = aci::ImageColorMap::new("examples/data/neon.png");
    img.dimensions(40, 20); // Width and height
    img.filter(20.0, -15);  // Contrast and brightness

    for line in img.build_pixel_map() {
        println!("{}", line);
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
    let mut img = aci::ImageColorMap::new("examples/data/poe.png");
    img.show_background_color(true);
    img.hide_foreground_character(true);

    for (img_line, poem_line) in img.build_pixel_map().iter().zip(poem.split("\n")) {
        // IMAGE:
        print!("{}", img_line);  // Print without newline
        // POEM:
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

    let mut img = aci::ImageColorMap::new("examples/data/poe.png");
    img.show_background_color(true);
    img.hide_foreground_character(true);

    for (img_line, poem_line) in img.build_pixel_map().iter().zip(poem.split("\n")) {
        // IMAGE:
        print!("{}", img_line);

        // Terminal size line
        let tput_command = Command::new("tput").arg("cols").output().expect("tput error");
        let term_size: u32 = String::from_utf8_lossy(&tput_command.stdout).trim().parse().unwrap();

        // FIX: not implement Slice -> line[..term_size]
        let (mut line, mut count) = (String::new(), 1);
        for c in poem_line.chars() {
            line.push(c);  //                      1: the space on println!
            if count == (term_size - 40 - 1) {break;} else {count += 1;}
        }

        // POEM:
        println!(" {}", line);
    }
}
