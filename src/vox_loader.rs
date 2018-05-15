extern crate dot_vox;

use self::dot_vox::load;

#[derive(Default)]
pub struct VoxLoader {

}

impl VoxLoader {
    pub fn new() -> Self {
        let mut model = VoxLoader::default();

        model.load_from_file("resources/models/base.vox");
        model
    }

    fn load_from_file(&mut self, path: &str) {
        let result = load(path);

        println!("result.unwrap(): {:#?}", result.unwrap());
    }
}
