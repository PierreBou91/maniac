use super::Sorter;

/// [`InsertionSorter`]
///
/// The classic [*insertion sort*](https://en.wikipedia.org/wiki/Insertion_sort) builds the result
/// incrementally: it scans the slice from left to right and inserts each element into the
/// already-sorted prefix that lies to its left.  After processing position `i` the sub-slice
/// `[..=i]` is guaranteed to be in order.
///
/// This implementation lets you choose **how** the out-of-place element is moved into position via
/// the [`Method`] field:
///
/// | Method                     | Stable | Inner data move                | Remarks                                      |
/// |----------------------------|:------:|--------------------------------|----------------------------------------------|
/// | [`Method::RotateStable`]   | ✅     | `rotate_right(1)` after *upper-bound* search | Each element moved exactly once; fastest **and** stable. |
/// | [`Method::RotateUnstable`] | ❌     | `rotate_right(1)` after *lower-bound* search | Slightly fewer comparisons, but equal keys can swap order. |
/// | [`Method::Swap`]           | ✅     | Classic adjacent swaps          | Text-book variant; simpler but 2 × more element moves. |
///
/// ### Complexity
/// | Best | Average | Worst | Space | In-place | Stable |
/// | ---- | ------- | ----- | ----- | -------- | ------ |
/// | *O(n)* (already sorted) | *O(n²)* | *O(n²)* | *O(1)* | Yes | see table above |
///
#[derive(Debug, Default, Clone, Copy)]
pub struct InsertionSorter {
    /// Strategy used to move the current element into its final position.
    pub method: Method,
}

/// Movement strategy for [`InsertionSorter`].
///
/// See the comparison table in [`InsertionSorter`] for details.
#[derive(Debug, Default, Clone, Copy)]
pub enum Method {
    /// Bulk-move the block `[idx..=i]` one position to the right **after** the *upper-bound*,
    /// preserving the order of elements that compare equal to the key (⚠️ *stable*).
    #[default]
    RotateStable,
    /// Rotate after the *lower-bound*; the key lands **before** equal elements, so the
    /// implementation is *not stable* but can be marginally faster.
    RotateUnstable,
    /// Swap neighbours until the key is in place – the traditional insertion-sort loop.
    /// Still stable, yet it writes each moved element twice.
    Swap,
}

impl<T: Ord> Sorter<T> for InsertionSorter {
    fn sort(self, slice: &mut [T]) {
        let slice_len = slice.len();
        match self.method {
            Method::RotateStable => {
                for i in 1..slice.len() {
                    let key_pos = i;
                    let idx = slice[..i].partition_point(|x| x <= &slice[key_pos]);

                    if idx != i {
                        slice[idx..=i].rotate_right(1);
                    }
                }
            }
            Method::RotateUnstable => {
                for i in 1..slice_len {
                    let index = match slice[..i].binary_search(&slice[i]) {
                        Ok(i) | Err(i) => i,
                    };
                    if index != i {
                        slice[index..=i].rotate_right(1);
                    }
                }
            }
            Method::Swap => {
                for i in 1..slice_len {
                    let mut j = i;
                    while j > 0 && slice[j - 1] > slice[j] {
                        slice.swap(j - 1, j);
                        j -= 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{sorter_common_tests, sorter_stability_tests};

    sorter_common_tests!(insertion_common_rotate_stable, InsertionSorter::default());
    sorter_common_tests!(
        insertion_common_rotate_unstable,
        InsertionSorter {
            method: Method::RotateUnstable
        }
    );
    sorter_common_tests!(
        insertion_common_swap,
        InsertionSorter {
            method: Method::Swap
        }
    );
    sorter_stability_tests!(
        insertion_stability_rotate_stable,
        InsertionSorter::default()
    );
    sorter_stability_tests!(
        insertion_stability_swap,
        InsertionSorter {
            method: Method::Swap
        }
    );
}
