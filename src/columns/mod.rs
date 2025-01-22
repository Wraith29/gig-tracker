pub mod data;
pub mod graph;

#[derive(Hash)]
pub enum ColumnName {
    Data,
    Graph,
}
