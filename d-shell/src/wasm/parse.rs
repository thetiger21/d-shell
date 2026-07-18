use crate::state::ShellState;

impl ShellState {
    pub fn program_parse(&mut self, input: &String) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let mut temp_string = String::new();

        for token in input.chars() {
            match token {
                '.' => (),
                '\n' => (),
                '\r' => (),
                ' ' => {
                    if !temp_string.is_empty() {
                        output.push(temp_string.clone());
                        temp_string.clear();
                    }
                }
                _ => {
                    temp_string.push(token);
                }
            }
        }
        if !temp_string.is_empty() {
            output.push(temp_string.clone());
            temp_string.clear();
        }
        output
    }
}
