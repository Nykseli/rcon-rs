use std::fs;

fn parse_file(text: &str) -> Vec<String> {
    let mut commands = Vec::new();

    for line in text.lines() {
        let line = line.trim();

        // Ignore empty lines
        if line.is_empty() {
            continue;
        }

        // ignore comments
        if line.starts_with("//") || line.starts_with('#') {
            continue;
        }

        // Split comments from the end of the line
        let line = line.split("//").next().unwrap_or(line).trim();
        let line = line.split('#').next().unwrap_or(line).trim();

        commands.push(line.into());
    }

    commands
}

pub fn commands_from_file(path: &str) -> Vec<String> {
    let text = fs::read_to_string(path).unwrap();
    parse_file(&text)
}

#[cfg(test)]
mod tests {
    use crate::command_file::parse_file;

    #[test]
    fn it_parses_commmands_correctly() {
        let file = "// This is foo!
        foo

        # Get status from the server
        status

        end_comment # foo
        end_comment2 // foo
        end_comment3 // foo # foo
        end_comment4 # foo // foo";

        let commands = parse_file(file);
        let target = vec![
            "foo",
            "status",
            "end_comment",
            "end_comment2",
            "end_comment3",
            "end_comment4",
        ];

        assert_eq!(target, commands);
    }
}
