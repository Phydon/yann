pub mod util;
use crate::util::GradientDescent;

use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::{error, info, warn};

fn main() {
    // initialize the logger
    let _logger = Logger::try_with_str("info") // log info, warn and error
        .unwrap()
        .format_for_files(detailed_format) // use timestamp for every log
        .log_to_file(FileSpec::default().suppress_timestamp()) // no timestamps in the filename
        .append() // use only one logfile
        .duplicate_to_stderr(Duplicate::Warn) // print warnings and errors also to the console
        .start()
        .unwrap();
}
