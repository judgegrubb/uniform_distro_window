use rand::Rng;
use std::collections::HashMap;

const NUM_CHUNKS = 8;

fn main() {

    let mut over_num = 4096;

    let mut median: HashMap<String, f32> = HashMap::new();
    let mut mean: HashMap<String, f32> = HashMap::new();
    let mut variance: HashMap<String, f32> = HashMap::new();

    while over_num <= 1048576 {
       
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

            //let mut count: HashMap<i32,i32> = HashMap::new();
            //let mut count_list: Vec<i32> = Vec::new();

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
                //count_list.push(key);
                //let cur_count = count.entry(key).or_insert(0);
                //*cur_count += 1;
            }

            //max_counts.push(max);

            //println!("{:?}", arr);
            //println!("{:?}", count_list);
            //println!("{:?}", count);
            //println!("{}: {}", over_num, count.len());
    
        }
   
        // find mean and variance and add it to overall map

        for (i, counts) in max_counts.iter().enumerate() {
            let sum: i32 = counts.iter().sum();
            let sum: f32 = sum as f32 / counts.len() as f32;
            mean.insert(format!("{}chunk{}", over_num, i+1), sum);
            println!("mean for {} chunk {}: {}", over_num, i+1, sum);
            
            // now variance
            let mut dist_sum: f32 = 0.0;
            for x in counts.iter() {
                dist_sum = dist_sum + ((*x as f32 - sum).powi(2));
            }
            dist_sum = dist_sum / (counts.len() - 1) as f32;
            // if you prefer std deviation...
            //dist_sum = dist_sum.sqrt();
            variance.insert(format!("{}chunk{}", over_num, i+1), dist_sum);
            println!("variance for {} chunk {}: {}", over_num, i+1, dist_sum);
        }

        // find median and add it to overall map
        for (i, c) in max_counts.iter().enumerate() {
            let mut counts = c.to_vec();
            counts.sort();
            let mid = counts.len() / 2;
            let median_val: f32 = if counts.len() % 2 == 0 {
                (counts[mid - 1] as f32 + counts[mid] as f32) / 2 as f32
            } else {
                counts[mid] as f32
            };
            median.insert(format!("{}chunk{}", over_num, i+1), median_val);

            println!("median for {} chunk {}: {}", over_num, i+1, median_val);
        }

        over_num = over_num * 2;
    }
    
    println!("Mean: {:?}", mean);
    println!("Median: {:?}", median);
    println!("Variance: {:?}", variance);
}
