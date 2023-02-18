pub enum Schedulers {
    PDS,
    Calule,
    BMQ,
    CFS,
    All,
}

/// It takes a string of numbers separated by underscores, and returns the sum of the numbers multiplied
/// by the base power of 10
/// 
/// Arguments:
/// 
/// * `external_version`: The version string that you want to convert to an integer.
/// 
/// Returns:
/// 
/// The internal version of the external version.
pub fn get_internal_version(external_version:&str) -> u32{
    let mut sum: u32 = 0;
    let mut base_power: i64 = 10_i64.pow(external_version.split('_').count() as u32);
    external_version.split('_').for_each(|num| {
        sum += num.parse::<u32>().unwrap() * base_power as u32;
        base_power /= 10;
    });
    sum
}

pub struct Patch {
    pub path: String,
    pub internal_version: u32,
    pub sched_compat: Schedulers,
}

impl core::fmt::Debug for Patch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scheduler = match &self.sched_compat {
            Schedulers::PDS => "PDS",
            Schedulers::Calule => "calULE",
            Schedulers::BMQ => "BMQ",
            Schedulers::CFS => "CFS",
            Schedulers::All => "*",
        };
        f.debug_struct("Version")
            .field("Path", &self.path)
            .field("Version", &self.internal_version)
            .field("Scheduler support", &scheduler)
            .finish()
    }
}
impl Patch {
    /// `new` is a function that takes a string and returns a `Patch` object
    /// 
    /// Arguments:
    /// 
    /// * `path`: The path to the patch file.
    /// 
    /// Returns:
    /// 
    /// A Patch object.
    pub fn new(path: &str) -> Self {
        let mut object = Patch {
            path: path.to_string(),
            internal_version: 0000,
            sched_compat: Schedulers::All,
        };
        object.initialise();
        object
    }
    /// It reads the first line of the file, and parses it to extract the scheduler compatibility and the
    /// internal version
    pub fn initialise(&mut self) {
        // we read the last line
        let binding = std::fs::read_to_string(self.path.clone()).expect("Error");
        let lines: Vec<&str> = binding.lines().collect();
        let first_line = lines[0];

        first_line.split(';').for_each(|token| {
            let sub_tokens: Vec<&str> = token.split(':').collect();
            if sub_tokens[0] == "Sched" {
                match sub_tokens[1] {
                    "PDS" => self.sched_compat = Schedulers::PDS,
                    "BMQ" => self.sched_compat = Schedulers::BMQ,
                    "CFS" => self.sched_compat = Schedulers::CFS,
                    "calULE" => self.sched_compat = Schedulers::Calule,
                    "*" => self.sched_compat = Schedulers::All,
                    &_ => self.sched_compat = Schedulers::All,
                }
            } else if sub_tokens[0] == "Version" {
                self.internal_version = get_internal_version(sub_tokens[1]);
            }
        });
    }
}
