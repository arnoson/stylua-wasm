use serde::Deserialize;
use stylua_lib::Config;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
pub struct JsConfig {
  pub column_width: Option<usize>,
  pub line_endings: Option<u8>,
  pub indent_type: Option<u8>,
  pub indent_width: Option<usize>,
  pub quote_style: Option<u8>,
  pub no_call_parentheses: Option<bool>,
}

#[wasm_bindgen]
#[repr(u8)]
pub enum IndentType {
  Tabs,
  Spaces,
}

#[wasm_bindgen]
#[repr(u8)]
pub enum LineEndings {
  Unix,
  Windows,
}

#[wasm_bindgen]
#[repr(u8)]
pub enum QuoteStyle {
  AutoPreferDouble,
  AutoPreferSingle,
  ForceDouble,
  ForceSingle,
}

pub fn from_json(config: JsConfig) -> Config {
  let line_endings = match config.line_endings {
    Some(0) => stylua_lib::LineEndings::Unix,
    Some(1) => stylua_lib::LineEndings::Windows,
    _ => stylua_lib::LineEndings::default(),
  };
  let indent_type = match config.indent_type {
    Some(0) => stylua_lib::IndentType::Tabs,
    Some(1) => stylua_lib::IndentType::Spaces,
    _ => stylua_lib::IndentType::default(),
  };

  let quote_style = match config.quote_style {
    Some(0) => stylua_lib::QuoteStyle::AutoPreferDouble,
    Some(1) => stylua_lib::QuoteStyle::AutoPreferSingle,
    Some(2) => stylua_lib::QuoteStyle::ForceDouble,
    Some(3) => stylua_lib::QuoteStyle::ForceSingle,
    _ => stylua_lib::QuoteStyle::default(),
  };

  Config::default()
    .with_column_width(config.column_width.unwrap_or(120))
    .with_line_endings(line_endings)
    .with_indent_width(config.indent_width.unwrap_or(4))
    .with_indent_type(indent_type)
    .with_quote_style(quote_style)
    .with_no_call_parentheses(config.no_call_parentheses.unwrap_or(false))
}
