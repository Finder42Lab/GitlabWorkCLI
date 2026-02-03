use colored::Colorize;

pub struct Printer {}

impl Printer {
    pub fn print(message: String, module_name: Option<String>) {
        let message_text =match module_name {
            None => {message}
            Some(mn) => {
                format!("{}{}", mn.on_cyan().white(), message)
            }
        };

        println!("{}", message_text);
    }

    pub fn print_error(text: String, module_name: Option<String>) {
        Self::print(format!("{} {}", " ERROR ".on_red().white().bold(), text), module_name);
    }

    pub fn print_warning(text: String, module_name: Option<String>) {
        Self::print(format!("{} {}", " WARN ".on_yellow().white().bold(), text), module_name);
    }

    pub fn print_info(text: String, module_name: Option<String>) {
        Self::print(format!("{} {}", " INFO ".on_blue().white().bold(), text), module_name);
    }

    pub fn print_success(text: String, module_name: Option<String>) {
        Self::print(format!("{}", text.green().bold()), module_name);
    }
}