use std::{
    fs::{read_dir, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};
fn main() {
    let folder_path = read_dir("");
    let mut file_names = Vec::new();
    let complete_file_path = Path::new("result.dxf");
    let mut counter = 0;
    let mut result_file = match File::create(&complete_file_path) {
        Err(error) => panic!("Невозможно создать {}: {}", "result.txt", error),
        Ok(file) => file,
    };
    result_file
        .write_all("0\nSECTION\n2\nENTITIES\n0\n".as_bytes())
        .expect("Неудалось записать!");
    match folder_path {
        Ok(rd) => {
            for file_name in rd {
                file_names.push(file_name.unwrap().path().display().to_string());
            }
        }
        Err(error) => println!("Невозможно просмотреть файлы в папке {}", error),
    }
    for file_name in file_names {
        if !file_name[file_name.len() - 3..file_name.len()].eq("ptx") {
            continue;
        }
        let file_path = Path::new(&file_name);
        let file = match File::open(&file_path) {
            Err(why) => panic!("Невозможно открыть {}: {}", file_path.display(), why),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        let mut line_counter = 1;
        for line in reader.lines() {
            if line_counter == 10 {
                let coords = line.unwrap();
                let mut coords = coords.split_whitespace().into_iter();
                let mut x = coords.next().unwrap();
                x = &x[0..x.len() - 3];
                let mut y = coords.next().unwrap();
                y = &y[0..y.len() - 3];
                let mut z = coords.next().unwrap();
                z = &z[0..z.len() - 3];
                let data = format!(
                    "POINT\n10\n{}\n20\n{}\n30\n{}\n0\nTEXT\n10\n{}\n20\n{}\n30\n{}\n40\n2.5\n1\n{}\n0\n",
                    x, y, z, x, y, z,&file_name[0..file_name.len() - 4]
                );
                result_file
                    .write_all(data.as_bytes())
                    .expect("Неудалось записать!");
                counter += 1;
                break;
            }
            line_counter += 1;
        }
    }
    result_file
        .write_all("ENDSEC\n0\nEOF".as_bytes())
        .expect("Неудалось записать!");
    println!("Успешно завершено! {} точек записаны в result.dxf", counter);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
}
