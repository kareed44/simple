use std::env;

static LOG_LEVEL_ENV_VAR_NAME: &'static str = "SIMPLE_LOG_LEVEL";

pub fn infoln(message: &str) {
    logln(message, "INFO");
}

pub fn debugln(message: &str) {
    logln(message, "DEBUG");
}

pub fn errorln(message: &str) {
    logln(message, "ERROR");
}

fn logln(message: &str, level: &str) {
    let mut print_message = true;
    match env::var(&LOG_LEVEL_ENV_VAR_NAME) {
        Ok(global_log_level) => {
            if level == "DEBUG" && global_log_level != "DEBUG" {
                print_message = false
            }
        },
        Err(_error) => { 
            if level == "DEBUG" {
                print_message = false
            }
         }
    }

    if print_message {
        if level == "ERROR" {
            eprintln!("[{}] {}", level, message);
        }
        else {
            println!("[{}] {}", level, message);
        }
    }
}