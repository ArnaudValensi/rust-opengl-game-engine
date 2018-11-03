extern crate dot_vox;

use self::dot_vox::load;
use failure::Error;
use voxel::chunk::Chunk;

const ASSETS_DIRECTORY: &str = "../assets";
const SUPPORTED_VOX_VERSION: u32 = 150;

// #[derive(Fail, Debug)]
// #[fail(display = "Input was invalid UTF-8 at index {}", index)]
// pub struct DotVoxError {
//     message: &str,
// }

#[derive(Default)]
pub struct VoxLoader {}

impl VoxLoader {
    pub fn load(asset_name: &str) -> Result<Chunk, Error> {
        let filepath = format!("{}/{}", ASSETS_DIRECTORY, asset_name);
        let dot_vox_data = match load(&filepath) {
            Err(message) => {
                return Err(format_err!(
                    "Tryed to open the asset file at '{}' and got the following error: {}.",
                    filepath,
                    message,
                ))
            }
            Ok(dot_vox_data) => dot_vox_data,
        };

        if dot_vox_data.version != SUPPORTED_VOX_VERSION {
            return Err(format_err!(
                "Cannot load the asset file at '{}' because its vox version is {} and only the version 150 is supported.",
                filepath,
                dot_vox_data.version,
            ));
        }

        let nb_models = dot_vox_data.models.len();

        if nb_models == 0 {
            return Err(format_err!(
                "No models found in the asset file at '{}'",
                filepath
            ));
        }

        if nb_models > 1 {
            warn!("Mutliple models are found in the asset file at '{}', but only one the first one will be loaded.", filepath);
        }

        let model = &dot_vox_data.models[0];
        let mut chunk2 = Chunk::new(model.size.x as u8, model.size.z as u8, model.size.y as u8);

        for &voxel in model.voxels.iter() {
            chunk2.set_voxel(
                i64::from(voxel.x),
                i64::from(voxel.z),
                i64::from(model.size.y - u32::from(voxel.y) - 1),
                voxel.i,
            )?;
        }

        Ok(chunk2)
    }
}

#[cfg(test)]
mod tests {
    use super::VoxLoader;
    use errors::print_errors_and_exit;

    #[test]
    fn load_vox() {
        if let Err(ref e) = VoxLoader::load("base.vox") {
            print_errors_and_exit(e);
        }
        // assert!(tree.nodes.len() == 3, "it should have 3 nodes in the tree");
    }
}
