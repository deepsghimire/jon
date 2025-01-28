use rustyline::DefaultEditor;

mod eval;
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
                let evaluator = eval::Evaluator;
                let result = evaluator.eval(&parser.parse_expr().unwrap());
                println!("{:#?}", result);
            }
            Err(err) => {
                println!("Exiting: {}", err);
                return Ok(());
            }
        }
    }
}
