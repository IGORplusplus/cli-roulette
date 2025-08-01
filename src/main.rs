use crossterm::event::{EnableMouseCapture, self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use crossterm::event::DisableMouseCapture;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use ratatui::Terminal;
use ratatui::backend::{CrosstermBackend, Backend};
use ratatui_json_editor::app::{App, CurrentScreen, CurrentlyEditing};
use ratatui_json_editor::ui::ui;
use std::io;
use anyhow::Result;
use serde_json::{Serializer, Deserializer};

fn main() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;

    let mut stderr 
        = io::stderr(); // This is a special case. Normally using stdout is fine

    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend 
        = CrosstermBackend::new(stderr);

    let mut terminal 
        = Terminal::new(backend)?;

    // create app and run it
    let mut app 
        = App::new();

    let res 
        = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;

    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app
                .print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>,app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue; // Skip events that are not KeyEventKind::Press
            }
            match app.current_screen {
                CurrentScreen::Main 
                => match key.code {
                    KeyCode::Char('e') 
                    => {
                        app.current_screen 
                            = CurrentScreen::Editing;

                        app.currently_editing 
                            = Some(CurrentlyEditing::Key);
                    }

                    KeyCode::Char('q') 
                    => {
                        app.current_screen 
                            = CurrentScreen::Exiting;
                    }

                    _ => {}
                },

                CurrentScreen::Exiting 
                => match key.code {
                    KeyCode::Char('y') 
                    => {
                        return Ok(true);
                    }

                    KeyCode::Char('n') | KeyCode::Char('q') 
                    => {
                        return Ok(false);
                    }
                        _ => {}
                },

                CurrentScreen::Editing if key.kind == KeyEventKind::Press 
                => {
                    match key.code {
                        KeyCode::Enter 
                        => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key 
                                    => {
                                        app.currently_editing 
                                            = Some(CurrentlyEditing::Value);
                                    }
                                        
                                        CurrentlyEditing::Value 
                                        => {
                                            app.save_key_value();

                                            app.current_screen 
                                                = CurrentScreen::Main;
                                        }
                                }
                            }
                        }

                        KeyCode::Backspace
                        => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key 
                                    => {
                                        app.key_input
                                            .pop();
                                    }
                                    CurrentlyEditing::Value 
                                    => {
                                        app.value_input
                                            .pop();
                                    }
                                }
                            }
                        }

                        KeyCode::Esc 
                        => {
                            app.current_screen 
                                = CurrentScreen::Main;

                            app.currently_editing 
                                = None;
                        }

                        KeyCode::Tab 
                        => {
                            app
                                .toggle_editing();
                        }

                        KeyCode::Char(value) 
                        => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key 
                                    => {
                                        app.key_input
                                            .push(value);
                                    }

                                    CurrentlyEditing::Value 
                                    => {
                                        app.value_input
                                            .push(value);
                                    }
                                }
                            }
                        }

                        _ => {}
                    }
                }
                
            _ => {}
            }
        }
    }
}
