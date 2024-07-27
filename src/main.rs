use num_bigint::BigUint;
use num_traits::{One, Zero};

fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    let mut x = a.clone();
    let mut y = b.clone();
    while y != BigUint::zero() {
        let temp = y.clone();
        y = x.clone() % y.clone();
        x = temp;
    }
    x
}

fn order_of_element(a: &BigUint, n: &BigUint) -> Option<BigUint> {
    if gcd(a, n) != BigUint::one() {
        return None; // Element `a` is not coprime with `n`, so it doesn't have an order in the group.
    }

    let mut k = BigUint::one();
    let mut power = a.clone() % n.clone();
    while power != BigUint::one() {
        power = (power * a.clone()) % n.clone();
        k += BigUint::one();
    }
    Some(k)
}

fn main() {
    let n = BigUint::parse_bytes(b"24", 10).unwrap(); // Modulus

    // Generate a range of elements up to a specified value
    let max_element = 24; // Small value for demonstration
    let elements: Vec<BigUint> = (1..=max_element as u32).map(|x| BigUint::from(x)).collect();

    for element in &elements {
        match order_of_element(&element, &n) {
            Some(order) => println!("The order of {} in Z_{} is {}", element, n, order),
            None => println!("Element {} is not coprime with {}", element, n),
        }
    }
}




