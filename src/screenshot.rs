use std::thread;
use std::time;
use std::io::ErrorKind::WouldBlock;
use scrap::{Capturer, Display};
use image::ImageBuffer;
use image::RgbaImage;
use image::DynamicImage;





pub fn test(){
    for i in (0..5).rev(){
        println!("{}..", i);
        thread::sleep(time::Duration::new(1, 0));
    }

    let im = get_screen_image();
    match im.save("screenshot_test.png"){
        Ok(_) => println!("screenshot taken!"),
        Err(_) => println!("screenshot failed to save!"),
    };

}

pub fn get_screen_image() -> DynamicImage{
    let (w, h, captured_buff) = get_screen_frame();
    let captured_image_buff: RgbaImage =
        ImageBuffer::from_vec(w as u32, h as u32, captured_buff.clone()).unwrap();

    let captured_image = DynamicImage::ImageRgba8(captured_image_buff);

    captured_image
}

pub fn get_screen_frame() -> (usize, usize, Vec<u8>){
    let maindisp = Display::primary().expect("Couldn't find primary display.");
    let mut maincap = Capturer::new(maindisp).expect("Could not initialize Capturer.");
    let (w, h) = (maincap.width(), maincap.height());

    println!("primary display has dimensions: {} x {}", w, h);

    let screen_frame = loop {
        let buffer = match maincap.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    // original code has a short delay here
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        break buffer;
    };

    // println!("The frame has length {}", screen_frame.len());

    // we need to convert from BGRA to RGBA

    let mut rearranged = Vec::with_capacity(w * h * 4);

    for y in 0..h {
        for x in 0..w {
            let i = (w * y + x) * 4;
            rearranged.extend_from_slice(&[screen_frame[i+2], screen_frame[i+1], screen_frame[i], 255]);
        }
    }

    (w, h, rearranged)
}
