use std::{collections::HashSet, error::Error, fs, path::PathBuf};

use crate::modules::utils::capitalize::Capitalize;
use regex::Regex;

const REGEX_STR: &str = "^(categories:).*";

pub fn find_keywords(
    file: PathBuf,
    exclude: &Option<Vec<String>>,
) -> Result<Option<HashSet<String>>, Box<dyn Error>> {
    //! returns a list of all keywords.
    //! if exclude is set to `true`-- any keywords in files containing
    //! what is stored at compile time will be ignored.

    let mut keywords: HashSet<String> = HashSet::new();
    // regex we use to find keywords.
    let re = Regex::new(REGEX_STR)?;
    dbg!(&re);
    let fcontent = fs::read_to_string(&file)?;

    for (line, text) in fcontent.lines().enumerate() {
        if re.is_match(text) {
            // use everything after the colon

            match &fcontent.lines().nth(line) {
                Some(v) => {
                    let keywords_line = match v.rsplit_once(':') {
                        Some(split) => split.1,
                        None => {
                            return Err("Searched Keywords String doesn't exist in YAML.".into());
                        }
                    };
                    eprintln!("{}", keywords_line);

                    // no data in line
                    if keywords_line.trim().is_empty() {
                        return Ok(None);
                    }
                    // no data after the colon

                    keywords_line.split(',').for_each(|k| {
                        keywords.insert(k.capitalize());
                    })
                }
                None => return Err("File ended at 'keywords'".into()),
            };
            // if we have an exclusion list...
            if let Some(exclude) = exclude {
                for word in exclude {
                    if !word.is_empty() {
                        let word = word.as_str().trim().to_string().to_lowercase();
                        keywords.remove(&word.capitalize());
                    }
                }
            };

            return Ok(Some(keywords));
        }
    }
    // we couldn't find a match
    Ok(None)
}
