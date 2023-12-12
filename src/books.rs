use async_graphql::{Object, Context};

pub struct Book {
    id: usize,
    name: &'static str,
    author_id: usize,
}

pub struct Author {
    id: usize,
    name: &'static str,
}
pub struct Library {
    books: Vec<Book>,
    authors: Vec<Author>,
}

pub struct QueryRoot;

#[Object]
impl Book {
    async fn id(&self) -> usize {
        self.id
    }

    async fn name(&self) -> &str {
        self.name
    }

    async fn author_id(&self) -> usize {
        self.author_id
    }
}

#[Object]
impl Author {
    async fn id(&self) -> usize {
        self.id
    }

    async fn name(&self) -> &str {
        self.name
    }
}

impl Library {
    pub fn new() -> Self {
        Self {
            books: vec![Book {id: 0, name: "Lord of the Rings", author_id: 0}, Book {id: 1, name: "Harry Potter and the Prizoner of Azkaban", author_id: 1}],
            authors: vec![Author {id: 0, name: "J. R. R. Tolkien"}, Author {id: 1, name: "J. K. Rowling"}],
        }
    }

    pub fn books(&self) -> Vec<&Book> {
        self.books
        .iter()
        .collect()
    }

    pub fn authors(&self) -> Vec<&Author> {
        self.authors
        .iter()
        .collect()
    }
}

#[Object]
impl QueryRoot {
    async fn books<'a>(&self, ctx: &Context<'a>) -> Vec<&'a Book> {
        let library = ctx.data_unchecked::<Library>();
        library.books()
    }

    async fn book<'a>(&self, ctx: &Context<'a>, id: usize) -> &'a Book {
        let library = ctx.data_unchecked::<Library>();
        library.books.get(id).unwrap()
    }

    async fn authors<'a>(&self, ctx: &Context<'a>) -> Vec<&'a Author> {
        let library = ctx.data_unchecked::<Library>();
        library.authors()
    }

    async fn author<'a>(&self, ctx: &Context<'a>, id: usize) -> &'a Author {
        let library = ctx.data_unchecked::<Library>();
        library.authors.get(id).unwrap()
    }
}