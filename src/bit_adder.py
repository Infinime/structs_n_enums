import unittest
def exor(a: int, b: int):
    if (a != 0 and a != 1) or (b != 0 and b != 1):
        raise("Why are your bits so large?")
    if a == b:
        return 0
    else:
        return 1

def bit_adder(a, b):
    # step 1: exor a and b
    if len(a) < len(b):
        diff = len(b) - len(a)
        a = [0]*diff + a

    elif len(b) < len(a):
        diff = len(a) - len(b)
        b = [0]*diff + b

    sum = []
    prevcarry = 0
    for p,q in list(zip(a, b))[::-1]:
        s = exor(p, q)
        c = p and q or prevcarry
        s = exor(s, prevcarry)
        sum.insert(0, s)
        prevcarry = c
    if prevcarry:
        sum.insert(0, prevcarry)
    return sum

class TestBitAdder(unittest.TestCase):
    def test_bit_adder_large_arrays(self):
        a = [1] * 1000
        b = [0] * 1000
        self.assertEqual(bit_adder(a, b), [1] * 1000)

    def test_bit_adder_large_arrays_with_carry(self):
        a = [1] * 1000
        b = [1] * 1000
        self.assertEqual(bit_adder(a, b), a + [0])

    def test_bit_adder_empty_arrays(self):
        a = []
        b = []
        self.assertEqual(bit_adder(a, b), [])

    def test_bit_adder_single_element(self):
        a = [1]
        b = [0]
        self.assertEqual(bit_adder(a, b), [1])

    def test_bit_adder_invalid_input(self):
        with self.assertRaises(Exception):
            bit_adder([2], [1])  # Invalid binary input

    def test_bit_adder_mismatched_lengths(self):
        a = [1, 0, 1]
        b = [1, 1]
        self.assertEqual(bit_adder(a, b), [1,0,0,0])

    def test_bit_adder_stress_test(self):
        a = [1] * 10_000
        b = [1] * 10_000
        self.assertEqual(bit_adder(a, b), a + [0])

    def test_bit_adder_basic(self):
        a = [1, 0, 0, 1]
        b = [1, 1, 1, 1]
        self.assertEqual(bit_adder(a, b), [1, 1, 0, 0, 0])

    def test_bit_adder_all_zeros(self):
        a = [0, 0, 0, 0]
        b = [0, 0, 0, 0]
        self.assertEqual(bit_adder(a, b), [0, 0, 0, 0])

    def test_bit_adder_all_ones(self):
        a = [1, 1, 1, 1]
        b = [0, 0, 0, 0]
        self.assertEqual(bit_adder(a, b), [1, 1, 1, 1])

    def test_bit_adder_alternating_bits(self):
        a = [1, 0, 1, 0]
        b = [0, 1, 0, 1]
        self.assertEqual(bit_adder(a, b), [1, 1, 1, 1])

    def test_bit_adder_mixed_bits(self):
        a = [1, 1, 0, 1]
        b = [1, 0, 1, 0]
        self.assertEqual(bit_adder(a, b), [1, 0, 1, 1, 1])

    def test_bit_adder_carry_over(self):
        a = [1, 1, 1, 1]
        b = [1, 1, 1, 1]
        self.assertEqual(bit_adder(a, b), [1, 1, 1, 1, 0])

if __name__ == "__main__":
    unittest.main()