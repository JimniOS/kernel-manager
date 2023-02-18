pub mod gz;
pub mod kernel;
pub mod utils;

fn main() {
    let mut kernel = kernel::Kernel::new(".jimni", kernel::NewObjectPathType::Makefile);
    kernel.set_gz("/proc/config.gz");
    kernel.get_kernel_version();
    kernel
        .add_patch("Random.patch")
        .add_patch("Random_incompatible.patch")
        .add_patch("Random copy.patch");
    println!("{:#?}", kernel);
}
