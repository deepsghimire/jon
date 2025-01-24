use rustyline::DefaultEditor;

mod reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("jon> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line)?;
                let mut scanner = reader::Scanner::new(&line);
                println!("{:#?}", scanner.scan_all())
            }
            Err(err) => {
                println!("Exiting: {}", err);
                return Ok(());
            }
        }
    }
}
