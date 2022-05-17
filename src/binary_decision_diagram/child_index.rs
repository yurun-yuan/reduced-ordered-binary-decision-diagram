pub enum ChildIndex {
    Left,
    Right,
}

impl From<ChildIndex> for bool {
    fn from(child_index: ChildIndex) -> Self {
        match child_index {
            ChildIndex::Left => false,
            ChildIndex::Right => true,
        }
    }
}

impl From<ChildIndex> for usize {
    fn from(child_index: ChildIndex) -> Self {
        match child_index {
            ChildIndex::Left => 0usize,
            ChildIndex::Right => 1usize,
        }
    }
}