fn main() {
    let mut sortable = vec![12, 11, 13, 5, 19, 189, 38, 18, 37, 60, 90, 6, 7];
    let length = sortable.len();
    sortable::merge_sort(&mut sortable, 0, length - 1);
    println!("{:?}", sortable);
}

mod sortable {
    fn merge(data: &mut Vec<i32>, left: usize, middle: usize, right: usize) {
        let size_left = middle - left + 1;
        let size_rigth = right - middle;

        let mut left_data = vec![0; size_left];
        let mut right_data = vec![0; size_rigth];

        for i in 0..size_left {
            left_data[i] = data[left + i];
        }

        for i in 0..size_rigth {
            right_data[i] = data[middle + 1 + i];
        }

        let mut i = 0;
        let mut j = 0;
        let mut k = left;

        while i < size_left && j < size_rigth {
            if left_data[i] <= right_data[j] {
                data[k] = left_data[i];
                i += 1;
            } else {
                data[k] = right_data[j];
                j += 1;
            }
            k += 1;
        }

        while i < size_left {
            data[k] = left_data[i];
            i += 1;
            k += 1;
        }

        while j < size_rigth {
            data[k] = right_data[j];
            j += 1;
            k += 1;
        }
    }

    pub(crate) fn merge_sort(arr: &mut Vec<i32>, start: usize, end: usize) {
        if start < end {
            let mid = (start + end) / 2;
            merge_sort(arr, start, mid);
            merge_sort(arr, mid + 1, end);
            merge(arr, start, mid, end);
        }
    }
}
