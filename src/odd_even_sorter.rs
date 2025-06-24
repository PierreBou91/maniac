// use std::fmt::Display;

use super::Sorter;

/// Doc
#[derive(Debug, Default, Clone, Copy)]
pub struct OddEvenSorter {
    parallel: bool,
}

impl<T: Ord> Sorter<T> for OddEvenSorter {
    fn sort(self, slice: &mut [T]) {
        if self.parallel {
            // get the number of CPU
            // divide the lengh of the array by 2
            // divide this number by the number of CPU
            // if we have 100 numbers and 3 CPU:
            // on first pass:
            // CPU 0 will have pairs 0 3 6 9... => [0 1] [6 7] [12 13] [18 19]... => 1 7 13 19 etc.
            // CPU 1 will have pairs 1 4 7 10... => [2 3] [8 9] [14 15] [20 21]... => 3 9 15 21 etc.
            // CPU 2 will have pairs 2 5 8 11... => [4 5] [10 11] [16 17] [22 23]... => 5 11 17 23 etc.
            let lenght = 100;
            let mut cpus = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

            let mut i = 1;

            while i < lenght {
                for cpu in 0..cpus.len() {
                    if !(i < lenght) {
                        break;
                    }
                    cpus[cpu].push(i);
                    i += 2;
                }
            }
            println!("{cpus:#?}");
        } else {
            let slice_len = slice.len();
            /*
            Create pairs of odd-even index:
            [1-2] [3-4] [5-6] ...
            swap these if necessary
            do the same with even-odd pairs => 1 [2-3] [4-5] 6
            if among these 2 passes there were 0 swap then it's sorted
            */
            let mut sorted = false;
            while !sorted {
                sorted = true;

                for i in (1..slice_len).filter(|val| val % 2 != 0) {
                    if slice[i - 1] > slice[i] {
                        slice.swap(i - 1, i);
                        sorted = false
                    }
                }

                for i in (1..slice_len).filter(|val| val % 2 == 0) {
                    if slice[i - 1] > slice[i] {
                        slice.swap(i - 1, i);
                        sorted = false
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::{sorter_common_tests, sorter_stability_tests};

    #[test]
    fn debug() {
        let mut slice = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        // println!("{slice:?}");
        let sorter = OddEvenSorter { parallel: true };
        sorter.sort(&mut slice);
        assert!(false);
    }

    // sorter_common_tests!(odd_even_common, OddEvenSorter { parallel: false });
    // sorter_stability_tests!(odd_even_stability, OddEvenSorter { parallel: false });
}
