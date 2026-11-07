//! softmaker-freeoffice model — FreeOffice office suite.

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub type_: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub documents: Vec<Document>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { documents: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, title: &str, type_: &str) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.documents.push(Document { id: id.clone(), title: title.into(), type_: type_.into(), content: String::new() });
        id
    }

    pub fn save(&mut self, id: &str, content: &str) -> bool {
        if let Some(d) = self.documents.iter_mut().find(|d| d.id == id) {
            d.content = content.into();
            true
        } else { false }
    }

    pub fn list(&self) -> &[Document] { &self.documents }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn document_create_save() {
        let mut s = Model::new();
        let id = s.create("Report", "word");
        assert!(s.save(&id, "content"));
    }
}
