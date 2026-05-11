use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DtoStyle {
    Record,
    Class,
}

impl Default for DtoStyle {
    fn default() -> Self {
        Self::Record
    }
}

impl DtoStyle {
    pub fn all() -> &'static [DtoStyle] {
        &[Self::Record, Self::Class]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Record => "Java 14+ Records",
            Self::Class => "Classic Classes (Lombok)",
        }
    }

    pub fn request_template(&self) -> &'static str {
        match self {
            Self::Record => "dto/request_record.java.tera",
            Self::Class => "dto/request_class.java.tera",
        }
    }

    pub fn response_template(&self) -> &'static str {
        match self {
            Self::Record => "dto/response_record.java.tera",
            Self::Class => "dto/response_class.java.tera",
        }
    }
}
