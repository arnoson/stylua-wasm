use stylua_lib::format_code;
use wasm_bindgen::prelude::*;
mod config;

#[wasm_bindgen]
#[repr(u8)]
pub enum OutputVerification {
  Full,
  None,
}

#[wasm_bindgen]
pub fn format(
  input: &str,
  config: JsValue,
  output_verification: Option<OutputVerification>,
) -> Result<String, JsValue> {
  let config = config::from_json(
    config.into_serde().expect("Error while parsing config."),
  );

  let output_verification = match output_verification {
    Some(OutputVerification::Full) => stylua_lib::OutputVerification::Full,
    _ => stylua_lib::OutputVerification::None,
  };

  match format_code(input, config, None, output_verification) {
    Ok(output) => Ok(output),
    Err(error) => Err(JsValue::from_str(&error.to_string())),
  }
}
