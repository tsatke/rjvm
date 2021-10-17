use crate::vm::class::Class;
use crate::vm::classloader::classpath::ClassPathEntry;


pub mod bootstrap;
pub mod class;
pub mod classpath;

pub trait ClassLoader {
    fn add_entry(&mut self, entry: ClassPathEntry);

    fn find_class<N>(&self, name: N) -> Option<&Class>
    where
        N: AsRef<str>;

    fn find_or_load_class<N>(&mut self, name: N) -> Option<&Class>
    where
        N: AsRef<str>;
}
