use std::collections::HashMap;

use crate::OperationInput;

#[allow(clippy::type_complexity)]
pub fn modify_request(
    input: &mut OperationInput,
    headers: HashMap<String, String>,
    parameters: HashMap<String, String>,
    modifiers: Vec<fn(&mut OperationInput) -> Result<(), Box<dyn std::error::Error + Send + Sync>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    headers.iter().for_each(|(k, v)| {
        input.headers.insert(k.clone(), v.clone());
    });
    parameters.iter().for_each(|(k, v)| {
        input.parameters.insert(k.clone(), v.clone());
    });

    // if let Some(body_reader) = &mut input.body {
    //     input.body_content = Some(read_to_string(body_reader.clone())?);
    // }

    for modifier in modifiers {
        modifier(input)?;
    }

    Ok(())
}
