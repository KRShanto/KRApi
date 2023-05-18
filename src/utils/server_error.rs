use std::fmt::Display;

/// Prints a error message to the console if a server error occurs.
///
/// This also prints to report the error to the developer.
// TODO: Add a way to report the error to the developer. and provide more information about the error.
pub fn server_error<T: Display>(e: T) {
    println!("An Error occured: {}", e);
}
