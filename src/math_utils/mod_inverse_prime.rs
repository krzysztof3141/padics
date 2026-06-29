use crate::math_utils::power_modulo::power_modulo;

fn mod_inverse_prime(a: u64, p: u64) -> Option<u64> {
    if p <= 1 { return None; }
    if a.rem_euclid(p) == 0 { return None; }

    Some(power_modulo(a, p - 1, p) .rem_euclid(p))
}