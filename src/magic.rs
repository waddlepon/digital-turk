use rand::{thread_rng, Rng};

const KNIGHT_MOVES: [u64; 64] = [
    9077567998918656,
    4679521487814656,
    38368557762871296,
    19184278881435648,
    9592139440717824,
    4796069720358912,
    2257297371824128,
    1128098930098176,
    2305878468463689728,
    1152939783987658752,
    9799982666336960512,
    4899991333168480256,
    2449995666584240128,
    1224997833292120064,
    576469569871282176,
    288234782788157440,
    4620693356194824192,
    11533718717099671552,
    5802888705324613632,
    2901444352662306816,
    1450722176331153408,
    725361088165576704,
    362539804446949376,
    145241105196122112,
    18049583422636032,
    45053588738670592,
    22667534005174272,
    11333767002587136,
    5666883501293568,
    2833441750646784,
    1416171111120896,
    567348067172352,
    70506185244672,
    175990581010432,
    88545054707712,
    44272527353856,
    22136263676928,
    11068131838464,
    5531918402816,
    2216203387392,
    275414786112,
    687463207072,
    345879119952,
    172939559976,
    86469779988,
    43234889994,
    21609056261,
    8657044482,
    1075839008,
    2685403152,
    1351090312,
    675545156,
    337772578,
    168886289,
    84410376,
    33816580,
    4202496,
    10489856,
    5277696,
    2638848,
    1319424,
    659712,
    329728,
    132096,
];

//only used to gen for hardcoded moves
pub fn gen_knight_moves() {
    let move_set = vec![
        (2, 1),
        (1, 2),
        (-2, -1),
        (-1, -2),
        (-2, 1),
        (-1, 2),
        (2, -1),
        (1, -2),
    ];
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

const KING_MOVES: [u64; 64] = [
    4665729213955833856,
    11592265440851656704,
    5796132720425828352,
    2898066360212914176,
    1449033180106457088,
    724516590053228544,
    362258295026614272,
    144959613005987840,
    13853283560024178688,
    16186183351374184448,
    8093091675687092224,
    4046545837843546112,
    2023272918921773056,
    1011636459460886528,
    505818229730443264,
    216739030602088448,
    54114388906344448,
    63227278716305408,
    31613639358152704,
    15806819679076352,
    7903409839538176,
    3951704919769088,
    1975852459884544,
    846636838289408,
    211384331665408,
    246981557485568,
    123490778742784,
    61745389371392,
    30872694685696,
    15436347342848,
    7718173671424,
    3307175149568,
    825720045568,
    964771708928,
    482385854464,
    241192927232,
    120596463616,
    60298231808,
    30149115904,
    12918652928,
    3225468928,
    3768639488,
    1884319744,
    942159872,
    471079936,
    235539968,
    117769984,
    50463488,
    12599488,
    14721248,
    7360624,
    3680312,
    1840156,
    920078,
    460039,
    197123,
    49216,
    57504,
    28752,
    14376,
    7188,
    3594,
    1797,
    770,
];

//only used to gen hard coded moves
pub fn gen_king_moves() {
    let move_set = vec![
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
    ];
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

const ROOK_MAGICS: [u64; 64] = [
    1157990257514250242,
    2882875507851132992,
    1161398080045057,
    4899934021695569920,
    2594095375631523840,
    9241387612189753344,
    7097690677972336640,
    4647714958296093184,
    9016021654700032,
    4701969254651068416,
    13837362910931730432,
    5189272756673331201,
    1125968694608904,
    148618925230329856,
    4621256446842109952,
    79657506474835968,
    1376805519364,
    180148384225592324,
    176059302608900,
    2305844108995919872,
    138043457536,
    36064532220805120,
    9223373137473732608,
    8806880773632,
    2253041596235780,
    4582774129295360,
    1126466851028992,
    2305984984461082624,
    1153000673756315652,
    13835093445825267714,
    4612249801671770112,
    3485857047268886016,
    2305984201976971264,
    9227950403281256456,
    4507997675978752,
    4582910495817728,
    2019864570318946305,
    4415293493248,
    88513774126080,
    595056242712444928,
    71064866652168,
    9081970877530116,
    1172114828951552,
    286431520030720,
    2306142097855741984,
    2306002455579852800,
    2882310358603857924,
    4755802308166631424,
    2342153556693894144,
    885420464866112,
    144150922491068448,
    18288575778848,
    1152941433322226688,
    2306969212453652480,
    1441154081141850368,
    6598143542400,
    721418207646212,
    18296595300356,
    4656735245365690372,
    8899241574404,
    4612812056042012676,
    18159121858644,
    87969528611848,
    9223373138279023620,
];

const BISHOP_MAGICS: [u64; 64] = [
    9523160548515053568,
    9376784978861375492,
    589415583842304,
    1171221798688129538,
    36108237304498690,
    185219906429517824,
    564668948021248,
    1152921651039780940,
    90142370049427456,
    4683964073145204736,
    10017136023911809024,
    4612134638515716228,
    1161428950077568,
    9295452737921613824,
    4614500777053126656,
    2314851310264123920,
    4612811978632593408,
    2323857450807214080,
    4503702719184896,
    288230376843776002,
    1152921522005345409,
    11710484931338830082,
    347905304622792714,
    1125917359931392,
    14074024033320960,
    5193794412542525444,
    108719712988340232,
    73324231449919785,
    577094071034589312,
    937100572673458176,
    4802675380392017,
    11529777996303110160,
    27586747014578176,
    288555866135724088,
    1300559529548840960,
    1153062929374117940,
    25473622864830468,
    99171550778900480,
    4611969692444131328,
    82190968077422601,
    2254028943761476,
    11529928234049208384,
    9229848546900967424,
    1126045935738880,
    13907186020211754496,
    2318230116039131136,
    4836902369587494928,
    9225633768782495744,
    9223936297880518656,
    9226822484750237696,
    10415126226150440,
    54043205238276100,
    4611826893388255881,
    2306019034186910216,
    92358981363695616,
    580967792216899716,
    2882393410386526232,
    18067454375067648,
    306246494418632720,
    4617456337059315744,
    360297969142071304,
    9223486661277646848,
    149920196626432,
    90076425054331904,
];


const GEN_MAGICS: bool = false;

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

impl MagicBoards {
    fn blockermask_rook(square: usize) -> u64 {
        let mut blockermask: u64 = 0;

        let file = square % 8;
        let rank = (square - file) / 8;

        for i in (rank + 1)..7 {
            blockermask |= 1 << (file + i * 8)
        }
        if rank != 0 {
            for i in 0..(rank - 1) {
                blockermask |= 1 << (file + i * 8)
            }
        }
        for i in (file + 1)..7 {
            blockermask |= 1 << (i + rank * 8)
        }
        if file != 0 {
            for i in 0..(file - 1) {
                blockermask |= 1 << (i + rank * 8)
            }
        }

        blockermask
    }

    fn blockermask_bishop(square: usize) -> u64 {
        let mut blockermask: u64 = 0;

        let file = square % 8;
        let rank = (square - file) / 8;

        let mut x = file + 1;
        let mut y = rank + 1;
        while x <= 6 && y <= 6 {
            blockermask |= 1 << (x + y * 8);
            x += 1;
            y += 1;
        }

        if rank != 0 {
            x = file + 1;
            y = rank - 1;
            while x <= 6 && y >= 1 {
                blockermask |= 1 << (x + y * 8);
                x += 1;
                y -= 1;
            }
        }

        if file != 0 {
            x = file - 1;
            y = rank + 1;
            while x >= 1 && y <= 6 {
                blockermask |= 1 << (x + y * 8);
                x -= 1;
                y += 1;
            }
        }

        if file != 0 && rank != 0 {
            x = file - 1;
            y = rank - 1;
            while x >= 1 && y >= 1 {
                blockermask |= 1 << (x + y * 8);
                x -= 1;
                y -= 1;
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

        if file != 0 {
            for i in (0..(file - 1)).rev() {
                let check: u64 = 1 << (rank * 8 + i);

                moveboard |= check;
                if (check & blockerboard) != 0 {
                    break;
                }
            }
        }

        for i in (rank + 1)..7 {
            let check: u64 = 1 << (i * 8 + file);

            moveboard |= check;
            if (check & blockerboard) != 0 {
                break;
            }
        }

        if rank != 0 {
            for i in (0..(rank - 1)).rev() {
                let check: u64 = 1 << (i * 8 + file);

                moveboard |= check;
                if (check & blockerboard) != 0 {
                    break;
                }
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

        if file != 0 {
            for x in (0..(file - 1)).rev() {
                for y in (rank + 1)..7 {
                    let check: u64 = 1 << (y * 8 + x);

                    moveboard |= check;
                    if (check & blockerboard) != 0 {
                        break;
                    }
                }
            }
        }

        for x in (file + 1)..7 {
            if rank != 0 {
                for y in (0..(rank - 1)).rev() {
                    let check: u64 = 1 << (y * 8 + x);

                    moveboard |= check;
                    if (check & blockerboard) != 0 {
                        break;
                    }
                }
            }
        }

        if file != 0 {
            for x in (0..(file - 1)).rev() {
                if rank != 0 {
                    for y in (0..(rank - 1)).rev() {
                        let check: u64 = 1 << (y * 8 + x);

                        moveboard |= check;
                        if (check & blockerboard) != 0 {
                            break;
                        }
                    }
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

    fn find_magic(&mut self, square: usize, rook: bool) {
        let mut rng = thread_rng();

        let bits;
        if rook {
            self.rook_magic_table[square].mask = MagicBoards::blockermask_rook(square);
            bits = self.rook_magic_table[square].mask.count_ones();
        } else {
            self.bishop_magic_table[square].mask = MagicBoards::blockermask_bishop(square);
            bits = self.bishop_magic_table[square].mask.count_ones();
        }

        let mut b: [u64; 4096] = [0; 4096];
        let mut m: [u64; 4096] = [0; 4096];
        if rook {
            for i in 0..(1 << bits) {
                b[i] = MagicBoards::blockerboard(i as u32, self.rook_magic_table[square].mask);
                m[i] = MagicBoards::moveboard_rook(square, b[i]);
            }
        } else {
            for i in 0..(1 << bits) {
                b[i] = MagicBoards::blockerboard(i as u32, self.bishop_magic_table[square].mask);
                m[i] = MagicBoards::moveboard_bishop(square, b[i]);
            }
        }

        let mut u: [u64; 4096];

        loop {
            let magic: u64;

            if GEN_MAGICS {
                magic = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
                if ((magic.wrapping_mul(self.rook_magic_table[square].mask)) >> 56).count_ones() < 6
                {
                    continue;
                }
            } else {
                if rook {
                    magic = ROOK_MAGICS[square];
                } else {
                    magic = BISHOP_MAGICS[square];
                }
            }

            u = [0; 4096];

            let mut fail = false;
            for j in 0..(1 << bits) {
                let index: usize = ((b[j].wrapping_mul(magic)) >> (64 - bits)) as usize;
                if u[index] == 0 {
                    u[index] = m[j];
                } else if u[index] != m[j] {
                    fail = true;
                    if !GEN_MAGICS {
                        panic!(
                            "Precalculated magic incorrect. Square: {0}, Piece: {1}",
                            square,
                            rook
                        );
                    }
                    break;
                }
            }
            if !fail {
                if rook {
                    self.rook_magic_table[square].magic = magic;
                    self.rook_magic_table[square].shift = (64 - bits) as u8;
                    self.moveboards_rook[square] = u.to_vec();
                } else {
                    self.bishop_magic_table[square].magic = magic;
                    self.bishop_magic_table[square].shift = (64 - bits) as u8;
                    self.moveboards_bishop[square] = u.to_vec();
                }
                if GEN_MAGICS {
                    println!("{},", magic);
                }
                break;
            }
        }
    }

    pub fn gen_magics() -> Self {
        let mut magic_boards: MagicBoards = MagicBoards {
            rook_magic_table: vec![
                SMagic {
                    mask: 0,
                    magic: 0,
                    shift: 0,
                };
                64
            ],
            bishop_magic_table: vec![
                SMagic {
                    mask: 0,
                    magic: 0,
                    shift: 0,
                };
                64
            ],
            moveboards_rook: vec![vec![0; 4096]; 64],
            moveboards_bishop: vec![vec![0; 4096]; 64],
        };

        if GEN_MAGICS {
            println!("Rook Magics:");
        }
        for i in 0..64 {
            magic_boards.find_magic(i, true);
        }
        if GEN_MAGICS {
            println!("\nBishop Magics:");
        }
        for i in 0..64 {
            magic_boards.find_magic(i, false);
        }

        magic_boards
    }

    pub fn magic_move_rook(&self, square: usize, occupancy: u64) -> u64 {
        let index = ((occupancy & self.rook_magic_table[square].mask) *
            self.rook_magic_table[square].magic) >>
            self.rook_magic_table[square].shift;

        self.moveboards_rook[square][index as usize]
    }

    pub fn magic_move_bishop(&self, square: usize, occupancy: u64) -> u64 {
        let index = ((occupancy & self.bishop_magic_table[square].mask) *
            self.bishop_magic_table[square].magic) >>
            self.bishop_magic_table[square].shift;

        self.moveboards_bishop[square][index as usize]
    }
}
