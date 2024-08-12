use std::io::Write as _;

fn main() {
    let stdin = std::io::stdin();
    let mut line = String::new();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        if stdin.read_line(&mut line).is_err() { break; }

        let mut lex = <krillscript::frontend::lexer::Token as krillscript::frontend::lexer::Logos>::lexer(&line);
        while let Some(t) = lex.next() {
            println!("{t:?} {:?}", lex.span());
        }

        line.clear();
    }
}
