use std::collections::HashSet;

use col_rs::{hex_to_rgb, Span, Style, Weight, RGB};
use roxmltree::{Attribute, Node, Document};

pub fn parse(input: &str) -> Result<String, Error> {
	let doc = Document::parse(input)?;

	Ok(format!("{:#?}", doc))
}

#[allow(unused)]
fn something_like_infix_traversal<'a, 'input>(
	node: Node<'a, 'input>,
	weight: Weight,
	style: Style,
) -> Vec<Node<'a, 'input>> {
	for child in node.children() {
		if child.is_text() {
		} else if child.is_element() {
			parse_attr(child.attributes().collect());
		}
	}
	todo!()
}

fn parse_attr(attrs: Vec<Attribute>) -> Result<Style, Error> {
	let mut style = Style::default();
	for attr in attrs {
		match attr.name() {
			"fg" => {
				// TODO: add reset fg option
				let fg = attr.value();
				if fg == "none" {
				} else {
					style = style.fg(parse_color(fg)?);
				}
			}
			"bg" => {
				// TODO: add reset bg option
				let bg = attr.value();
				if bg == "none" {
				} else {
					style = style.bg(parse_color(attr.value())?);
				}
			}
			// TODO: use strum macros
			// on second thought, maybe that complexity is not required here
			"modifiers" => {
				let modifiers = attr
					.value()
					.split(',')
					.map(str::trim)
					.collect::<HashSet<&str>>();
				let bold = modifiers.contains("bold");
				let faint = modifiers.contains("faint");
				if bold && faint {
					return Err(Error::ConflictingModifiers);
				}
				if bold {
					style = style.weight(Weight::Bold);
				}
				if faint {
					style = style.weight(Weight::Faint);
				}

				if modifiers.contains("underline") {
					style = style.underline();
				}
				if modifiers.contains("strike-through") {
					style = style.strike_through();
				}
			}
			other => {
				eprintln!(
					"{}",
					Span::new(format!("unknown modifier: {other}")).fg((229, 192, 123))
				);
			}
		}
	}

	Ok(style)
}

fn parse_color(code: &str) -> Result<RGB, Error> {
	if let Some(rest) = code.strip_prefix("rgb(") {
		if let Some(rest) = rest.strip_suffix(')') {
			let colors: Vec<u8> = rest
				.split(',')
				.map(|n| n.trim().parse())
				.collect::<Result<Vec<u8>, std::num::ParseIntError>>()
				.map_err(|_| Error::ColorParseError(code.to_string()))?;
			if colors.len() == 3 {
				return Ok((colors[0], colors[1], colors[2]));
			}
		}
		return Err(Error::ColorParseError(code.to_string()));
	} else {
		return hex_to_rgb(code).map_err(|_| Error::ColorParseError(code.to_string()));
	}
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error(transparent)]
	TreeParseError(#[from] roxmltree::Error),
	#[error("unable to parse color: {0}")]
	ColorParseError(String),
    #[error("conflicting modifiers")]
	ConflictingModifiers,
}
