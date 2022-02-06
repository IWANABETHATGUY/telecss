use insta::assert_debug_snapshot;
use tele_parser::Parser;
use tele_tokenizer::Tokenizer;

#[test]
fn ruleset_with_a_single_decl() {
  let mut tokenizer: Tokenizer = r"  .foo { color: red; }  ".into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(parser.parse());
}

#[test]
fn ruleset_with_a_single_decl_no_trailing_semicolon() {
  let mut tokenizer: Tokenizer = r"  .foo { color: red }  ".into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(parser.parse());
}

#[test]
fn ruleset_with_multi_decls() {
  let mut tokenizer: Tokenizer = r"  .foo { color: red; background: url('bar.png') }  ".into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(parser.parse());
}

#[test]
fn decl_with_important() {
  let mut tokenizer: Tokenizer = r"  .foo { color: red !important; }  ".into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(parser.parse());
}