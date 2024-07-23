pub fn parse(input: &str) -> Result<String, Error> {
	let doc = roxmltree::Document::parse(input)?;

	Ok(format!("{:#?}", doc))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error(transparent)]
	ParseError(#[from] roxmltree::Error),
	#[error("spec error")]
	SpecError,
}

