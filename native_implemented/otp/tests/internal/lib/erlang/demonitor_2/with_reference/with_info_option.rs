#[path = "with_info_option/with_monitor.rs"]
mod with_monitor;

test_stdout!(without_monitor_returns_false, "false\n");
