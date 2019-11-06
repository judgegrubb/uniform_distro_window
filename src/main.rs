use rand::Rng;
use std::collections::HashMap;

const NUM_CHUNKS: i32 = 8;
const MIN_SIZE: i32 = 8;
const MAX_SIZE: i32 = 2048;

fn main() {

    let mut over_num = MIN_SIZE;

    let mut median: HashMap<String, f32> = HashMap::new();
    let mut mean: HashMap<String, f32> = HashMap::new();
    let mut variance: HashMap<String, f32> = HashMap::new();

    while over_num <= MAX_SIZE {
       
        let mut max_counts: Vec<Vec<i32>> = Vec::new();
        for _ in 0..NUM_CHUNKS {
            max_counts.push(Vec::new());
        }
        println!("Testing {}", over_num);

        for _ in 1..1000 {
            // okay assume that main is some set of power of 2 length m
            // consisting of the numbers 1..m inclusive

            // now uniformly sample m numbers from the set and place them in a new array

            let mut arr: Vec<i32> = Vec::new();

            for _ in 1..(over_num+1) {
                arr.push(rand::thread_rng().gen_range(1, over_num+1));
            }

            arr.sort();

            let mut max = 0;
            let mut cur_chunk = 1;

            for (i, num) in arr.iter().enumerate() {
                let key = num - (i as i32) - 1;
                let key = key.abs();
                if key > max {
                    max = key;
                }
                if (i as i32 + 1) == (cur_chunk * over_num) / NUM_CHUNKS {
                    max_counts[cur_chunk as usize - 1].push(max);
                    max = 0;
                    cur_chunk += 1;
                }
            }
        }
   
        // find mean and variance and add it to overall map
        for (i, counts) in max_counts.iter().enumerate() {
            let (sum, dist_sum) = mean_and_variance(counts);
            
            mean.insert(format!("{}chunk{}", over_num, i+1), sum);
            println!("mean for {} chunk {}: {}", over_num, i+1, sum);
            
            variance.insert(format!("{}chunk{}", over_num, i+1), dist_sum);
            println!("variance for {} chunk {}: {}", over_num, i+1, dist_sum);
        }

        // find median and add it to overall map
        for (i, counts) in max_counts.iter().enumerate() {
            let median_val: f32 = median_vec(counts);
            
            median.insert(format!("{}chunk{}", over_num, i+1), median_val);

            println!("median for {} chunk {}: {}", over_num, i+1, median_val);
        }

        over_num = over_num * 2;
    }
    
    println!("Mean: {:?}", mean);
    println!("Median: {:?}", median);
    println!("Variance: {:?}", variance);
}

fn mean_and_variance(counts: &Vec<i32>) -> (f32, f32) {
    let sum: i32 = counts.iter().sum();
    let sum: f32 = sum as f32 / counts.len() as f32;
    
    // now variance
    let mut dist_sum: f32 = 0.0;
    for x in counts.iter() {
        dist_sum = dist_sum + ((*x as f32 - sum).powi(2));
    }
    dist_sum = dist_sum / (counts.len() - 1) as f32;
    
    // if you prefer std deviation...
    //dist_sum = dist_sum.sqrt();
    
    (sum, dist_sum)
}

fn median_vec(c: &Vec<i32>) -> f32 {
    let mut counts = c.to_vec();
    counts.sort();
    let mid = counts.len() / 2;
    if counts.len() % 2 == 0 {
        (counts[mid - 1] as f32 + counts[mid] as f32) / 2 as f32
    } else {
        counts[mid] as f32
    }
}

