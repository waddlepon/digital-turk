pub fn bit_indexes(n: u64) -> Vec<u32> {
    let mut set_bits = Vec::new();
    for i in 0..64 {
        let check = 1 << i;
        if (n & check) > 0 {
            set_bits.push(i as u32);
        }
    }
    set_bits
}
