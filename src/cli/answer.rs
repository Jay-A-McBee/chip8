use crate::cli::question::Question;
use std::io;
use std::io::Write;

pub struct Answer<'a> {
    question: &'a mut Question,
    stdin_handle: io::Stdin,
}

impl<'a> Answer<'a> {
    pub fn response_to(question: &'a mut Question) -> Self {
        let stdin_handle = io::stdin();
        Answer {
            question,
            stdin_handle,
        }
    }

    pub fn get_response(&mut self) -> io::Result<String> {
        let mut response_buf = String::new();

        self.question
            .stdout_handle
            .write_all("\x1b[?25l".as_bytes());
        self.stdin_handle.read_line(&mut response_buf)?;

        loop {
            for &key in response_buf.as_bytes() {
                if key == 32 as u8 {
                    self.question.active_idx = Some(1);
                    self.question.ask();
                }
            }
        }

        Ok(response_buf)
    }
}
