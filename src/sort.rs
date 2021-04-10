use crate::calc::Record;

// sort keys/methods
pub enum SortKey {
    Order,
    Name,
    Size,
    Created,
    Modified
}

impl SortKey {
    // parse a sort key from args.
    pub fn parse(x: &str) -> Option<SortKey> {
        match x {
            "order" => Some(Self::Order),
            "name" => Some(Self::Name),
            "size" => Some(Self::Size),
            "created" => Some(Self::Created),
            "modified" => Some(Self::Modified),

            _ => None
        }
    }

    // sort a mut ref to an array of records using given key type.
    pub fn sort(&self, records: & mut Vec<Record>) {
        match &self {
            SortKey::Order => {},
            SortKey::Name => records.sort_by_key(|x| x.name.to_string()),
            SortKey::Size => records.sort_by_key(|x| x.size),
            SortKey::Created => records.sort_by_key(|x| x.created),
            SortKey::Modified => records.sort_by_key(|x| x.modified),
        }
    }
}