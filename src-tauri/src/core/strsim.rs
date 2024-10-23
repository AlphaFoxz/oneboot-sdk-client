#[cfg(test)]
mod strsim {
    use strsim::{
        jaro, jaro_winkler, normalized_damerau_levenshtein, normalized_levenshtein, sorensen_dice,
    };

    #[test]
    fn test() {
        let x = "ip";
        let y = "ipAddress";
        let binding = split_camel(y);
        let dy = binding.as_str();
        let n = jaro(x, y);
        eprintln!("jaro: {n}");
        let n = jaro_winkler(x, y);
        eprintln!("jaro_winkler: {n}");
        let n = normalized_levenshtein(x, dy);
        eprintln!("normalized_levenshtein: {n}");
        let n = normalized_damerau_levenshtein(x, dy);
        eprintln!("normalized_damerau_levenshtein: {n}");
        let n = sorensen_dice(x, dy);
        eprintln!("sorensen_dice: {n}");
        assert!(n > 0.8);
    }

    // 将驼峰变量名用空格分隔
    pub fn split_camel(name: &str) -> String {
        let mut result = String::new();
        for (i, c) in name.chars().enumerate() {
            if i > 0 && c.is_uppercase() {
                result.push(' ');
            }
            result.push(c.to_ascii_lowercase());
        }
        result
    }
}
