fn main() {}

pub fn find_different_binary_string_cantor(nums: Vec<String>) -> String {
    nums.iter()
        .enumerate()
        .map(
            |(idx, num)| match num.chars().nth(idx).expect("Failed to get at index") {
                '1' => '0',
                _ => '1',
            },
        )
        .collect()
}

fn find_different_binary_string_my_solution(nums: Vec<String>) -> String {
    let items: Vec<_> = nums
        .iter()
        .map(|n| u64::from_str_radix(&n, 2).expect("Failed to read nums"))
        .collect();

    for n in 0..u64::MAX {
        if !items.contains(&n) {
            return format!("{:0width$b}", n, width = nums[0].len());
        };
    }

    panic!("Unable to find result");
}
