use super::Sorter;

pub struct QuickSort;


fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1)
            }
            return;
        }
        _ => {}
    }

    // split slice into [ slice containing pivot | rest of slice ]
    // allows us to get the pivot but still modify the slice
    // the pivot is always on the left side
    let (pivot, rest) = slice.split_first_mut().expect("Slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;

    // keep going until the slice is seperated into left and right
    // left variable being the length of the left part
    // right variable being the length of the right path
    while left <= right {
        if &rest[left] <= pivot {
            // Already on the left side, stays in place
            left += 1;
        }
        else if &rest[right] > pivot {
            // last element can get into the right side
            // because bigger than pivot
            // Avoid unecessary back and forth
            right -= 1;
        }
        else {
            // Move element to the right side
            rest.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    // Realign left to take pivot into account
    let left = left + 1;

    // Place the pivot to its final position to avoid using the same pivot
    slice.swap(0, left - 1);

    // split_at_mut -> [..mid], [mid..]
    let (left, right) = slice.split_at_mut(left - 1);
    quicksort(left);
    // right[1..] to avoid using the pivot
    quicksort(&mut right[1..]);
    // Then merge both together
}

impl Sorter for QuickSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord
    {
        // [ unsorted | pivot | unsorted ]
        quicksort(slice);
    }
}


#[test]
fn it_works() {
    let mut unsorted_vec = [18, 2, 1, 4, 9];
    QuickSort::sort(&mut unsorted_vec);
    assert_eq!(unsorted_vec, [1, 2, 4, 9, 18]);
}
