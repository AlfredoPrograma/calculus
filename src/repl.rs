use std::io::{self, Write};

use crate::{ast::parser::Parser, tokenizer::Tokenizer};

fn display_caret(stdout: &mut io::Stdout) {
    stdout
        .write("> ".as_bytes())
        .expect("cannot write caret in stdout");

    stdout.flush().expect("cannot flush caret in stdout");
}

fn read_input(stdin: &io::Stdin) -> String {
    let mut input = String::new();

    stdin
        .read_line(&mut input)
        .expect("cannot read line from stdin");

    input
}

pub fn run() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        display_caret(&mut stdout);
        let input = read_input(&stdin);

        let mut tokenizer = Tokenizer::new(&input.trim());
        tokenizer.tokenize();

        let mut parser = Parser::new(tokenizer.tokens.into_iter());

        let ast = parser.program();
        println!("{}", ast);

        let result = ast.eval();
        println!("{}", result);
    }
}
