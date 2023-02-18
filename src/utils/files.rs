use md5_rs;
use std::{fmt::Error, fs, io::Write};

fn get_hash(string: &str) -> String {
    let mut ctx = md5_rs::Context::new();
    ctx.read(string.as_bytes());
    let digest = ctx.finish();

    digest
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>()
}
pub fn copy_file(src: &str, dest: &str) {
    let src_buffer = fs::read_to_string(src).expect("Unable to access source file!");
    let mut dest_file = fs::File::create(dest).expect("Unable to create dest file");
    write!(dest_file, "{}", src_buffer).expect("Unable to write to dest file");

    assert_eq!(
        get_hash(&src_buffer),
        get_hash(&fs::read_to_string(dest).unwrap())
    );
}
