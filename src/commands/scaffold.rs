use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn scaffold(_year: u16, _day: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("year and day {} {}", _year, _day);
    let directory = format!("./src/year_{}", _year);
    let file_path = format!("day_{}.rs", _day);

    if !Path::new(&directory).exists() {
        fs::create_dir(&directory)?;
    }

    println!(
        "{}",
        Path::new(&format!("{directory}"))
            .join(&file_path)
            .to_str()
            .unwrap()
    );
    let mut file = File::create_new(Path::new(&format!("{directory}")).join(file_path))?;
    let contents = fs::read_to_string("./src/template/template.txt")?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
