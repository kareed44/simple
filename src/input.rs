pub struct Options {
    pub show_help: bool,
    pub enable_debug_logging: bool,
    pub show_version: bool
}

pub fn parse_input_to_options(args: &[String]) -> Options {

    //Skip the first argument because it contains the executable path
    let mut iter = args.iter();
    iter.next();
    let arguments = iter.as_slice();

    let mut parsed_options = Options {
        show_help: false,
        enable_debug_logging: false,
        show_version: false
    };

    for a in arguments {
        let first_char = a.chars().next().unwrap();
        if first_char == '-' {
            if a == "-v" || a == "--verbose" {
                parsed_options.enable_debug_logging = true;
            } else if a == "-h" || a == "--help" {
                parsed_options.show_help = true;
            }
            else if a == "-V" || a == "--version" {
                parsed_options.show_version = true;
            }
        }
    }

    parsed_options
}