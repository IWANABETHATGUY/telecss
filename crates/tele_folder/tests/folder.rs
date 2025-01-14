use insta::assert_debug_snapshot;
use tele_folder::Folder;
use tele_parser::{DeclarationNode, Parser, StyleSheetNode};
use tele_tokenizer::Tokenizer;

struct Renamer;
impl Folder for Renamer {
  fn fold_decl_node(&self, decl_node: &DeclarationNode) -> DeclarationNode {
    let mut new_decl_node = DeclarationNode::default();
    new_decl_node.loc = decl_node.loc;
    new_decl_node.name = format!("prefix-{}", decl_node.name);
    new_decl_node.value = decl_node.value.clone();
    new_decl_node.important = decl_node.important;

    new_decl_node
  }
}

fn transform(ast: &StyleSheetNode, folder: impl Folder) -> StyleSheetNode {
  folder.fold_ss_node(ast)
}

#[test]
fn ruleset_with_a_single_decl() {
  let mut tokenizer: Tokenizer = r"  .foo { color: red; }  ".into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(transform(&parser.parse().unwrap(), Renamer));
}
