#[cfg(test)]
mod tests {

    // 👺 Havoc: Fuzzing the log system for crashes on extreme memory bounds!
    #[test]
    fn havoc_log_system_oom_allocation() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .arg("--exact")
            .arg("havoc_log_system_oom_allocation_inner")
            .arg("--nocapture")
            .status();

        if let Ok(status) = status {
            // If it exited with signal (e.g. SIGABRT from OOM or slice panic), we proved fragility!
            assert!(!status.success(), "👺 Havoc: System safely handled massive allocations without OOM crashing! Our chaos hunt failed!");
        }
    }
}

#[test]
fn havoc_log_system_oom_allocation_inner() {
    // Only run this test if explicitly requested, as it is designed to abort the process.
    if std::env::args().any(|arg| arg == "havoc_log_system_oom_allocation_inner") {
        use ratatui::widgets::Widget;
        use tui_shared::LogList;

        // Try allocating a gargantuan log list string and rendering it
        // This targets `get_log_style_and_prefix` inside tui_shared/src/log_list.rs
        // which iterates through .windows(keyword.len())

        let massive_str = "A".repeat(usize::MAX / 4); // Ask for ~4.6 Exabytes of contiguous memory (causes capacity overflow)
        let list = LogList::new(vec![massive_str]);

        let area = ratatui::layout::Rect::new(0, 0, 100, 100);
        let mut buffer = ratatui::buffer::Buffer::empty(area);
        list.render(area, &mut buffer);
    }
}
