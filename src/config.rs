use serde::Deserialize;
use stylua_lib::{Config, IndentType, LineEndings, QuoteStyle};

#[derive(Deserialize)]
pub struct JsConfig {
  pub column_width: Option<usize>,
  pub line_endings: Option<u8>,
  pub indent_type: Option<u8>,
  pub indent_width: Option<usize>,
  pub quote_style: Option<u8>,
  pub no_call_parentheses: Option<bool>,
}

pub fn from_json(config: JsConfig) -> Config {
  let line_endings = match config.line_endings {
    Some(0) => LineEndings::Unix,
    Some(1) => LineEndings::Windows,
    _ => LineEndings::default(),
  };
  let indent_type = match config.indent_type {
    Some(0) => IndentType::Spaces,
    Some(1) => IndentType::Tabs,
    _ => IndentType::default(),
  };

  let quote_style = match config.indent_type {
    Some(0) => QuoteStyle::AutoPreferDouble,
    Some(1) => QuoteStyle::AutoPreferSingle,
    Some(2) => QuoteStyle::ForceDouble,
    Some(3) => QuoteStyle::ForceSingle,
    _ => QuoteStyle::default(),
  };

  Config::default()
    .with_column_width(config.column_width.unwrap_or(120))
    .with_line_endings(line_endings)
    .with_indent_type(indent_type)
    .with_indent_width(config.indent_width.unwrap_or(4))
    .with_quote_style(quote_style)
    .with_no_call_parentheses(config.no_call_parentheses.unwrap_or(false))
}
