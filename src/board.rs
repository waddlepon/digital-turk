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
        let sections = fen.split_whitespace().collect();
    }
}
