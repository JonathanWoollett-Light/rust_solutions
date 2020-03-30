#[cfg(test)]
mod tests {
    use rand::{thread_rng,Rng};
    use std::time::Instant;

    use rust_solutions::*;

    const SIZE:usize = 10000usize;

    #[test]
    fn two_sum() {
        let mut rng = thread_rng();
        // A random list of integers.
        let mut arr = [0i16;SIZE];
        rng.fill(&mut arr[..]);
        // An ordered random list of integers.
        let mut sorted_arr = [0i16;SIZE];
        rng.fill(&mut sorted_arr[..]);
        sorted_arr.sort();
        // A random integer.
        let target:i16 = rng.gen();

        // two_sum_unordered
        print!("two_sum_unordered:");
        let start = Instant::now();
        let result = two_sum::unordered(&arr,target);
        println!(" {}ms, {:.?}",start.elapsed().as_millis(),result);
        // two_sum_ordered
        print!("two_sum_ordered:");
        let start = Instant::now();
        let result = two_sum::ordered(&sorted_arr,target);
        println!(" {}ms, {:.?}",start.elapsed().as_millis(),result);
    }
    #[test]
    fn double_exists() {
        let start = Instant::now();
        let result = double_exists::run(vec![-2i16,0i16,10i16,-19i16,4i16,6i16,-8i16]); // TODO Could this be generalized?
        println!(" {}ms, {:.?}",start.elapsed().as_millis(),result);
    }
    #[test]
    fn min_steps_to_make_strings_anagram() {
        let start = Instant::now();
        let result = min_steps_to_make_strings_anagram::run("friend".to_owned(),"family".to_owned());
        println!(" {}ms, {:.?}",start.elapsed().as_millis(),result);
    }
    #[test]
    fn tweet_counts() {
        let start = Instant::now();
        let mut tweet_counts = tweet_counts::TweetCounts::new();
        tweet_counts.record_tweet("tweet0".to_owned(), 33); // TODO Fix function so the awkward '.to_owned()' is not neccessary.
        tweet_counts.record_tweet("tweet1".to_owned(), 34);
        tweet_counts.record_tweet("tweet2".to_owned(), 36);
        tweet_counts.record_tweet("tweet3".to_owned(), 59);
        tweet_counts.record_tweet("tweet4".to_owned(), 64);
        tweet_counts.record_tweet("tweet2".to_owned(), 48);
        tweet_counts.record_tweet("tweet4".to_owned(), 21);
        tweet_counts.record_tweet("tweet2".to_owned(), 44);
        tweet_counts.record_tweet("tweet2".to_owned(), 74);
        let result = tweet_counts.get_tweet_counts_per_frequency(tweet_counts::Frequency::Minute,"tweet4",64,2783);
        print!(" {}ms, {:.?}",start.elapsed().as_millis(),result);
        let result = tweet_counts.get_tweet_counts_per_frequency(tweet_counts::Frequency::Hour,"tweet4",64,2783);
        print!(" | {}ms, {:.?}",start.elapsed().as_millis(),result);
        let result = tweet_counts.get_tweet_counts_per_frequency(tweet_counts::Frequency::Day,"tweet4",64,2783);
        println!(" | {}ms, {:.?}",start.elapsed().as_millis(),result);
    }
    #[test]
    fn count_negatives() {
        let start = Instant::now();
        // let result = count_negatives::run(vec![
        //     vec![2],
        // ]);
        // let result = count_negatives::run(vec![
        //     vec![4,3,2,-1],
        //     vec![3,2,1,-1],
        //     vec![1,1,-1,-2],
        //     vec![-1,-1,-2,-3]
        // ]);
        let result = count_negatives::run(vec![
            vec![ 10,  9,  8,  8,  8,  7,  6,  6,  5,  5,  4,  3,  2,  1],
            vec![ 10,  9,  8,  8,  8,  7,  6,  6,  5,  5,  4,-13,-14,-15],
            vec![ 10,  9,  7,  7,  7,  7,  5,  4,  3,  2,  1,-13,-15,-15],
            vec![ 10,  9,  7,  6,  6,  5,  4,  4,  2,  2,  0,-14,-16,-16],
            vec![  9,  9,  7,  5,  4,  4,  4,  4,  1,  1, -1,-15,-16,-16],
            vec![  9,  8,  7,  4,  3,  2,  1,  1,-15,-15,-16,-17,-18,-18],
            vec![  9,  7,  7,  3,  2,  2,-20,-20,-20,-20,-20,-20,-20,-20],
            vec![  8,  7,  6,  3,  2,  1,-20,-20,-20,-20,-20,-20,-20,-20],
            vec![  8,  6,  6,-16,-16,-16,-20,-20,-20,-20,-20,-20,-20,-20],
            vec![-19,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20],
            vec![-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20],
            vec![-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20],
            vec![-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20],
            vec![-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20],
            vec![-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20],
            vec![-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20,-20]
        ]);
        println!(" {}ms, {:.?}",start.elapsed().as_millis(),result);

        assert_eq!(result,144)
    }
}