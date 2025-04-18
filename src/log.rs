use colored::Colorize;
use spinoff::{Spinner, spinner, spinners};

pub struct Logger {
    spinner: Spinner,
    current_text: Option<String>,
}

impl Logger {
    pub fn new() -> Self {
        let frames = spinner!(["[-]", "[\\]", "[|]", "[/]"], 100);
        let spinner = Spinner::new(frames, "", None);

        Self {
            spinner,
            current_text: None,
        }
    }

    pub fn next(&mut self, text: &str) {
        if let Some(text) = &self.current_text {
            println!("\r[{}] {text}", "+".green())
        }
        print!("[ ] {text}");
        self.current_text = Some(text.to_string());
    }

    pub fn fail(&mut self, text: String) {
        if let Some(old_text) = &self.current_text {
            println!("\r[{}] {old_text}: {text}", "x".red())
        } else {
            println!("[{}] {text}", "x".red())
        }
        println!();
        self.spinner.clear();
    }

    pub fn finish(&mut self, text: String) {
        if let Some(text) = &self.current_text {
            println!("\r[{}] {text}", "+".green())
        }
        println!("[{}] {text}", "✓".green());
        println!();
        self.spinner.clear();
    }
}

pub trait ExtPrintAndExit<T> {
    /// Unwrap the result or log an error and exit
    fn unwrap_or_exit(self, logger: &mut Logger) -> T;
}

impl<T> ExtPrintAndExit<T> for anyhow::Result<T> {
    fn unwrap_or_exit(self, logger: &mut Logger) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                logger.fail(err.to_string());
                std::process::exit(1)
            }
        }
    }
}
