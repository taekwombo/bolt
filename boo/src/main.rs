mod command;
mod mode;
mod print_query;
mod textarea;

use textarea::TextArea;
use print_query::{print_query_response, print_query_error};
use mode::Mode;
use bolt::{error::BoltResult, Client, response::Response};
use packstream_serde::Value;
use std::io::{self, Write};
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;

pub struct LastResponse {
    pub size: usize,
    pub index: usize,
    pub response: Response<Value>,
}

fn get_user_input(stdin: &io::Stdin, prompt: &str, default: &str) -> String {
    let mut input = String::new();

    println!("{} (default: {}):", prompt, default);

    stdin.read_line(&mut input).expect("readline succeeds");

    input = input.replace("\n", "");

    if input.len() == 0 {
        String::from(default)
    } else {
        input
    }
}

fn get_password(stdin: &mut io::Stdin, stdout: &mut io::Stdout, prompt: &str, default: &str) -> String {
    println!("{} (default: {}):", prompt, default);

    let input = stdin.read_passwd(stdout).unwrap().unwrap();
    println!("{}", input);

    if input.len() == 0 {
        String::from(default)
    } else {
        input
    }
}

async fn run_statement(client: &mut Client, statement: String) -> BoltResult<Response<Value>> {
    use packstream_serde::message::Run;

    let message = Run {
        statement,
        parameters: std::collections::HashMap::new(),
    };

    return client.send(&message, true).await;
}

fn connect_to_server(stdin: &mut io::Stdin, stdout: &mut io::Stdout) -> BoltResult<Client> {
    let server_address = get_user_input(stdin, "Server address", "localhost:7687");
    let auth_username = get_user_input(stdin, "Auth username", "neo4j");
    let auth_password = get_password(stdin, stdout, "Auth password", "bolt-rs");
    
    smol::block_on(async {
        Client::connect(server_address, auth_username, auth_password).await
    })
}

macro_rules! help_str {
    (normal { $($n:literal)+ } statement { $($s:literal)+ } results { $($r:literal)+ }) => {
        concat!(
            help_str!("[N]", $($n)+),
            help_str!("[S]", $($s)+),
            help_str!("[R]", $($r)+)
        )
    };
    ($e:expr, $($l:literal)+) => {
        concat!(
            $($e, " ", $l, "\r\n",)+
        )
    };
}

static HELP_TEXT: &'static str = help_str!(
    normal {
       "Press [?] to display this message"
       "Press [i] to enter a Statement mode"
       "Press [ESC] to exit program"
    }
    statement {
       "Press [C-r] to run statement"
       "Press [C-w] to remove previous word"
       "Press [C-u] to clear statement"
       "Press [Backspace] to remove previous character"
       "Press [C-n] to select next statement from history"
       "Press [C-p] to select previous statement from history"
       "Press [ESC] to return to Normal mode"
    }
    results {
        "Press [n] to display next page"
        "Press [p] to display previous page"
        "Pres [ESC] to go back to Statement mode"
    }
);

fn main() -> Result<(), io::Error> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let (width, _) = termion::terminal_size()?;

    let mut client = match connect_to_server(&mut stdin, &mut stdout) {
        Ok(c) => c,
        Err(e) => {
            println!("Could not acquire connection: {:?}", e);
            return Ok(());
        },
    };

    let mut stdout = stdout.into_raw_mode()?;
    // TODO: display mode before prompt.
    let mut mode: Mode = Mode::Normal;
    let mut text_area = TextArea::new();
    let mut keys = stdin.keys();
    let mut last_response: Option<LastResponse> = None;

    stdout.write(HELP_TEXT.as_bytes())?;
    stdout.flush()?;

    loop {
        if mode == Mode::Normal {
            if let Some(Ok(key)) = keys.next() {
                match key {
                    Key::Esc => {
                        stdout.flush()?;
                        break;
                    },
                    Key::Char('i') => {
                        mode = Mode::Statement;

                        text_area.print();
                        stdout.flush()?;
                    },
                    Key::Char('?') => {
                        stdout.write(HELP_TEXT.as_bytes())?;
                        stdout.flush()?;
                    },
                    _ => {},
                }
                continue;
            }
        }

        if mode == Mode::Results {
            if last_response.is_none() {
                mode = Mode::Statement;
                continue;
            }

            let result = last_response.as_mut().unwrap();
            stdout.write(print_query_response(width, &result).as_bytes())?;

            if result.size <= 20 {
                mode = Mode::Statement;

                text_area.print();
                stdout.flush()?;
                continue;
            }

            stdout.write(format!(
                "Results {}-{} of {}. Use <C-n> <C-p> to scroll.\r\n",
                result.index,
                (result.index + 20).min(result.size),
                result.size,
            ).as_bytes())?;

            stdout.flush()?;

            if let Some(Ok(key)) = keys.next() {
                match key {
                    Key::Ctrl('p') => {
                        if result.index == 0 {
                            continue;
                        }

                        result.index = result.index.saturating_sub(20);
                    },
                    Key::Ctrl('n') => {
                        let next = result.index + 20;

                        if next >= result.size {
                            continue;
                        }

                        result.index = next;
                    },
                    Key::Esc => {
                        mode = Mode::Statement;

                        text_area.print();
                        stdout.flush()?;
                    },
                    _ => {},
                }
            }

            continue;
        }

        if let Some(Ok(key)) = keys.next() {
            match key {
                Key::Ctrl('p') => {
                    text_area.update_command(|command| {
                        command.use_prev_command();
                    });
                    stdout.flush()?;
                },
                Key::Ctrl('n') => {
                    text_area.update_command(|command| {
                        command.use_next_command();
                    });
                    stdout.flush()?;
                },
                Key::Ctrl('r') => {
                    let statement = text_area.command.get_buffer().clone();
                    let result = smol::block_on(run_statement(&mut client, statement.clone()));

                    match result {
                        Ok(response) => {
                            text_area.command.add();
                            text_area.command.get_buffer_mut().clear();

                            mode = Mode::Results;
                            let size = response.rows().len();
                            last_response = Some(LastResponse {
                                response,
                                size,
                                index: 0,
                            });
                        },
                        Err(error) => {
                            stdout.write(print_query_error(width, error).as_bytes())?;
                        },
                    }

                    text_area.print();
                    stdout.flush()?;
                },
                Key::Ctrl('h') => {
                    text_area.update_command(|command| {
                        command.get_buffer_mut().pop();
                    });
                    stdout.flush()?;
                },
                Key::Ctrl('u') => {
                    text_area.update_command(|command| {
                        command.get_buffer_mut().clear();
                    });
                    stdout.flush()?;
                },
                Key::Ctrl('w') => {
                    text_area.update_command(|command| {
                        let buffer = command.get_buffer_mut();
                        let len = buffer.len();

                        if len == 0 {
                            return;
                        }

                        let mut trim_count = 0;
                        let mut ignore_whitespace;

                        for (i, c) in buffer.chars().rev().enumerate() {
                            if i == 0 && c.is_ascii_whitespace() {
                                ignore_whitespace = true;
                            } else {
                                ignore_whitespace = false
                            }

                            if !ignore_whitespace && c.is_ascii_whitespace() {
                                break;
                            }

                            trim_count += 1;
                        }

                        buffer.truncate(len - trim_count);
                    });
                    stdout.flush()?;
                },
                Key::Esc => {
                    text_area.clear();
                    stdout.flush()?;
                    mode = Mode::Normal;
                },
                Key::Backspace => {
                    text_area.update_command(|command| {
                        command.get_buffer_mut().pop();
                    });
                    stdout.flush()?;
                },
                Key::Char(c) => {
                    text_area.update_command(|command| {
                        command.get_buffer_mut().push(c);
                    });
                    stdout.flush()?;
                },
                _ => {},
            }
        }
    }

    Ok(())
}
