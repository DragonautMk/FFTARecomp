//! Named host input slots. Choosing a reference never starts the tool.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContentSlot {
    Replay,
    Catalog,
    Session,
    Recording,
    Snapshot,
    Input,
    Dataset,
    RuntimeA,
    RuntimeB,
    Library,
    Labels,
    KnownPlan,
}
impl ContentSlot {
    pub const ALL: [Self; 12] = [
        Self::Replay,
        Self::Catalog,
        Self::Session,
        Self::Recording,
        Self::Snapshot,
        Self::Input,
        Self::Dataset,
        Self::RuntimeA,
        Self::RuntimeB,
        Self::Library,
        Self::Labels,
        Self::KnownPlan,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Replay => "Replay / audit / capture / compare profile",
            Self::Catalog => "Save catalog",
            Self::Session => "Isolated session",
            Self::Recording => "Input recording",
            Self::Snapshot => "Snapshot",
            Self::Input => "Input sequence",
            Self::Dataset => "Dataset",
            Self::RuntimeA => "Reference runtime (A)",
            Self::RuntimeB => "Candidate runtime (B)",
            Self::Library => "Native library",
            Self::Labels => "Code labels",
            Self::KnownPlan => "Known resource plan",
        }
    }
}
