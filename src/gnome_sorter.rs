use super::Sorter;

/// [`GnomeSorter`]
///
/// [*Gnome sort*](https://en.wikipedia.org/wiki/Gnome_sort)—also known as *stupid sort* or the
/// *Dutch garden gnome algorithm*—is conceptually similar to insertion sort but uses only adjacent
/// swaps and a single index that moves **backward when a swap is made** and **forward otherwise**:
///
/// 1. Start at position `1`.
/// 2. If the current element is larger / equal to the one on its left, advance one step.
/// 3. Otherwise swap them and step **back** one position.
/// 4. Stop when the index moves past the end of the slice.
///
/// Because every swap moves an element exactly one slot, the algorithm is
/// **stable**, needs no extra memory and is easy to implement with a single
/// while-loop.  
///
/// ### Complexity
/// | Best                  | Average | Worst | Space | In-place | Stable |
/// |-----------------------|---------|-------|-------|----------|--------|
/// | *O(n)* (already sorted) | *O(n²)* | *O(n²)* | *O(1)* | Yes | Yes |
#[derive(Debug, Default, Clone, Copy)]
pub struct GnomeSorter;

impl<T: Ord> Sorter<T> for GnomeSorter {
    fn sort(self, slice: &mut [T]) {
        let slice_len = slice.len();
        let mut position = 1;

        while position < slice_len {
            if position == 0 || slice[position] >= slice[position - 1] {
                position += 1;
            } else {
                slice.swap(position, position - 1);
                position -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{sorter_common_tests, sorter_stability_tests};

    sorter_common_tests!(gnome_common, GnomeSorter);
    sorter_stability_tests!(gnome_stable, GnomeSorter);
}
