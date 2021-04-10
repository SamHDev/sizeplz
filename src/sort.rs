use crate::calc::Record;

pub enum SortKey {
    Order,
    Name,
    Size,
    Created,
    Modified
}

impl SortKey {
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

    pub fn sort(&self, mut records: & mut Vec<Record>) {
        match &self {
            SortKey::Order => {},
            SortKey::Name => records.sort_by_key(|x| x.name.to_string()),
            SortKey::Size => records.sort_by_key(|x| x.size),
            SortKey::Created => records.sort_by_key(|x| x.created),
            SortKey::Modified => records.sort_by_key(|x| x.modified),
        }
    }
}