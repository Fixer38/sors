pub mod bubblesort;
pub mod insertionsort;

pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where 
        T: Ord;
}


fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord
        {
            slice.sort();
        }
    }
    #[test]
    fn it_works() {
        let mut unsorted_vec = [18, 2, 1, 4];
        sort::<_, StdSorter>(&mut unsorted_vec);
        assert_eq!(unsorted_vec, [1, 2, 4, 18]);
    }
}
