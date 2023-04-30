mod known;

pub fn get_components(num: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut remainder = num;
    let mut i = 2;

    while i <= (num / 2 + 1) {
        if remainder % i != 0 {
            i += 1;
            continue;
        }

        factors.push(i);
        remainder /= i;
        i = 2;
    }

    factors
}

pub fn get_prime_factors(num: u32) -> Vec<u32> {
    let mut factors = Vec::from([]);

    if is_prime(num) {
        factors.push(num);
    } else {
        for i in 2..=(num / 2 + 1) {
            if is_prime(i) && num % i == 0 {
                factors.push(i);
            }
        }
    }

    factors
}

pub fn is_prime(num: u32) -> bool {
    if num <= known::MAX {
        return known::is_prime(num);
    }

    for i in 2..=(num / 2 + 1) {
        if num % i == 0 {
            return false;
        }
    }
    false
}
