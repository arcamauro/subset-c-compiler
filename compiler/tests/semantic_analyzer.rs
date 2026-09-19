use compiler::ast::{DecoratedProgram, Type};
use compiler::lexer::Lexer;
use compiler::parser::Parser;
use compiler::semantic_analyzer::SemanticAnalyzer;

fn analyze(source: &str) -> Result<DecoratedProgram, Vec<String>> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.scan_token() {
        tokens.push(token);
    }
    let program = Parser::new(tokens).parse_program();
    SemanticAnalyzer::new().analyze(&program)
}

fn errors(source: &str) -> Vec<String> {
    analyze(source).expect_err("expected semantic analysis to fail")
}

fn analyzed_program(source: &str) -> DecoratedProgram {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.scan_token() {
        tokens.push(token);
    }
    let program = Parser::new(tokens).parse_program();
    SemanticAnalyzer::new()
        .analyze(&program)
        .expect("expected semantic analysis to succeed")
}

#[test]
fn analyzer_returns_the_decorated_ast() {
    let program = analyzed_program("int main() { return 0; }");
    assert_eq!(program.items.len(), 1);
    match &program.items[0] {
        compiler::ast::DecoratedItem::Function(func) => {
            assert_eq!(func.function.name, "main");
            assert_eq!(func.body[0].inferred_type, Some(Type::Int));
        }
        compiler::ast::DecoratedItem::FunctionDecl(_) => panic!("expected function item"),
    }
}

#[test]
fn empty_main_is_ok() {
    assert!(analyze("int main() { return 0; }").is_ok());
}

#[test]
fn function_using_its_own_params_is_ok() {
    assert!(analyze("int add(int a, int b) { return a + b; }").is_ok());
}

#[test]
fn calling_a_previously_declared_function_is_ok() {
    assert!(
        analyze(
            "int add(int a, int b) { return a + b; }
             int main() { return add(1, 2); }"
        )
        .is_ok()
    );
}

#[test]
fn undeclared_identifier_is_reported() {
    let errs = errors("int main() { return x; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Undeclared identifier 'x'"));
}

#[test]
fn multiple_undeclared_identifiers_are_all_reported() {
    let errs = errors("int main() { return a + b; }");
    assert_eq!(errs.len(), 2);
    assert!(errs.iter().any(|e| e.contains("Undeclared identifier 'a'")));
    assert!(errs.iter().any(|e| e.contains("Undeclared identifier 'b'")));
}

#[test]
fn undeclared_function_call_is_reported() {
    let errs = errors("int main() { return foo(); }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Undeclared function 'foo'"));
}

#[test]
fn calling_a_non_function_identifier_is_reported() {
    let errs = errors("int main() { int x; return x(); }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("'x' is not a function"));
}

#[test]
fn redeclaration_in_same_scope_is_reported() {
    let errs = errors("int main() { int a; int a; return 0; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Redeclaration of 'a' in same scope"));
}

#[test]
fn redeclaring_a_parameter_as_a_local_is_reported() {
    let errs = errors("int foo(int a) { int a; return 0; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Redeclaration of 'a' in same scope"));
}

#[test]
fn shadowing_in_a_nested_block_is_allowed() {
    assert!(analyze("int main() { int a; { int a; } return 0; }").is_ok());
}

#[test]
fn variable_declared_in_a_block_is_not_visible_outside_it() {
    let errs = errors("int main() { { int a; } return a; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Undeclared identifier 'a'"));
}

#[test]
fn for_loop_init_variable_is_not_visible_after_the_loop() {
    let errs = errors("int main() { for (int i = 0; i < 10; i++) {} return i; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Undeclared identifier 'i'"));
}

#[test]
fn function_call_arity_mismatch_is_reported() {
    let errs = errors(
        "int add(int a, int b) { return a; }
         int main() { return add(1); }",
    );
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Function 'add' expects 2 argument(s), found 1"));
}

#[test]
fn function_call_argument_type_mismatch_is_reported() {
    let errs = errors(
        "int takes_int(int a) { return a; }
         int main() { char c = 'x'; return takes_int(c); }",
    );
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Argument 1 of 'takes_int' expected type Int, found Char"));
}

#[test]
fn return_type_mismatch_is_reported() {
    let errs = errors("int main() { return 'a'; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Return type mismatch: expected Int, found Char"));
}

#[test]
fn missing_return_value_is_reported() {
    let errs = errors("int main() { return; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Missing return value, expected Int"));
}

#[test]
fn char_function_returning_char_is_ok() {
    assert!(analyze("char get() { return 'x'; }").is_ok());
}

#[test]
fn return_type_mismatch_inside_nested_blocks_is_reported() {
    let errs = errors("int main() { while (1) { if (1) { return 'a'; } } return 0; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Return type mismatch: expected Int, found Char"));
}

#[test]
fn return_type_mismatch_inside_switch_case_is_reported() {
    let errs = errors("int main() { switch (1) { case 1: return 'a'; } return 0; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Return type mismatch: expected Int, found Char"));
}

#[test]
fn assignment_type_mismatch_is_reported() {
    let errs = errors("int main() { int a; a = 'x'; return 0; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Cannot assign value of type Char to variable of type Int"));
}

#[test]
fn assignment_to_a_non_identifier_is_reported() {
    let errs = errors("int main() { 1 = 2; return 0; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Left-hand side of assignment must be a variable"));
}

#[test]
fn binary_op_type_mismatch_is_reported() {
    let errs = errors("int main() { char c = 'x'; return 1 + c; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Type mismatch in binary expression: Int vs Char"));
}

#[test]
fn break_outside_loop_or_switch_is_reported() {
    let errs = errors("int main() { break; return 0; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("'break' used outside of a loop or switch"));
}

#[test]
fn continue_outside_loop_is_reported() {
    let errs = errors("int main() { continue; return 0; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("'continue' used outside of a loop"));
}

#[test]
fn break_inside_while_loop_is_ok() {
    assert!(analyze("int main() { while (1) { break; } return 0; }").is_ok());
}

#[test]
fn continue_inside_for_loop_is_ok() {
    assert!(
        analyze("int main() { for (int i = 0; i < 10; i++) { continue; } return 0; }")
            .is_ok()
    );
}

#[test]
fn break_inside_switch_without_a_loop_is_ok() {
    assert!(analyze("int main() { switch (1) { case 1: break; } return 0; }").is_ok());
}

#[test]
fn continue_inside_switch_without_a_loop_is_reported() {
    let errs = errors("int main() { switch (1) { case 1: continue; } return 0; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("'continue' used outside of a loop"));
}

#[test]
fn continue_inside_switch_nested_in_a_loop_is_ok() {
    assert!(
        analyze(
            "int main() {
                 while (1) {
                     switch (1) {
                         case 1: continue;
                     }
                 }
                 return 0;
             }"
        )
        .is_ok()
    );
}

#[test]
fn loop_depth_resets_after_the_loop_body_ends() {
    let errs = errors(
        "int main() {
             while (1) {
                 break;
             }
             break;
             return 0;
         }",
    );
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("'break' used outside of a loop or switch"));
}

#[test]
fn function_with_no_return_statement_is_reported() {
    let errs = errors("int main() { int a; }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Function 'main' may not return a value on all paths"));
}

#[test]
fn if_else_where_both_branches_return_is_ok() {
    assert!(analyze("int main() { if (1) { return 1; } else { return 0; } }").is_ok());
}

#[test]
fn if_without_else_is_reported_as_possibly_not_returning() {
    let errs = errors("int main() { if (1) { return 1; } }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Function 'main' may not return a value on all paths"));
}

#[test]
fn return_inside_a_loop_is_not_enough_to_guarantee_a_return() {
    let errs = errors("int main() { while (1) { return 0; } }");
    assert_eq!(errs.len(), 1);
    assert!(errs[0].contains("Function 'main' may not return a value on all paths"));
}
