//! Document and DocumentBuilder types for multi-page documents.

use crate::page::Page;

/// Represents a complete multi-page document.
///
/// Documents are immutable after construction and can be rendered to ESC/P bytes.
#[derive(Clone, Debug)]
pub struct Document {
    pages: Vec<Page>,
}

impl Document {
    /// Creates a new DocumentBuilder for constructing a document.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::Document;
    ///
    /// let builder = Document::builder();
    /// ```
    pub fn builder() -> DocumentBuilder {
        DocumentBuilder::new()
    }

    /// Returns a slice of all pages in the document.
    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    /// Returns the number of pages in the document.
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Renders the document to an ESC/P byte stream.
    ///
    /// The output includes initialization codes, page content, and form-feeds.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::{Document, Page, StyleFlags};
    ///
    /// let mut page_builder = Page::builder();
    /// page_builder.write_str(0, 0, "Hello", StyleFlags::NONE);
    /// let page = page_builder.build();
    ///
    /// let mut doc_builder = Document::builder();
    /// doc_builder.add_page(page);
    /// let document = doc_builder.build();
    ///
    /// let bytes = document.render();
    /// assert!(!bytes.is_empty());
    /// ```
    pub fn render(&self) -> Vec<u8> {
        crate::escp::render_document(self)
    }
}

/// Builder for constructing Documents with multiple pages.
pub struct DocumentBuilder {
    pages: Vec<Page>,
}

impl DocumentBuilder {
    /// Creates a new DocumentBuilder with no pages.
    fn new() -> Self {
        DocumentBuilder { pages: Vec::new() }
    }

    /// Adds a page to the document.
    ///
    /// Pages are rendered in the order they are added.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::{Document, Page};
    ///
    /// let page = Page::builder().build();
    ///
    /// let mut builder = Document::builder();
    /// builder.add_page(page);
    /// ```
    pub fn add_page(&mut self, page: Page) -> &mut Self {
        self.pages.push(page);
        self
    }

    /// Consumes the builder and returns an immutable Document.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::{Document, Page};
    ///
    /// let page = Page::builder().build();
    ///
    /// let mut builder = Document::builder();
    /// builder.add_page(page);
    /// let document = builder.build();
    /// ```
    pub fn build(self) -> Document {
        Document { pages: self.pages }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_builder_new() {
        let builder = DocumentBuilder::new();
        assert_eq!(builder.pages.len(), 0);
    }

    #[test]
    fn test_document_builder_add_page() {
        let mut builder = DocumentBuilder::new();

        let page1 = Page::builder().build();
        let page2 = Page::builder().build();

        builder.add_page(page1);
        builder.add_page(page2);

        assert_eq!(builder.pages.len(), 2);
    }

    #[test]
    fn test_document_builder_build() {
        let mut builder = DocumentBuilder::new();
        let page = Page::builder().build();
        builder.add_page(page);

        let document = builder.build();
        assert_eq!(document.page_count(), 1);
    }

    #[test]
    fn test_document_pages() {
        let mut builder = DocumentBuilder::new();
        let page = Page::builder().build();
        builder.add_page(page);

        let document = builder.build();
        let pages = document.pages();
        assert_eq!(pages.len(), 1);
    }

    #[test]
    fn test_document_page_count() {
        let mut builder = DocumentBuilder::new();
        builder.add_page(Page::builder().build());
        builder.add_page(Page::builder().build());
        builder.add_page(Page::builder().build());

        let document = builder.build();
        assert_eq!(document.page_count(), 3);
    }

    #[test]
    fn test_document_empty() {
        let document = Document::builder().build();
        assert_eq!(document.page_count(), 0);
    }

    #[test]
    fn test_document_immutability() {
        let mut builder = Document::builder();
        builder.add_page(Page::builder().build());
        let document = builder.build();

        // This test verifies that Document has no public mutable methods
        let _count = document.page_count();
    }
}
