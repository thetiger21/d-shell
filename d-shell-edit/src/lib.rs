use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::widgets::{Block, Borders}; // Make sure to import these
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Position, Rect},
};
use ratatui_code_editor::editor::Editor;
use ratatui_code_editor::theme::vesper;
use std::{fs, io::stdout};

pub fn run_editor(file_path: &str) -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let content = fs::read_to_string(file_path)?;
    let mut editor = Editor::new("rust", &content, vesper())?;
    let mut editor_area = Rect::default();

    loop {
        terminal.draw(|f| {
            let full_area = f.area();

            // 1. Create the surrounding box (Block)
            let outer_block = Block::default()
                .title(format!(" Editing: {} ", file_path))
                .borders(Borders::ALL);

            // 2. Compute the safe area inside the borders for the editor
            editor_area = outer_block.inner(full_area);

            // 3. Render the background block first, then the editor inside it
            f.render_widget(outer_block, full_area);
            f.render_widget(&editor, editor_area);

            // 4. Position the cursor relative to the inner editor area
            let cursor = editor.get_visible_cursor(&editor_area);
            if let Some((x, y)) = cursor {
                f.set_cursor_position(Position::new(x, y));
            }
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match (key.modifiers, key.code) {
                    (_, KeyCode::Esc) => break,
                    (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                        println!("Pressed control s");
                        fs::write(file_path, editor.get_content());
                        break;
                    }
                    _ => (),
                };
            }

            editor.input(key, &editor_area)?;
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
