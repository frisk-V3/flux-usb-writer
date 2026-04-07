use indicatif::{ProgressBar, ProgressStyle};
use std::time::Instant;

pub struct WriterProgress {
    bar: ProgressBar,
    start: Instant,
    total: u64,
}

impl WriterProgress {
    pub fn new(total: u64) -> Self {
        let bar = ProgressBar::new(total);
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        Self {
            bar,
            start: Instant::now(),
            total,
        }
    }

    pub fn inc(&self, n: u64) {
        self.bar.inc(n);
    }

    pub fn finish(&self) {
        self.bar.finish_with_message("Done");
        let elapsed = self.start.elapsed();
        println!("Completed in {:.2?}", elapsed);
    }
}
