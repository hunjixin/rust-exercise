use std::vec;

pub fn find_mid_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let len1 = nums1.len();
    let len2 = nums2.len();
    let mid_pos = (len1 + len2) / 2;
    let is_odd = (len1 + len2) % 2 == 1;

    let mut offset1 = 0;
    let mut offset2 = 0;
    let mut cur: i32 = 0;
    for index in 0..(len1 + len2) {
        let mut last = 0;
        if offset1 < len1 && offset2 < len2 {
            if nums1[offset1] > nums2[offset2] {
                last = cur;
                cur = nums2[offset2];
                offset2 += 1;
            } else {
                last = cur;
                cur = nums1[offset1];
                offset1 += 1;
            }
        } else if offset1 < len1 {
            last = cur;
            cur = nums1[offset1];
            offset1 += 1;
        } else {
            last = cur;
            cur = nums2[offset2];
            offset2 += 1;
        }

        if index == mid_pos {
            if is_odd {
                return cur as f64;
            } else {
                return (cur + last) as f64 / 2f64;
            }
        }
    }
    return 0f64;
}

fn main() {
    let arr1 = vec![1, 2, 3];
    let arr2 = vec![4, 5, 6];
    let result = find_mid_sorted_arrays(arr1, arr2);
    println!("result {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arr_fn() {
        let arr1 = vec![1, 2, 3];
        let arr2 = vec![4, 5, 6];
        let result = find_mid_sorted_arrays(arr1, arr2);
        println!("result {result}");
        assert_eq!(3.5f64, result);
    }

    #[test]
    fn test_arr2_fn() {
        let arr1 = vec![1, 2, 2];
        let arr2 = vec![2, 2, 6];
        let result = find_mid_sorted_arrays(arr1, arr2);
        println!("result {result}");
        assert_eq!(2f64, result);
    }
    #[test]
    fn test_arr3_fn() {
        let arr1 = vec![1, 3, 4];
        let arr2 = vec![5, 6, 7, 8, 9, 10, 11, 12, 13];
        let result = find_mid_sorted_arrays(arr1, arr2);
        println!("result {result}");
        assert_eq!(7.5f64, result);
    }
}
