use image;
use image::DynamicImage;
use image::GenericImage;

use super::screenshot;
use super::game;

// In 1440x900 resolution, each marble is 52 pixels by 52 pixels
// the horizontal offset in a row is 66
// the vertical offset between rows is 57
// the bottom of row 0 is at 173 (0 indexed from bottom)

// converts board coordinates to screen coordinates (top left corner)
fn get_screen_coords(xi: usize, yi: usize) -> (u32, u32){
    let xi = xi as i32;
    let yi = yi as i32;

    let offsetvec = (456, 675);
    let xivec = (66, 0);
    let yivec = (33, -57);
    let pos = (
        (offsetvec.0 + xivec.0*xi + yivec.0*yi) as u32,
        (offsetvec.1 + xivec.1*xi + yivec.1*yi) as u32
    );

    pos
}

// converts board coordinates to screen coordinates (approximate center of marble)
fn get_screen_coords_center(xi: usize, yi: usize) -> (u32, u32){
    let marblesize = (52, 52);
    let topleftpos = get_screen_coords(xi, yi);
    let pos = (
        topleftpos.0 + marblesize.0/2,
        topleftpos.1 + marblesize.1/2
    );

    pos
}

pub fn test(){
    {
        let mut im = image::open("images/Game1.png").unwrap();
        for xi in 0..11{
            for yi in 0..11{
                let (x, y) = get_screen_coords(xi, yi);
                if(x < 1440 && y < 900){
                    im.put_pixel(x, y, image::Rgba::<u8>([0, 255, 0, 255]))
                }
                let (x, y) = get_screen_coords_center(xi, yi);
                if(x < 1440 && y < 900){
                    im.put_pixel(x, y, image::Rgba::<u8>([255, 0, 0, 255]))
                }
            }
        }
        match im.save("images/game_reader_test.png"){
            Ok(_) => println!("image saved!"),
            Err(_) => println!("screenshot failed to save!"),
        };
    }
}
