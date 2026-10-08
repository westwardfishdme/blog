use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    ffi::OsStr,
    path::PathBuf,
};

mod regex;

fn get_all_markdown<T: Into<PathBuf>>(
    directory: Option<T>,
    ignore_dir: &HashSet<PathBuf>,
) -> Vec<PathBuf> {
    //! recursive search in directories within the pwd for all files with the extension 'md'
    let mut md_files: Vec<PathBuf> = Vec::new();
    // use pwd if no directory is given as a parameter.
    let dir = match directory {
        Some(dir) => std::fs::read_dir(dir.into()).expect("failed to read the directory"),
        None => {
            let cwd = std::env::current_dir().expect("failed to get the env for $PWD");
            std::fs::read_dir(cwd).expect("failed to read the directory")
        }
    };
    for entry in dir.into_iter() {
        let file = entry.expect("Failed to read file");
        let ftype = file.file_type().expect("Failed to extract file_type");
        // if the file is a directory and is not in our hashset...
        if ftype.is_dir() && !ignore_dir.contains(file.path().as_path()) {
            let recursive_md = get_all_markdown(Some(file.path()), ignore_dir);
            // concatenate the files
            for i in recursive_md {
                md_files.push(i);
            }
        }
        // otherwise if it's a file, we just push it directly to our vec of .md files
        if ftype.is_file()
            && file
                .path()
                .extension()
                .is_some_and(|x| x == OsStr::new("md"))
            && !ignore_dir.contains(file.path().as_path())
        {
            md_files.push(file.path());
        }
    }
    md_files
}
pub fn format_md(keywords: HashMap<String, u64>) {
    //! outputs the keywords in a markdown table

    let mut table = String::from("|Keyword|Occurrences|\n|-|-|\n");

    for kw in keywords.keys().sorted() {
        let count = keywords.get(kw).unwrap();
        table.push_str(format!("|{kw}|{count}|\n").as_str());
    }
    println!("{table}")
}

pub fn format_csv(keywords: HashMap<String, u64>) {
    //! outputs the keywords in a markdown table

    let mut table = String::from("Keyword,Occurrences\n");

    for kw in keywords.keys().sorted() {
        let count = keywords.get(kw).unwrap();
        table.push_str(format!("{kw},{count}\n").as_str());
    }
    println!("{table}")
}

pub fn kw_count<T: Into<PathBuf>>(
    path: Option<T>,
    exclude: Option<Vec<String>>,
    ignore_dir: HashSet<PathBuf>,
) -> Result<HashMap<String, u64>, Box<dyn Error>> {
    //! gets the keywords and their counts.
    let files: Vec<PathBuf> = get_all_markdown(path, &ignore_dir);

    let mut kw_map: HashMap<String, u64> = HashMap::new();

    for in_file in files {
        match regex::find_keywords(in_file, &exclude)? {
            Some(v) => v.iter().for_each(|kw| {
                match kw_map.get(kw) {
                    Some(v) => kw_map.insert(kw.into(), v + 1),
                    None => kw_map.insert(kw.into(), 1),
                };
            }),
            None => (),
        };
    }
    Ok(kw_map)
}

#[cfg(debug_assertions)]
#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_recursion() {
        let path = format!("{}/tests/", env!("CARGO_MANIFEST_DIR"));
        let testdir = PathBuf::from(&path);

        let mut v = get_all_markdown(Some(testdir), &HashSet::new());

        let files = ["foo.md", "bar.md", "some.md"];
        let f1 = format!("{path}regex/{}", files[0]);
        let f2 = format!("{path}regex/{}", files[1]);
        let f3 = format!("{path}regex/inner/{}", files[2]);
        let mut files = vec![PathBuf::from(f1), PathBuf::from(f2), PathBuf::from(f3)];

        v.sort();
        files.sort();

        assert_eq!(v, files);
    }

    #[test]
    fn test_regex() {
        let path = format!("{}/tests/", env!("CARGO_MANIFEST_DIR"));
        let testdir = PathBuf::from(&path);

        let v = get_all_markdown(Some(testdir), &HashSet::new());

        for path in v {
            let k = super::regex::find_keywords(path, &None)
                .expect("failed to read the files.")
                .expect("expected keywords, got an error");
            eprintln!("{:#?}", k);

            for i in k {
                assert!(["Foo", "Bar", "Biz", "Baz"].contains(&i.as_str()), "{i}");
            }
        }
    }
    #[test]
    fn test_count() {
        let path = format!("{}/tests/", env!("CARGO_MANIFEST_DIR"));
        let testdir = PathBuf::from(&path);

        let v = kw_count(Some(testdir), None, HashSet::new())
            .expect("failed to get a hashmap from counting");

        eprintln!("{:#?}", v);

        assert_eq!(
            v.get(&String::from("Foo"))
                .expect("key 'Foo' did not exist"),
            &1,
            "Foo had bad value"
        );
        assert_eq!(
            v.get(&String::from("Bar"))
                .expect("key 'Bar' did not exist"),
            &2,
            "Bar had bad value"
        );
        assert_eq!(
            v.get(&String::from("Baz"))
                .expect("key 'Baz' did not exist"),
            &1,
            "Baz had wrong value"
        );
        assert_eq!(
            v.get(&String::from("Biz"))
                .expect("key 'Biz' did not exist"),
            &1,
            "Biz had bad value"
        );
    }
}
