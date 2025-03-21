pub mod file;
pub mod folder;

pub use file::File;
pub use folder::Folder;

pub trait Component {
    fn search(&self,keyword: &str);
}