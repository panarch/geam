use crate::ast::{
    Arg, AssignName, Assignment, BinOp, CallArg, Clause, ClauseGuard, CustomType, Definition, Expr,
    Function, HasLocation, Import, Module, Pattern, Publicity, RecordConstructor,
    RecordConstructorArg, SourceModule, SourceStatement, SpannedString, SrcSpan, Statement,
    TypeAlias, TypeAst,
};
use crate::parse::error::{LexicalError, ParseError, ParseErrorType, parse_error};
use crate::parse::lexer::{Spanned, tokenize};
use crate::parse::token::Token;
use camino::Utf8PathBuf;
use ecow::EcoString;

pub fn parse_module(path: Utf8PathBuf, src: &str) -> Result<SourceModule, ParseError> {
    let tokens = tokenize(src).map_err(parse_lex_error)?;
    Parser::new(path, tokens).parse_module()
}

fn parse_lex_error(error: LexicalError) -> ParseError {
    ParseError {
        location: error.location,
        error: ParseErrorType::LexError { error },
    }
}

struct Parser {
    path: Utf8PathBuf,
    tokens: Vec<Spanned>,
    position: usize,
}

impl Parser {
    fn new(path: Utf8PathBuf, tokens: Vec<Spanned>) -> Self {
        Self {
            path,
            tokens,
            position: 0,
        }
    }

    fn parse_module(&mut self) -> Result<SourceModule, ParseError> {
        let mut definitions = Vec::new();

        self.skip_newlines();
        while !self.at(&Token::EndOfFile) {
            definitions.push(self.parse_definition()?);
            self.skip_newlines();
        }

        let name = self
            .path
            .with_extension("")
            .to_string()
            .trim_start_matches("./")
            .into();

        Ok(Module {
            name,
            path: self.path.clone(),
            documentation: Vec::new(),
            type_info: (),
            definitions,
        })
    }

    fn parse_definition(&mut self) -> Result<Definition<(), Expr>, ParseError> {
        let publicity = if self.at(&Token::Pub) {
            self.bump();
            Publicity::Public
        } else {
            Publicity::Private
        };

        match self.current_token() {
            Token::Import if publicity == Publicity::Private => {
                Ok(Definition::Import(self.parse_import()?))
            }
            Token::Fn => Ok(Definition::Function(self.parse_function(publicity)?)),
            Token::Type => self.parse_type_definition(publicity),
            Token::Const => self.unsupported_current("module constants"),
            Token::Opaque => self.unsupported_current("opaque types"),
            Token::At => self.unsupported_current("attributes"),
            Token::Use => self.unsupported_current("use expressions"),
            Token::Assert => self.unsupported_current("assert expressions"),
            Token::Todo => self.unsupported_current("todo expressions"),
            Token::Panic => self.unsupported_current("panic expressions"),
            Token::Echo => self.unsupported_current("echo expressions"),
            Token::Import => self.unexpected(vec!["a private import".into()]),
            Token::EndOfFile => self.unexpected(vec!["a definition".into()]),
            _ => parse_error(
                ParseErrorType::ExpectedDefinition,
                self.current().location(),
            ),
        }
    }

    fn parse_import(&mut self) -> Result<Import, ParseError> {
        let start = self.expect(&Token::Import, vec!["`import`".into()])?.start;
        let (module_start, mut module, mut end) = self.expect_module_segment()?;

        while self.at(&Token::Slash) {
            self.bump();
            let (_, next, next_end) = self.expect_module_segment()?;
            module.push('/');
            module.push_str(&next);
            end = next_end;
        }

        let alias = if self.at(&Token::As) {
            self.bump();
            let (span, name) = self.expect_name()?;
            end = span.end;
            Some((span, name))
        } else {
            None
        };

        Ok(Import {
            location: SrcSpan::new(start, end.max(module_start)),
            module,
            alias,
        })
    }

    fn parse_type_definition(
        &mut self,
        publicity: Publicity,
    ) -> Result<Definition<(), Expr>, ParseError> {
        let start = self.expect(&Token::Type, vec!["`type`".into()])?.start;
        let (name, parameters) = self.parse_type_name_with_parameters()?;

        if self.at(&Token::Equal) {
            self.expect(&Token::Equal, vec!["`=`".into()])?;
            let alias = self.parse_type()?;
            let end = alias.location().end;
            return Ok(Definition::TypeAlias(TypeAlias {
                location: SrcSpan::new(start, end),
                publicity,
                name,
                parameters,
                alias,
                type_: (),
            }));
        }

        self.expect(&Token::LeftBrace, vec!["`{` or `=`".into()])?;
        let mut constructors = Vec::new();
        self.skip_newlines();

        while !self.at(&Token::RightBrace) {
            if self.at(&Token::EndOfFile) {
                return self.unexpected(vec!["`}`".into()]);
            }
            if self.at(&Token::At) {
                return self.unsupported_current("constructor attributes");
            }
            constructors.push(self.parse_record_constructor()?);
            let has_comma = self.maybe(&Token::Comma).is_some();
            if self.at(&Token::RightBrace) {
                break;
            }
            if has_comma {
                self.skip_newlines();
            } else {
                self.expect_newline_or_right_brace()?;
                self.skip_newlines();
            }
        }

        let end = self.expect(&Token::RightBrace, vec!["`}`".into()])?.end;
        Ok(Definition::CustomType(CustomType {
            location: SrcSpan::new(start, end),
            publicity,
            name,
            parameters,
            constructors,
            type_: (),
        }))
    }

    fn parse_type_name_with_parameters(
        &mut self,
    ) -> Result<(SpannedString, Vec<SpannedString>), ParseError> {
        let name = self.expect_up_name()?;
        let parameters = if self.maybe(&Token::LeftParen).is_some() {
            let mut parameters = Vec::new();
            self.skip_newlines();
            if !self.at(&Token::RightParen) {
                loop {
                    parameters.push(self.expect_name()?);
                    if self.maybe(&Token::Comma).is_none() {
                        break;
                    }
                    self.skip_newlines();
                    if self.at(&Token::RightParen) {
                        break;
                    }
                }
            }
            self.expect(&Token::RightParen, vec!["`)`".into()])?;
            parameters
        } else {
            Vec::new()
        };
        Ok((name, parameters))
    }

    fn parse_record_constructor(&mut self) -> Result<RecordConstructor<()>, ParseError> {
        let (name_span, name) = self.expect_up_name()?;
        let mut arguments = Vec::new();
        let mut end = name_span.end;

        if self.maybe(&Token::LeftParen).is_some() {
            self.skip_newlines();
            if !self.at(&Token::RightParen) {
                loop {
                    arguments.push(self.parse_record_constructor_arg()?);
                    if self.maybe(&Token::Comma).is_none() {
                        break;
                    }
                    self.skip_newlines();
                    if self.at(&Token::RightParen) {
                        break;
                    }
                }
            }
            end = self.expect(&Token::RightParen, vec!["`)`".into()])?.end;
        }

        Ok(RecordConstructor {
            location: SrcSpan::new(name_span.start, end),
            name: (name_span, name),
            arguments,
        })
    }

    fn parse_record_constructor_arg(&mut self) -> Result<RecordConstructorArg<()>, ParseError> {
        let label =
            if matches!(self.current_token(), Token::Name { .. }) && self.next_is(&Token::Colon) {
                let label = self.expect_name()?;
                self.expect(&Token::Colon, vec!["`:`".into()])?;
                Some(label)
            } else {
                None
            };

        let annotation = self.parse_type()?;
        let start = label
            .as_ref()
            .map(|(span, _)| span.start)
            .unwrap_or_else(|| annotation.location().start);
        let end = annotation.location().end;

        Ok(RecordConstructorArg {
            location: SrcSpan::new(start, end),
            label,
            annotation,
            type_: (),
        })
    }

    fn parse_function(&mut self, publicity: Publicity) -> Result<Function<(), Expr>, ParseError> {
        let start = self.expect(&Token::Fn, vec!["`fn`".into()])?.start;
        let name = self.expect_name()?;
        self.expect(&Token::LeftParen, vec!["`(`".into()])?;

        let mut arguments = Vec::new();
        self.skip_newlines();
        if !self.at(&Token::RightParen) {
            loop {
                arguments.push(self.parse_arg()?);
                if self.maybe(&Token::Comma).is_none() {
                    break;
                }
                self.skip_newlines();
                if self.at(&Token::RightParen) {
                    break;
                }
            }
        }

        self.expect(&Token::RightParen, vec!["`)`".into()])?;
        let return_annotation = if self.maybe(&Token::RArrow).is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };

        let open_brace = self.expect(&Token::LeftBrace, vec!["`{`".into()])?;
        let body = self.parse_statements_until_right_brace()?;
        let end = self.expect(&Token::RightBrace, vec!["`}`".into()])?.end;

        Ok(Function {
            location: SrcSpan::new(start, name.0.end),
            body_start: Some(open_brace.start),
            end_position: end,
            name: Some(name),
            arguments,
            body,
            publicity,
            return_annotation,
            return_type: (),
        })
    }

    fn parse_arg(&mut self) -> Result<Arg<()>, ParseError> {
        let (name_span, name) = self.expect_assignable_name()?;
        let annotation = if self.maybe(&Token::Colon).is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };

        let end = annotation
            .as_ref()
            .map(HasLocation::location)
            .map(|span| span.end)
            .unwrap_or(name_span.end);

        Ok(Arg {
            location: SrcSpan::new(name_span.start, end),
            name: (name_span, name),
            annotation,
            type_: (),
        })
    }

    fn parse_statements_until_right_brace(&mut self) -> Result<Vec<SourceStatement>, ParseError> {
        let mut statements = Vec::new();
        self.skip_newlines();

        while !self.at(&Token::RightBrace) {
            if self.at(&Token::EndOfFile) {
                return self.unexpected(vec!["`}`".into()]);
            }
            statements.push(self.parse_statement()?);
            if self.at(&Token::RightBrace) {
                break;
            }
            self.expect_newline_or_right_brace()?;
            self.skip_newlines();
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<SourceStatement, ParseError> {
        match self.current_token() {
            Token::Let => self.parse_assignment(),
            Token::Use => self.unsupported_current("use expressions"),
            Token::Assert => self.unsupported_current("assert expressions"),
            _ => Ok(Statement::Expression(self.parse_expression()?)),
        }
    }

    fn parse_assignment(&mut self) -> Result<SourceStatement, ParseError> {
        let start = self.expect(&Token::Let, vec!["`let`".into()])?.start;
        if self.at(&Token::Assert) {
            return self.unsupported_current("let assert");
        }

        let pattern = self.parse_pattern()?;
        let annotation = if self.maybe(&Token::Colon).is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(&Token::Equal, vec!["`=`".into()])?;
        let value = self.parse_expression()?;
        let end = value.location().end;

        Ok(Statement::Assignment(Box::new(Assignment {
            location: SrcSpan::new(start, end),
            pattern,
            annotation,
            value,
        })))
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_pipeline()
    }

    fn parse_pipeline(&mut self) -> Result<Expr, ParseError> {
        let mut expressions = vec![self.parse_binary(1)?];

        while self.maybe(&Token::Pipe).is_some() {
            expressions.push(self.parse_binary(1)?);
        }

        if expressions.len() == 1 {
            Ok(expressions.pop().expect("one expression was parsed"))
        } else {
            Ok(Expr::PipeLine { expressions })
        }
    }

    fn parse_binary(&mut self, min_precedence: u8) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;

        while let Some((operator, precedence, operator_start)) = self.current_bin_op() {
            if precedence < min_precedence {
                break;
            }

            self.bump();
            let right = self.parse_binary(precedence + 1)?;
            let location = left.location().merge(&right.location());
            left = Expr::BinOp {
                location,
                operator,
                operator_start,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.current_token() {
            Token::Bang => {
                let start = self.bump().start;
                let value = self.parse_unary()?;
                let location = SrcSpan::new(start, value.location().end);
                Ok(Expr::NegateBool {
                    location,
                    value: Box::new(value),
                })
            }
            Token::Minus => {
                let start = self.bump().start;
                let value = self.parse_unary()?;
                let location = SrcSpan::new(start, value.location().end);
                Ok(Expr::NegateInt {
                    location,
                    value: Box::new(value),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expression = self.parse_primary()?;

        loop {
            match self.current_token() {
                Token::LeftParen => {
                    let open_parenthesis = self.bump().start;
                    let arguments = self.parse_call_args()?;
                    let end = self.expect(&Token::RightParen, vec!["`)`".into()])?.end;
                    expression = Expr::Call {
                        location: SrcSpan::new(expression.location().start, end),
                        fun: Box::new(expression),
                        arguments,
                        open_parenthesis,
                    };
                }
                Token::Dot => {
                    self.bump();
                    match self.current_token().clone() {
                        Token::Name { name } | Token::UpName { name } => {
                            let label = self.bump();
                            expression = Expr::FieldAccess {
                                location: SrcSpan::new(expression.location().start, label.end),
                                label_location: label.location(),
                                label: name,
                                container: Box::new(expression),
                            };
                        }
                        Token::Int { value, .. } => {
                            let index_token = self.bump();
                            let index = value.parse::<u64>().map_err(|_| ParseError {
                                error: ParseErrorType::ExpectedName,
                                location: index_token.location(),
                            })?;
                            expression = Expr::TupleIndex {
                                location: SrcSpan::new(
                                    expression.location().start,
                                    index_token.end,
                                ),
                                index,
                                tuple: Box::new(expression),
                            };
                        }
                        _ => {
                            return parse_error(
                                ParseErrorType::ExpectedName,
                                self.current().location(),
                            );
                        }
                    }
                }
                _ => break,
            }
        }

        Ok(expression)
    }

    fn parse_call_args(&mut self) -> Result<Vec<CallArg<Expr>>, ParseError> {
        let mut arguments = Vec::new();
        self.skip_newlines();

        if self.at(&Token::RightParen) {
            return Ok(arguments);
        }

        loop {
            if self.at(&Token::DotDot) {
                return self.unsupported_current("record update");
            }

            arguments.push(self.parse_call_arg()?);
            self.skip_newlines();

            if self.at(&Token::RightParen) {
                break;
            }

            if self.maybe(&Token::Comma).is_none() {
                break;
            }
            self.skip_newlines();
            if self.at(&Token::RightParen) {
                break;
            }
        }

        Ok(arguments)
    }

    fn parse_call_arg(&mut self) -> Result<CallArg<Expr>, ParseError> {
        let label =
            if matches!(self.current_token(), Token::Name { .. }) && self.next_is(&Token::Colon) {
                let label = self.expect_name()?;
                self.expect(&Token::Colon, vec!["`:`".into()])?;
                Some(label)
            } else {
                None
            };

        let value = self.parse_expression()?;
        let start = label
            .as_ref()
            .map(|(span, _)| span.start)
            .unwrap_or_else(|| value.location().start);
        let end = value.location().end;

        Ok(CallArg {
            location: SrcSpan::new(start, end),
            label,
            value,
        })
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.current_token().clone() {
            Token::Int { value, int_value } => {
                let token = self.bump();
                Ok(Expr::Int {
                    location: token.location(),
                    value,
                    int_value,
                })
            }
            Token::Float { value } => {
                let token = self.bump();
                Ok(Expr::Float {
                    location: token.location(),
                    value,
                })
            }
            Token::String { value } => {
                let token = self.bump();
                Ok(Expr::String {
                    location: token.location(),
                    value,
                })
            }
            Token::Name { name } | Token::UpName { name } => {
                let token = self.bump();
                Ok(Expr::Var {
                    location: token.location(),
                    name,
                })
            }
            Token::LeftBrace => self.parse_block_expression(),
            Token::LeftSquare => self.parse_list_expression(),
            Token::Hash => self.parse_tuple_expression(),
            Token::LeftParen => {
                self.bump();
                self.skip_newlines();
                let expression = self.parse_expression()?;
                self.skip_newlines();
                self.expect(&Token::RightParen, vec!["`)`".into()])?;
                Ok(expression)
            }
            Token::Case => self.parse_case_expression(),
            Token::Fn => self.unsupported_current("anonymous functions"),
            Token::Todo => self.unsupported_current("todo expressions"),
            Token::Panic => self.unsupported_current("panic expressions"),
            Token::Echo => self.unsupported_current("echo expressions"),
            Token::LtLt => self.unsupported_current("bit arrays"),
            Token::Use => self.unsupported_current("use expressions"),
            Token::Assert => self.unsupported_current("assert expressions"),
            Token::Const => self.unsupported_current("module constants"),
            Token::Opaque => self.unsupported_current("opaque types"),
            Token::At => self.unsupported_current("attributes"),
            Token::EndOfFile => {
                parse_error(ParseErrorType::ExpectedExpr, self.current().location())
            }
            _ => parse_error(ParseErrorType::ExpectedExpr, self.current().location()),
        }
    }

    fn parse_block_expression(&mut self) -> Result<Expr, ParseError> {
        let start = self.expect(&Token::LeftBrace, vec!["`{`".into()])?.start;
        let statements = self.parse_statements_until_right_brace()?;
        let end = self.expect(&Token::RightBrace, vec!["`}`".into()])?.end;
        Ok(Expr::Block {
            location: SrcSpan::new(start, end),
            statements,
        })
    }

    fn parse_list_expression(&mut self) -> Result<Expr, ParseError> {
        let start = self.expect(&Token::LeftSquare, vec!["`[`".into()])?.start;
        let mut elements = Vec::new();
        self.skip_newlines();

        if !self.at(&Token::RightSquare) {
            loop {
                if self.at(&Token::DotDot) {
                    return self.unsupported_current("list spread");
                }
                elements.push(self.parse_expression()?);
                if self.maybe(&Token::Comma).is_none() {
                    break;
                }
                self.skip_newlines();
                if self.at(&Token::RightSquare) {
                    break;
                }
                if self.at(&Token::DotDot) {
                    return self.unsupported_current("list spread");
                }
            }
        }

        let end = self.expect(&Token::RightSquare, vec!["`]`".into()])?.end;
        Ok(Expr::List {
            location: SrcSpan::new(start, end),
            elements,
        })
    }

    fn parse_tuple_expression(&mut self) -> Result<Expr, ParseError> {
        let start = self.expect(&Token::Hash, vec!["`#`".into()])?.start;
        self.expect(&Token::LeftParen, vec!["`(`".into()])?;
        let mut elements = Vec::new();
        self.skip_newlines();

        if !self.at(&Token::RightParen) {
            loop {
                elements.push(self.parse_expression()?);
                if self.maybe(&Token::Comma).is_none() {
                    break;
                }
                self.skip_newlines();
                if self.at(&Token::RightParen) {
                    break;
                }
            }
        }

        let end = self.expect(&Token::RightParen, vec!["`)`".into()])?.end;
        Ok(Expr::Tuple {
            location: SrcSpan::new(start, end),
            elements,
        })
    }

    fn parse_case_expression(&mut self) -> Result<Expr, ParseError> {
        let start = self.expect(&Token::Case, vec!["`case`".into()])?.start;
        let mut subjects = Vec::new();

        loop {
            subjects.push(self.parse_expression()?);
            self.skip_newlines();

            if self.at(&Token::LeftBrace) {
                break;
            }

            if self.maybe(&Token::Comma).is_none() {
                break;
            }
            self.skip_newlines();
        }

        self.expect(&Token::LeftBrace, vec!["`{`".into()])?;
        let mut clauses = Vec::new();
        self.skip_newlines();

        while !self.at(&Token::RightBrace) {
            if self.at(&Token::EndOfFile) {
                return self.unexpected(vec!["`}`".into()]);
            }
            clauses.push(self.parse_case_clause()?);
            if self.at(&Token::RightBrace) {
                break;
            }
            self.expect_newline_or_right_brace()?;
            self.skip_newlines();
        }

        let end = self.expect(&Token::RightBrace, vec!["`}`".into()])?.end;
        Ok(Expr::Case {
            location: SrcSpan::new(start, end),
            subjects,
            clauses,
        })
    }

    fn parse_case_clause(&mut self) -> Result<Clause<Expr, ()>, ParseError> {
        let pattern = self.parse_pattern_list()?;
        let mut alternative_patterns = Vec::new();

        while self.maybe(&Token::Vbar).is_some() {
            alternative_patterns.push(self.parse_pattern_list()?);
        }

        let guard = if self.maybe(&Token::If).is_some() {
            let expression = self.parse_expression()?;
            Some(ClauseGuard {
                location: expression.location(),
                expression,
            })
        } else {
            None
        };

        self.expect(&Token::RArrow, vec!["`->`".into()])?;
        let then = self.parse_expression()?;
        let start = pattern
            .first()
            .map(HasLocation::location)
            .map(|span| span.start)
            .unwrap_or(then.location().start);
        let end = then.location().end;

        Ok(Clause {
            location: SrcSpan::new(start, end),
            pattern,
            alternative_patterns,
            guard,
            then,
        })
    }

    fn parse_pattern_list(&mut self) -> Result<Vec<Pattern<()>>, ParseError> {
        let mut patterns = vec![self.parse_pattern()?];

        while self.maybe(&Token::Comma).is_some() {
            self.skip_newlines();
            patterns.push(self.parse_pattern()?);
        }

        Ok(patterns)
    }

    fn parse_pattern(&mut self) -> Result<Pattern<()>, ParseError> {
        let pattern = match self.current_token().clone() {
            Token::Name { name } => {
                let token = self.bump();
                if self.at(&Token::Concatenate) {
                    return self.unsupported_current(
                        "variable on the left side of a string prefix pattern",
                    );
                }
                if self.maybe(&Token::Dot).is_some() {
                    let (constructor_span, constructor_name) = self.expect_up_name()?;
                    self.finish_constructor_pattern(
                        Some((name, token.location())),
                        constructor_span,
                        constructor_name,
                    )?
                } else {
                    Pattern::Variable {
                        location: token.location(),
                        name,
                        type_: (),
                    }
                }
            }
            Token::UpName { name } => {
                let token = self.bump();
                self.finish_constructor_pattern(None, token.location(), name)?
            }
            Token::DiscardName { name } => {
                let token = self.bump();
                if self.at(&Token::Concatenate) {
                    return self.unsupported_current(
                        "discard on the left side of a string prefix pattern",
                    );
                }
                Pattern::Discard {
                    name,
                    location: token.location(),
                    type_: (),
                }
            }
            Token::String { value } => self.parse_string_pattern(value)?,
            Token::Int { value, int_value } => {
                let token = self.bump();
                Pattern::Int {
                    location: token.location(),
                    value,
                    int_value,
                }
            }
            Token::Float { value } => {
                let token = self.bump();
                Pattern::Float {
                    location: token.location(),
                    value,
                }
            }
            Token::Hash => self.parse_tuple_pattern()?,
            Token::LeftSquare => self.parse_list_pattern()?,
            Token::LtLt => return self.unsupported_current("bit array patterns"),
            _ => return parse_error(ParseErrorType::ExpectedPattern, self.current().location()),
        };

        if self.maybe(&Token::As).is_some() {
            let (location, name) = self.expect_name()?;
            Ok(Pattern::Assign {
                name,
                location,
                pattern: Box::new(pattern),
            })
        } else {
            Ok(pattern)
        }
    }

    fn parse_string_pattern(&mut self, value: EcoString) -> Result<Pattern<()>, ParseError> {
        let token = self.bump();

        if self.maybe(&Token::As).is_some() {
            let (name_span, name) = self.expect_name()?;
            if self.maybe(&Token::Concatenate).is_some() {
                let (right_location, right_side_assignment) = self.expect_assign_name()?;
                return Ok(Pattern::StringPrefix {
                    location: SrcSpan::new(token.start, right_location.end),
                    left_location: SrcSpan::new(token.start, name_span.end),
                    left_side_assignment: Some((name_span, name)),
                    right_location,
                    left_side_string: value,
                    right_side_assignment,
                });
            }

            return Ok(Pattern::Assign {
                name,
                location: name_span,
                pattern: Box::new(Pattern::String {
                    location: token.location(),
                    value,
                }),
            });
        }

        if self.maybe(&Token::Concatenate).is_some() {
            let (right_location, right_side_assignment) = self.expect_assign_name()?;
            return Ok(Pattern::StringPrefix {
                location: SrcSpan::new(token.start, right_location.end),
                left_location: token.location(),
                left_side_assignment: None,
                right_location,
                left_side_string: value,
                right_side_assignment,
            });
        }

        Ok(Pattern::String {
            location: token.location(),
            value,
        })
    }

    fn finish_constructor_pattern(
        &mut self,
        module: Option<(EcoString, SrcSpan)>,
        name_location: SrcSpan,
        name: EcoString,
    ) -> Result<Pattern<()>, ParseError> {
        let mut arguments = Vec::new();
        let mut end = name_location.end;

        if self.maybe(&Token::LeftParen).is_some() {
            self.skip_newlines();
            if !self.at(&Token::RightParen) {
                loop {
                    arguments.push(self.parse_pattern_call_arg()?);
                    if self.maybe(&Token::Comma).is_none() {
                        break;
                    }
                    self.skip_newlines();
                    if self.at(&Token::RightParen) {
                        break;
                    }
                }
            }
            end = self.expect(&Token::RightParen, vec!["`)`".into()])?.end;
        }

        let start = module
            .as_ref()
            .map(|(_, span)| span.start)
            .unwrap_or(name_location.start);

        Ok(Pattern::Constructor {
            location: SrcSpan::new(start, end),
            name_location,
            name,
            arguments,
            module,
            type_: (),
        })
    }

    fn parse_pattern_call_arg(&mut self) -> Result<CallArg<Pattern<()>>, ParseError> {
        let label =
            if matches!(self.current_token(), Token::Name { .. }) && self.next_is(&Token::Colon) {
                let label = self.expect_name()?;
                self.expect(&Token::Colon, vec!["`:`".into()])?;
                Some(label)
            } else {
                None
            };

        let value = self.parse_pattern()?;
        let start = label
            .as_ref()
            .map(|(span, _)| span.start)
            .unwrap_or_else(|| value.location().start);
        let end = value.location().end;

        Ok(CallArg {
            location: SrcSpan::new(start, end),
            label,
            value,
        })
    }

    fn parse_tuple_pattern(&mut self) -> Result<Pattern<()>, ParseError> {
        let start = self.expect(&Token::Hash, vec!["`#`".into()])?.start;
        self.expect(&Token::LeftParen, vec!["`(`".into()])?;
        let mut elements = Vec::new();
        self.skip_newlines();

        if !self.at(&Token::RightParen) {
            loop {
                elements.push(self.parse_pattern()?);
                if self.maybe(&Token::Comma).is_none() {
                    break;
                }
                self.skip_newlines();
                if self.at(&Token::RightParen) {
                    break;
                }
            }
        }

        let end = self.expect(&Token::RightParen, vec!["`)`".into()])?.end;
        Ok(Pattern::Tuple {
            location: SrcSpan::new(start, end),
            elements,
        })
    }

    fn parse_list_pattern(&mut self) -> Result<Pattern<()>, ParseError> {
        let start = self.expect(&Token::LeftSquare, vec!["`[`".into()])?.start;
        let mut elements = Vec::new();
        self.skip_newlines();

        if !self.at(&Token::RightSquare) {
            loop {
                if self.at(&Token::DotDot) {
                    return self.unsupported_current("list pattern spread");
                }
                elements.push(self.parse_pattern()?);
                if self.maybe(&Token::Comma).is_none() {
                    break;
                }
                self.skip_newlines();
                if self.at(&Token::RightSquare) {
                    break;
                }
                if self.at(&Token::DotDot) {
                    return self.unsupported_current("list pattern spread");
                }
            }
        }

        let end = self.expect(&Token::RightSquare, vec!["`]`".into()])?.end;
        Ok(Pattern::List {
            location: SrcSpan::new(start, end),
            elements,
            type_: (),
        })
    }

    fn parse_type(&mut self) -> Result<TypeAst, ParseError> {
        if self.at(&Token::Fn) {
            return self.parse_fn_type();
        }

        self.parse_type_atom()
    }

    fn parse_fn_type(&mut self) -> Result<TypeAst, ParseError> {
        let start = self.expect(&Token::Fn, vec!["`fn`".into()])?.start;
        self.expect(&Token::LeftParen, vec!["`(`".into()])?;
        let mut arguments = Vec::new();
        self.skip_newlines();

        if !self.at(&Token::RightParen) {
            loop {
                arguments.push(self.parse_type()?);
                if self.maybe(&Token::Comma).is_none() {
                    break;
                }
                self.skip_newlines();
                if self.at(&Token::RightParen) {
                    break;
                }
            }
        }

        self.expect(&Token::RightParen, vec!["`)`".into()])?;
        self.expect(&Token::RArrow, vec!["`->`".into()])?;
        let return_ = self.parse_type()?;
        let end = return_.location().end;

        Ok(TypeAst::Fn {
            location: SrcSpan::new(start, end),
            arguments,
            return_: Box::new(return_),
        })
    }

    fn parse_type_atom(&mut self) -> Result<TypeAst, ParseError> {
        match self.current_token().clone() {
            Token::Name { name } => {
                let token = self.bump();
                if self.maybe(&Token::Dot).is_some() {
                    let qualified_name = self.expect_up_name()?;
                    self.finish_type_constructor(Some((name, token.location())), qualified_name)
                } else {
                    Ok(TypeAst::Var {
                        location: token.location(),
                        name,
                    })
                }
            }
            Token::UpName { name } => {
                let token = self.bump();
                self.finish_type_constructor(None, (token.location(), name))
            }
            Token::DiscardName { name } => {
                let token = self.bump();
                Ok(TypeAst::Hole {
                    location: token.location(),
                    name,
                })
            }
            Token::Hash => self.parse_tuple_type(),
            _ => parse_error(ParseErrorType::ExpectedType, self.current().location()),
        }
    }

    fn finish_type_constructor(
        &mut self,
        module: Option<(EcoString, SrcSpan)>,
        name: SpannedString,
    ) -> Result<TypeAst, ParseError> {
        let mut arguments = Vec::new();
        let mut end = name.0.end;

        if self.maybe(&Token::LeftParen).is_some() {
            self.skip_newlines();
            if !self.at(&Token::RightParen) {
                loop {
                    arguments.push(self.parse_type()?);
                    if self.maybe(&Token::Comma).is_none() {
                        break;
                    }
                    self.skip_newlines();
                    if self.at(&Token::RightParen) {
                        break;
                    }
                }
            }
            end = self.expect(&Token::RightParen, vec!["`)`".into()])?.end;
        }

        let start = module
            .as_ref()
            .map(|(_, span)| span.start)
            .unwrap_or(name.0.start);
        Ok(TypeAst::Constructor {
            location: SrcSpan::new(start, end),
            module,
            name,
            arguments,
        })
    }

    fn parse_tuple_type(&mut self) -> Result<TypeAst, ParseError> {
        let start = self.expect(&Token::Hash, vec!["`#`".into()])?.start;
        self.expect(&Token::LeftParen, vec!["`(`".into()])?;
        let mut elements = Vec::new();
        self.skip_newlines();

        if !self.at(&Token::RightParen) {
            loop {
                elements.push(self.parse_type()?);
                if self.maybe(&Token::Comma).is_none() {
                    break;
                }
                self.skip_newlines();
                if self.at(&Token::RightParen) {
                    break;
                }
            }
        }

        let end = self.expect(&Token::RightParen, vec!["`)`".into()])?.end;
        Ok(TypeAst::Tuple {
            location: SrcSpan::new(start, end),
            elements,
        })
    }

    fn current_bin_op(&self) -> Option<(BinOp, u8, u32)> {
        let operator = match self.current_token() {
            Token::VbarVbar => BinOp::Or,
            Token::AmperAmper => BinOp::And,
            Token::EqualEqual => BinOp::Eq,
            Token::NotEqual => BinOp::NotEq,
            Token::Less => BinOp::LtInt,
            Token::LessEqual => BinOp::LtEqInt,
            Token::LessDot => BinOp::LtFloat,
            Token::LessEqualDot => BinOp::LtEqFloat,
            Token::Greater => BinOp::GtInt,
            Token::GreaterEqual => BinOp::GtEqInt,
            Token::GreaterDot => BinOp::GtFloat,
            Token::GreaterEqualDot => BinOp::GtEqFloat,
            Token::Plus => BinOp::AddInt,
            Token::PlusDot => BinOp::AddFloat,
            Token::Minus => BinOp::SubInt,
            Token::MinusDot => BinOp::SubFloat,
            Token::Star => BinOp::MultInt,
            Token::StarDot => BinOp::MultFloat,
            Token::Slash => BinOp::DivInt,
            Token::SlashDot => BinOp::DivFloat,
            Token::Percent => BinOp::RemainderInt,
            Token::Concatenate => BinOp::Concatenate,
            _ => return None,
        };
        Some((operator, operator.precedence(), self.current().start))
    }

    fn expect_module_segment(&mut self) -> Result<(u32, EcoString, u32), ParseError> {
        match self.current_token().clone() {
            Token::Name { name } | Token::UpName { name } => {
                let token = self.bump();
                Ok((token.start, name, token.end))
            }
            _ => parse_error(ParseErrorType::ExpectedName, self.current().location()),
        }
    }

    fn expect_name(&mut self) -> Result<SpannedString, ParseError> {
        match self.current_token().clone() {
            Token::Name { name } => {
                let token = self.bump();
                Ok((token.location(), name))
            }
            _ => parse_error(ParseErrorType::ExpectedName, self.current().location()),
        }
    }

    fn expect_up_name(&mut self) -> Result<SpannedString, ParseError> {
        match self.current_token().clone() {
            Token::UpName { name } => {
                let token = self.bump();
                Ok((token.location(), name))
            }
            _ => parse_error(ParseErrorType::ExpectedUpName, self.current().location()),
        }
    }

    fn expect_assignable_name(&mut self) -> Result<SpannedString, ParseError> {
        match self.current_token().clone() {
            Token::Name { name } | Token::DiscardName { name } => {
                let token = self.bump();
                Ok((token.location(), name))
            }
            _ => parse_error(ParseErrorType::ExpectedName, self.current().location()),
        }
    }

    fn expect_assign_name(&mut self) -> Result<(SrcSpan, AssignName), ParseError> {
        match self.current_token().clone() {
            Token::Name { name } => {
                let token = self.bump();
                let location = token.location();
                Ok((location, AssignName::Variable((location, name))))
            }
            Token::DiscardName { name } => {
                let token = self.bump();
                let location = token.location();
                Ok((location, AssignName::Discard((location, name))))
            }
            _ => parse_error(ParseErrorType::ExpectedName, self.current().location()),
        }
    }

    fn expect(
        &mut self,
        expected: &Token,
        expected_text: Vec<EcoString>,
    ) -> Result<Spanned, ParseError> {
        if self.at(expected) {
            Ok(self.bump())
        } else {
            self.unexpected(expected_text)
        }
    }

    fn unsupported_current<T>(&self, syntax: &str) -> Result<T, ParseError> {
        parse_error(
            ParseErrorType::UnsupportedSyntax {
                syntax: syntax.into(),
            },
            self.current().location(),
        )
    }

    fn unexpected<T>(&self, expected: Vec<EcoString>) -> Result<T, ParseError> {
        parse_error(
            ParseErrorType::UnexpectedToken {
                token: self.current_token().clone(),
                expected,
            },
            self.current().location(),
        )
    }

    fn skip_newlines(&mut self) {
        while self.at(&Token::NewLine) {
            self.bump();
        }
    }

    fn consume_newlines(&mut self) -> bool {
        let mut consumed = false;
        while self.at(&Token::NewLine) {
            consumed = true;
            self.bump();
        }
        consumed
    }

    fn expect_newline_or_right_brace(&mut self) -> Result<(), ParseError> {
        if self.at(&Token::RightBrace) || self.consume_newlines() {
            Ok(())
        } else {
            self.unexpected(vec!["a newline or `}`".into()])
        }
    }

    fn at(&self, expected: &Token) -> bool {
        self.current_token().same_variant(expected)
    }

    fn next_is(&self, expected: &Token) -> bool {
        self.next_token().same_variant(expected)
    }

    fn maybe(&mut self, expected: &Token) -> Option<Spanned> {
        if self.at(expected) {
            Some(self.bump())
        } else {
            None
        }
    }

    fn bump(&mut self) -> Spanned {
        let token = self.current().clone();
        if !matches!(token.token, Token::EndOfFile) {
            self.position += 1;
        }
        token
    }

    fn current(&self) -> &Spanned {
        self.tokens
            .get(self.position)
            .unwrap_or_else(|| self.tokens.last().expect("parser has no EOF token"))
    }

    fn current_token(&self) -> &Token {
        &self.current().token
    }

    fn next_token(&self) -> &Token {
        self.tokens
            .get(self.position + 1)
            .map(|token| &token.token)
            .unwrap_or_else(|| &self.current().token)
    }
}

#[cfg(test)]
mod tests;
