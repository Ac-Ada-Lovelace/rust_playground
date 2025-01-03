mod parallel_sum {
    fn serial_sum(nums: Vec<i64>) -> i64 {
        let mut sum: i64 = 0;
        for i in nums {
            sum += i;
        }
        sum
    }

    fn parallel_sum(nums: Vec<i64>, chunk_size: i64) -> i64 {
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

    fn init() {
        println!("This is a simple example of parallel programming in Rust");
        // generate a large vector of numbers;
        let mut nums = vec![];

        for i in 0..10000000 {
            nums.push(i);
        }

        let mut real_ans = 0i64;
        for i in 0..10000000 {
            real_ans += i;
        }

        use std::time::Instant;

        let clone_for_serial = nums.clone();
        let now = Instant::now();

        let serial_ans = serial_sum(clone_for_serial);

        let serial_time = Instant::now() - now;

        let clone_for_parallel = nums.clone();
        let now = Instant::now();

        let parallel_ans = parallel_sum(clone_for_parallel, 1000);

        let parallel_time = Instant::now() - now;

        // show results
        println!("Real answer: {}", real_ans);
        println!("Serial answer: {}", serial_ans);
        println!("Parallel answer: {}", parallel_ans);

        println!("Serial time: {:?}", serial_time);

        println!("Parallel time: {:?}", parallel_time);
    }
}
