mod cf;

use cf::common_types::ErrorNumberType;
use cf::file::FileException;

fn main() {
    println!(
        "Hello, world {}!",
        FileException::new(ErrorNumberType::CF_EAGAIN, String::from("Generic Error"))
    );
}
