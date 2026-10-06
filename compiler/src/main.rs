use compiler::lexer::Lexer;
use compiler::parser::Parser;
use compiler::preprocessor::preprocess;
use compiler::semantic_analyzer::SemanticAnalyzer;

fn main() {
    println!("---- source 1 should display generated tokens, ast and decorated ast ----");
    let source = preprocess(
        r#"
        int main() {
            int x = 0;

            while (x < 10) {
            x = x + 1;
        }

        return x;
}
        "#,
    );

    let mut lexer = Lexer::new(&source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.scan_token() {
        tokens.push(token);
    }
    
    println!("Tokens:\n{:#?}\n", tokens);

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    println!("AST:\n{:#?}\n", program);

    match SemanticAnalyzer::new().analyze(&program) {
        Ok(annotated_program) => println!("Decorated AST:\n{:#?}", annotated_program),
        Err(errors) => {
            for error in errors {
                eprintln!("Semantic error: {}", error);
            }
        }
    }

    println!("---- source 2 should display semantic error (a is not declarated)----");
    let source2 = preprocess(
        r#"
        #include <stdio.h>
        
        int main() {
            a = 10;
            return 0;
        }
        "#,
    );

    let mut lexer2 = Lexer::new(&source2);
    let mut tokens2 = Vec::new();
    while let Some(token) = lexer2.scan_token() {
        tokens2.push(token);
    }

    let mut parser2 = Parser::new(tokens2);
    let program2 = parser2.parse_program();

    println!("AST:\n{:#?}\n", program2);

    match SemanticAnalyzer::new().analyze(&program2) {
        Ok(annotated_program) => println!("Decorated AST:\n{:#?}", annotated_program),
        Err(errors) => {
            for error in errors {
                eprintln!("Semantic error: {}", error);
            }
        }
    }

}
