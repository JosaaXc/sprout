use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Persistence {
    #[default]
    JpaRelational,
    MongoDb,
}

impl Persistence {
    pub fn all() -> &'static [Persistence] {
        &[Self::JpaRelational, Self::MongoDb]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::JpaRelational => "JPA (SQL)",
            Self::MongoDb => "MongoDB (NoSQL)",
        }
    }

    pub fn is_jpa(&self) -> bool {
        matches!(self, Self::JpaRelational)
    }

    pub fn entity_template(&self) -> &'static str {
        match self {
            Self::JpaRelational => "entity/jpa.java.tera",
            Self::MongoDb => "entity/mongo.java.tera",
        }
    }

    pub fn repository_template(&self) -> &'static str {
        match self {
            Self::JpaRelational => "repository/jpa.java.tera",
            Self::MongoDb => "repository/mongo.java.tera",
        }
    }

    pub fn repository_base_class(&self) -> &'static str {
        match self {
            Self::JpaRelational => "JpaRepository",
            Self::MongoDb => "MongoRepository",
        }
    }

    pub fn id_type(&self) -> &'static str {
        match self {
            Self::JpaRelational => "Long",
            Self::MongoDb => "String",
        }
    }
}
