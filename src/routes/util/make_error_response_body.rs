pub fn make_error_response_body(status: u16) -> String {
    let error_message = if status == 500 {
        "internal server error happens"
    } else {
        "TBD"
    };

    format!("{{ error_message: \"{:?}\" }}", error_message)
}
