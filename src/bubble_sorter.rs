use super::Sorter;

/// [`BubbleSorter`]
///
/// The classic [*bubble sort*](https://en.wikipedia.org/wiki/Bubble_sort) repeatedly “bubbles” the
/// largest remaining element to the end of the slice by swapping adjacent out-of-order pairs. After
/// pass *k* the last *k* items are in their final positions.
///
/// Bubble sort is prized for its extreme simplicity but runs in *O(n²)* time, so it is suitable
/// only for very small slices or as a teaching example.  It is, however, *stable* and needs no
/// extra memory.
///
/// This implementation supports three optimisation levels, chosen with the [`Optimization`] field:
///
/// | Variant | Stable | Pass length of next round | Remarks                                                  |
/// |---------|:------:|---------------------------|----------------------------------------------------------|
/// | `NewLen` *(default)* | ✅ | index of the **last swap** | Stops scanning where the suffix is already sorted – the popular Wikipedia improvement. |
/// | `LenMinusOne`        | ✅ | `n − 1`                   | Ignores the final item of each pass, which is guaranteed to be in place.               |
/// | `None`               | ✅ | full slice length         | Classic textbook algorithm; simplest but does the most work.                           |
///
/// ### Complexity
/// | Best | Average | Worst | Space | In-place | Stable |
/// | ---- | ------- | ----- | ----- | -------- | ------ |
/// | *O(n)* (already sorted) | *O(n²)* | *O(n²)* | *O(1)* | Yes | Yes |
#[derive(Debug, Default, Clone, Copy)]
pub struct BubbleSorter {
    /// Strategy that decides how far the next pass needs to scan.
    optimization: Optimization,
}

/// Pass-length optimisation strategies for [`BubbleSorter`].
///
/// All variants keep bubble sort’s canonical behaviour (*stable*, *in-place*), but trade a bit of
/// code complexity for fewer comparisons on average.
#[derive(Debug, Default, Clone, Copy)]
pub enum Optimization {
    /// **Wikipedia optimisation** – after a pass, the slice from the last swap to the end is
    /// already sorted, so the next pass can stop *earlier*.
    ///
    /// Gives the largest constant-factor speed-up and is therefore the default.
    #[default]
    NewLen,
    /// Discards only the *very last* element of the previous pass (which is now in its correct
    /// position). Simpler but less effective than `NewLen`.
    LenMinusOne,
    /// Performs a full pass over the slice every time – the traditional bubble sort found in
    /// introductory textbooks.
    None,
}

impl<T: Ord> Sorter<T> for BubbleSorter {
    fn sort(self, slice: &mut [T]) {
        let mut n = slice.len();

        while n > 1 {
            let mut last_swap = 0;
            for i in 1..n {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    last_swap = i;
                }
            }

            if last_swap == 0 {
                break;
            }

            n = match self.optimization {
                Optimization::NewLen => last_swap,
                Optimization::LenMinusOne => n - 1,
                Optimization::None => n,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{sorter_common_tests, sorter_stability_tests};

    sorter_common_tests!(
        bubble_common_new_len,
        BubbleSorter {
            optimization: Optimization::NewLen
        }
    );
    sorter_common_tests!(
        bubble_common_len_minus_one,
        BubbleSorter {
            optimization: Optimization::LenMinusOne
        }
    );
    sorter_common_tests!(
        bubble_common_no_optim,
        BubbleSorter {
            optimization: Optimization::None
        }
    );

    sorter_stability_tests!(
        bubble_stability_new_len,
        BubbleSorter {
            optimization: Optimization::NewLen
        }
    );
    sorter_stability_tests!(
        bubble_stability_len_minus_one,
        BubbleSorter {
            optimization: Optimization::LenMinusOne
        }
    );
    sorter_stability_tests!(
        bubble_stability_no_optim,
        BubbleSorter {
            optimization: Optimization::None
        }
    );
}
