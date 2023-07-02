// Ideas:
// Reddit: https://www.reddit.com/r/adventofcode/comments/zesk40/2022_day_7_solutions/
// Rust example: https://gitlab.com/TeNNoX/advent-of-code-2022/-/blob/main/day07/src/main.rs

use std::num::ParseIntError;
use num_traits::Saturating;
use transiter::IntoTransIter;

type Path = Vec<String>;

#[derive(Debug, PartialEq)]
enum File {
    Plain { name: String, size: usize },
    Dir { name: String, contents: Vec<File> },
}

impl File {
    fn get_name(&self) -> &str {
        match self {
            File::Plain { name, size: _ } => name,
            File::Dir { name, contents: _ } => name,
        }
    }
    fn get_path_mut(&mut self, path: &Path) -> Option<&mut File> {
        if path.is_empty() {
            return Some(self);
        }
        let (dir_name, rest_of_path) = path.split_first()?;
        match self {
            File::Plain { name: _, size: _ } => None,
            File::Dir { name: _, contents } => contents
                .iter_mut()
                .find(|f| (**f).get_name() == dir_name)?
                .get_path_mut(&Path::from(rest_of_path)),
        }
    }
    fn get_size(&self) -> usize {
        match self {
            File::Plain { name: _, size } => *size,
            File::Dir { name: _, contents } => contents.iter().map(|f| f.get_size()).sum(),
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            File::Plain { .. } => false,
            File::Dir { .. } => true,
        }
    }

    fn dirs_recursive(&self) -> Vec<&File> {
        self.trans_iter_with(|&f| {
            match f {
                File::Plain { .. } => vec![],
                File::Dir { name: _, contents } => contents
                .iter()
                .filter(|&f| f.is_dir())
                .collect::<Vec<_>>()
            }
        }).collect()
    }
}


impl IntoIterator for File {
    type Item = File;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut result: Vec<Self> = vec![];
        match self {
            File::Plain { .. } => result.push(self),
            File::Dir { name: _, contents } => result.extend(contents.into_iter().flat_map(|s| s.into_iter())),
        }
        result.into_iter()
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Cd { target: String },
    Ls { result: Vec<File> },
}

#[derive(Debug, PartialEq)]
enum ParseFileError {
    Word(String),
    Int(ParseIntError),
}

impl From<ParseIntError> for ParseFileError {
    fn from(value: ParseIntError) -> Self {
        ParseFileError::Int(value)
    }
}

fn parse_file(input: &str) -> Result<File, ParseFileError> {
    let (first, name) = input
        .split_once(' ')
        .ok_or(ParseFileError::Word(input.into()))?;
    match first {
        "dir" => Ok(File::Dir {
            name: name.into(),
            contents: vec![],
        }),
        size => Ok(File::Plain {
            name: name.into(),
            size: size.parse::<usize>()?,
        }),
    }
}

#[derive(Debug, PartialEq)]
enum ParseCommandError {
    Cmd(String),
    File(ParseFileError),
}

impl From<ParseFileError> for ParseCommandError {
    fn from(value: ParseFileError) -> Self {
        ParseCommandError::File(value)
    }
}

fn parse_command_with_output(input: &str) -> Result<Command, ParseCommandError> {
    let first_line = input
        .lines()
        .next()
        .ok_or(ParseCommandError::Cmd(input.into()))?;
    let cmd = first_line
        .split_whitespace()
        .next()
        .ok_or(ParseCommandError::Cmd(input.into()))?;
    match cmd {
        "cd" => Ok(Command::Cd {
            target: first_line
                .strip_prefix("cd ")
                .ok_or(ParseCommandError::Cmd(input.into()))?
                .into(),
        }),
        "ls" => Ok(Command::Ls {
            result: input
                .lines()
                .skip(1)
                .map(parse_file)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        _ => Err(ParseCommandError::Cmd(cmd.into())),
    }
}

fn parse_a_command((pwd, mut fs): (Path, File), cmd: Command) -> (Path, File) {
    let current_dir = fs.get_path_mut(&pwd).unwrap();
    if let File::Dir {
        name: _,
        ref mut contents,
    } = current_dir
    {
        match cmd {
            Command::Cd { target } => {
                let mut new_pwd = pwd.clone();
                match target.as_str() {
                    ".." => {
                        new_pwd.pop();
                    }
                    "/" => new_pwd.clear(),
                    _ => new_pwd.push(target),
                }
                (new_pwd, fs)
            }
            Command::Ls { mut result } => {
                contents.append(&mut result);
                (pwd, fs)
            }
        }
    } else {
        panic!("Path should be a dir!");
    }
}

fn parse_session(cmds: Vec<Command>) -> File {
    let fs = File::Dir {
        name: "".to_string(),
        contents: vec![],
    };
    let (_, final_fs) = cmds.into_iter().fold((vec![], fs), parse_a_command);
    final_fs
}

fn parse_file_to_commands(s: String) -> Result<Vec<Command>,ParseCommandError> {
    s.strip_prefix("$ ").unwrap().split("\n$ ").map(parse_command_with_output).collect()
}

fn main() {
    let input = std::fs::read_to_string("input/day07.txt").unwrap();
    let commands: Vec<Command> = parse_file_to_commands(input).unwrap();
    let fs = parse_session(commands);
    let total_size = fs.get_size();
    let available_space = 70_000_000 - total_size;
    let must_free: usize = 30_000_000.saturating_sub(available_space);
    let result: usize = fs
        .dirs_recursive()
        .into_iter()
        .map(|f| f.get_size())
        .filter(|size| *size >= must_free)
        .min().unwrap();

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_parse_cd() {
        let input = "cd asdf";
        let output = Ok(Command::Cd {
            target: "asdf".into(),
        });
        assert_eq!(
            parse_command_with_output(input),
            output,
            "Parse cd command invalid."
        );
    }

    #[test]
    fn test_parse_ls() {
        let input = "ls\n\
            dir gftgshl\n\
            dir grct\n\
            57336 tbqpqfgd.wvz\n\
            267191 vqms\n\
            dir wtgzgmvr";
        let output = Ok(Command::Ls {
            result: vec![
                File::Dir {
                    name: "gftgshl".to_string(),
                    contents: vec![],
                },
                File::Dir {
                    name: "grct".to_string(),
                    contents: vec![],
                },
                File::Plain {
                    name: "tbqpqfgd.wvz".to_string(),
                    size: 57336,
                },
                File::Plain {
                    name: "vqms".to_string(),
                    size: 267191,
                },
                File::Dir {
                    name: "wtgzgmvr".to_string(),
                    contents: vec![],
                },
            ],
        });
        assert_eq!(
            parse_command_with_output(input),
            output,
            "Parse ls command invalid."
        );
    }

    #[test]
    fn test_parse_session() {
        use super::File::{Dir, Plain};
        let input = "$ cd /\n\
        $ ls\n\
        dir a\n\
        14848514 b.txt\n\
        8504156 c.dat\n\
        dir d\n\
        $ cd a\n\
        $ ls\n\
        dir e\n\
        29116 f\n\
        2557 g\n\
        62596 h.lst\n\
        $ cd e\n\
        $ ls\n\
        584 i\n\
        $ cd ..\n\
        $ cd ..\n\
        $ cd d\n\
        $ ls\n\
        4060174 j\n\
        8033020 d.log\n\
        5626152 d.ext\n\
        7214296 k".to_string();

        let output = Dir {
            name: "".to_string(),
            contents: vec![
                Dir {
                    name: "a".to_string(),
                    contents: vec![
                        Dir {
                            name: "e".to_string(),
                            contents: vec![
                                Plain { name: "i".to_string(), size: 584 }
                            ]
                        },
                        Plain { name: "f".to_string(), size: 29116 },
                        Plain { name: "g".to_string(), size: 2557 },
                        Plain { name: "h.lst".to_string(), size: 62596 }
                    ]
                },
                Plain { name: "b.txt".to_string(), size: 14848514 },
                Plain { name: "c.dat".to_string(), size: 8504156 },
                Dir {
                    name: "d".to_string(),
                    contents: vec![
                        Plain { name: "j".to_string(), size: 4060174 },
                        Plain { name: "d.log".to_string(), size: 8033020 },
                        Plain { name: "d.ext".to_string(), size: 5626152 },
                        Plain { name: "k".to_string(), size: 7214296 }
                    ]
                }
            ]
        };

        let commands = parse_file_to_commands(input).unwrap();
        assert_eq!(parse_session(commands), output);
    }
}
