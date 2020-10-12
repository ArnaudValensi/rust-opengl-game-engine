extern crate dot_vox;

use self::dot_vox::load;
use failure::Error;
use voxel::chunk::Chunk;

const ASSETS_DIRECTORY: &str = "./resources/voxs";
const SUPPORTED_VOX_VERSION: u32 = 150;

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
        let mut chunk = Chunk::new(model.size.x as u8, model.size.z as u8, model.size.y as u8);

        for &voxel in model.voxels.iter() {
            chunk.set_voxel(
                i64::from(voxel.x),
                i64::from(voxel.z),
                i64::from(model.size.y - u32::from(voxel.y) - 1),
                voxel.i,
            )?;
        }

        Ok(chunk)
    }
}

#[cfg(test)]
mod tests {
    use super::VoxLoader;
    use errors::print_errors_and_exit;

    #[test]
    fn load_vox() {
        let result = VoxLoader::load("base.vox");
        if let Err(ref e) = result {
            print_errors_and_exit(e);
        }

        let chunk = result.unwrap();

        assert_eq!(chunk.get_voxel(0, 0, 2).unwrap(), 78);
        assert_eq!(chunk.get_voxel(0, 0, 1).unwrap(), 78);
        assert_eq!(chunk.get_voxel(0, 0, 0).unwrap(), 78);
        assert_eq!(chunk.get_voxel(1, 0, 0).unwrap(), 78);
        assert_eq!(chunk.get_voxel(0, 1, 0).unwrap(), 78);
        assert_eq!(chunk.get_voxel(1, 3, 2).unwrap(), 0);
    }
}
