use rustyline::DefaultEditor;

mod parser;
mod scanner;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("jon> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line)?;
                let mut scanner = scanner::Scanner::new(&line);
                let mut parser = parser::Parser::new(&mut scanner);
                println!("{:#?}", parser.parse_expr());
            }
            Err(err) => {
                println!("Exiting: {}", err);
                return Ok(());
            }
        }
    }
}
