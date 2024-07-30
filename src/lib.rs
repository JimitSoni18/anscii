use std::collections::HashSet;

use col_rs::{hex_to_rgb, Span, Style, Weight, RGB};
use roxmltree::{Attribute, Document, Node};

// TODO: implement nested divs, and validate for divs inside of spans

pub fn parse(input: &str) -> Result<String, Error> {
	let doc = Document::parse(input)?;
    let mut par_str = String::new();

	for child in doc.root_element().children() {
		let mut slice = String::new();
		something_like_infix_traversal(
			child,
			parse_attr(child.attributes().collect(), Style::default())?,
			&mut slice,
		)?;
        if child.tag_name().name() == "div" {
            par_str.push('\n');
        }
		par_str.push_str(&slice);
	}

	Ok(format!("{par_str}"))
}

fn something_like_infix_traversal<'a, 'input>(
	node: Node<'a, 'input>,
	style: Style,
	styled_string: &mut String,
) -> Result<(), Error> {
	for child in node.children() {
		if child.is_text() {
            let text = child.text().unwrap_or_default().trim_matches(['\t', '\n']);
            // this looks like it could be faster
			// text = text.trim_matches(|c| c == '\t' && c == '\n');
			if !text.is_empty() {
				let span = Span::with_style(text.to_string(), style.clone());
				styled_string.push_str(&format!("{span}"));
			}
		} else if child.is_element() {
            // FIXME: fix unwrap
			let style = parse_attr(child.attributes().collect(), style.clone())?;
			something_like_infix_traversal(child, style, styled_string)?;
		}
	}

    Ok(())
}

fn parse_attr(attrs: Vec<Attribute>, mut style: Style) -> Result<Style, Error> {
	for attr in attrs {
		match attr.name() {
			"fg" => {
				let fg = attr.value();
				if fg == "none" {
					style.reset_fg();
				} else {
					let [r, g, b] = parse_color(fg)?;
					style.set_fg(r, g, b);
				}
			}
			"bg" => {
				let bg = attr.value();
				if bg == "none" {
					style.reset_bg();
				} else {
					let [r, g, b] = parse_color(bg)?;
					style = style.bg(r, g, b);
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
				let underline = modifiers.contains("underline");
				let strike_through = modifiers.contains("strike-through");
				let italic = modifiers.contains("italic");

				if bold && faint {
					return Err(Error::ConflictingModifiers);
				}
				if bold {
					style = style.weight(Weight::Bold);
				}
				if faint {
					style = style.weight(Weight::Faint);
				}

				if underline {
					style = style.underline();
				}

				if strike_through {
					style = style.strike_through();
				}

				if italic {
					style = style.italic();
				}

				if modifiers.contains("reset") {
					if bold || faint || underline || strike_through || italic {
						return Err(Error::ConflictingModifiers);
					}
					style.reset();
				}
			}
			_ => {}
		}
	}

	Ok(style)
}

fn parse_color(code: &str) -> Result<RGB, Error> {
	if let Some(rest) = code.strip_prefix("rgb(") {
		if let Some(rest) = rest.strip_suffix(')') {
			return rest
				.split(',')
				.map(|n| n.trim().parse())
				.collect::<Result<Vec<u8>, std::num::ParseIntError>>()
				.map_err(|_| Error::ColorParseError(code.to_string()))?
				.try_into()
				.map_err(|_| Error::ColorParseError(code.to_string()));
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

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_parse_color() {
		let parsed = parse_color("rgb(213, 33, 54)").unwrap();
		println!("{parsed:?}");
	}
}
