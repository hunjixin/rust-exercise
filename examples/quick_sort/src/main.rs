use std::vec;


fn quick_sort_inner(arr: &mut [i32], left: usize, right: usize) {
    if left == right {
        return
    }
    let pivot =  arr[left];
    let mut i = left;
    let mut j = left;

    while i <= right {
        if arr[i] > pivot {
            arr.swap(i, j);
            j+=1;
        } 
        i+=1;
    }

    if j > left {
        quick_sort_inner(arr, left , j);
        quick_sort_inner(arr, j, right);
    }
}

fn quick_sort(arr:  &mut [i32],){
    let len = arr.len();
   quick_sort_inner(arr, 0, len-1);
}

fn main() {
   let mut arr = vec![3,2,1,5,6,7];
   quick_sort(arr.as_mut_slice());
   println!("{:?}", arr);
}
