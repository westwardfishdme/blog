use crate::args::NewArgs;
use crate::modules::utils::capitalize::Capitalize;
use std::{error::Error, fs, io::Write};
mod formatting;

const YAML_SEGMENT: &str = "---\n";

fn create_file(
    filename: String,
    keywords: Box<[String]>,
    title: Option<String>,
) -> Result<String, Box<dyn Error>> {
    //! Creates the actual file with the proper format.
    //! returns the filename.
    let note_path = format!("./{}", filename);
    let mut file = fs::File::create(note_path)?;
    let title = match title {
        // double space it.
        Some(v) => format!("title: \"{v}\"\n"),
        None => {
            let temp: Box<[&str]> = filename.split(".").collect();
            format!("title: \"{}\"\n", temp[0])
        }
    };
    let date = formatting::date();

    let mut keyword_str = String::from("categories: ");

    for (idx, keyword) in keywords.iter().enumerate() {
        if !keyword.trim().is_empty() {
            let key = keyword.trim();
            // must have wrote this tired, this looks like it's
            // doing the same thing redundantly lmao.
            if idx == keywords.len() - 1 {
                keyword_str.push_str(key.capitalize().as_str());
            } else {
                keyword_str.push_str(format!("{}, ", key.capitalize()).as_str());
            }
        }
    }
    keyword_str.push_str("\n");

    // format is yaml:

    file.write_all(YAML_SEGMENT.as_bytes())?;
    // todo: be able to define a layout elsewhere.
    file.write_all("layout: default\n".as_bytes())?;
    file.write_all(title.as_bytes())?;
    file.write_all(date.as_bytes())?;
    file.write_all(keyword_str.as_bytes())?;
    file.write_all(YAML_SEGMENT.as_bytes())?;
    Ok(filename)
}

pub fn create_new_notes(newargs: NewArgs) -> Result<String, Box<dyn Error>> {
    //! creates a new note document in md.
    //!

    let mut filename = newargs.filename;

    // split the string and get what comes during periods.
    let mut split_string: Vec<&str> = filename.split(".").collect();
    if split_string.len() > 2 {
        // should only be the first and what comes after the last period
        split_string = vec![split_string[0], split_string[split_string.len() - 1]]
    }
    // if it's not an md file, we add md as the extension
    match split_string[split_string.len() - 1] {
        "md" => (),
        _ => split_string.push("md"),
    };
    // format it to be: filename.md
    filename = format!("{}.{}", split_string[0], split_string[1]);

    // Handle the keywords by merging them and then splitting by commas.
    // Commas are important when delimiting keywords from each argument provided.
    let keywords: Box<[String]> = newargs.keywords.into_boxed_slice();

    let title = newargs.title;

    create_file(filename, keywords, title)
}
