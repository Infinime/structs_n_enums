fn exor(a: i32, b: i32) -> i32 {
    if (a != 0 && a != 1) || (b != 0 && b != 1) {
        panic!("Why are your binary numbers so large?")
    }
    if a == b {
        return 0;
    } else {
        return 1;
    }
}
fn bit_adder(a: &[i32], b: &[i32], n: usize) -> Vec<i32> {
    if a.len() != b.len() {
        panic!("Unequal lengths of bit arrays")
    }
    let mut sum: Vec<i32> = vec![];
    let mut prevcarry = 0;
    for i in (0..n).rev() {
        let c: i32 = a[i] & b[i] | prevcarry;
        let mut s: i32 = exor(a[i], b[i]);
        // dbg!(prevcarry);
        s = exor(s, prevcarry);
        // print!("s again: {}",s);
        sum.push(s);
        prevcarry = c;
    }
    if prevcarry == 1 {
        sum.push(prevcarry);
    }
    sum.into_iter().rev().collect()
}
fn main() {
    let a: [i32; 4] = [1, 0, 0, 1];
    let b: [i32; 4] = [1, 1, 1, 1];
    let n: usize = a.len();
    // dbg!(a);
    dbg!(bit_adder(&a, &b, n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_adder_large_arrays() {
        let a: Vec<i32> = vec![1; 1000];
        let b: Vec<i32> = vec![0; 1000];
        assert_eq!(bit_adder(&a, &b, a.len()), vec![1; 1000]);
    }

    #[test]
    fn test_bit_adder_large_arrays_with_carry() {
        let a: Vec<i32> = vec![1; 1000];
        let b: Vec<i32> = vec![1; 1000];
        let mut expected = vec![1; 1000];
        expected.push(0);
        assert_eq!(bit_adder(&a, &b, a.len()), expected);
    }

    #[test]
    fn test_bit_adder_empty_arrays() {
        let a: Vec<i32> = vec![];
        let b: Vec<i32> = vec![];
        assert_eq!(bit_adder(&a, &b, a.len()), vec![]);
    }

    #[test]
    fn test_bit_adder_single_element() {
        let a: Vec<i32> = vec![1];
        let b: Vec<i32> = vec![0];
        assert_eq!(bit_adder(&a, &b, a.len()), vec![1]);
    }

    #[test]
    #[should_panic(expected = "Why are your binary numbers so large?")]
    fn test_bit_adder_invalid_input() {
        let a: Vec<i32> = vec![2];
        let b: Vec<i32> = vec![1];
        bit_adder(&a, &b, a.len()); // Invalid binary input
    }

    #[test]
    fn test_bit_adder_padded_mismatched_lengths() {
        let a: Vec<i32> = vec![1, 0, 1];
        let mut b: Vec<i32> = vec![1, 1];
        while b.len() < a.len() {
            b.insert(0, 0); // Pad b with leading zeros
        }
        assert_eq!(bit_adder(&a, &b, a.len()), vec![1, 0, 0, 0]);
    }

    #[test]
    #[should_panic(expected = "Unequal lengths of bit arrays")]
    fn test_bit_adder_mismatched_lengths() {
        let a: Vec<i32> = vec![1, 0, 1];
        let b: Vec<i32> = vec![1, 1];
        assert_eq!(bit_adder(&a, &b, a.len()), vec![1, 0, 0, 0]);
    }

    #[test]
    fn test_bit_adder_stress_test() {
        let a: Vec<i32> = vec![1; 10_000];
        let b: Vec<i32> = vec![1; 10_000];
        let mut expected = vec![1; 10_000];
        expected.push(0);
        assert_eq!(bit_adder(&a, &b, a.len()), expected);
    }

    #[test]
    fn test_bit_adder_basic() {
        let a: Vec<i32> = vec![1, 0, 0, 1];
        let b: Vec<i32> = vec![1, 1, 1, 1];
        assert_eq!(bit_adder(&a, &b, a.len()), vec![1, 1, 0, 0, 0]);
    }

    #[test]
    fn test_bit_adder_all_zeros() {
        let a: Vec<i32> = vec![0, 0, 0, 0];
        let b: Vec<i32> = vec![0, 0, 0, 0];
        assert_eq!(bit_adder(&a, &b, a.len()), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_bit_adder_all_ones() {
        let a: Vec<i32> = vec![1, 1, 1, 1];
        let b: Vec<i32> = vec![0, 0, 0, 0];
        assert_eq!(bit_adder(&a, &b, a.len()), vec![1, 1, 1, 1]);
    }

    #[test]
    fn test_bit_adder_alternating_bits() {
        let a: Vec<i32> = vec![1, 0, 1, 0];
        let b: Vec<i32> = vec![0, 1, 0, 1];
        assert_eq!(bit_adder(&a, &b, a.len()), vec![1, 1, 1, 1]);
    }

    #[test]
    fn test_bit_adder_mixed_bits() {
        let a: Vec<i32> = vec![1, 1, 0, 1];
        let b: Vec<i32> = vec![1, 0, 1, 0];
        assert_eq!(bit_adder(&a, &b, a.len()), vec![1, 0, 1, 1, 1]);
    }

    #[test]
    fn test_bit_adder_carry_over() {
        let a: Vec<i32> = vec![1, 1, 1, 1];
        let b: Vec<i32> = vec![1, 1, 1, 1];
        assert_eq!(bit_adder(&a, &b, a.len()), vec![1, 1, 1, 1, 0]);
    }
}
