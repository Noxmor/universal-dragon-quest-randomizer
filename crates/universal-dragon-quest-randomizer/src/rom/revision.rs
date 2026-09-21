pub type RevisionValue = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Revision(RevisionValue);

impl Revision {
    pub const fn new(rev: RevisionValue) -> Self {
        Self(rev)
    }

    pub const fn value(&self) -> RevisionValue {
        self.0
    }
}
