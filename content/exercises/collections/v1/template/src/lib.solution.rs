use std::collections::HashMap;

pub fn total_for(records: &[(String, u32)], target: &str) -> Option<u32> {
    let mut totals: HashMap<&str, u32> = HashMap::new();
    for (name, score) in records {
        *totals.entry(name.as_str()).or_insert(0) += *score;
    }
    totals.get(target).copied()
}

#[cfg(test)]
mod tests {
    use super::total_for;

    #[test]
    fn totals_scores_for_a_matching_name() {
        let records = vec![
            (String::from("Ada"), 10),
            (String::from("Lin"), 7),
            (String::from("Ada"), 5),
        ];
        assert_eq!(total_for(&records, "Ada"), Some(15));
    }

    #[test]
    fn returns_none_for_missing_names_and_empty_records() {
        assert_eq!(total_for(&[], "Ada"), None);
        let records = vec![(String::from("Lin"), 7)];
        assert_eq!(total_for(&records, "Ada"), None);
    }
}
