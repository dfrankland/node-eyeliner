use eyeliner::servo_embedder_traits::resources::{Resource, ResourceReaderMethods};
use std::{fs::File, io::Read, path::PathBuf};

pub struct ResourceReader {
    pub prefs: PathBuf,
}

impl ResourceReaderMethods for ResourceReader {
    fn sandbox_access_files(&self) -> Vec<PathBuf> {
        vec![]
    }
    fn sandbox_access_files_dirs(&self) -> Vec<PathBuf> {
        vec![]
    }
    fn read(&self, file: Resource) -> Vec<u8> {
        let path = match file {
            Resource::Preferences => &self.prefs,
            _ => panic!("Can't find file"),
        };
        let mut buffer = vec![];
        File::open(path)
            .expect(&format!("Can't find file: {:?}", path))
            .read_to_end(&mut buffer)
            .expect("Can't read file");
        buffer
    }
}
