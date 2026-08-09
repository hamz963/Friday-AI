use friday_core::{SystemMetricsTracker, detect_hardware};
use std::time::Instant;

#[test]
fn run_performance_benchmarks() {
    println!("\n=== Running Friday AI Performance Benchmarks ===");
    
    // 1. Hardware context
    let hw = detect_hardware();
    println!("Platform: {}, CPU: {}, Memory: {:.1} GB", hw.platform, hw.cpu_brand, hw.ram_gb);

    let mut tracker = SystemMetricsTracker::new();
    let start = Instant::now();

    // 2. Perform mock intensive operations (String processing loops)
    let mut sum = 0;
    for i in 0..1_000_000 {
        let text = format!("refining_prompt_filler_word_anomaly_check_{}", i);
        sum += text.len();
    }
    
    let report = tracker.capture_metrics(start);
    
    println!("\n=== Benchmarking Result ===");
    println!("Processed Loops : 1,000,000 text iterations");
    println!("System CPU Load : {:.2}%", report.cpu_usage);
    println!("RAM Utilized    : {} MB / {} MB", report.used_memory_mb, report.total_memory_mb);
    println!("Total Latency   : {} ms", report.elapsed_latency_ms);
    println!("Average Iter    : {:.6} ms/loop", report.elapsed_latency_ms as f64 / 1_000_000.0);
    println!("===========================");
    
    assert!(report.elapsed_latency_ms < 5000); // Must complete within 5 seconds in optimized release target
}
