pub trait Capitalize {
    fn capitalize(self) -> String;
}
impl<T: ToString> Capitalize for T {
    fn capitalize(self) -> String {
        let s = self.to_string().as_str().trim().to_string();

        let mut ret = String::new();
        let words = s.split(' ');

        for (i, word) in words.enumerate() {
            let mut word = word
                .to_string()
                .as_str()
                .trim()
                .chars()
                .collect::<Box<[char]>>();
            let cap = word[0].to_uppercase().next().expect("no char");
            word[0] = cap;
            match i {
                0_usize => ret.push_str(word.iter().collect::<String>().as_str()),
                _ => ret.push_str(format!(" {}", word.iter().collect::<String>()).as_str()),
            }
        }

        ret
    }
}
