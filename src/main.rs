use std::thread;
use std::time;
use std::io;

mod screenshot;
mod game;
mod game_reader;
mod image_manipulation;

fn main() {
    println!("Program will begin playing in 5 seconds.");

    for i in (0..5).rev(){
        println!("{}..", i);
        thread::sleep(time::Duration::new(1, 0));
    }

    let screen_image = screenshot::get_screen_image();
    let mut game_state = game_reader::read_board(&screen_image);
    let solution = game_state.solve();
    match solution {
        Some(s) => game_reader::perform_solution(&s),
        None => {println!("Failed to find a solution; read boardstate:"); game_state.print()}
    };

    // game::test();
    // screenshot::test();
    // game_reader::test();
    // game_reader::mousetest();
}
