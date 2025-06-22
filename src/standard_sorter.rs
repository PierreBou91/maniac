use super::Sorter;
pub struct StandardSorter;

impl<T: Ord> Sorter<T> for StandardSorter {
    fn sort(self, slice: &mut [T]) {
        slice.sort();
    }
}
