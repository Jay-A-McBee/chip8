use std::io;
use std::io::Write;

pub enum QuestionFormat {
    Statement,
    Menu(Vec<&'static str>),
}

#[derive(Debug)]
pub struct Question {
    pub active_idx: Option<usize>,
    content: &'static str,
    possible_answers: Option<Vec<&'static str>>,
    pub stdout_handle: std::io::Stdout,
}

impl Question {
    pub fn question(content: &'static str, question_format: Option<QuestionFormat>) -> Self {
        let handle = io::stdout();
        let (possible_answers, active_idx) = Question::format(question_format).unwrap();
        Question {
            active_idx,
            content,
            possible_answers,
            stdout_handle: handle,
        }
    }

    fn format(
        question_format: Option<QuestionFormat>,
    ) -> Option<(Option<Vec<&'static str>>, Option<usize>)> {
        if let Some(QuestionFormat::Menu(answers)) = question_format {
            return Some((Some(answers), Some(0)));
        }

        Some((None, None))
    }

    pub fn ask(&mut self) -> std::io::Result<()> {
        self.stdout_handle.write_all(self.content.as_bytes())?;

        if let Some(answers) = &self.possible_answers {
            self.stdout_handle
                .write_all(String::from("\n\n").as_bytes())?;

            for (idx, answer) in answers.iter().enumerate() {
                let cursor = if let Some(active_idx) = self.active_idx {
                    if idx == active_idx {
                        "> "
                    } else {
                        ""
                    }
                } else {
                    ""
                };

                let formatted = format!("{}{}\n", cursor, answer);
                self.stdout_handle.write_all(formatted.as_bytes())?;
            }
        }

        Ok(())
    }
}

pub trait Read {
    fn read(buffer: String) -> std::io::Result<String>;
}
