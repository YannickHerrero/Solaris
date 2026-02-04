use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub fn render(frame: &mut Frame, area: Rect) {
    let fake_output = r#"$ cargo build --release
   Compiling solaris v0.1.0 (/home/user/projects/solaris)
    Finished release [optimized] target(s) in 2.34s

$ cargo test
   Compiling solaris v0.1.0 (/home/user/projects/solaris)
    Finished test [unoptimized + debuginfo] target(s) in 1.21s
     Running unittests src/main.rs (target/debug/deps/solaris-a1b2c3d4)

running 12 tests
test game::economy::tests::test_bulk_cost ... ok
test game::economy::tests::test_max_affordable ... ok
test game::economy::tests::test_single_cost ... ok
test format::tests::test_format_duration ... ok
test format::tests::test_format_scientific ... ok
test format::tests::test_format_small_numbers ... ok
test game::producer::tests::test_producer_count ... ok
test game::upgrade::tests::test_upgrade_availability ... ok
test save::tests::test_save_load ... ok
test ui::tests::test_layout ... ok
test ui::tests::test_render ... ok
test integration::tests::test_full_game_loop ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

$ git status
On branch main
Your branch is up to date with 'origin/main'.

nothing to commit, working tree clean

$ _"#;

    let paragraph =
        Paragraph::new(fake_output).style(Style::default().fg(Color::White).bg(Color::Black));

    frame.render_widget(paragraph, area);
}
