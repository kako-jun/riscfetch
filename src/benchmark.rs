use colored::*;
use std::time::Instant;

pub fn run_benchmarks() {
    println!(
        "{}",
        "⚡ Running RISC-V Benchmarks...".bright_yellow().bold()
    );
    println!();

    // Integer multiplication benchmark (M extension)
    let int_score = benchmark_integer_ops();
    println!(
        "{} {} {}",
        "🔢 Integer Ops (M):".bright_cyan().bold(),
        format!("{:.2}", int_score).bright_white(),
        "MOPS".bright_white().dimmed()
    );

    // Floating-point benchmark (F/D extension)
    let float_score = benchmark_float_ops();
    println!(
        "{} {} {}",
        "🎯 Float Ops (F/D):".bright_green().bold(),
        format!("{:.2}", float_score).bright_white(),
        "MFLOPS".bright_white().dimmed()
    );

    // Memory bandwidth benchmark
    let mem_score = benchmark_memory();
    println!(
        "{} {} {}",
        "💾 Memory Bandwidth:".bright_magenta().bold(),
        format!("{:.2}", mem_score).bright_white(),
        "MB/s".bright_white().dimmed()
    );

    println!();
    println!("{}", "✨ Benchmarks complete!".bright_yellow().bold());
    println!();
}

fn benchmark_integer_ops() -> f64 {
    const ITERATIONS: u64 = 10_000_000;
    let start = Instant::now();

    let mut result: u64 = 1;
    for i in 1..ITERATIONS {
        result = result.wrapping_mul(i).wrapping_add(i);
    }

    let elapsed = start.elapsed();
    let ops_per_sec = (ITERATIONS as f64 / elapsed.as_secs_f64()) / 1_000_000.0;

    // Use result to prevent optimization
    std::hint::black_box(result);

    ops_per_sec
}

fn benchmark_float_ops() -> f64 {
    const ITERATIONS: u64 = 5_000_000;
    let start = Instant::now();

    let mut result: f64 = 1.0;
    for i in 1..ITERATIONS {
        let x = i as f64;
        result = (result * x).sqrt() + x.sin();
    }

    let elapsed = start.elapsed();
    let ops_per_sec = (ITERATIONS as f64 * 2.0 / elapsed.as_secs_f64()) / 1_000_000.0;

    // Use result to prevent optimization
    std::hint::black_box(result);

    ops_per_sec
}

fn benchmark_memory() -> f64 {
    const SIZE: usize = 10_000_000;
    let mut data = vec![0u8; SIZE];

    let start = Instant::now();

    // Write benchmark
    for (i, item) in data.iter_mut().enumerate().take(SIZE) {
        *item = (i & 0xFF) as u8;
    }

    // Read benchmark
    let mut sum: u64 = 0;
    for &byte in &data {
        sum = sum.wrapping_add(byte as u64);
    }

    let elapsed = start.elapsed();
    let mb_per_sec = (SIZE as f64 * 2.0 / elapsed.as_secs_f64()) / 1_000_000.0;

    // Use sum to prevent optimization
    std::hint::black_box(sum);

    mb_per_sec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_integer_ops() {
        let score = benchmark_integer_ops();
        assert!(score > 0.0);
    }

    #[test]
    fn test_benchmark_float_ops() {
        let score = benchmark_float_ops();
        assert!(score > 0.0);
    }

    #[test]
    fn test_benchmark_memory() {
        let score = benchmark_memory();
        assert!(score > 0.0);
    }
}
