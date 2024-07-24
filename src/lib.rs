use roxmltree::Node;
use col_rs::Style;

pub fn parse(input: &str) -> Result<String, Error> {
	let doc = roxmltree::Document::parse(input)?;

	Ok(format!("{:#?}", doc))
}

#[allow(unused)]
fn something_like_infix_traversal<'a, 'input>(
	node: Node<'a, 'input>,
	weight: col_rs::Weight,
    style: &Style,
) -> Vec<Node<'a, 'input>> {
	for child in node.children() {
		if child.is_text() {
		} else if child.is_element() {
		}
	}
	todo!()
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error(transparent)]
	ParseError(#[from] roxmltree::Error),
	#[error("spec error")]
	SpecError,
}
