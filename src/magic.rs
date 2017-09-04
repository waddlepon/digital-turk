use rand::{Rng, thread_rng};

const KNIGHT_MOVES: [u64; 64] =
[9077567998918656, 4679521487814656, 38368557762871296, 19184278881435648, 9592139440717824,
4796069720358912, 2257297371824128, 1128098930098176, 2305878468463689728, 1152939783987658752,
9799982666336960512, 4899991333168480256, 2449995666584240128, 1224997833292120064,
576469569871282176, 288234782788157440, 4620693356194824192, 11533718717099671552,
5802888705324613632, 2901444352662306816, 1450722176331153408, 725361088165576704,
362539804446949376, 145241105196122112, 18049583422636032, 45053588738670592, 22667534005174272,
11333767002587136, 5666883501293568, 2833441750646784, 1416171111120896, 567348067172352,
70506185244672, 175990581010432, 88545054707712, 44272527353856, 22136263676928, 11068131838464,
5531918402816, 2216203387392, 275414786112, 687463207072, 345879119952, 172939559976, 86469779988,
43234889994, 21609056261, 8657044482, 1075839008, 2685403152, 1351090312, 675545156, 337772578,
168886289, 84410376, 33816580, 4202496, 10489856, 5277696, 2638848, 1319424, 659712, 329728, 132096,];

//only used to gen for hardcoded moves
pub fn gen_knight_moves() {
    let move_set = vec![(2,1), (1,2), (-2,-1), (-1,-2), (-2,1), (-1,2), (2,-1), (1,-2)];
    for i in (0..64).rev() { 
        let mut moves: u64 = 0;
        let file = i % 8;
        let rank = 7 - ((i - file) / 8);

        for &(x, y) in &move_set {
            if file + x >= 0 && file + x < 8 && rank + y >= 0 && rank + y < 8 {
                moves |= 1 << ((7 - (rank + y)) * 8 + file + x);
            }
        }
        println!("{},", moves);
    }
}

const ROOK_MAGICS: [u64; 64] = [0; 64];
const BISHOP_MAGICS: [u64; 64] = [0; 64]; 

const GEN_MAGICS: bool = true;

#[derive(Copy, Clone)]
struct SMagic {
    mask: u64,
    magic: u64,
    shift: u8,
}

//space inefficient, may improve later
pub struct MagicBoards {
    rook_magic_table: Vec<SMagic>,
    bishop_magic_table: Vec<SMagic>,
    moveboards_rook: Vec<Vec<u64>>,
    moveboards_bishop: Vec<Vec<u64>>,
}

impl MagicBoards{
    fn blockermask_rook(square: usize) -> u64 {
        let mut blockermask: u64 = 0;

        let file = square % 8;
        let rank = (square - file) / 8;

        for i in (rank + 1)..7         { blockermask |= 1 << (file + i * 8) };
        for i in (0..(rank - 1)).rev() { blockermask |= 1 << (file + i * 8) };
        for i in (file + 1)..7         { blockermask |= 1 << (i + rank * 8) };
        for i in (0..(file - 1)).rev() { blockermask |= 1 << (i + rank * 8) };

        blockermask
    }

    fn blockermask_bishop(square: usize) -> u64 {
        let mut blockermask: u64 = 0;

        let file = square % 8;
        let rank = (square - file) / 8;

        for x in (file + 1)..7 {
            for y in (rank + 1)..7 {
                blockermask |= 1 << (x + y * 8);
            }
        }

        for x in (0..(file - 1)).rev() {
            for y in (rank + 1)..7 {
                blockermask |= 1 << (x + y * 8);
            }
        }

        for x in (file + 1)..7 {
            for y in (0..(rank - 1)).rev() {
                blockermask |= 1 << (x + y * 8);
            }
        }

        for x in (0..(file - 1)).rev() {
            for y in (0..(rank - 1)).rev() {
                blockermask |= 1 << (x + y * 8);
            }
        }

        blockermask
    } 

    fn moveboard_rook(square: usize, blockerboard: u64) -> u64 {
        let mut moveboard = 0;

        let file = square % 8;
        let rank = (square - file) / 8;

        for i in (file + 1)..7 {
            let check: u64 = 1 << (rank * 8 + i);

            moveboard |= check;
            if (check & blockerboard) != 0 {
                break; 
            }
        }

        for i in (0..(file - 1)).rev() {
            let check: u64 = 1 << (rank * 8 + i);

            moveboard |= check;
            if (check & blockerboard) != 0 {
                break; 
            }
        }

        for i in (rank + 1)..7 {
            let check: u64 = 1 << (i * 8 + file);

            moveboard |= check;
            if (check & blockerboard) != 0 {
                break; 
            }
        }

        for i in (0..(rank - 1)).rev() {
            let check: u64 = 1 << (i * 8 + file);

            moveboard |= check;
            if (check & blockerboard) != 0 {
                break; 
            }
        }

        moveboard
    }

    fn moveboard_bishop(square: usize, blockerboard: u64) -> u64 {
        let mut moveboard: u64 = 0;

        let file = square % 8;
        let rank = (square - file) / 8;

        for x in (file + 1)..7 {
            for y in (rank + 1)..7 {
                let check: u64 = 1 << (y * 8 + x);

                moveboard |= check;
                if (check & blockerboard) != 0 {
                    break;
                }
            }
        }

        for x in (0..(file - 1)).rev() {
            for y in (rank + 1)..7 {
                let check: u64 = 1 << (y * 8 + x);

                moveboard |= check;
                if (check & blockerboard) != 0 {
                    break;
                }
            }
        }

        for x in (file + 1)..7 {
            for y in (0..(rank - 1)).rev() {
                let check: u64 = 1 << (y * 8 + x);

                moveboard |= check;
                if (check & blockerboard) != 0 {
                    break;
                }
            }
        }

        for x in (0..(file - 1)).rev() {
            for y in (0..(rank - 1)).rev() {
                let check: u64 = 1 << (y * 8 + x);

                moveboard |= check;
                if (check & blockerboard) != 0 {
                    break;
                }
            }
        }

        moveboard
    }

    fn blockerboard(index: u32, blockermask: u64) -> u64 {
        let mut blockerboard = blockermask;

        let mut bits: u8 = 0;
        for i in 0..64 {
            if (blockermask & (1 << i)) != 0 {
                if (index as u64 & (1 << bits)) == 0 {
                    blockerboard &= !(1 << i);
                }
                bits += 1;
            }
        }

        blockerboard
    }

    fn hamming_weight(i: u64) -> u64 {
        let mut j = i;
        j = j - ((j >> 1) & 0x5555555555555555);
        j = (j & 0x3333333333333333) + ((j >> 2) & 0x3333333333333333);
        (((j + (j >> 4)) & 0x0F0F0F0F0F0F0F0F) * 0x0101010101010101) >> 24
    }

    fn find_magic(&mut self, square: usize, rook: bool) {
        let mut rng = thread_rng();

        let bits;
        if rook {
            self.rook_magic_table[square].mask = MagicBoards::blockermask_rook(square);
            bits = MagicBoards::hamming_weight(self.rook_magic_table[square].mask);
        }
        else {
            self.bishop_magic_table[square].mask = MagicBoards::blockermask_bishop(square);
            bits = MagicBoards::hamming_weight(self.bishop_magic_table[square].mask);
        }

        let mut b: [u64; 4096] = [0; 4096];
        let mut m: [u64; 4096] = [0; 4096];
        if rook {
            for i in 0..(1 << bits) {
                b[i] = MagicBoards::blockerboard(i as u32, self.rook_magic_table[square].mask);
                m[i] = MagicBoards::moveboard_rook(square, b[i]);
            }
        }
        else {
            for i in 0..(1 << bits) {
                b[i] = MagicBoards::blockerboard(i as u32, self.bishop_magic_table[square].mask);
                m[i] = MagicBoards::moveboard_bishop(square, b[i]);
            }
        }

        let mut u: [u64; 4096] = [0; 4096];

        for i in 0..100000000 {
            let magic: u64;

            if GEN_MAGICS {
                magic = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
            }
            else {
                if rook {
                    magic = ROOK_MAGICS[square];
                }
                else {
                    magic = BISHOP_MAGICS[square];
                }
            }

            let mut fail = false;
            for j in 0..(1 << bits) {
                let index: usize = ((b[j] * magic) >> (64 - bits)) as usize;
                if u[index] == 0 {
                    u[index] = m[j];
                }
                else if u[index] != m[j] {
                    fail = true;
                    if !GEN_MAGICS {
                        panic!("Precalculated magic incorrect. Square: {0}, Piece: {1}", square, rook);
                    }
                    break;
                }
            }
            if !fail {
                if rook {
                    self.rook_magic_table[square].magic = magic;
                    self.rook_magic_table[square].shift = (64 - bits) as u8;
                    self.moveboards_rook[square] = u.to_vec();
                }
                else {
                    self.bishop_magic_table[square].magic = magic;
                    self.bishop_magic_table[square].shift = (64 - bits) as u8;
                    self.moveboards_bishop[square] = u.to_vec();
                }
                if GEN_MAGICS {
                    println!("{}", magic);
                }
                break;
            }
        }
    }

    pub fn gen_magics() {
        let mut magic_boards: MagicBoards = MagicBoards { 
            rook_magic_table: vec![SMagic { mask: 0, magic: 0, shift: 0 }; 64],
            bishop_magic_table: vec![SMagic { mask: 0, magic: 0, shift: 0 }; 64],
            moveboards_rook: vec![vec![0; 4096]; 64],
            moveboards_bishop: vec![vec![0; 4096]; 64],
        };

        /*for i in 0..63 {
            magic_boards.find_magic(i, true);
        }
        for i in 0..63 {
            magic_boards.find_magic(i, false);
        }
        */
        //magic_boards
    }
}
