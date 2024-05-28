use core::num;
use std::vec;

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();

    let mut resuslt: Vec<Vec<i32>> = vec![];
    for first_index in 0..(nums.len() - 2) {
        if first_index > 0 && nums[first_index] == nums[first_index - 1] {
            continue;
        }

        for second_index in (first_index + 1)..(nums.len() - 1) {
            if second_index > first_index + 1 && nums[second_index] == nums[second_index - 1] {
                continue;
            }

            let mut third_index = nums.len() - 1;
            while second_index < third_index
                && nums[first_index] + nums[second_index] + nums[third_index] > 0
            {
                third_index -= 1;
            }

            if third_index == second_index {
                break;
            }

            if nums[first_index] + nums[second_index] + nums[third_index] == 0 {
                resuslt.push(vec![
                    nums[first_index],
                    nums[second_index],
                    nums[third_index],
                ]);
            }
        }
    }

    resuslt
}
fn main() {
    let result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
    println!("{:?}", result);
}
