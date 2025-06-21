use super::Sorter;
pub struct StandardSorter;

impl<T: Ord> Sorter<T> for StandardSorter {
    fn sort(self, slice: &mut [T]) {
        slice.sort();
    }
}

#[cfg(test)]
mod test {
    use crate::{Sorter, standard_sorter::StandardSorter};

    #[test]
    fn test_standard_sorter() {
        let sorter = StandardSorter;
        let mut slice = [1, 2, 3, 5, 4];
        sorter.sort(&mut slice);
        assert_eq!(slice, [1, 2, 3, 4, 5]);
    }
}
