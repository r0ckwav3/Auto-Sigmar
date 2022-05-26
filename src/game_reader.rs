use std::collections::HashSet;
use std::collections::HashMap;
use std::thread;
use std::time;

use image;
use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
// use image::Pixel;

use mouce::Mouse;
use mouce::common::MouseButton;

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

pub fn read_board(im: &DynamicImage) -> game::GameState{
    let piece_images = [
        (Some(game::Piece::Element(game::Element::Fire)), image::open("images/Pieces/Fire.png").unwrap()),
        (Some(game::Piece::Element(game::Element::Water)), image::open("images/Pieces/Water.png").unwrap()),
        (Some(game::Piece::Element(game::Element::Earth)), image::open("images/Pieces/Earth.png").unwrap()),
        (Some(game::Piece::Element(game::Element::Air)), image::open("images/Pieces/Air.png").unwrap()),
        (Some(game::Piece::Salt), image::open("images/Pieces/Salt.png").unwrap()),
        (Some(game::Piece::Metal(0)), image::open("images/Pieces/Metal1.png").unwrap()),
        (Some(game::Piece::Metal(1)), image::open("images/Pieces/Metal2.png").unwrap()),
        (Some(game::Piece::Metal(2)), image::open("images/Pieces/Metal3.png").unwrap()),
        (Some(game::Piece::Metal(3)), image::open("images/Pieces/Metal4.png").unwrap()),
        (Some(game::Piece::Metal(4)), image::open("images/Pieces/Metal5.png").unwrap()),
        (Some(game::Piece::Metal(5)), image::open("images/Pieces/Metal6.png").unwrap()),
        (Some(game::Piece::Quicksilver), image::open("images/Pieces/Quicksilver.png").unwrap()),
        (Some(game::Piece::Vitae), image::open("images/Pieces/Vitae.png").unwrap()),
        (Some(game::Piece::Mors), image::open("images/Pieces/Mors.png").unwrap()),
        (None, image::open("images/Pieces/Empty.png").unwrap())
    ];

    let mut gs = game::GameState::new();
    let mut candidates = Vec::new();

    for xi in 0..11{
        for yi in 0..11{
            if game::GameState::on_board(xi, yi){
                let (x, y) = get_screen_coords(xi, yi);
                for (piece, piece_im) in &piece_images{
                    let imdiff = image_manipulation::image_diff_normalized_middle(&piece_im, &im.crop_imm(x, y, 52, 52));
                    let imdiff = match imdiff {
                        Ok(x) => x,
                        Err(s) => {
                            println!("imdiff Error: {}", s);
                            0.0
                        }
                    };

                    candidates.push((imdiff, piece, xi, yi))
                }
            }
        }
    }

    let mut pieces_left = HashMap::from([
        (game::Piece::Element(game::Element::Fire), 8),
        (game::Piece::Element(game::Element::Water), 8),
        (game::Piece::Element(game::Element::Earth), 8),
        (game::Piece::Element(game::Element::Air), 8),
        (game::Piece::Salt, 4),
        (game::Piece::Metal(0), 1),
        (game::Piece::Metal(1), 1),
        (game::Piece::Metal(2), 1),
        (game::Piece::Metal(3), 1),
        (game::Piece::Metal(4), 1),
        (game::Piece::Metal(5), 1),
        (game::Piece::Quicksilver, 5),
        (game::Piece::Vitae, 4),
        (game::Piece::Mors, 4),
    ]);

    // candidates.sort_by_key(|a| a.0);
    candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut placed_pieces = HashSet::new();

    for c in candidates {
        let (imdiff, piece, xi, yi) = c;
        if !placed_pieces.contains(&(xi, yi)) {
            let availible = match piece {
                Some(p) => *pieces_left.entry(*p).or_insert(0) > 0,
                None => true,
            };
            if availible {
                // println!("{:?}: {:?} , {:?}", (xi, yi), piece, imdiff);
                placed_pieces.insert((xi, yi));
                if let Some(p) = piece{
                    *pieces_left.entry(*p).or_insert(0) -= 1;
                }

                match gs.set_piece(*piece, xi as usize, yi as usize) {
                    Ok(_) => (),
                    Err(message) => panic!("{}", message)
                };
            }
        }

    }


    gs
}

pub fn perform_solution(solution: &Vec<[usize; 4]>){
    let mouse_manager = Mouse::new();
    let clickdelay = time::Duration::from_millis(5);

    for action in solution{
        let (x, y) = get_screen_coords_center(action[0], action[1]);
        mouse_manager.move_to(x as usize, y as usize)
            .expect(&format!("Attempted to move to ({}, {}) -> ({}, {})",
                action[0], action[1], x, y)[..]);
        thread::sleep(clickdelay);
        mouse_manager.click_button(&MouseButton::Left).expect("Attempted to click");
        thread::sleep(clickdelay);

        let (x, y) = get_screen_coords_center(action[2], action[3]);
        mouse_manager.move_to(x as usize, y as usize)
            .expect(&format!("Attempted to move to ({}, {}) -> ({}, {})",
                action[2], action[3], x, y)[..]);
        thread::sleep(clickdelay);
        mouse_manager.click_button(&MouseButton::Left).expect("Attempted to click");
        thread::sleep(clickdelay);
    }
}

pub fn test(){
    let im = image::open("images/Game3.png").unwrap();
    let gs = read_board(&im);
    gs.print();
}

pub fn mousetest(){
    let mouse_manager = Mouse::new();
    for i in (0..5).rev(){
        println!("{}..", i);
        thread::sleep(time::Duration::new(1, 0));
    }

    for xi in 0..11{
        for yi in 0..11{
            if game::GameState::on_board(xi, yi){
                let (x, y) = get_screen_coords_center(xi, yi);
                mouse_manager.move_to(x as usize, y as usize).expect("Attempted move");
                thread::sleep(time::Duration::from_millis(500));
            }
        }
    }

    mouse_manager.move_to(720, 450);
    mouse_manager.click_button(&MouseButton::Left).expect("Attempted to click");
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

    let mut im = image::open("images/Game1.png").unwrap();
    for xi in 0..11{
        for yi in 0..11{
            if game::GameState::on_board(xi, yi){
                let (x, y) = get_screen_coords(xi, yi);

                let mc_subim = image_manipulation::max_contrast_grayscale(&im.crop_imm(x, y, 52, 52));
                for dx in 0..52{
                    for dy in 0..52{
                        im.put_pixel(x+dx, y+dy, mc_subim.get_pixel(dx, dy));
                    }
                }
            }
        }
    }
    match im.save("images/game_reader_test_2.png"){
        Ok(_) => println!("image saved!"),
        Err(_) => println!("screenshot failed to save!"),
    };
}
