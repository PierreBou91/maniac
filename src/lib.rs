#![warn(
    clippy::correctness,
    clippy::pedantic,
    clippy::suspicious,
    clippy::complexity,
    clippy::style,
    clippy::cargo,
    clippy::perf,
    missing_docs
)]
//! # Maniac
//! The maniac crate implements various sorting algorithms. Most implementations are based on the
//! wikipedia page of each specific algorithm.

/// [`CombSorter`] is the implementation of the [comb sort algorithm](https://en.wikipedia.org/wiki/Comb_sort)
pub mod comb_sorter;
pub use comb_sorter::*;
/// [`InsertionSorter`] is the implementation of the [insertion sort algorithm](https://en.wikipedia.org/wiki/Insertion_sort)
pub mod insertion_sorter;
pub use insertion_sorter::*;
/// [`BubbleSorter`] is the implementation of the [bubble sort algorithm](https://en.wikipedia.org/wiki/Bubble_sort)
pub mod bubble_sorter;
pub use bubble_sorter::*;
/// [`CocktailShakerSorter`] is the implementation of the [cocktail shaker sort](https://en.wikipedia.org/wiki/Cocktail_shaker_sort)
pub mod cocktail_shaker_sorter;
pub use cocktail_shaker_sorter::*;
/// [`GnomeSorter`] is the implementation of the [gnome sort](https://en.wikipedia.org/wiki/Gnome_sort)
pub mod gnome_sorter;
pub use gnome_sorter::*;

mod standard_sorter;
mod test_macros;

/// The [`Sorter`] trait is the only trait that is required to be implemented by any sorter.
/// The only expected method is the sort method that is generic over a T where T is Ord.
pub trait Sorter<T>
where
    T: Ord,
{
    /// This method is the implementation of the sorting algorithm
    fn sort(self, slice: &mut [T]);
}
