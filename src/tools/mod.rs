pub mod arxiv;

pub use self::arxiv::{ArxivClient, ArxivPaper};

pub mod prelude {
    pub use super::arxiv::{ArxivClient, ArxivPaper};
}
