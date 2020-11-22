use super::Sorter;

pub struct BubbleSort;


impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord
    {
        // Implementation of bubblesort starts here
        // check element[index] and element[index+1]
        // Redorder if needed
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..slice.len()-1 {
                if slice[i] > slice[i+1] {
                    slice.swap(i, i+1);
                    swapped = true;
                }
            }
        }
    }
}


#[test]
fn it_works() {
    use super::Sorter;

    let mut unsorted_vec = [18, 2, 1, 4, 9];
    BubbleSort::sort(&mut unsorted_vec);
    assert_eq!(unsorted_vec, [1, 2, 4, 9, 18]);
}
