use thiserror::Error;

#[derive(Debug, Error)]
pub enum SproutError {
    #[error("template error: {0}")]
    Template(#[from] tera::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("not a Spring Boot project: {0}")]
    NotSpringBoot(String),
    #[error("schematic '{0}' is not supported")]
    UnknownSchematic(String),
}
