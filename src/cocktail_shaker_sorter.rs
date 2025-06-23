use super::Sorter;

/// [`CocktailShakerSorter`]
///
/// The [*cocktail shaker sort*](https://en.wikipedia.org/wiki/Cocktail_shaker_sort) (also called
/// **bidirectional bubble sort** or *shuttle sort*) is a variant of bubble sort that traverses the
/// slice in **both directions** on each pass:
///
/// 1.  A **left-to-right scan** “bubbles” the largest unsorted element to the end of the working
/// interval.
/// 2.  A **right-to-left scan** then “sinks” the smallest element back to the front.
///
/// After every pair of scans the working interval shrinks to the range `left..right` between the
/// last swap positions, so fewer comparisons are made than in the plain bubble sort.
///
/// The algorithm is still *O(n²)* in the average and worst case but performs better on data with
/// large elements already near their final positions.  It is **stable**, needs only *O(1)* extra
/// space and can serve as a simple in-place two-way pass for small inputs.
///
/// ### Complexity
/// | Best                  | Average | Worst | Space | In-place | Stable |
/// |-----------------------|---------|-------|-------|----------|--------|
/// | *O(n)* (already sorted) | *O(n²)* | *O(n²)* | *O(1)* | Yes | Yes |
#[derive(Debug, Default, Clone, Copy)]
pub struct CocktailShakerSorter;

impl<T: Ord> Sorter<T> for CocktailShakerSorter {
    fn sort(self, slice: &mut [T]) {
        let slice_len = slice.len();
        let mut left = 1;
        let mut right = slice_len;
        let mut sorted = false;

        while !sorted {
            sorted = true;
            for i in left..right {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    right = i;
                    sorted = false;
                }
            }
            for i in (left..right).rev() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    left = i;
                    sorted = false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{sorter_common_tests, sorter_stability_tests};

    sorter_common_tests!(cocktail_common, CocktailShakerSorter);
    sorter_stability_tests!(cocktail_stability, CocktailShakerSorter);
}
