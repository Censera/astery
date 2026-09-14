#[path = "extra/cast_pointer.rs"]
mod cast_pointer;
#[path = "utility/compiler.rs"]
mod compiler;
#[path = "backend/context.rs"]
mod context;
#[path = "extra/embed.rs"]
mod embed;
#[path = "utility/error.rs"]
mod error;
#[path = "backend/function.rs"]
mod function;
#[path = "backend/handler.rs"]
mod handler;
#[path = "lexer/lexer.rs"]
mod lexer;
#[path = "backend/module.rs"]
mod module;
#[path = "parser/parser.rs"]
mod parser;
#[path = "parser/program.rs"]
mod program;
#[path = "semantic/semantic.rs"]
mod semantic;
#[path = "backend/shortcuts.rs"]
pub mod shortcuts;
#[path = "semantic/span.rs"]
mod span;
#[path = "semantic/user_type.rs"]
mod user_type;

pub use cast_pointer::{AddressOfExpression, CastExpression, PointerType};
pub use compiler::{Compiler, CompilerOptions, Output, Program, Source, Sources, Target};
pub use context::Context;
pub use embed::EmbeddedBlock;
pub use error::{Error, Stage};
pub use function::Function;
pub use handler::Handler;
pub use lexer::{Token, TokenKind};
pub use module::Module;
pub use parser::{
    BinaryOperator, Binding, BindingDeclaration, BindingKind, Block, EnumDeclaration, EnumField,
    EnumVariant, EnumVariantKind, Expression, ForStatement, FunctionDeclaration, IfStatement,
    Import, ImportItem, IntoImplementation, LoopStatement, MatchArm, MatchStatement,
    MethodDeclaration, ModuleDeclaration, Parameter, Statement, StructDeclaration, StructField,
    UnaryOperator, Visibility, WhileStatement,
};
pub use user_type::{UserTypeDeclaration, UserTypeDefinition};

#[cfg(test)]
mod tests {
    use super::{Compiler, CompilerOptions, Error, Output, Source, Sources, Stage, Target};
    use crate::lexer::{Token, TokenKind};
    use crate::parser::{
        BinaryOperator, BindingKind, Expression, Import, ModuleDeclaration, Statement,
    };
    use crate::shortcuts;

    #[test]
    fn creates_i32_function() {
        let context = crate::Context::create();
        let module = context.module("test").unwrap();
        let function = shortcuts::i32_function(&module, "answer").unwrap();
        let handler = shortcuts::handler(&function, &context).unwrap();
        shortcuts::return_i32(&handler, 42).unwrap();
        assert_eq!(function.name(), "answer");
        assert!(module.as_ir().contains("define i32 @answer()"));
        assert!(module.as_ir().contains("ret i32 42"));
    }

    #[test]
    fn creates_void_function() {
        let context = crate::Context::create();
        let module = context.module("test").unwrap();
        let function = shortcuts::void_function(&module, "main").unwrap();
        let handler = function.handler(&context).unwrap();
        handler.return_void().unwrap();
        assert!(module.as_ir().contains("define void @main()"));
    }

    #[test]
    fn compiler_reports_semantic_analysis_as_next_stage() {
        let compiler = Compiler::new();
        let error = compiler.compile(Source::new("test.astery", "fn main() {}"));
        assert_eq!(error.unwrap_err().stage(), Some(Stage::Semantic));
    }

    #[test]
    fn source_owns_name_and_text() {
        let source = Source::new("test.astery", "fn main() {}");
        assert_eq!(source.name(), "test.astery");
        assert_eq!(source.text(), "fn main() {}");
    }

    #[test]
    fn source_sets_reject_duplicate_names() {
        let error = Sources::new([
            Source::new("main.as", "fn main() {}"),
            Source::new("main.as", "fn main() {}"),
        ])
        .unwrap_err();
        assert!(matches!(error, Error::InvalidOptions(_)));
    }

    #[test]
    fn compiler_options_have_explicit_native_defaults() {
        let options = CompilerOptions::default();
        assert_eq!(options.target, Target::Native);
        assert_eq!(options.output, Output::Executable);
        assert!(options.validate().is_ok());
    }

    #[test]
    fn io_errors_are_explicit() {
        let error = Error::from(std::io::Error::other("test"));
        assert_eq!(error.stage(), None);
    }

    #[test]
    fn tokenizes_basic_function() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "fn main() { return }");
        let tokens = compiler.tokenize(&source).unwrap();
        assert_eq!(
            tokens.iter().map(Token::kind).collect::<Vec<_>>(),
            vec![
                &TokenKind::Fn,
                &TokenKind::Identifier("main".into()),
                &TokenKind::OpenParen,
                &TokenKind::CloseParen,
                &TokenKind::OpenBrace,
                &TokenKind::Return,
                &TokenKind::CloseBrace,
            ]
        );
    }

    #[test]
    fn tokenizes_literals_and_operators() {
        let compiler = Compiler::new();
        let source = Source::new(
            "main.as",
            "let value = 42 + 1.5; let ok = true && false; let c = 'x';",
        );
        let tokens = compiler.tokenize(&source).unwrap();
        let kinds = tokens.iter().map(Token::kind).collect::<Vec<_>>();
        assert!(kinds.contains(&&TokenKind::Integer("42".into())));
        assert!(kinds.contains(&&TokenKind::Float("1.5".into())));
        assert!(kinds.contains(&&TokenKind::And));
        assert!(kinds.contains(&&TokenKind::Character('x')));
    }

    #[test]
    fn tokenizes_valid_escapes_and_preserves_locations() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "\nlet text = \"a\\n\\t\\0\\\\\\\"\";");
        let tokens = compiler.tokenize(&source).unwrap();
        let string = tokens
            .iter()
            .find(|token| matches!(token.kind(), TokenKind::String(_)))
            .unwrap();
        assert_eq!(string.line(), 2);
        assert_eq!(string.column(), 12);
        assert_eq!(string.span().start.line, 2);
        assert_eq!(string.span().start.column, 12);
        assert_eq!(string.kind(), &TokenKind::String("a\n\t\0\\\"".into()));
    }

    #[test]
    fn rejects_invalid_numeric_literals_at_the_lexer_stage() {
        let compiler = Compiler::new();
        for source in ["12abc", "1.2.3"] {
            let error = compiler
                .tokenize(&Source::new("main.as", source))
                .unwrap_err();
            assert_eq!(error.stage(), Some(Stage::Lexer));
            assert!(error.to_string().contains("invalid numeric literal"));
        }
    }

    #[test]
    fn tokenizes_ranges_after_integer_literals() {
        let compiler = Compiler::new();
        let tokens = compiler
            .tokenize(&Source::new("main.as", "0..10 0..=9"))
            .unwrap();
        let kinds = tokens.iter().map(Token::kind).collect::<Vec<_>>();
        assert_eq!(kinds[0], &TokenKind::Integer("0".into()));
        assert_eq!(kinds[1], &TokenKind::Range);
        assert_eq!(kinds[2], &TokenKind::Integer("10".into()));
        assert_eq!(kinds[3], &TokenKind::Integer("0".into()));
        assert_eq!(kinds[4], &TokenKind::RangeInclusive);
        assert_eq!(kinds[5], &TokenKind::Integer("9".into()));
    }

    #[test]
    fn reports_unknown_escapes_at_the_lexer_stage() {
        let compiler = Compiler::new();
        let error = compiler
            .tokenize(&Source::new("main.as", "let value = \"bad\\q\";"))
            .unwrap_err();
        assert_eq!(error.stage(), Some(Stage::Lexer));
        assert!(error.to_string().contains("unknown escape `\\q`"));
    }

    #[test]
    fn tokenizes_labels_and_function_flags() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "@striped loop 'outer {} break 'outer");
        let tokens = compiler.tokenize(&source).unwrap();
        let kinds = tokens.iter().map(Token::kind).collect::<Vec<_>>();
        assert!(kinds.contains(&&TokenKind::At));
        assert!(kinds.contains(&&TokenKind::Identifier("striped".into())));
        assert!(kinds.contains(&&TokenKind::Label("outer".into())));
    }

    #[test]
    fn tokenizes_standard_library_names_as_identifiers() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "print eprint sizeof length format");
        let tokens = compiler.tokenize(&source).unwrap();
        assert_eq!(
            tokens.iter().map(Token::kind).collect::<Vec<_>>(),
            vec![
                &TokenKind::Identifier("print".into()),
                &TokenKind::Identifier("eprint".into()),
                &TokenKind::Identifier("sizeof".into()),
                &TokenKind::Identifier("length".into()),
                &TokenKind::Identifier("format".into()),
            ]
        );
    }

    #[test]
    fn reports_lexical_errors_with_position() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "fn main() { \"unterminated }");
        let error = compiler.tokenize(&source).unwrap_err();
        assert_eq!(error.stage(), Some(Stage::Lexer));
        assert!(error.to_string().contains("[1][13]"));
    }

    #[test]
    fn parses_direct_imports() {
        let compiler = Compiler::new();
        let imports = compiler
            .parse_imports(&Source::new(
                "main.as",
                "use { mygame, standard, memory, engine }",
            ))
            .unwrap();
        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].module, None);
        assert_eq!(
            imports[0]
                .items
                .iter()
                .map(|item| item.name.as_str())
                .collect::<Vec<_>>(),
            vec!["mygame", "standard", "memory", "engine"]
        );
    }

    #[test]
    fn parses_nested_imports() {
        let compiler = Compiler::new();
        let imports = compiler
            .parse_imports(&Source::new(
                "main.as",
                "use math { function, variable { that } } use { memory }",
            ))
            .unwrap();
        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0].module.as_deref(), Some("math"));
        assert_eq!(imports[1].module, None);
    }

    #[test]
    fn parses_a_complete_program() {
        let compiler = Compiler::new();
        let program = compiler
            .parse_program(&Source::new(
                "main.as",
                "mod main use { standard } struct Point { x i32 } fn i32 answer() { return 42 }",
            ))
            .unwrap();
        assert_eq!(
            program.module.as_ref().map(|module| module.name.as_str()),
            Some("main")
        );
        assert_eq!(program.imports.len(), 1);
        assert_eq!(program.structs.len(), 1);
        assert_eq!(program.functions.len(), 1);
    }

    #[test]
    fn parser_rejection_matrix_reports_parse_errors() {
        let compiler = Compiler::new();
        for source in [
            "fn main() {",
            "fn main() { [1, 2;",
            "fn main() { <1, 2;",
            "fn main() { (1, 2;",
            "fn main() { value(1, 2;",
            "fn main() { match value { item { },",
            "use { math, memory",
        ] {
            let error = compiler
                .parse_program(&Source::new("main.as", source))
                .unwrap_err();
            assert_eq!(error.stage(), Some(Stage::Parser));
        }
    }

    #[test]
    fn parser_errors_point_at_the_current_unexpected_token() {
        let compiler = Compiler::new();
        let error = compiler
            .parse_program(&Source::new("main.as", "fn main() { return ) }"))
            .unwrap_err();
        match error {
            Error::Parse { line, column, .. } => assert_eq!((line, column), (1, 20)),
            other => panic!("expected parse error, got {other:?}"),
        }
    }

    #[test]
    fn parser_expression_precedence_and_associativity_are_stable() {
        let compiler = Compiler::new();
        let program = compiler
            .parse_program(&Source::new(
                "main.as",
                "fn main() { 1 + 2 * 3 - 4; 10 - 3 - 2; }",
            ))
            .unwrap();
        let statements = &program.functions[0].body.statements;
        assert!(matches!(
            &statements[0],
            Statement::Expression(Expression::Binary { operator: BinaryOperator::Subtract, left, right })
                if matches!(right.as_ref(), Expression::Integer(value) if value == "4")
                    && matches!(left.as_ref(), Expression::Binary { operator: BinaryOperator::Add, left, right }
                        if matches!(left.as_ref(), Expression::Integer(value) if value == "1")
                            && matches!(right.as_ref(), Expression::Binary { operator: BinaryOperator::Multiply, .. }))
        ));
        assert!(matches!(
            &statements[1],
            Statement::Expression(Expression::Binary { operator: BinaryOperator::Subtract, left, right })
                if matches!(left.as_ref(), Expression::Binary { operator: BinaryOperator::Subtract, .. })
                    && matches!(right.as_ref(), Expression::Integer(value) if value == "2")
        ));
    }

    #[test]
    fn parser_keeps_postfix_range_and_cast_forms_unambiguous() {
        let compiler = Compiler::new();
        let program = compiler
            .parse_program(&Source::new(
                "main.as",
                "fn main() { value.field(1); value -> i8 -> char; 1..3; <1 < 2, 3>; }",
            ))
            .unwrap();
        let statements = &program.functions[0].body.statements;
        assert!(matches!(
            &statements[0],
            Statement::Expression(Expression::Call { function, .. })
                if matches!(function.as_ref(), Expression::Member { .. })
        ));
        assert!(matches!(
            &statements[1],
            Statement::Expression(Expression::Cast { value, target_type })
                if target_type == &vec![TokenKind::Identifier("char".into())]
                    && matches!(value.as_ref(), Expression::Cast { .. })
        ));
        assert!(matches!(
            &statements[2],
            Statement::Expression(Expression::Range {
                inclusive: false,
                ..
            })
        ));
        assert!(matches!(
            &statements[3],
            Statement::Expression(Expression::Vector(values))
                if matches!(&values[0], Expression::Binary { operator: BinaryOperator::Less, .. })
        ));
    }

    #[allow(dead_code)]
    fn _keep_parser_types_reachable_for_tests(
        _: BinaryOperator,
        _: BindingKind,
        _: Expression,
        _: Import,
        _: ModuleDeclaration,
        _: Statement,
    ) {
    }
}
