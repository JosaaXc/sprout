use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NameSet {
    pub pascal: String,
    pub camel: String,
    pub kebab: String,
    pub snake: String,
    pub plural_kebab: String,
    pub plural_snake: String,
    pub plural_camel: String,
}

impl NameSet {
    pub fn from_raw(raw: &str) -> Self {
        let pascal = raw.to_pascal_case();
        let camel = pascal.to_lower_camel_case();
        let kebab = pascal.to_kebab_case();
        let snake = pascal.to_snake_case();
        let plural_kebab = pluralize(&kebab);
        let plural_snake = pluralize(&snake);
        let plural_camel = pluralize(&camel);
        Self {
            pascal,
            camel,
            kebab,
            snake,
            plural_kebab,
            plural_snake,
            plural_camel,
        }
    }
}

fn pluralize(word: &str) -> String {
    if word.is_empty() {
        return word.to_string();
    }
    if word.ends_with('y') && !ends_with_vowel_y(word) {
        format!("{}ies", &word[..word.len() - 1])
    } else if word.ends_with('s')
        || word.ends_with('x')
        || word.ends_with("ch")
        || word.ends_with("sh")
    {
        format!("{word}es")
    } else {
        format!("{word}s")
    }
}

fn ends_with_vowel_y(word: &str) -> bool {
    if word.len() < 2 {
        return false;
    }
    let chars: Vec<char> = word.chars().collect();
    let prev = chars[chars.len() - 2];
    matches!(prev, 'a' | 'e' | 'i' | 'o' | 'u')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_simple_name() {
        let n = NameSet::from_raw("user");
        assert_eq!(n.pascal, "User");
        assert_eq!(n.camel, "user");
        assert_eq!(n.kebab, "user");
        assert_eq!(n.snake, "user");
        assert_eq!(n.plural_kebab, "users");
    }

    #[test]
    fn converts_compound_name() {
        let n = NameSet::from_raw("user-account");
        assert_eq!(n.pascal, "UserAccount");
        assert_eq!(n.camel, "userAccount");
        assert_eq!(n.kebab, "user-account");
        assert_eq!(n.snake, "user_account");
    }

    #[test]
    fn pluralizes_y() {
        let n = NameSet::from_raw("Category");
        assert_eq!(n.plural_kebab, "categories");
    }

    #[test]
    fn pluralizes_x() {
        let n = NameSet::from_raw("Box");
        assert_eq!(n.plural_kebab, "boxes");
    }
}
