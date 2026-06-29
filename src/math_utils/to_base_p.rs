pub fn to_base_p(mut n: u64, p: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    if p < 2 {
        return vec![];
    }

    let mut digits = Vec::new();

    while n > 0 {
        let remainder = n % p;
        digits.push(remainder);
        n /= p;
    }

    digits.reverse();

    digits
}