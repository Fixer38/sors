use super::Sorter;

pub struct SelectionSort;


// Beneficial of selectionsort is not using additional memory
impl Sorter for SelectionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord
    {
        // Find smallest element of the unsorted list, stick it to the front
        // Repeat the process

        for unsorted in 0..slice.len() {
            // returns index starting at unsorted
            // We need the index starting from the entire slice
            let (mut smallest_in_unordered, _) = slice[unsorted..]
                .iter()
                .enumerate()
                // Fish out data by value and not key
                // closure from enumerate being k,v
                // ignore the key and find min by value _, v
                .min_by_key(|&(_, v)| v)
                .expect("Slice is non empty");

            smallest_in_unordered = smallest_in_unordered + unsorted;
            
            // OR slow implementation 
            
            let mut smallest_in_unordered_2 = unsorted;
            for i in unsorted..slice.len() {
                if slice[i] < slice[smallest_in_unordered_2] {
                    smallest_in_unordered_2 = i;
                }
            }

            assert_eq!(smallest_in_unordered, smallest_in_unordered_2);
            if unsorted != smallest_in_unordered_2 {
                slice.swap(unsorted, smallest_in_unordered_2);
            }
        }
    }
}


#[test]
fn it_works() {
    let mut unsorted_vec = [18, 2, 1, 4, 9];
    SelectionSort::sort(&mut unsorted_vec);
    assert_eq!(unsorted_vec, [1, 2, 4, 9, 18]);
}
