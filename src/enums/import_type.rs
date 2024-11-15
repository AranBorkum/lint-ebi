use regex::Regex;

pub enum ImportType {
    Entity,
    RepositoryInterface,
}

impl ImportType {
    pub fn import_regex(&self) -> Regex {
        match self {
            ImportType::Entity => Regex::new(
                r"^\s*(?:import|from)\s+([a-zA-Z0-9_.]+)\s*(?:\.\s*entities\b|entities\b)",
            )
            .unwrap(),
            ImportType::RepositoryInterface => Regex::new(
                r"^\s*(?:import|from)\s+([a-zA-Z0-9_.]+)\s*(?:\.\s*repository_interfaces\b|repository_interfaces\b)",
            )
            .unwrap(),
        }
    }
}
