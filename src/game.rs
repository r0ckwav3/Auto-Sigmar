// structs and enums to represent game state
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Element{
    Water,
    Fire,
    Earth,
    Air
}

#[derive(Copy, Clone, Debug)]
pub enum Piece{
    Element(Element),
    Salt,
    Metal(u8), // the metals, with lead being 0 and gold being 7 (6?)
    Quicksilver,
    Vitae,
    Mors
}

impl Piece{
    // metals can always be matched according to this, but they will not be considered
    // "unlocked" until other metals have been cleared.
    fn legal_pair(&self, other: &Piece) -> bool{
        match (self, other){
            (Piece::Element(a), Piece::Element(b)) => a == b,
            (Piece::Element(_), Piece::Salt) => true,
            (Piece::Salt, Piece::Element(_)) => true,
            (Piece::Salt, Piece::Salt) => true,

            (Piece::Metal(_), Piece::Quicksilver) => true,
            (Piece::Quicksilver, Piece::Metal(_)) => true,

            (Piece::Vitae, Piece::Mors) => true,
            (Piece::Mors, Piece::Vitae) => true,

            _other => false
        }
    }

    fn tochar(&self) -> String{
        match self{
            Piece::Element(Element::Fire) => String::from("F"),
            Piece::Element(Element::Water) => String::from("W"),
            Piece::Element(Element::Earth) => String::from("E"),
            Piece::Element(Element::Air) => String::from("A"),
            Piece::Salt => String::from("S"),
            Piece::Metal(x) => format!("{}", x),
            Piece::Quicksilver => String::from("Q"),
            Piece::Vitae => String::from("V"),
            Piece::Mors => String::from("M"),
        }
    }
}

// the hexagonal board of sidelength 6 is represented as an 11x11 array.
// visualize it as taking the square and sorta moving the top edge to the right.
// the board is accessed using board[x][y] from the bottom left
pub struct GameState{
    board: [[Option<Piece>; 11]; 11],
    metals_taken: u8
}

impl GameState{
    pub fn new() -> GameState{
        GameState{
            board: [[None; 11]; 11], // depending on how this works, the rows may be pointers to the same mem address
            metals_taken: 0u8
        }
    }

    pub fn on_board(xi: usize, yi: usize) -> bool{
        (xi+yi >= 5) && (xi+yi <= 15)
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<&Piece>{
        if x >= 11 || y >= 11{
            None
        }else{
            self.board[x][y].as_ref()
        }
    }

    pub fn set_piece(&mut self, piece: Option<Piece>, x: usize, y: usize) -> Result<Option<&Piece>, String>{
        if x >= 11 || y >= 11{
            Err(String::from("Attempted to place piece out of bounds"))
        }else{
            self.board[x][y] = piece;
            Ok(self.board[x][y].as_ref())
        }
    }

    pub fn print(& self){
        for y in (0..11).rev(){
            for _ in 0..y{
                print!(" ");
            }
            for x in 0..11{
                print!("{} ", match self.get_piece(x, y){
                    Some(e) => e.tochar(),
                    None => String::from(" "),
                });
            }
            println!("");
        }
    }
}

pub fn test(){
    {
        let a = Piece::Element(Element::Fire);
        let b = Piece::Element(Element::Water);
        let c = Piece::Salt;
        let d = Piece::Quicksilver;

        assert!(a.legal_pair(&a));
        assert!(a.legal_pair(&c));
        assert!(c.legal_pair(&a));

        assert!(!a.legal_pair(&b));
        assert!(!c.legal_pair(&d));
        assert!(!d.legal_pair(&d));
    }
    {
        let mut gs = GameState::new();
        let a = Some(Piece::Element(Element::Fire));
        assert!(gs.set_piece(a, 11, 11).is_err());
        assert!(gs.set_piece(a, 5, 6).is_ok());
        assert!(gs.get_piece(5, 6).is_some());
        if let Piece::Element(e) = gs.get_piece(5, 6).unwrap(){
            assert!(e == &Element::Fire);
        }else{
            panic!("expected piece of type 'Element'");
        }
    }
}
