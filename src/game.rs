// structs and enums to represent game state
#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
pub enum Element{
    Water,
    Fire,
    Earth,
    Air
}

#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
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

    // same as Game1.png
    pub fn example() -> GameState{
        GameState{
            board: [[None, None, None, None, None, Some(Piece::Element(Element::Air)), None, None, None, None, Some(Piece::Element(Element::Water))], [None, None, None, None, None, Some(Piece::Element(Element::Earth)), Some(Piece::Mors), Some(Piece::Salt), Some(Piece::Metal(4)), Some(Piece::Element(Element::Fire)), None], [None, None, None, None, Some(Piece::Element(Element::Water)), Some(Piece::Vitae), Some(Piece::Element(Element::Air)), None, Some(Piece::Quicksilver), Some(Piece::Metal(3)), None], [None, None, None, Some(Piece::Vitae), None, Some(Piece::Element(Element::Fire)), Some(Piece::Element(Element::Fire)), Some(Piece::Element(Element::Earth)), Some(Piece::Metal(1)), Some(Piece::Metal(2)), None], [None, None, Some(Piece::Mors), Some(Piece::Element(Element::Air)), Some(Piece::Element(Element::Air)), None, None, Some(Piece::Element(Element::Fire)), None, Some(Piece::Element(Element::Fire)), None], [Some(Piece::Element(Element::Earth)), Some(Piece::Element(Element::Earth)), Some(Piece::Element(Element::Earth)), Some(Piece::Element(Element::Earth)), None, Some(Piece::Metal(5)), None, Some(Piece::Salt), Some(Piece::Element(Element::Water)), Some(Piece::Metal(0)), Some(Piece::Element(Element::Fire))], [None, Some(Piece::Vitae), None, Some(Piece::Salt), None, None, Some(Piece::Element(Element::Earth)), Some(Piece::Element(Element::Air)), Some(Piece::Mors), None, None], [None, Some(Piece::Quicksilver), Some(Piece::Element(Element::Earth)), Some(Piece::Element(Element::Water)), Some(Piece::Element(Element::Water)), Some(Piece::Element(Element::Air)), None, Some(Piece::Element(Element::Water)), None, None, None], [None, Some(Piece::Element(Element::Water)), Some(Piece::Element(Element::Fire)), None, Some(Piece::Quicksilver), Some(Piece::Mors), Some(Piece::Element(Element::Water)), None, None, None, None], [None, Some(Piece::Quicksilver), Some(Piece::Vitae), Some(Piece::Element(Element::Fire)), Some(Piece::Salt), Some(Piece::Quicksilver), None, None, None, None, None], [Some(Piece::Element(Element::Air)), None, None, None, None, Some(Piece::Element(Element::Air)), None, None, None, None, None]],
            metals_taken: 0u8,
        }
    }

    pub fn on_board(x: usize, y: usize) -> bool{
        (x+y >= 5) && (x+y <= 15) && (x<11) && (y<11)
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

    pub fn open_pieces(&self) -> Vec<(usize, usize)>{
        let mut ans = Vec::new();
        for x in 0..11{
            for y in 0..11{
                if self.board[x][y].is_some(){
                    if self.is_open(x, y){
                        ans.push((x, y));
                    }
                }
            }
        }

        ans
    }

    pub fn is_open(&self, x: usize, y: usize) -> bool{
        if GameState::on_board(x, y){
            if let Some(p) = self.board[x][y]{
                if let Piece::Metal(m) = p{
                    if m != self.metals_taken{
                        return false;
                    }
                }

                let mut neighbors = [false; 6];
                for (i, d) in [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)].iter().enumerate(){
                    if (x as i32 + d.0) < 0 || (y as i32 + d.1) < 0{
                        continue;
                    }
                    let (xnew, ynew) = ((x as i32 + d.0) as usize,
                                        (y as i32 + d.1) as usize);
                    if GameState::on_board(xnew, ynew){
                        if self.board[xnew][ynew].is_some(){
                            neighbors[i] = true;
                        }
                    }
                }

                let mut isgood = false;
                for i in 0..6{
                    if !neighbors[i] && !neighbors[(i+1) % 6] && !neighbors[(i+2) % 6]{
                        isgood = true;
                    }
                }

                isgood
            }else{
                false
            }
        }else{
            false
        }
    }

    pub fn is_solved(&self) -> bool{
        let mut ans = true;

        for x in 0..11{
            for y in 0..11{
                if self.board[x][y].is_some(){
                    if (x, y) != (5, 5){
                        ans = false;
                    }
                }
            }
        }

        ans
    }

    pub fn print(&self){
        // println!("{:?}", self.board);

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

    // solves in place, but should return the board to its original state by the end of execution
    pub fn solve(&mut self) -> Option<Vec<[usize; 4]>>{
        if self.is_solved() {
            Some(Vec::from([[5, 5, 5, 5]]))
        }else{
             let open_pieces = self.open_pieces();
             let mut loopresult = None;

             'outerpair:
             for i in 0..open_pieces.len(){
                 let ipos = open_pieces[i];
                 let ipiece = self.board[ipos.0][ipos.1].expect("Open piece was None");
                 for j in 0..open_pieces.len(){
                     let jpos = open_pieces[j];
                     let jpiece = self.board[jpos.0][jpos.1]
                        .expect("Open piece was None");
                     if i != j && ipiece.legal_pair(&jpiece){
                         let metalchange = match (ipiece, jpiece){
                             (_, Piece::Metal(_)) => 1,
                             (Piece::Metal(_), _) => 1,
                             _other => 0
                         };
                         self.metals_taken += metalchange;
                         self.set_piece(None, ipos.0, ipos.1).expect("Failed to place piece in solve");
                         self.set_piece(None, jpos.0, jpos.1).expect("Failed to place piece in solve");
                         let rec = self.solve();
                         self.metals_taken -= metalchange;
                         self.set_piece(Some(ipiece), ipos.0, ipos.1).expect("Failed to place piece in solve");
                         self.set_piece(Some(jpiece), jpos.0, jpos.1).expect("Failed to place piece in solve");

                         if let Some(mut v) = rec{
                             v.insert(0, [ipos.0, ipos.1, jpos.0, jpos.1]);
                             loopresult = Some(v);
                             break 'outerpair;
                         }
                     }
                 }
             }

             loopresult
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
        assert!(gs.set_piece(a, 5, 5).is_ok());
        assert!(gs.get_piece(5, 5).is_some());
        if let Piece::Element(e) = gs.get_piece(5, 5).unwrap(){
            assert!(e == &Element::Fire);
        }else{
            panic!("expected piece of type 'Element'");
        }

        // makes a circle in the middle
        gs.set_piece(Some(Piece::Salt), 6, 5).expect("Failed to place piece.");
        gs.set_piece(Some(Piece::Salt), 4, 5).expect("Failed to place piece.");
        gs.set_piece(Some(Piece::Salt), 5, 6).expect("Failed to place piece.");
        gs.set_piece(Some(Piece::Salt), 5, 4).expect("Failed to place piece.");
        gs.set_piece(Some(Piece::Salt), 6, 4).expect("Failed to place piece.");
        gs.set_piece(Some(Piece::Salt), 4, 6).expect("Failed to place piece.");

        assert!(gs.is_open(6, 5));
        assert!(gs.is_open(4, 5));
        assert!(gs.is_open(5, 6));
        assert!(gs.is_open(5, 4));
        assert!(gs.is_open(6, 4));
        assert!(gs.is_open(4, 6));

        gs.set_piece(Some(Piece::Metal(5)), 5, 7).expect("Failed to place piece.");
        assert!(!gs.is_open(5, 5));
        assert!(!gs.is_open(5, 6));
    }
    {
        let mut gs = GameState::example();
        let solution = gs.solve();
        println!("{:?}",solution);
        if let Some(unpacked_solution) = solution{
            println!("length: {}", unpacked_solution.len());
        }

        let gs2 = GameState::example();
        for x in 0..11{
            for y in 0..11{
                assert_eq!(gs.get_piece(x, y), gs2.get_piece(x, y));
            }
        }
    }
}
