pub struct Profile {
    pub name: String,
    pub age: u8,
    display_name: String,
}

impl Profile {
    pub fn new(name: String, age: u8) -> Self {
        let display_name = if name.is_empty() {
            String::from("Anonymous")
        } else {
            name.clone()
        };
        Self {
            name,
            age,
            display_name,
        }
    }

    pub fn summary(&self) -> String {
        format!("{} ({})", self.display_name, self.age)
    }
}

#[cfg(test)]
mod tests {
    use super::Profile;

    #[test]
    fn constructs_profile_and_reads_public_fields() {
        let profile = Profile::new(String::from("Ada"), 36);
        assert_eq!(profile.name, "Ada");
        assert_eq!(profile.age, 36);
        assert_eq!(profile.summary(), "Ada (36)");
    }

    #[test]
    fn uses_anonymous_for_empty_name() {
        let profile = Profile::new(String::new(), 0);
        assert_eq!(profile.summary(), "Anonymous (0)");
    }
}
