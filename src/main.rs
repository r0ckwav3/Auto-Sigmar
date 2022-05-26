use std::thread;
use std::time;

mod screenshot;
mod game;
mod game_reader;
mod image_manipulation;

fn main() {
    for i in (0..5).rev(){
        println!("{}..", i);
        thread::sleep(time::Duration::new(1, 0));
    }

    let mut gs = game::GameState::example();
    let solution = gs.solve();
    match solution {
        Some(s) => game_reader::perform_solution(&s),
        None => {println!("Failed to find a solution; read boardstate:"); gs.print()}
    };

    // game::test();
    // screenshot::test();
    // game_reader::test();
    // game_reader::mousetest();
}
