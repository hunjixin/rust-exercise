pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() == 0 {
        return 0;
    }
    let mut min = prices[0];
    let mut max_fit = 0;
    for index in 1..prices.len() {
        if prices[index] < min {
            min = prices[index];
        } else {
            let new_profit = prices[index] - min;
            if new_profit > max_fit {
                max_fit = new_profit;
            }
        }
    }

    max_fit
}
fn main() {
    println!("Hello, world! {}", max_profit(vec![7, 6, 4, 3, 1]));
}
