// structs and enums to represent game state
#[derive(PartialEq, Copy, Clone)]
pub enum Element{
    Water,
    Fire,
    Earth,
    Air
}

#[derive(Copy, Clone)]
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
}

// the hexagonal board of sidelength 6 is represented as an 11x11 array.
// visualize it as taking the square and sorta moving the top edge to the right.
// the board is accessed using board[x][y] from the bottom left
struct GameState{
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

    pub fn get_peice(&self, x: usize, y: usize) -> Option<&Piece>{
        if x >= 11 || y >= 11{
            None
        }else{
            self.board[x][y].as_ref()
        }
    }

    pub fn set_peice(&mut self, piece: Option<Piece>, x: usize, y: usize) -> Result<Option<&Piece>, String>{
        if x >= 11 || y >= 11{
            Err(String::from("Attempted to place piece out of bounds"))
        }else{
            self.board[x][y] = piece;
            Ok(self.board[x][y].as_ref())
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
        assert!(gs.set_peice(a, 11, 11).is_err());
        assert!(gs.set_peice(a, 5, 6).is_ok());
        assert!(gs.get_peice(5, 6).is_some());
        if let Piece::Element(e) = gs.get_peice(5, 6).unwrap(){
            assert!(e == &Element::Fire);
        }else{
            panic!("expected piece of type 'Element'");
        }
    }
}
