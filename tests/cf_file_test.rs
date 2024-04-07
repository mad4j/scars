#[cfg(test)]
mod tests {
    use std::path::Path;

    use scars::cf::file::{File, FileTrait};

    #[test]
    fn it_works() {
        if let Ok(mut f) = File::open(&String::from("Cargo.toml"), Path::new("./")) {
            let data = &mut vec![0; 1024];
            let result = f.read(data);
            println!("{:?}", data);
            println!("{:?}", result);
        } else {
            panic!();
        }
    }

    #[test]
    fn test_open_exception() {
        let n = String::from("xxxxx.xxx");
        let r = File::open(&n, Path::new("./"));
        match r {
            Ok(_) => todo!(),
            Err(e) => print!("{:?}", e),
        }
    }
}
