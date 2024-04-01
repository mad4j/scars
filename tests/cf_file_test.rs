
#[cfg(test)]
mod tests {
    use scars::cf::file::{File, FileTrait};


    #[test]
    fn it_works() {
        if let Ok(mut f) = File::open(&String::from("Cargo.toml")) {

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

        let f = File::open(&String::from("xxxxx.xxx"));
    }
}
