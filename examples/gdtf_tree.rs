fn main() {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: {} <path>", std::env::args().next().unwrap());
            std::process::exit(1);
        }
    };

    let gdtf = {
        let start = std::time::Instant::now();
        let gdtf = rigger::gdtf::Gdtf::from_archive(path);
        let duration = start.elapsed();
        println!("Loaded GDTF in {:?}", duration);
        gdtf
    };

    dbg!(gdtf);
}
