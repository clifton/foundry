//! Console command

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::cmd::Cmd;

#[derive(Debug, Clone, structopt::StructOpt)]
pub struct ConsoleArgs {}

impl Cmd for ConsoleArgs {
    type Output = ();
    fn run(self) -> eyre::Result<Self::Output> {
        // `()` can be used when no completer is required
        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    println!("Line: {}", line);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        rl.save_history(".history").unwrap();

        Ok(())
    }
}
