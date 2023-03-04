use crate::utils;
use crate::utils::files::{self};
use crate::utils::patches::get_internal_version;
/// Importing the function `get_internal_version` from the module `versioning` in the directory `utils
use crate::utils::patches::Patch;
use regex::Regex;
use std::io::Write;
use std::path::Path;
/// A way to specify the type of configuration that is being used.
enum ConfigType {
    GZIP,
    CONFIG,
    DISTKERNEL,
}

/// A way to specify the distro configs.
enum DistConfig {
    Arch,
    Gentoo,
    Ubuntu,
    Fedora,
}

/// A way to create a new kernel object.
/// Relational way to set required paths in the kernel object
/// by specifying which path has been specified to us.
/// Example: On specifying KernelDirectory the program will append `Makefile` and
/// `.config` and assign them to Kernel::make_path and Kernel::config_path
pub enum NewObjectPathType {
    Makefile,
    Config,
    KernelDirectory,
}

pub struct Kernel {
    dir_path: String,
    make_path: String,
    config_path: String,
    config_type: ConfigType,
    internal_version: u32,
    patch_list: Vec<Patch>,
}

/// A way to print the struct in a formatted way.
impl std::fmt::Debug for Kernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let config_type = match self.config_type {
            ConfigType::GZIP => "Gzip",
            ConfigType::CONFIG => "User provided configuration",
            ConfigType::DISTKERNEL => "Dist Config",
        };
        f.debug_struct("Kernel")
            .field("Kernel Directory", &self.dir_path)
            .field("Kernel makefile", &self.make_path)
            .field("Kernel config ", &self.config_path)
            .field("Configuration type", &config_type)
            .field("Kernel version (internal)", &self.internal_version)
            .field("Patch List", &self.patch_list)
            .finish()
    }
}

// Methods for Kernel struct
impl Kernel {
    pub fn new(path: &str, path_type: NewObjectPathType) -> Self {
        let mut kdir_path: String = String::new();
        let mut makefile_path: String = String::new();
        let mut conf_path: String = String::new();
        match path_type {
            NewObjectPathType::Makefile => {
                // get kernel directory if Makefile is kdir/Makefile
                let _path = Path::new(path).parent().unwrap();
                conf_path = _path
                    .join(".config")
                    .into_os_string()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_string();

                kdir_path = _path.to_str().unwrap().to_string();
                makefile_path = path.to_string();
            }
            NewObjectPathType::Config => {
                let _path = Path::new(path).parent().unwrap();
                makefile_path = _path
                    .join("Makefile")
                    .into_os_string()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_string();
                conf_path = path.to_string();
            }
            NewObjectPathType::KernelDirectory => {
                let _path = Path::new(path);
                makefile_path = _path
                    .join(".config")
                    .into_os_string()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_string();
                conf_path = _path
                    .join(".config")
                    .into_os_string()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_string();
                kdir_path = path.to_string();
            }
        }
        if kdir_path.is_empty() {
            kdir_path = "./".to_string();
        }

        // check if config file exists
        // if not, try copying gz, if failing, copy dist config (wip)

        let mut obj = Kernel {
            dir_path: kdir_path,
            make_path: makefile_path,
            config_path: conf_path,
            config_type: ConfigType::CONFIG,
            internal_version: 0,
            patch_list: vec![],
        };
        if !Path::new(&obj.config_path).exists() {
            println!("Config file not found, copying gz file");
            obj.set_gz("/proc/config.gz");
        }

        obj
    }

    pub fn set_gz(&mut self, path: &str) -> bool {
        self.config_type = ConfigType::GZIP;
        // we will copy content of gz files to .config
        //backup file
        let mut dest_dir = self.config_path.clone();
        dest_dir.push_str(".bak");
        if !Path::new(&self.config_path).exists() {
            std::fs::File::create(&self.config_path).expect("Couldnt create file");
        }
        files::copy_file(&self.config_path, &dest_dir);

        let gz_config_buffer = crate::gz::read_gz(path);

        let gz_config_buffer = match gz_config_buffer {
            Some(_) => gz_config_buffer.unwrap(),
            None => return false,
        };

        let mut dest_file =
            std::fs::File::create(&self.config_path).expect("Unable to create dest file");
        write!(dest_file, "{}", gz_config_buffer).expect("Error writing to file");

        true
    }

    pub fn get_kernel_version(&mut self) {
        let input_str = std::fs::read_to_string(&self.make_path).unwrap();
        let tokens = &input_str
            .split('\n')
            .map(|line| line.to_string())
            .collect::<Vec<String>>()[0..3];

        let input_str = format!("{}\n{}\n{}\n", tokens[0], tokens[1], tokens[2]);

        let re =
            Regex::new(r"(?m)^VERSION\s*=\s*(\d+)\nPATCHLEVEL\s*=\s*(\d+)\nSUBLEVEL\s*=\s*(\d+)")
                .unwrap();

        let output_str = format!(
            "{}_{}_{}",
            re.replace_all(&input_str, "$1").trim(),
            re.replace_all(&input_str, "$2").trim(),
            re.replace_all(&input_str, "$3").trim(),
            // re.replace_all(input_str, "$3")
        );
        self.internal_version = get_internal_version(&output_str)
    }

    pub fn add_patch(&mut self, patch_path: &str) -> &mut Self {
        let patch = crate::utils::patches::Patch::new(patch_path);
        if patch.internal_version == self.internal_version {
            self.patch_list.push(patch);
        } else {
            println!("Incompatible patch detected! Skipping");
        };
        self
    }
}
