// Find number of negative values in sorted matrix (descending order along both axis).
// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/
//-----------------------------------------------------------------------------------
// Ω(n) O(n log m) | O(1)
pub mod count_negatives {
    pub fn run(grid: Vec<Vec<i32>>) -> u32 {
        let (height,width) = (grid.len(),grid[0].len());
        let mut neg_space = 0usize;
        for y in 0..grid.len() {
            println!("{:.?}",bin_search_smallest_negative(&grid[y]));
            if let Some(index) = bin_search_smallest_negative(&grid[y]) {
                neg_space += width-index;
                println!("{}",width-index);
            }
        }
        println!("neg_space:{}",neg_space);
        return neg_space as u32;
    
        // Inner logic seems bit excessive here
        // Returns index of smallest positive number from descending order vector
        fn bin_search_smallest_negative(vec:&Vec<i32>) -> Option<usize> {
            let (mut left, mut right) = (0usize,vec.len()-1);
            let mut index = usize::default();
            while left != right {
                index = left + (right-left) / 2;

                if vec[index] < 0 {
                    if index == 0 { return Some(0); }
                    if vec[index-1] > 0 { return Some(index); }
                    //if vec[index-1] >= 0 { return Some(index); }
                    right = index - 1;
                }
                else {
                    left = index + 1;
                }
            }
            if vec[left] < 0 { return Some(left); }
            return None;
        }
    }
}

// Given 2 equal length strings s,t composed of lowercase english characters,
//  find the minimum number of steps to make t an anagram of s.
//  A step is changing a letter in t.
// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/
//-----------------------------------------------------------------------------------
// I am certain this can be improved (Perhaps not Onotationally, but practically).
// O(n+k) | O(k)
// where k=26 (number of lowercase englsih characters) and n = length of s and t.
pub mod min_steps_to_make_strings_anagram {
    // O(n) | O(k)
    // Where k = number of different values in `nums`.
    pub fn run(s:String,t:String) -> u32 {
        let s_chars:&[u8] = s.as_bytes();
        let t_chars:&[u8] = t.as_bytes();
    
        // O(n) | O(k)
        let mut counts:Vec<i32> = vec!(0i32;26usize);
        for i in 0..s_chars.len() {
            counts[s_chars[i] as usize - 97usize] += 1;
            counts[t_chars[i] as usize - 97usize] -= 1;
        }
    
        // O(26) | O(1)
        let mut count = 0u32;
        for i in 0..counts.len() {
            if counts[i] > 0 {
                count += counts[i] as u32;
            }
        }
        return count;
    }
    
}

// Get indicies of 2 integers which sum to `target`
// https://leetcode.com/problems/two-sum/
//-----------------------------------------------------------------------------------
pub mod two_sum {
    use std::collections::HashMap;
    // O(n) | O(k)
    // Where k = number of different values in `nums`.
    pub fn unordered(nums: &[i16], target: i16) -> Option<(usize,usize)> {
        let mut values:HashMap<i16,usize> = HashMap::new();
        for i in 0..nums.len() {
            if let Some(lookup) = target.checked_sub(nums[i]) {
                if let Some(value) = values.get(&lookup) {
                    return Some((*value,i));
                }
            }
            values.insert(nums[i],i);
        }
        return None; // Cannot find 2 values which equal `target`
    }
    // `nums` is assumed ordered.
    // O(n) | O(2)
    pub fn ordered(nums: &[i16], target: i16) -> Option<(usize,usize)> {
        // O(n) | O(2)
        let mut low:usize = 0usize;
        let mut high:usize = nums.len()-1usize;
        while high != low {
            if let Some(val) = nums[low].checked_add(nums[high]) {
                if val == target { return Some((low,high)); }
                else if val > target { high -= 1; }
                else { low += 1; } // val < target
            }
        }
        return None; // Cannot find 2 values which equal `target`
    }
    
}

// (god this question is badly formed, and the marking for Rust doesn't even work on Leetcode)
// Allows tweets to be added with a time specified in seconds.
// Allow the user to retrieve the number of tweets in each interval (intervals can be minutes/hours/days) 
//  from a start time until an end time.
// An example output: [3,4,5]:
//     3 tweets in 1st interval (start time -> interval period),
//     4 tweets in 2nd interval,
//     5 tweets in 3rd interval (this last interval will likely be less than the set interval period, since it ends at `end_time` regardless of interval)

// https://leetcode.com/problems/tweet-counts-per-frequency/
//-----------------------------------------------------------------------------------
// We could insert tweets inorder O(log n) (binary search) for each insertion,
//  or we can sort tweets when `get_tweet_counts_per_frequency` called O(n log n),
//  presuming tweets will be inserted vastly more (more than n) than `get_tweet_counts_per_frequency` is called,
//  I have chosen the 2nd approach. (n refers to the number of recordings under each tweet)
pub mod tweet_counts {
    use std::collections::HashMap;
    pub enum Frequency {
        Minute,Hour,Day
    }
    pub struct TweetCounts {
        tweet_times:HashMap<String,Vec<u32>>,
    }
    impl TweetCounts {
        // O(1)
        pub fn new() -> Self {
            TweetCounts { tweet_times:HashMap::new() }
        }
        // O(1)
        pub fn record_tweet(&mut self,tweet_name:String,time:u32) {
            if let Some(recordings) = self.tweet_times.get_mut(&tweet_name) {
                recordings.push(time);
            }
            else {
                self.tweet_times.insert(tweet_name,vec![time]);
            }
        }
        // O(n lon n + n) | O(n/2)
        pub fn get_tweet_counts_per_frequency(&mut self,freq:Frequency,tweet_name:&str,start_time:u32,end_time:u32) -> Option<Vec<u32>> {
            if let Some(recordings) = self.tweet_times.get_mut(tweet_name) {
                // If so `sec_frequency` can be u32 and we can adjust some casts.
                let sec_frequency:u32 =  match freq {
                    Frequency::Minute => 60,
                    Frequency::Hour => 3600,
                    Frequency::Day => 86400,
                };
    
                // O(n log n) | O(n/2)
                recordings.sort();
    
                // TODO This could be improved. Firstly don't do the interval calculation every element.
                // O(n)
                let mut start_element = 0usize;
                while start_time > recordings[start_element] {
                    start_element += 1;
                    if start_element == recordings.len() { break; }
                }
                let intervals = ((end_time-start_time) / sec_frequency) as usize + 1;
                let mut frequencies = vec!(0u32;intervals);
                for i in start_element..recordings.len() {
                    let interval = ((recordings[i] - start_time) / sec_frequency) as usize;
                    if interval >= intervals { break; }
                    frequencies[interval] += 1;
                }
    
                return Some(frequencies);
            }
            else {
                return None;
            }
        }
    }
}

// Check if there exists 2 integers N,M where N = 2 * M
// https://leetcode.com/problems/check-if-n-and-its-double-exist/
//-----------------------------------------------------------------------------------
// O(n) | O(n)
pub mod double_exists {
    use std::collections::HashMap;
    // O(n) | O(k)
    // Where k = number of different values in `nums`.
    pub fn run(arr:Vec<i16>) -> bool {
        let mut doubles:HashMap<i16,usize> = HashMap::new();
        for i in 0..arr.len() {
            if let Some(val) = arr[i].checked_mul(2i16) {
                doubles.insert(val,i);
            }
        }

        // O(n)
        for i in 0..arr.len() {
            if let Some(index) = doubles.get(&arr[i]) {
                // Accounts for 0, value cannot be double of itself, but a 0 can be double of another 0.
                if *index != i { return true; }
            }
        }
        return false;
    }
    
}
