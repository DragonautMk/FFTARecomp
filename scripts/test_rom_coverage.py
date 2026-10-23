"""ROM-free checks for atlas accounting and classification boundaries."""
import random
import unittest

import rom_coverage as coverage


def points(ranges):
    return {i for a, b in ranges for i in range(a, b)}


class CoverageAccountingTests(unittest.TestCase):
    def test_interval_accounting_matches_independent_byte_sets(self):
        rng = random.Random(932)
        for _ in range(250):
            def sample():
                return [[a, rng.randrange(a + 1, 65)]
                        for a in (rng.randrange(64) for _ in range(rng.randrange(20)))]
            a, b = sample(), sample()
            self.assertEqual(points(coverage.union(a)), points(a))
            self.assertEqual(coverage.length(a), len(points(a)))
            self.assertEqual(points(coverage.intersection(a, b)), points(a) & points(b))
            unknown = coverage.complement(a + b, 64)
            self.assertEqual(points(unknown), set(range(64)) - points(a + b))
            self.assertEqual(coverage.length(a + b) + coverage.length(unknown), 64)

    def test_distinct_resource_owners_not_duplicate_ranges(self):
        owners = {'font': [[0, 20], [0, 20], [5, 15]],
                  'text': [[10, 30]], 'ui': [[15, 25], [25, 32]]}
        shared = {i for i in range(40) if sum(i in points(s) for s in owners.values()) >= 2}
        self.assertEqual(points(coverage.multiple_owners(owners)), shared)
        self.assertEqual(coverage.multiple_owners({'one': [[0, 20], [0, 20]]}), [])

    def test_endpoints_are_exclusive_and_adjacent_ranges_do_not_overlap(self):
        self.assertEqual(coverage.intersection([[0, 4]], [[4, 8]]), [])
        self.assertEqual(coverage.union([[0, 4], [4, 8]]), [[0, 8]])
        self.assertEqual(coverage.complement([[0, 4], [8, 12]], 12), [[4, 8]])
        self.assertEqual(coverage.multiple_owners({'a': [[0, 4]], 'b': [[4, 8]]}), [])

    def test_bad_boundaries_are_rejected_not_clipped_or_inflated(self):
        for span in ((-1, 4), (4, 4), (5, 4), (0, 65)):
            with self.assertRaises(ValueError):
                coverage.checked_span(*span, 64)
        with self.assertRaises(ValueError):
            coverage.complement([[60, 66]], 64)


if __name__ == '__main__':
    unittest.main()
