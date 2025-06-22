use super::Sorter;

/// [`CombSorter`]
///
/// The [comb sort](https://en.wikipedia.org/wiki/Comb_sort) is essentially a bubble sort but instead
/// of comparing an item at index i with the item at index i+1, there is a wider gap between two
/// compared items.
///
/// This gap is the ratio between the lenght of the sorted slice and shrink factor.
///
/// The shrink factor is arbitrarily decided by the implementor, some benchmark have empirically
/// determined 1.3 to be close to the optimal value. You'll find more intersting facts on the wiki
/// page.
///
/// Every time the algorithm finishes an iteration the gap is shrinked again by the same factor.
///
/// One notable aspect of this algorithm is the "rule of 11" described as such on the [wikipedia
/// page](https://en.wikipedia.org/wiki/Comb_sort)
/// > One additional refinement suggested by Lacey and Box is the "rule of 11": always use a gap
/// > size of 11, rounding up gap sizes of 9 or 10 (reached by dividing gaps of 12, 13 or 14 by 1.3)
/// > to 11. This eliminates turtles surviving until the final gap-1 pass.
///
/// Complexity:
/// |Best|Average|Worst|Space|In-place|
/// |---|---|---|---|---|
/// |n log n|n<sup>2</sup>|n<sup>2</sup>|1|Yes|
pub struct CombSorter {
    // TODO: determine wether it should be public, or at least give a `with` constructor or builder
    // pattern
    shrink_factor: f32,
}

impl Default for CombSorter {
    fn default() -> Self {
        Self { shrink_factor: 1.3 }
    }
}

// TODO: arrange for handling of precision loss when casting from usize to f32. This is relevant for
// very large arrays.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
impl<T: Ord> Sorter<T> for CombSorter {
    fn sort(self, slice: &mut [T]) {
        let slice_len = slice.len();
        let mut gap = slice_len;
        let mut sorted = false;
        while !sorted {
            gap = (gap as f32 / self.shrink_factor).floor() as usize;
            if gap <= 1 {
                gap = 1;
                sorted = true;
            } else if gap == 9 | 10 {
                // The aforementioned rule of 11
                gap = 11;
            }
            let mut iteration = 0;
            while gap + iteration < slice_len {
                if slice[iteration] > slice[iteration + gap] {
                    slice.swap(iteration, iteration + gap);
                    sorted = false;
                }
                iteration += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Sorter, comb_sorter::CombSorter};

    #[test]
    fn test_comb_sorter() {
        let sorter = CombSorter::default();
        let mut slice = [1, 2, 3, 5, 4];
        println!("{slice:?}");
        sorter.sort(&mut slice);
        assert_eq!(slice, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_comb_sorter_strings() {
        let sorter = CombSorter::default();
        let mut slice = ["z", "d", "d", "q", "l", "a"];
        println!("{slice:?}");
        sorter.sort(&mut slice);
        assert_eq!(slice, ["a", "d", "d", "l", "q", "z"]);
    }
}
