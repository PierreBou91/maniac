use super::Sorter;

/// [`CombSorter`]
pub struct CombSorter {
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
