use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};

#[derive(Debug)]
pub struct Question();

impl Question {
    pub fn select(
        options: &[&str],
        prompt: Option<&str>,
        default: Option<&usize>,
    ) -> std::io::Result<Option<usize>> {
        let default_idx = if let Some(&default_idx) = default {
            default_idx
        } else {
            0
        };

        let prompt = if let Some(prompt) = prompt {
            prompt
        } else {
            "Make a selection"
        };

        Select::with_theme(&ColorfulTheme::default())
            .items(options)
            .with_prompt(prompt)
            .default(default_idx)
            .interact_on_opt(&Term::stderr())
    }

    pub fn input<'a>(
        (prompt, initial_text, default): (Option<&'a str>, Option<&'a str>, Option<&'a str>),
    ) -> std::io::Result<String> {
        let mut input = Input::<String>::new();

        if let Some(prompt) = prompt {
            input.with_prompt(prompt);
        }

        if let Some(initial_text) = initial_text {
            input.with_initial_text(initial_text);
        }

        if let Some(default) = default {
            input.default(default.to_string());
        }

        input.interact()
    }
}
