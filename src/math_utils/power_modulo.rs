pub fn power_modulo(mut base: u64, mut exp: u64, modular: u64) -> u64 {
    let mut res = 1;
    base = base.rem_euclid(modular);

    while exp > 0 {
        if exp & 1 == 1 {
            res = (res * base).rem_euclid(modular);
        }
        exp >>= 1;
        base = (base * base).rem_euclid(modular);
    }
    res
}