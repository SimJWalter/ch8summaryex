pub(crate) struct PigLatin {
    to_convert: String,
}

impl PigLatin {
    pub fn new(to_convert: String) -> Self {
        Self {
            to_convert: to_convert.to_lowercase(),
        }
    }

    pub fn translate(&self) -> String {
        self.to_convert.split_whitespace().into_iter().map(|word| {
            let ch = word.chars().take(1).last().unwrap();
            if ['a', 'e', 'i', 'o', 'u'].contains(&ch) {
                format!("{}-hay", word)
            } else {
                format!("{}-{}ay", &word[ch.len_utf8()..], ch)
            }
        }).collect::<Vec<String>>().join(" ")
    }
}
