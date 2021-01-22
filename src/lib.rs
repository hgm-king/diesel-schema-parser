pub struct Table {
    pub name: String,
    pub primary_key: String,
    pub columns: Vec<Column>,
}

// name, type, nullable
pub type Column = (String, DataType);

pub type DataType = (String, bool);
