use image;
// use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
// use image::Pixel;

use super::screenshot;
use super::game;
use super::image_manipulation;

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
    let piece_images = [
        (Some(game::Piece::Element(game::Element::Fire)), image::open("images/Pieces/Fire.png").unwrap()),
        (Some(game::Piece::Element(game::Element::Water)), image::open("images/Pieces/Water.png").unwrap()),
        (Some(game::Piece::Element(game::Element::Earth)), image::open("images/Pieces/Earth.png").unwrap()),
        (Some(game::Piece::Element(game::Element::Air)), image::open("images/Pieces/Air.png").unwrap()),
        (Some(game::Piece::Salt), image::open("images/Pieces/Salt.png").unwrap()),
        (Some(game::Piece::Quicksilver), image::open("images/Pieces/Quicksilver.png").unwrap()),
        (Some(game::Piece::Vitae), image::open("images/Pieces/Vitae.png").unwrap()),
        (Some(game::Piece::Mors), image::open("images/Pieces/Mors.png").unwrap()),
        (None, image::open("images/Pieces/Empty.png").unwrap())
    ];

    let mut im = image::open("images/Game1.png").unwrap();
    let mut gs = game::GameState::new();
    for xi in 0..11{
        for yi in 0..11{
            if game::GameState::on_board(xi, yi){
                let (x, y) = get_screen_coords(xi, yi);
                let mut best_diff: f64 = -1.0;
                let mut piece_guess = &None;
                for (piece, piece_im) in &piece_images{
                    let imdiff = image_manipulation::image_diff_normalized(&piece_im, &im.crop_imm(x, y, 52, 52));
                    let imdiff = match imdiff {
                        Ok(x) => x,
                        Err(s) => {
                            println!("{}", s);
                            best_diff
                        }
                    };

                    if imdiff < best_diff || best_diff == -1.0{
                        best_diff = imdiff;
                        piece_guess = piece;
                    }
                }

                let mc_subim = image_manipulation::max_contrast_grayscale(&im.crop_imm(x, y, 52, 52));
                for dx in 0..52{
                    for dy in 0..52{
                        im.put_pixel(x+dx, y+dy, mc_subim.get_pixel(dx, dy));
                    }
                }

                // println!("{:?}: {:?} , {:?}", (xi, yi), piece_guess, best_diff);
                match gs.set_piece(*piece_guess, xi as usize, yi as usize)  {
                    Ok(_) => (),
                    Err(message) => panic!("{}", message)
                };
                // if !colorcount.contains_key(&color){
                //     colorcount.insert(color, Vec::<(usize, usize)>::new());
                // }
                // match colorcount.get_mut(&colordiff){
                //     Some(v) => v.push((xi, yi)),
                //     None => (),
                // }
            }
        }
    }

    gs.print();

    // for (color, coords) in &colorcount {
    //     println!("{:?}: {:?}", color, coords);
    // }

    match im.save("images/game_reader_test_2.png"){
        Ok(_) => println!("image saved!"),
        Err(_) => println!("screenshot failed to save!"),
    };
}

pub fn oldtest(){
    let mut im = image::open("images/Game1.png").unwrap();
    for xi in 0..11{
        for yi in 0..11{
            if game::GameState::on_board(xi, yi){
                let (x, y) = get_screen_coords(xi, yi);
                im.put_pixel(x, y, image::Rgba::<u8>([0, 255, 0, 255]));

                let (x, y) = get_screen_coords_center(xi, yi);
                im.put_pixel(x, y, image::Rgba::<u8>([255, 0, 0, 255]));
            }
        }
    }
    match im.save("images/game_reader_test.png"){
        Ok(_) => println!("image saved!"),
        Err(_) => println!("screenshot failed to save!"),
    };
}
