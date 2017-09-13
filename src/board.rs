use std::ascii::AsciiExt;
use std::fmt;
use magic::MagicBoards;
use magic::KING_MOVES;
use magic::KNIGHT_MOVES;
use util::bit_indexes;

const WHITE: usize = 0;
const BLACK: usize = 7;

const KING: usize = 1;
const QUEEN: usize = 2;
const ROOK: usize = 3;
const KNIGHT: usize = 4;
const BISHOP: usize = 5;
const PAWN: usize = 6;

const ALL: usize = 14;

const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

//MSB is a1, LSB is h8
pub struct BitBoards(pub [u64; 15]);

impl BitBoards {
    pub fn update_all(&mut self) {
        self.0[WHITE] = self.0[WHITE + KING] | self.0[WHITE + QUEEN] | self.0[WHITE + ROOK] |
            self.0[WHITE + KNIGHT] | self.0[WHITE + BISHOP] |
            self.0[WHITE + PAWN];

        self.0[BLACK] = self.0[BLACK + KING] | self.0[BLACK + QUEEN] | self.0[BLACK + ROOK] |
            self.0[BLACK + KNIGHT] | self.0[BLACK + BISHOP] |
            self.0[BLACK + PAWN];
        self.0[ALL] = self.0[WHITE] | self.0[BLACK];
    }

    //rank and file are 1 indexed
    pub fn set_square(&mut self, bitboard: usize, rank: u8, file: u8, value: bool) {
        if value {
            self.0[bitboard] |= 1 << ((rank - 1) * 8 + (8 - file));
        } else {
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
            } else if i < 14 {
                write!(f, "b")?;
            } else {
                write!(f, "a");
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

//moves stored as follows file: 6 bits rank: 6 bits flags(indicate type of move): 4 bits
type Move = u16;

pub struct Board<'a> {
    bitboards: BitBoards,
    to_move: u8,
    castling: u8,
    en_passant: u8,
    half_move: u8,
    full_move: u16,
    magic_boards: &'a MagicBoards,
}

impl<'a> Board<'a> {
    pub fn from_fen<'b>(
        fen: &str,
        magic_boards: &'b MagicBoards,
    ) -> Result<Board<'b>, &'static str> {
        let mut split_iter = fen.split(' ');

        let position_str = split_iter.next().unwrap();
        let to_move_str = split_iter.next().unwrap();
        let castling_str = split_iter.next().unwrap();
        let en_passant_str = split_iter.next().unwrap();
        let half_move: u8 = split_iter.next().unwrap().parse().unwrap();
        let full_move: u16 = split_iter.next().unwrap().parse().unwrap();

        let mut bitboards = BitBoards([0; 15]);

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
                    if skip < 8 {
                        file += skip;
                    }
                } else {
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
                    } else {
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
        if castling_str.contains('K') {
            castling |= WK_CASTLE
        };
        if castling_str.contains('Q') {
            castling |= WQ_CASTLE
        };
        if castling_str.contains('k') {
            castling |= BK_CASTLE
        };
        if castling_str.contains('q') {
            castling |= BQ_CASTLE
        };

        let mut en_passant: u8 = 0;
        if en_passant_str != "-" {
            let mut chars = en_passant_str.chars();
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
            en_passant |= file << 3;
            en_passant |= chars.nth(1).unwrap().to_digit(10).unwrap() as u8 + 1;
        }

        Ok(Board {
            bitboards: bitboards,
            to_move: to_move,
            castling: castling,
            en_passant: en_passant,
            half_move: half_move,
            full_move: full_move,
            magic_boards: magic_boards,
        })
    }

    pub fn start_position(magic_boards: &MagicBoards) -> Result<Board, &'static str> {
        Board::from_fen(START_POSITION, magic_boards)
    }

    pub fn king_moves(&self) -> u64 {
        let moving = if self.to_move == 0 { WHITE } else { BLACK };
        let not_moving = if self.to_move == 1 { WHITE } else { BLACK };

        let square = self.bitboards.0[moving + KING].trailing_zeros();
        if square == 64 {
            panic!("No king");
        }
        let king_moves = KING_MOVES[square as usize];

        let mut attacks: u64 = 0;
        let occupancy: u64 = self.bitboards.0[ALL] & !(self.bitboards.0[moving + KING]);

        for i in bit_indexes(self.bitboards.0[not_moving + ROOK]).iter() {
            attacks |= self.magic_boards.magic_move_rook(*i as usize, occupancy);
        }
        for i in bit_indexes(self.bitboards.0[not_moving + BISHOP]).iter() {
            attacks |= self.magic_boards.magic_move_bishop(*i as usize, occupancy);
        }
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(100);
        moves
    }
}

impl<'a> fmt::Debug for Board<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}\n", self.bitboards)?;
        write!(f, "to_move: {:?}\n", self.to_move)?;
        write!(f, "castling: {:?}\n", self.castling)?;
        write!(f, "en_passant: {:?}\n", self.en_passant)?;
        write!(f, "half_moves: {:?}\n", self.half_move)?;
        write!(f, "full_moves: {:?}", self.full_move)
    }
}
