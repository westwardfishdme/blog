/// Note creator.
/// usage:
mod args;
use std::collections::HashSet;
use std::error::Error;
use std::fs::canonicalize;
use std::path::PathBuf;

use args::{Commands, NoteArgs};
use clap::Parser;
mod modules;
use modules::{keywords, new};
use std::env;
use std::ffi::OsString;
use std::process::{Command, Stdio, exit};

#[cfg(target_os = "windows")]
fn edit_file<T: ToString>(file: T) -> Result<(), Box<dyn Error>> {
    //! edit the file for windows

    Command::new("notepad.exe").arg(file.to_string()).status()?;
    Ok(())
}

fn try_nano() -> Result<OsString, Box<dyn Error>> {
    //! tries to use nano
    let editor = match Command::new("nano").arg("--version").status() {
        Ok(_) => OsString::from("vim"),
        Err(_) => return Err("".into()),
    };
    Ok(editor)
}
#[cfg(target_family = "unix")]
fn try_other_editors() -> Option<OsString> {
    //! first tries vim, then tries nano
    match Command::new("vim")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
    {
        Ok(_) => Some(OsString::from("vim")),
        Err(_) => try_nano().ok(),
    }
}

#[cfg(target_family = "unix")]
fn edit_file<T: ToString>(file: T) -> Result<(), Box<dyn Error>> {
    //! edit the file for linux/macos
    let editor: Option<OsString> = match env::var_os("EDITOR") {
        Some(v) => {
            if v.is_empty() {
                try_other_editors()
            } else {
                Some(v)
            }
        }
        None => try_other_editors(),
    };
    if editor.is_none() {
        return Err("User has no valid editor, or it is not set".into());
    }
    // run our command
    let editor = editor
        .unwrap()
        .into_string()
        .expect("failed to get a valid unicode string when parsing the editor.");

    Command::new(editor).arg(file.to_string()).status()?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let args = NoteArgs::parse();
    match args.commands {
        Commands::New(newargs) => {
            let file = match new::create_new_notes(newargs) {
                Ok(v) => v,
                Err(_) => {
                    return Err("Failed to create the file.".into());
                }
            };
            if args.edit {
                edit_file(file)?
            }
        }
        Commands::Keywords(keywordargs) => {
            let ignore_dirs: HashSet<PathBuf> = match keywordargs.ignore_paths {
                Some(dirs) => {
                    let mut idirs = HashSet::new();

                    for dir in dirs {
                        let dir = canonicalize(dir)?;
                        idirs.insert(dir);
                    }
                    idirs
                }
                None => HashSet::new(),
            };
            let keywords =
                keywords::kw_count(keywordargs.directory, keywordargs.ignore, ignore_dirs)?;
            if keywords.is_empty() {
                eprintln!("[error]: no valid notes found in this directory");
                exit(1)
            }
            match keywordargs.output.to_lowercase().as_str() {
                "markdown" | "md" => keywords::format_md(keywords),
                "csv" => keywords::format_csv(keywords),
                _ => return Err("Bad file type, please choose from 'markdown' or 'csv'".into()),
            }
        }
    };
    Ok(())
}
