use packstream_serde::Value;
use packstream_serde::message::Success;

#[derive(Debug)]
pub struct Response<T> {
    fields: Vec<Value>,
    rows: Vec<Vec<T>>,
}

impl<T> Response<T> {
    pub fn new() -> Self {
        Self { fields: Vec::new(), rows: Vec::new() }
    }

    pub fn push_row(&mut self, row: Vec<T>) -> () {
        self.rows.push(row);
    }
    
    pub fn into_rows(self) -> Vec<Vec<T>> {
        self.rows
    }

    pub fn into_fields(self) -> Vec<Value> {
        self.fields
    }

    pub fn fields(&self) -> &Vec<Value> {
        &self.fields
    }

    pub fn rows(&self) -> &Vec<Vec<T>> {
        &self.rows
    }
}

impl<T> From<Success> for Response<T> {
    // TODO: check if there is always "fields" key in the success metadata message.
    fn from(mut success: Success) -> Self {
        if let Some(Value::List(v)) = success.metadata.remove("fields") {
            Self {
                fields: v,
                rows: Vec::new(),
            }
        } else {
            unreachable!("It is assumed that success message has 'fields' key in metadata.");
        }
    }
}
