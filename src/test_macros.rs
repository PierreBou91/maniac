#![cfg(test)]

/// Runs basic correctness checks on a sorter type.
///
/// ```
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///     use crate::sorter_common_tests;
///
///     sorter_common_tests!(InsertionSorter);
/// }
/// ```
#[macro_export]
macro_rules! sorter_common_tests {
    ($mod_name:ident, $ctor:expr) => {
        mod $mod_name {
            use super::*;
            use std::fmt::Debug;
            use $crate::Sorter;

            fn run_sort<T: Ord + Debug + Clone>(mut data: Vec<T>) -> Vec<T> {
                let mut expected = data.clone();
                expected.sort();

                let sorter = $ctor;
                sorter.sort(&mut data);
                assert_eq!(data, expected);

                data
            }

            #[test]
            fn empty_slice() {
                run_sort::<i32>(vec![]);
            }
            #[test]
            fn single_element() {
                run_sort(vec![42]);
            }
            #[test]
            fn already_sorted() {
                run_sort(vec![1, 2, 3, 4, 5]);
            }
            #[test]
            fn reverse_sorted() {
                run_sort(vec![5, 4, 3, 2, 1]);
            }
            #[test]
            fn duplicates() {
                run_sort(vec![3, 1, 2, 3, 2, 1]);
            }
            #[test]
            fn strings() {
                run_sort(vec!["z", "d", "q", "a"]);
            }
        }
    };
}

/// Runs stability checks on a sorter type.
/// Invoke this only for algorithms that are supposed to be stable.
///
/// ```
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///     use crate::sorter_stability_tests;
///
///     sorter_stability_tests!(InsertionSorter);
/// }
/// ```
#[macro_export]
macro_rules! sorter_stability_tests {
    ($mod_name:ident, $ctor:expr) => {
        mod $mod_name {
            use super::*;
            use $crate::Sorter;

            #[derive(Clone, Debug, Eq, PartialEq)]
            struct Item {
                key: i32,
                tag: usize,
            }

            impl Ord for Item {
                fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
                    self.key.cmp(&rhs.key)
                }
            }
            impl PartialOrd for Item {
                fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.cmp(rhs))
                }
            }

            #[test]
            fn preserves_relative_order() {
                let mut data = vec![
                    Item { key: 2, tag: 0 },
                    Item { key: 1, tag: 1 },
                    Item { key: 2, tag: 2 },
                    Item { key: 1, tag: 3 },
                ];

                let expected = vec![
                    Item { key: 1, tag: 1 },
                    Item { key: 1, tag: 3 },
                    Item { key: 2, tag: 0 },
                    Item { key: 2, tag: 2 },
                ];

                let sorter = $ctor;
                sorter.sort(&mut data);
                assert_eq!(data, expected);
            }
        }
    };
}
