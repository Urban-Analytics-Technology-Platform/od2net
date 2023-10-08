use indicatif::{ProgressBar, ProgressStyle};

pub fn progress_bar_for_count(count: usize) -> ProgressBar {
    ProgressBar::new(count as u64).with_style(ProgressStyle::with_template(
        "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})").unwrap())
}
