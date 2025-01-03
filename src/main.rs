fn parallel_sum(nums: Vec<i64>, chunk_size: usize) -> i64 {
    let mut sum: i64 = 0;
    let mut handles = vec![];

    for chunk in nums.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let handle = std::thread::spawn(move || {
            let mut local_sum = 0;
            for i in chunk {
                local_sum += i;
            }
            local_sum
        });
        handles.push(handle);
    }
    for handle in handles {
        sum += handle.join().unwrap();
    }
    sum
}

fn main() {
    // generart a large vector of numbers with random;
    let mut nums = vec![];
    for i in 0..10000000 {
        nums.push(i);
    }

    // test serial sum speed
    let start = std::time::Instant::now();
    let sum = nums.iter().sum::<i64>();
    let duration = start.elapsed();
    println!("serial sum: {}, \t\tduration: {:?}", sum, duration);

    let mut chunk_size = 32000;

    while chunk_size <= 64000 {
        let start = std::time::Instant::now();
        let sum = parallel_sum(nums.clone(), chunk_size);
        let duration = start.elapsed();
        println!(
            "chunk_size: {}, \t\ttsum: {}, \t\tduration: {:?}",
            chunk_size, sum, duration
        );
        chunk_size += 2000;
    }
}
