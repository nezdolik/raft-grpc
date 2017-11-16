use std::cmp::Ordering;
use std::time::Instant;

pub struct Term {
    start: Instant,
    end:   Instant
}

impl Ord for Term {
    fn cmp(&self, other: &Term) -> Ordering {
        self.end.cmp(&other.end)
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Term) -> bool {
        self.end.eq(&other.end) && self.start.eq(&other.start)
    }
}

impl Eq for Term {

}



