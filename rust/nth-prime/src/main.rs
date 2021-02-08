use std::time::{Instant};

mod lib;

fn main() {
    bench()
}

fn bench() {
    println!("warming up...");
    lib::nth_naive(10_000);

    {
        println!("running benchmark for 10,000th prime...");

        let now = Instant::now();
        lib::nth_naive(10_000);
        let method_1_time = now.elapsed().as_millis();
        
        let now2 = Instant::now();
        lib::nth_with_caching(10_000);
        let method_2_time = now2.elapsed().as_millis();

        let now3 = Instant::now();
        lib::nth_with_caching_opt(10_000);
        let method_3_time = now3.elapsed().as_millis();

        println!("method 1: {}", method_1_time);
        println!("method 2: {}", method_2_time);
        println!("method 3: {}", method_3_time);
    }

    {
        println!("running benchmark for 1,000,000th prime...");

        // let now = Instant::now();
        // lib::nth_naive(1_000_000);
        // let method_1_time = now.elapsed().as_millis();
        
        let now2 = Instant::now();
        lib::nth_with_caching(1_000_000);
        let method_2_time = now2.elapsed().as_millis();

        let now3 = Instant::now();
        lib::nth_with_caching_opt(1_000_000);
        let method_3_time = now3.elapsed().as_millis();

        // println!("method 1: {}", method_1_time);
        println!("method 2: {}", method_2_time);
        println!("method 3: {}", method_3_time);
    }
}