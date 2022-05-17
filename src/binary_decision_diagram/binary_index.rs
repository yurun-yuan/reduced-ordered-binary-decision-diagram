pub enum BinaryIndex {
    Left,
    Right,
}

impl From<BinaryIndex> for bool {
    fn from(child_index: BinaryIndex) -> Self {
        match child_index {
            BinaryIndex::Left => false,
            BinaryIndex::Right => true,
        }
    }
}

impl From<BinaryIndex> for usize {
    fn from(child_index: BinaryIndex) -> Self {
        match child_index {
            BinaryIndex::Left => 0usize,
            BinaryIndex::Right => 1usize,
        }
    }
}