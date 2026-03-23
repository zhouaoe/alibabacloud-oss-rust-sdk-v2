use crate::{OperationInput, HTTP_HEADER_CONTENT_LENGTH};

/// Updates the Content-Length header in the given `OperationInput`.
///
/// This function checks if the `OperationInput` already has a content length
/// header. If not, it calculates the length of the body and inserts the content
/// length header with the calculated value.
///
/// # Arguments
///
/// * `input` - A mutable reference to the `OperationInput` struct.
///
/// # Returns
///
/// This function returns `Ok(())` if the content length header is successfully
/// updated.
pub fn update_content_length(
    input: &mut OperationInput,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // if !input
    //     .headers
    //     .keys()
    //     .any(|k| k.eq_ignore_ascii_case(HTTP_HEADER_CONTENT_LENGTH))
    // {
    if let Some(data) = &mut input.body {
        input.headers.insert(
            HTTP_HEADER_CONTENT_LENGTH.to_string(),
            data.content_length().unwrap().to_string(),
        );
    };
    // }

    Ok(())
}
