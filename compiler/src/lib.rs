pub(crate) mod db;
mod field;
mod generator;
mod message;
pub mod options_pb;
pub mod plugin_pb;
mod util;

pub use self::db::Database;
pub use self::generator::Generator;

use pecan::reflection::FileDescriptorSet;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn generate(includes: FileDescriptorSet, source: &[impl AsRef<str>], target: impl AsRef<Path>) {
    let target = target.as_ref();
    let mut db = Database::default();
    for f in includes.file {
        db.load(f);
    }
    for path in source {
        let g = match db.generator_for(path.as_ref()) {
            Some(g) => g,
            None => panic!("file {} not in includes", path.as_ref()),
        };
        let res = g.generate();
        let t = target.join(g.file().target_path());
        if t.parent().map_or(false, |p| !p.exists()) {
            fs::create_dir_all(t.parent().unwrap()).unwrap();
        }
        fs::File::create(t.as_path())
            .unwrap()
            .write_all(res.to_string().as_bytes())
            .unwrap();
    }
}
