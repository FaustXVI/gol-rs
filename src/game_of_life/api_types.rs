
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Row(pub usize);
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Column(pub usize);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

pub trait Grid {
    fn size(&self) -> Size;
    fn has_cell_at(&self, row: Row, column: Column) -> bool;
}

pub trait Evolving {
    fn next_generation(self) -> Self;
}
