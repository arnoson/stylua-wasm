use stylua_lib::{format_code, OutputVerification};
use wasm_bindgen::prelude::*;
mod config;

#[wasm_bindgen]
pub fn format(
  input: &str,
  config: JsValue,
  output_verification: u8,
) -> Result<String, JsValue> {
  let config = config::from_json(
    config.into_serde().expect("Error while parsing config."),
  );

  let output_verification = match output_verification {
    0 => OutputVerification::Full,
    _ => OutputVerification::None,
  };

  match format_code(input, config, None, output_verification) {
    Ok(output) => Ok(output),
    Err(error) => Err(JsValue::from_str(&error.to_string())),
  }
}
