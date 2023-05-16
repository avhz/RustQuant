// Degenerate base class for the Acyclic Visitor pattern
pub trait AcyclicVisitor: Drop {}

// Visitor for a specific class
pub trait Visitor<T> {
    fn visit(&mut self, t: &mut T);
}
