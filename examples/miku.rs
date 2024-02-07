use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, prelude::*, widgets::TableState, Terminal};
use std::{io::stdout, thread::sleep, time::Duration};
use tui_dashboard::*;

fn main() -> AppResult<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|frame| render(frame).expect("failed to render"))?;
    sleep(Duration::from_secs(4));
    terminal.clear()?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn render(frame: &mut Frame) -> AppResult<()> {
    let avatar = " \
⠀⠀⠀⠀⠀⠀⢀⡤⣢⠟⢁⣴⣾⡿⠋⢉⠱⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠑⠒⠦⢄⣀⣴⠟⢡⣠⣼⣿⡿⢳⣄⡀⠀⠀
⠀⠀⠀⠀⠀⢀⣾⡿⠃⣠⣿⣿⠿⠂⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢲⡿⠋⢰⣾⣿⣿⡟⠀⠀⠈⠙⢆⠀
⠀⠀⠀⠀⠀⡜⠻⣷⣾⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⣁⣰⢸⣿⢻⠟⢀⠀⠀⠀⠀⠀⠁
⠀⠀⠀⠀⠰⠀⠀⢙⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⣿⣿⣿⣯⡀⠀⢃⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢠⠎⠀⠀⠀⠀⠀⠀⣼⠀⢀⠀⠀⠀⠀⠀⢠⣷⡀⠀⠀⠀⠀⡀⠄⠀⠀⠀⠀⢻⣿⣿⣿⣧⠑⠀⣢⡄⠀⠀⠀⠀
⠀⠀⠀⠀⠀⡰⠃⢀⠄⠀⠀⠀⠀⣼⡿⡆⢸⠀⠀⠀⠀⠀⠈⣿⢷⡄⠀⠀⠀⠱⡀⠰⡀⠀⠀⠈⢿⣿⣿⣿⣧⠀⢸⣧⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠡⢢⠋⠀⠀⠀⠀⣼⡟⠀⣇⢸⡆⠀⠀⠀⡄⠀⢿⠀⢳⡄⠀⠀⠀⢳⠀⢳⠀⠀⠀⠈⣿⣿⣿⣿⣷⣘⡟⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢀⠇⠀⠀⠀⠀⣸⡿⢤⠤⠸⡸⣷⠀⠀⠀⢱⠀⣾⡤⠤⢿⡤⢀⡀⠀⢧⠘⡆⠀⠀⠀⢸⡟⠻⢿⠟⣿⣷⡄⠀⠀⠀
⠀⠀⠀⠀⠀⡞⠀⠀⠀⠀⢰⡿⢠⠇⠀⠀⢳⣿⢇⠀⠀⠈⡇⣿⡇⠀⠀⠻⣄⠀⠀⠘⡆⡇⠀⠀⠀⠀⣇⢀⡏⠀⣿⡿⣄⠀⠀⠀
⠀⠀⠀⠀⢰⠁⠀⠀⠀⠀⣿⠁⣄⣀⣀⡀⠈⢿⡜⡄⠀⠀⢹⣿⡇⠐⢄⣀⠘⢧⡀⠀⠹⣿⠀⠀⠀⠀⢸⣿⣷⣶⣿⡇⢹⡇⠀⠀
⠀⠀⠀⠀⠾⠀⠀⠀⠀⢸⣧⣾⠟⢉⣽⣿⣦⠈⢷⡘⣆⠀⠸⡟⣷⣶⠟⠛⢻⣷⣦⣀⠀⢻⠀⠀⠀⠀⢸⣏⣩⣼⣿⡇⠈⣷⠀⠀
⠀⠀⠀⠃⠀⠀⠀⠀⠀⣿⡿⠁⠀⣠⣾⣿⣿⠀⠈⢿⠺⡆⠀⣧⢸⠀⠀⢀⣹⣿⣿⣿⣷⣼⣤⠀⠀⠀⢸⣿⣿⣿⣿⠀⠀⣿⠀⠀
⠀⠀⣠⠄⣀⠀⠀⠀⢠⣿⡇⠀⠀⢻⢻⣟⢿⠀⠀⠈⠣⠈⠓⠾⠀⠀⠀⣿⣿⢿⣿⣿⠘⡇⡞⠀⠀⢠⣾⣿⣿⣿⡏⠀⠀⢹⠀⠀
⠀⠀⠛⠀⣿⠀⠀⠀⢸⣿⣿⡀⠀⠈⠃⠐⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣄⣐⣠⠏⢠⣿⠁⠀⠀⢸⣿⣿⣿⣿⠀⠀⠀⢸⠀⠀
⠀⠀⠀⠀⢹⡆⠰⡀⢸⡟⠩⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⠃⠀⠀⠀⢸⣿⣿⣿⠟⠀⠀⠀⠘⠀⠀
⠀⠀⠀⠀⢎⣿⡀⢱⢞⣁⣀⡿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠞⡏⡼⠀⠀⠀⣾⣿⠋⠁⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠈⠿⠻⡇⠀⠀⠒⠢⢵⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣽⠁⠀⠀⢠⡿⢹⠀⠀⠀⠀⠀⠀⠀⠀⠀
 ⠀⠀⠀⠀⠀⠀⡟⣦⡀⠀⠀⠀⠈⠓⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣴⢿⡇⠀⠀⡄⣸⣇⣼⣀⣀⣀⠀⠀⠀⠀⠀⠀
 ⠀⠀⠀⠀⠀⢰⠇⣿⢸⣦⡀⠀⠀⠀⠀⠈⠲⣄⡀⠀⠀⠀⠀⠀⣀⡤⠒⢉⡴⠃⣸⠀⠀⢰⣿⣿⣿⠃⡤⠊⠁⠉⠑⢄⠀⠀⠀
 ⠀⠀⠀⠀⠀⢸⠀⣿⣾⣿⢿⠲⣄⠀⠀⠀⠀⠘⠟⣦⣤⣴⡒⠉⢀⡠⠖⠉⠀⣠⠃⠀⣠⣿⣿⡿⠁⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢸⠀⣿⠛⢿⠈⢢⠏⠀⠀⠀⠀⠀⣰⣏⣀⣿⠗⠊⠁⠀⠀⣠⣾⠃⢀⡴⠿⠛⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢸⢀⠇⠀⠈⢠⠃⠀⠀⠀⠀⠀⢰⠟⠁⠀⢹⢇⠀⣀⠴⠊⡱⠥⠔⠋⠀⠀⢰⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢸⡟⠀⢀⡴⠁⠀⠀⠀⠀⠀⢠⡟⠀⠀⣰⢿⡘⣾⡅⠀⠀⠀⠀⢀⠄⠀⢠⠏⢀⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢸⠀⣰⣿⠀⠀⠀⠀⠀⠀⢠⣿⠃⢀⡾⡇⠘⠻⡿⢷⡀⠀⠀⠒⠁⠀⢠⠏⢀⠏⣸⠃⢻⠏⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⣧⣾⣹⣿⠀⠀⠀⠀⠀⢠⠏⢉⠀⡞⣰⡇⠀⣴⣥⠞⢷⠀⠀⠀⠀⣠⠎⠀⠸⣶⠋⣠⡟⠀⠀⠀
";

    let dashboard = DashboardBuilder::default()
        .title("WI EDITOR")
        .subtitle("My custom editor")
        .actions(vec![
            ("  New file", "n"),
            ("  Find file", "f"),
            ("  Recently opned file", "r"),
            ("  Jump to bookmarks", "j"),
        ])
        .avatar(avatar)
        .footer(vec![
            "一粒沙里阅世界, 一朵花中觅天堂, 无穷尽在掌心中, 永恒不过一刻钟".into(),
            "To see a world in a grain of sand, And a heaven in a wild flower, Hold infinity in the palm of your hand, And eternity in an hour.".into(), 
        ])
        .build();
    frame.render_stateful_widget(dashboard, frame.size(), &mut TableState::new().with_selected(Some(0)));
    Ok(())
}
