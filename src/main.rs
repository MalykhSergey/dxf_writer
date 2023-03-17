use std::{
    fs::{read_dir, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};
fn main() {
    let folder_path = read_dir("");
    let mut file_names = Vec::new();
    let mut counter = 0;
    let dxf_result_file = match File::create(&Path::new("result.dxf")) {
        Err(error) => panic!("Невозможно создать {}: {}", "result.txt", error),
        Ok(file) => file,
    };
    let txt_result_file = match File::create(&Path::new("result.txt")) {
        Err(error) => panic!("Невозможно создать {}: {}", "result.txt", error),
        Ok(file) => file,
    };
    let mut dxf_buf_writer = BufWriter::new(dxf_result_file);
    let mut txt_buf_writer = BufWriter::new(txt_result_file);
    match folder_path {
        Ok(read_directory) => {
            for path in read_directory {
                let file_name = path.unwrap().path().display().to_string();
                if file_name[file_name.len() - 3..file_name.len()].eq("ptx") {
                    file_names.push(file_name);
                }
            }
        }
        Err(error) => println!("Невозможно просмотреть файлы в папке {}", error),
    }
    dxf_buf_writer
        .write_all("0\nSECTION\n2\nENTITIES\n0\n".as_bytes())
        .expect("Неудалось записать!");
    for file_name in file_names {
        let file_path = Path::new(&file_name);
        let file = match File::open(&file_path) {
            Err(why) => panic!("Невозможно открыть {}: {}", file_path.display(), why),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        let coords = reader.lines().skip(9).next().unwrap().unwrap();
        let mut coords = coords.split_whitespace().into_iter();
        let mut x = coords.next().unwrap();
        x = &x[0..x.len() - 3];
        let mut y = coords.next().unwrap();
        y = &y[0..y.len() - 3];
        let mut z = coords.next().unwrap();
        z = &z[0..z.len() - 3];
        let dxf_data = format!(
            "POINT\n10\n{}\n20\n{}\n30\n{}\n0\nTEXT\n10\n{}\n20\n{}\n30\n{}\n40\n2.5\n1\n{}\n0\n",
            x,
            y,
            z,
            x,
            y,
            z,
            &file_name[0..file_name.len() - 4]
        );
        let txt_data =  format!("{};{};{};{}\n", &file_name[0..file_name.len() - 4],x,y,z);
        dxf_buf_writer
            .write_all(dxf_data.as_bytes())
            .expect("Неудалось записать основной файл!");
        txt_buf_writer.write_all(txt_data.as_bytes()).expect("Не удалось записать дополнительный файл!");
        counter += 1;
    }
    dxf_buf_writer
        .write_all("ENDSEC\n0\nEOF".as_bytes())
        .expect("Неудалось записать!");
    dxf_buf_writer.flush().expect("Ошибка записи в основной файл");
    txt_buf_writer.flush().expect("Ошибка записи в дополнительный файл");
    println!("Успешно завершено! {} точек записаны в result.dxf и result.txt", counter);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
