/// Parses command arguments into a vector of strings.
///
/// # Arguments
///
/// * `args` - The argument string to parse.
///
/// # Returns
///
/// A vector of parsed arguments.
#[allow(dead_code)]
pub fn parse_arguments(args: &str) -> Vec<&str> {
    args.split_whitespace().collect()
}
