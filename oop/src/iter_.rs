use std::iter::IntoIterator;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Archiver {
    pub value: Option<i32>,
    pub archive: Vec<i32>
}

impl Archiver {
    pub fn new() -> Self {
        Self {
            value: None,
            archive: Vec::new()
        }
    }

    // Pushes an item to value and pushes old value to archive
    pub fn push(&mut self, value: i32) {
        // if value is None, then this is a new archiver
        match self.value {
            None => {
                self.value = Some(value)
            }
            Some(v) => {
                self.archive.push(v)
            }
        }
    }

    // Pops value and makes value last item in archiver
    pub fn pop(&mut self) -> Option<i32> {
        let value = self.value;
        let new_value = self.archive.pop();
        self.value = new_value;
        value
    }
}

impl IntoIterator for Archiver {
    // each item in the iterator will be i32 
    type Item = i32;
    // our into_iter type will use the Vec implementation
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.archive.into_iter()
    }
}
