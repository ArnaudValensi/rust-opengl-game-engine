extern crate dot_vox;

use self::dot_vox::load;

#[derive(Default)]
pub struct VoxLoader {

}

impl VoxLoader {
    pub fn load(filepath: &str) -> Result<(), Error> {
        let dot_vox_data = match load(filepath) {
            Err(message) => return Err(format_err!("Tryed to open vox file at '{}' and got the following error: {}", filepath, message)),
            Ok(dot_vox_data) => dot_vox_data
        };

        model.load_from_file("resources/models/base.vox");
        model
    }

    fn load_from_file(&mut self, path: &str) {
        let result = load(path);

        println!("result.unwrap(): {:#?}", result.unwrap());
    }
}
