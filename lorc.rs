use std::env;
use std::fs;
use std::fs::read_to_string;
use std::fs::remove_file;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn main() -> io::Result<()> {
    let command_line_arguments: Vec<String> = env::args().collect(); // COMMAND LINE ARGUMENTS
    let cleanup_mode: &String = &command_line_arguments[1];
    let mut skip: bool = false;

    // OTHER ARGS {
    if cleanup_mode == "-v" || cleanup_mode == "--version" {
        println!(
            "{}",
            read_to_string("STD/.version").expect("Version Missing")
        );
        std::process::abort();
    } else if cleanup_mode == "-h" || cleanup_mode == "--help" || cleanup_mode == "-?" {
        println!("(./)lorc(.exe) [Cleanup mode] [File path] [Config file path]\nCleanup modes:\n-a : All\n-m : Manifest\n-j : .java\n-c : .class\n-n : None");
        std::process::abort();
    }
    // }

    let source_file: &String = &command_line_arguments[2];
    let import_path: &String = &command_line_arguments[3];

    // IMPORTS {
    if import_path == "None" {
        skip = true;
    }
    let config_path: &Path = Path::new(import_path);
    let splitted_config: Vec<&str> = import_path.split(".").collect();

    if splitted_config[1] != "vn" {
        panic!("Wrong filetype! Please ensure that the file ends with \".vn\"");
    }

    let display_config = config_path.display();
    let mut config_file: File = match File::open(&config_path) {
        Err(why) => panic!("Could not open {}: {}", display_config, why),
        Ok(config_file) => config_file,
    };

    let mut config_file_contents: String = String::new();
    match config_file.read_to_string(&mut config_file_contents) {
        Err(why) => panic!("Could not read {}: {}", display_config, why),
        Ok(_) => println!(""),
    }

    let to_import = file_to_vec(import_path.to_string())?;

    // }

    let source_path: &Path = Path::new(source_file);

    // FILE {

    let display_source = source_path.display();
    let mut file_source: File = match File::open(&source_path) {
        Err(why) => panic!("Could not open {}: {}", display_source, why),
        Ok(file_source) => file_source,
    };

    let mut file_conents: String = String::new();
    match file_source.read_to_string(&mut file_conents) {
        Err(why) => panic!("Could not read {}: {}", display_source, why),
        Ok(_) => println!(""),
    }

    // }

    // FILEEXT & FILENAME

    let splitted_name: Vec<&str> = source_file.split(".").collect();
    let endf: Vec<&str> = splitted_name[0].split("/").collect();
    let namef = endf[endf.len() - 1];

    let javaf: String = format!("{}.java", namef);
    let classf: String = format!("{}.class", namef);
    let jarf: String = format!("{}.jar", namef);

    if splitted_name[1] != "lsmx" {
        panic!("Wrong filetype! Please ensure that the file ends with \".lsmx\"");
    }

    // MANIFEST {

    let path_manifest: &Path = Path::new("Manifest.txt");
    let display_manifest = path_manifest.display();

    let mut file_manifest: File = match File::create("Manifest.txt") {
        Err(why) => panic!("Could not create {}: {}", display_manifest, why),
        Ok(file_manifest) => file_manifest,
    };

    let manifest: String = format!("Main-Class: {}", namef);
    match file_manifest.write_all(manifest.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", display_manifest, why),
        Ok(_) => println!("Successfully wrote to {}", display_manifest),
    }

    // }

    // JAVA {

    let path_java: &Path = Path::new(&javaf);
    let display_java = path_java.display();

    let mut java_write: File = match File::create(&path_java) {
        Err(why) => panic!("Could not create {}: {}", display_java, why),
        Ok(java_write) => java_write,
    };

    // }

    let mut imported: String = "".to_owned();
    let mut aditlibs: String = "".to_owned();

    // IMPORTS {

    if !skip {
        for import in to_import {
            if import == "STD.SQRT" {
                let sqrtlib: String =
                    read_to_string("STD/sqrts.ryx").expect("SQRT Library Missing");
                let str: String = format!("{}", sqrtlib);
                aditlibs.push_str(&str);
            } else if import == "STD.RANDOM" {
                let randomlib: String =
                    read_to_string("STD/random.ryx").expect("RANDOM Library Missing");
                let str: String = format!("{}", randomlib);
                aditlibs.push_str(&str);
            } else if import == "STD.IN" {
                let inlib: String = read_to_string("STD/in.ryx").expect("INPUT Library Missing");
                let str: String = format!("{}", inlib);
                aditlibs.push_str(&str);
            } else if import == "STD.CASE" {
                let caselib: String = read_to_string("STD/case.ryx").expect("CASE Library Missing");
                let str: String = format!("{}", caselib);
                aditlibs.push_str(&str);
            } else if import == "STD.SLEEP" {
                let waitlib: String =
                    read_to_string("STD/sleep.ryx").expect("SLEEP Library Missing");
                let str: String = format!("{}", waitlib);
                aditlibs.push_str(&str);
            } else if import == "STD.OUT" {
                let outlib: String = read_to_string("STD/out.ryx").expect("OUTPUT Library Missing");
                let str: String = format!("{}", outlib);
                aditlibs.push_str(&str);
            } else if import == "STD.SHELL" {
                let shelllib: String =
                    read_to_string("STD/shell.ryx").expect("SHELL Library Missing");
                let str: String = format!("{}", shelllib);
                aditlibs.push_str(&str);
            } else {
                let splitted_import: Vec<&str> = import.split(".").collect();
                if splitted_import[0] == "java" {
                    let str: String = format!("import {};", import);
                    imported.push_str(&str);
                } else if splitted_import[1] == "ryx" {
                    let error: String = format!("Unable to read import: {}", import);
                    let importfile: String = read_to_string(import).expect(&error);
                    aditlibs.push_str(&importfile);
                } else {
                    panic!(
                        "Wrong file name! Imported file must end in \".ryx\" or be a Java import."
                    );
                }
            }
        }
    }

    // }

    // TRANSPARSING {
    let self_dund = format!("var __self__ = new {}();", namef);

    let to_write: String = format!(
        "{imports}\npublic class {name} {{\n{adit}\npublic static void main(String[] args){{{code}}}\n}}",
        name = namef,
        code = file_conents
        .replace("_fn", "} private static ")
        .replace(":=", "{")
        .replace("=:", "")
        .replace("ret","return")
        .replace("bool","boolean")
        .replace("_match","switch")
        .replace("elif","else if")
        .replace(":::", "//")
        .replace("_const", "final")
        .replace("new_self!", &self_dund)
        .replace("exit!", "System.exit(0);")
        .replace("abort!", "System.exit(1);"),
        imports = imported,
        adit = aditlibs,
    );
    match java_write.write_all(to_write.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", display_source, why),
        Ok(_) => println!("Successfully wrote to {}", display_source),
    }

    // }

    // COMPILATION {

    let output_class = Command::new("javac") // TO .class
        .arg(javaf)
        .output()
        .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));

    if output_class.status.success() {
        let output_java = Command::new("jar") // TO .jar
            .args(["cvfm", &jarf, "Manifest.txt", &classf])
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));

        if output_java.status.success() {
            println!("JAR succeeded\n");
        } else {
            let s = String::from_utf8_lossy(&output_java.stderr);

            println!("JAR failed and stderr was:\n{}", s);
        }
    } else {
        let s = String::from_utf8_lossy(&output_class.stderr);

        println!("JAVAC failed and stderr was:\n{}", s);
    }

    let os: &str = env::consts::OS;

    if os == "windows" {
        let cmd: String = format!("@echo off\njava {}\npause>nul\nexit", namef);
        let cmd_path: &Path = Path::new("Command.cmd"); // CREATE Command.cmd
        let display_cmd = cmd_path.display();

        let mut file_command: File = match File::create(&cmd_path) {
            Err(why) => panic!("Could not create {}: {}", display_cmd, why),
            Ok(file_command) => file_command,
        };

        match file_command.write_all(cmd.as_bytes()) {
            Err(why) => panic!("Could not write to {}: {}", display_cmd, why),
            Ok(_) => println!("Successfully wrote to {}", display_cmd),
        }

        let output_run = Command::new("cmd") // START Command.cmd
            .args(["/C", "start Command.cmd"])
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));
        if output_run.status.success() {
            println!("JAVA succeeded");
        } else {
            let s = String::from_utf8_lossy(&output_run.stderr);

            println!("JAVA failed and stderr was:\n{}", s);
        }
        remove_file("Command.cmd").expect("Command file delete failed");
    } else {
        let cmd: String = format!(
            "java {}\nread -rsp $'Press enter to continue...\n'\nexit",
            namef
        );
        let cmd_path: &Path = Path::new("Command.sh"); // CREATE Command.sh
        let display_cmd = cmd_path.display();

        let mut file_command: File = match File::create(&cmd_path) {
            Err(why) => panic!("Could not create {}: {}", display_cmd, why),
            Ok(file_command) => file_command,
        };

        match file_command.write_all(cmd.as_bytes()) {
            Err(why) => panic!("Could not write to {}: {}", display_cmd, why),
            Ok(_) => println!("Successfully wrote to {}", display_cmd),
        }

        let output_run = Command::new("./") // START Command.sh
            .arg("Command.sh")
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));
        if output_run.status.success() {
            let s = String::from_utf8_lossy(&output_run.stdout);

            println!("JAVA succeeded and stdout was:\n{}", s);
        } else {
            let s = String::from_utf8_lossy(&output_run.stderr);

            println!("JAVA failed and stderr was:\n{}", s);
        }
        remove_file("Command.sh").expect("Command file delete failed");
    }

    // }

    // COMAND LINE ARGS

    let javaf: String = format!("{}.java", namef);
    let classf: String = format!("{}.class", namef);

    if cleanup_mode == "-a" {
        remove_file("Manifest.txt").expect("Manifest delete failed");
        remove_file(javaf).expect("Java delete failed");
        remove_file(classf).expect("Class delete failed");
    } else if cleanup_mode == "-m" {
        remove_file("Manifest.txt").expect("Manifest delete failed");
    } else if cleanup_mode == "-c" {
        remove_file(classf).expect("Class delete failed");
    } else if cleanup_mode == "-j" {
        remove_file(javaf).expect("Java delete failed");
    } else if cleanup_mode == "-mc" || cleanup_mode == "-cm" {
        remove_file("Manifest.txt").expect("Manifest delete failed");
        remove_file(classf).expect("Class delete failed");
    } else if cleanup_mode == "-cj" || cleanup_mode == "-jc" {
        remove_file(classf).expect("Class delete failed");
        remove_file(javaf).expect("Java delete failed");
    } else if cleanup_mode == "-jm" || cleanup_mode == "-mj" {
        remove_file(javaf).expect("Java delete failed");
        remove_file("Manifest.txt").expect("Manifest delete failed");
    } else if cleanup_mode == "-n" {
    } else {
        panic!("Invalid cleanup mode!");
    }

    Ok(())
}
