use std::fs;
use std::path::Path;

const DUMMY_PATH: &str = "E:\\Code\\Rust\\Rust-10\\file_organizer\\dummy_dir";

fn main() {
    println!("File Organizer");

    //1. start a loop over the files in the provided dir in dummy path
    if let Ok(entries) = fs::read_dir(DUMMY_PATH) {
        //2. loop over the entries and extract the extension
        for entry in entries {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    let file_name = path.file_name().unwrap();
                    //-> if it is dir, then skip it
                    if path.is_dir() {
                        continue; // skip the sub dirs
                    } else {
                        //-> if not then extract the extension name
                        match path.extension() {
                            Some(ext) => {
                                let ext_str = ext.to_string_lossy().to_string();
                                let sub_dir_path = Path::new(DUMMY_PATH).join(&ext_str);
                                //-> if the folder with ext name is present store that specific entry in that sub dir
                                if !(sub_dir_path.exists() && sub_dir_path.is_dir()) {
                                    match fs::create_dir(sub_dir_path) {
                                        Ok(_) => {
                                            println!("Directory created");
                                        }
                                        Err(e) => {
                                            println!("Error while creating directory,{:?}", e);
                                            break;
                                        }
                                    }
                                }
                                //move the current entry to the created directory or existed directory
                                //source path -> DUMMY_DIR+FILENAME
                                let source_path = Path::new(DUMMY_PATH).join(file_name);
                                //dest path -> DUMMY_DIR + ext_str + FILENAME
                                let dest_path =
                                    Path::new(DUMMY_PATH).join(&ext_str).join(file_name);
                                match fs::rename(source_path, dest_path) {
                                    Ok(_) => (),
                                    Err(e) => println!("Error occured while moving the file{}", e),
                                }
                            }
                            None => println!("Error extracting extension"),
                        }
                    }
                }

                Err(e) => eprintln!("Error reading entry{:?}", e),
            }
        }
    }

    //-> if there is no folder with name, create one, and store that entry
}
