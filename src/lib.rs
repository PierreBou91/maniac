mod comb_sorter;
mod standard_sorter;

pub trait Sorter<T>
where
    T: Ord,
{
    fn sort(self, slice: &mut [T]);
}
