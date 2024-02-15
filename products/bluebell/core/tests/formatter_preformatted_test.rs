#[cfg(test)]
mod tests {
    extern crate diffy;
    use std::{fs, fs::File, io::Read};

    use bluebell::{
        formatter::BluebellFormatter,
        parser::{
            lexer,
            lexer::{Lexer, SourcePosition},
            parser, ParserError,
        },
    };
    use diffy::{create_patch, PatchFormatter};

    fn strip_comments(input: &str) -> String {
        let re = regex::Regex::new(r"[ ]*\(\*([^*]|\*+[^*)])*\*+\)\n*").unwrap();
        let result = re.replace_all(input, "");
        result.to_string()
    }

    fn parse_and_format(path: String) -> bool {
        let mut file = File::open(&path).expect("Unable to open file");
        let mut script = String::new();
        file.read_to_string(&mut script)
            .expect("Unable to read file");
        let lexer = Lexer::new(&script);

        let mut errors: Vec<lexer::ParseError> = [].to_vec();
        let parser = parser::ProgramParser::new();
        assert!(errors.len() == 0);

        // TODO: Hack to deal with that we do not support comments yet
        let script = strip_comments(&script);

        match parser.parse(&mut errors, lexer) {
            Ok(ast) => {
                let mut formatter = BluebellFormatter::new();
                let mut ast2 = ast.clone();
                let formatted = formatter.emit(&mut ast2);

                if formatted != script {
                    println!("Orignial:\n{}\n\n", script);
                    println!("Formatted:\n{}\n\n", formatted);
                    let diff = create_patch(&script, &formatted);
                    let f = PatchFormatter::new().with_color();
                    println!("Diff:\n{}\n\n", f.fmt_patch(&diff));
                    println!("Filename: {}\n\n", path)
                }
                assert_eq!(formatted, script);
                formatted == script
            }
            Err(error) => {
                let _ret = error.clone();
                let message = format!("Syntax error {:?}", error);
                let mut pos: Vec<SourcePosition> = [].to_vec();
                error.map_location(|l| {
                    pos.push(l.clone());
                    l
                });

                let mut n = 0;
                let mut line_counter = 0;
                let mut char_counter = 0;
                let mut line_start = 0;
                let mut line_end = 0;
                let mut should_stop = false;
                for ch in script.chars() {
                    if ch == '\n' {
                        if should_stop {
                            line_end = n;
                            break;
                        } else {
                            line_start = n + 1;
                        }
                    }
                    if !should_stop && n == pos[0].position {
                        should_stop = true;
                    }

                    n += 1;
                    if !should_stop {
                        char_counter += 1;
                    }

                    if ch == '\n' {
                        line_counter += 1;
                        char_counter = 0;
                    }
                }

                if line_end < line_start {
                    line_end = script.len();
                }

                let line = &script[line_start..line_end];
                println!("Line {},{}:{}", line_counter, char_counter, line);
                print!(
                    "{}",
                    " ".repeat(
                        char_counter + format!("Line {},{}:", line_counter, char_counter).len()
                    )
                );
                println!("{}", "^".repeat(pos[1].position - pos[0].position));

                let my_error = ParserError {
                    message,
                    line: 0,   //error.location_line(),
                    column: 0, // err.location_column(),
                };
                println!("{}", my_error);

                false
            }
        }
    }

    #[test]
    fn test_scilla_files() {
        let mut success = true;

        let mut entries: Vec<_> = fs::read_dir("./tests/data/formatter/formatted")
            .expect("read_dir call failed")
            .collect::<Result<_, _>>()
            .expect("Failed to collect directory entries");
        entries.sort_by_key(|entry| entry.path().to_string_lossy().into_owned());
        for entry in entries {
            let path = entry.path();
            let path_str = path.to_string_lossy().into_owned();

            // Run the .scilla file using your Scilla execution command
            println!("- Formatting {}", path_str);
            let ret = parse_and_format(path_str);
            success = ret && success;
        }

        assert!(success)
    }
}
