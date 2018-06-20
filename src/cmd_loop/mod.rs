mod cmd_handler;
use std::io::stdin;
use std::fs::File;
use std::io::Write;
use cmd_loop::cmd_handler::CmdChars;
use compiler::Compiler;

enum SpecialCmd {
    Time,
    File(String),
    SaveFile,
    Load(Vec<String>),
    Error,
    Help,
    Quit,
}

pub fn run(cmp: &mut Compiler) {
    let mut timed = false;
    let mut save_file: Option<File> = None;
    let help_str = ":q => exit program\n\
    :l [filename] => compile and run file\n\
    :t => toggle timer\n\
    :s [filename] => toggle saving to file\n\
    type expressions to evaluate\n";
    let err_str = "Command not understood, type :h for help\n";
    let mut cmd = String::new();
    loop {
        cmd = String::new();
        stdin().read_line(&mut cmd).expect("Failed to read from stdin.");
        let mut chs = CmdChars::new(cmd.clone());
        match chs.peek() {
            Some(':') => {
                match special_command(&mut chs) {
                    SpecialCmd::Time => timed = !timed,
                    SpecialCmd::File(f) => {
                        if save_file.is_some() {
                            save_file.unwrap().flush().expect("Error writing to file.");
                        }
                        save_file = Some(File::create(f).expect("Could not create file"));
                    }
                    SpecialCmd::SaveFile => {
                        if save_file.is_none() {
                            println!("No file open for saving");
                        } else {
                            save_file.unwrap().flush().expect("Error writing to file.");
                            save_file = None;
                        }
                    }
                    SpecialCmd::Load(files) => {
                        for file in files.iter() {
                            cmp.compile_file(file.clone());
                            cmp.execute(timed);
                        }
                    }
                    SpecialCmd::Error => println!("{}", err_str),
                    SpecialCmd::Help => println!("{}", help_str),
                    SpecialCmd::Quit => {
                        if save_file.is_some() {
                            save_file.unwrap().flush().expect("Error writing to file.");
                        }
                        return;
                    }
                }
            }
            Some(_) => {
                match read_sequence(&mut chs) {
                    Ok(s) => {
                        if save_file.is_some() {
                            save_file = save_file.map(|mut x| {
                                x.write(s.as_bytes()).expect("Could not write to file");
                                x
                            });
                        }
                        cmp.compile_string(s);
                        cmp.execute(timed);
                    }
                    Err(s) => println!("Error: {}, nothing saved", s),
                }
            }
            None => panic!("Could not read from stdin: CmdChars started empty."),
        }
    }
}

fn special_command(chs: &mut CmdChars) -> SpecialCmd {
    chs.next();
    match chs.peek() {
        Some('l') => {
            chs.next();chs.next();
            let mut files: Vec<String> = Vec::new();
            while chs.peek().is_some() && chs.peek() != Some('\n') {
                let mut file = String::new();
                while let Some(ch) = chs.peek() {
                    chs.next();
                    if ch == ' ' || ch == '\n' {
                        break;
                    }
                    file.push(ch);
                }
                files.push(file);
            }
            return SpecialCmd::Load(files);
        }
        Some('t') => return SpecialCmd::Time,
        Some('s') => {
            chs.next();chs.next();
            if chs.peek().is_none() {
                return SpecialCmd::SaveFile;
            }
            let mut file = String::new();
            while chs.peek().is_some() && chs.peek() != Some('\n') {
                file.push(chs.peek().unwrap());
                chs.next();
            }
            return SpecialCmd::File(file);
        }
        Some('q') => return SpecialCmd::Quit,
        Some('h') => return SpecialCmd::Help,
        _ => return SpecialCmd::Error,
    }
}

fn read_sequence(chs: &mut CmdChars) -> Result<String, &str> {
    let mut cmd = String::new();
    let mut aux = String::new();
    let mut loops_open = 0;
    let mut fun_open = false;
    while chs.peek().is_some() || loops_open > 0 || fun_open {
        if chs.peek().is_none() {
            stdin().read_line(&mut aux).expect("Failed to read from stdin.");
            chs.push(aux.clone());
        }
        match chs.peek() {
            Some('[') => loops_open += 1,
            Some(']') => {
                loops_open -= 1;
                if loops_open < 0 {
                    return Err("Loop without start point closed");
                }
            },
            Some('~') => {
                if fun_open {
                    if loops_open > 0 {
                        return Err("Function with unfinished loops closed");
                    }
                    fun_open = false;
                } else {
                    if loops_open > 0 {
                        return Err("Cannot define function in loop");
                    }
                    fun_open = true;
                }
            }
            _ => {}
        }
        cmd.push(chs.peek().unwrap());
        chs.next();
    }
    Ok(cmd)
}