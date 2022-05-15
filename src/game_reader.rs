use scrap::{Capturer, Display, Frame};

pub fn test(){
    let maindisp = Display::primary().expect("Fetching Display");
    let maincap = Capturer::new(maindisp).expect("Initializing Capturer");

    println!("primary display has dimensions: {} x {}",
        maincap.width(),
        maincap.height());
}
