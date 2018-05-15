extern crate dot_vox;

use std::path::Path;
use dot_vox::load;

struct Chunk {

}

impl Chunk {
    pub fn new() -> Self {
        let mut model = Chunk::default();

        model.load_from_file("resources/models/chr_old.vox");
        model
    }

    fn load_from_file(&mut self, path: &str) {
        let path = Path::new(path);

        // retrieve the directory path of the filepath
        let directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();

        println!("directory: {:?}", directory);

        let result = load("src/resources/placeholder.vox");

        println!("result.unwrap(): {:?}", result.unwrap());
    }
}
