const WHITE: u8 = 0;
const BLACK: u8 = 7;

const KING: u8 = 1;
const QUEEN: u8 = 2;
const ROOK: u8 = 3;
const KNIGHT: u8 = 4;
const BISHOP: u8 = 5;
const PAWN: u8 = 6;

pub struct BitBoards(pub [u64; 14]);

impl BitBoards {
    pub fn update_all(&mut self) {
        self[WHITE] = self[WHITE + KING] | self[WHITE + QUEEN] | self[WHITE + ROOK] | self[WHITE + KNIGHT]
            | self[WHITE + BISHOP] | self[WHITE + PAWN];

        self[BLACK] = self[BLACK + KING] | self[BLACK + QUEEN] | self[BLACK + ROOK] | self[BLACK + KNIGHT]
            | self[BLACK + BISHOP] | self[BLACK + PAWN];
    }

    //rank and file are 1 indexed
    pub fn set_square(&mut self, bitboard: u8, rank: u8, file: u8, value: bool) {
        if value {
            self[bitboard] |= 1 << ((rank - 1) * 8 + (8 - file));
        }
        else {
            self[bitboard] ^= 1 << ((rank - 1) * 8 + (8 - file));
        }
    }
}

const WHITE_TO_MOVE: u8 = 0;
const BLACK_TO_MOVE: u8 = 1;

const WK_CASTLE: u8 = 0b00000001;
const WQ_CASTLE: u8 = 0b00000010;
const BK_CASTLE: u8 = 0b00000100;
const BQ_CASTLE: u8 = 0b00001000;

pub struct Board {
    bitboards: BitBoards,
    to_move: u8,
    castling: u8,
    en_passant: u64,
    half_move: u8,
    full_move: u16,
}

impl Board {
    pub fn from_fen(fen: &str) -> Self {
        let mut split_iter = fen.split(' ');

        let position_str = split_iter.next();
        let to_move_str = split_iter.next();
        let castling_str = split_iter.next();
        let en_passant_str = split_iter.next();
        let half_move: u8 = split_iter.next().parse().expect("error parsing halfmove string");
        let full_move: u16 = split_iter.next().parse().expect("error parsing fullmove string");

        let mut bitboards = BitBoards([0; 14]);

        //position in fen string goes in order of rank 8 -> rank 1
        let mut file  = 0;
        for (rank, rank_str) in position_str.split('/').enumerate() {
            if rank > 7 {
                return Err("Too many ranks in position string");
            }

            file = 0;
            for c in rank_str.chars() {
                if c.is_digit(10) {
                    let skip: u8 = c.parse().unwrap();
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
                    }

                    if c.is_ascii_uppercase() {
                        bitboards.set_square(WHITE + piece, rank + 1, file + 1, true);
                    }
                    else {
                        bitboards.set_square(BLACK + piece, rank + 1, file + 1, true);
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
        if en_passant_str != '-' {
            let chars = x.chars();
            //this implementation makes me want to end myself but i dunno how to get ascii codes from chars in rust
            let file = match chars.nth(0) {
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
                                (chars.nth(1).parse().expect("Invalid en passant target square") - 1));
        }

        Board { bitboards: bitboards, to_move: to_move, castling: castling, en_passant: en_passant,
        half_move: half_move, full_move: full_move }
    }
}
