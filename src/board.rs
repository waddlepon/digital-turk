use std::ascii::AsciiExt;
use std::fmt;

const WHITE: usize = 0;
const BLACK: usize = 7;

const KING: usize = 1;
const QUEEN: usize = 2;
const ROOK: usize = 3;
const KNIGHT: usize = 4;
const BISHOP: usize = 5;
const PAWN: usize = 6;

const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct BitBoards(pub [u64; 14]);

impl BitBoards {
    pub fn update_all(&mut self) {
        self.0[WHITE] = self.0[WHITE + KING] | self.0[WHITE + QUEEN] | self.0[WHITE + ROOK] | self.0[WHITE + KNIGHT]
            | self.0[WHITE + BISHOP] | self.0[WHITE + PAWN];

        self.0[BLACK] = self.0[BLACK + KING] | self.0[BLACK + QUEEN] | self.0[BLACK + ROOK] | self.0[BLACK + KNIGHT]
            | self.0[BLACK + BISHOP] | self.0[BLACK + PAWN];
    }

    //rank and file are 1 indexed
    pub fn set_square(&mut self, bitboard: usize, rank: u8, file: u8, value: bool) {
        if value {
            self.0[bitboard] |= 1 << ((rank - 1) * 8 + (8 - file));
        }
        else {
            self.0[bitboard] ^= 1 << ((rank - 1) * 8 + (8 - file));
        }
    }
}

impl fmt::Debug for BitBoards {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        for (i, elem) in self.0.iter().enumerate() {
            if i < 7 {
                write!(f, "w")?;
            }
            else {
                write!(f, "b")?;
            }

            match i % 7 {
                KING => write!(f, "k: ")?,
                QUEEN => write!(f, "q: ")?,
                ROOK => write!(f, "r: ")?, 
                KNIGHT => write!(f, "n: ")?,
                BISHOP => write!(f, "b: ")?,
                PAWN => write!(f, "p: ")?,
                _ => write!(f, ":  ")?,
            }

            write!(f, "{:064b}\n", elem)?;
        }
        Ok(())
    }
}

const WHITE_TO_MOVE: u8 = 0;
const BLACK_TO_MOVE: u8 = 1;

const WK_CASTLE: u8 = 0b00000001;
const WQ_CASTLE: u8 = 0b00000010;
const BK_CASTLE: u8 = 0b00000100;
const BQ_CASTLE: u8 = 0b00001000;

#[derive(Debug)]
pub struct Board {
    bitboards: BitBoards,
    to_move: u8,
    castling: u8,
    en_passant: u64,
    half_move: u8,
    full_move: u16,
}

impl Board {
    pub fn from_fen(fen: &str) -> Result <Board, &'static str> {
        let mut split_iter = fen.split(' ');

        let position_str = split_iter.next().unwrap();
        let to_move_str = split_iter.next().unwrap();
        let castling_str = split_iter.next().unwrap();
        let en_passant_str = split_iter.next().unwrap();
        let half_move: u8 = split_iter.next().unwrap().parse().unwrap();
        let full_move: u16 = split_iter.next().unwrap().parse().unwrap();

        let mut bitboards = BitBoards([0; 14]);

        //position in fen string goes in order of rank 8 -> rank 1
        let mut file;
        for (rank, rank_str) in position_str.split('/').enumerate() {
            if rank > 7 {
                return Err("Too many ranks in position string");
            }

            file = 0;
            for c in rank_str.chars() {
                if c.is_digit(10) {
                    let skip: u8 = c.to_digit(10).unwrap() as u8;
                    if skip > 8 {
                        file += skip;
                    }
                }
                else {
                    let piece = match c.to_ascii_lowercase() {
                        'p' => PAWN,
                            'n' => KNIGHT,
                            'b' => BISHOP,
                            'r' => ROOK,
                            'q' => QUEEN,
                            'k' => KING,
                            _ => return Err("Invalid piece in position string"),
                    };

                    if c.is_uppercase() {
                        bitboards.set_square(WHITE + piece, rank as u8 + 1, file + 1, true);
                    }
                    else {
                        bitboards.set_square(BLACK + piece, rank as u8 + 1, file + 1, true);
                    }
                    file += 1;
                }
            }
            bitboards.update_all();
        }

        let to_move = match to_move_str {
            "w" => WHITE_TO_MOVE,
            "b" => BLACK_TO_MOVE,
            _ => return Err("Invalid to_move string"),
        };

        let mut castling: u8 = 0;
        if castling_str.contains('K') { castling |= WK_CASTLE };
        if castling_str.contains('Q') { castling |= WQ_CASTLE };
        if castling_str.contains('k') { castling |= BK_CASTLE };
        if castling_str.contains('q') { castling |= BQ_CASTLE };

        let mut en_passant: u64 = 0;
        if en_passant_str != "-" {
            let mut chars = en_passant_str.chars();
            //this implementation makes me want to end myself but i dunno how to get ascii codes from chars in rust
            let file = match chars.nth(0).unwrap() {
                'a' => 1,
                'b' => 2,
                'c' => 3,
                'd' => 4,
                'e' => 5,
                'f' => 6,
                'g' => 7,
                'h' => 8,
                _ => return Err("Invalid en passant target square"),
            };
            en_passant |= 1 << ((8 - file) + 8 * 
                                (chars.nth(1).unwrap().to_digit(10).unwrap() - 1));
        }

        Ok(Board { bitboards: bitboards, to_move: to_move, castling: castling, en_passant: en_passant,
        half_move: half_move, full_move: full_move })
    }
    
    pub fn start_position() -> Result <Board, &'static str> {
        Board::from_fen(START_POSITION)
    }
}
