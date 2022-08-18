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
            read_to_string("STD/CORE/.version").expect("Version Missing")
        );
        std::process::abort();
    } else if cleanup_mode == "-h" || cleanup_mode == "--help" || cleanup_mode == "-?" {
        println!("(./)lnrc(.exe) [Cleanup mode] [File path] [Config file path]\nCleanup modes:\n-a : All\n-m : Manifest\n-j : .java\n-c : .class\n-n : None");
        std::process::abort();
    }

    let source_file: &String = &command_line_arguments[2];

    if cleanup_mode == "-innit" || cleanup_mode == "-init" || cleanup_mode == "-i" {
        fs::create_dir(source_file)?;

        let formatted_config_path = format!("{}/config.vn", source_file);

        // CONFIG {
        let path_config: &Path = Path::new(&formatted_config_path);
        let display_config = path_config.display();

        let _file_config: File = match File::create(format!("{}/config.vn", source_file)) {
            Err(why) => panic!("Could not create {}: {}", display_config, why),
            Ok(file_config) => file_config,
        };

        // }

        // SOURCE {

        let formatted_source_path = format!("{}/main.lsmx", source_file);

        let path_source: &Path = Path::new(&formatted_source_path);
        let display_source = path_source.display();

        let _file_source: File = match File::create(format!("{}/main.lsmx", source_file)) {
            Err(why) => panic!("Could not create {}: {}", display_source, why),
            Ok(file_source) => file_source,
        };

        std::process::abort();
        // }
    } else if cleanup_mode == "-innit_default"
        || cleanup_mode == "-init-default"
        || cleanup_mode == "-id"
    {
        fs::create_dir(source_file)?;

        let formatted_config_path = format!("{}/config.vn", source_file);

        // CONFIG {
        let path_config: &Path = Path::new(&formatted_config_path);
        let display_config = path_config.display();

        let mut file_config: File = match File::create(format!("{}/config.vn", source_file)) {
            Err(why) => panic!("Could not create {}: {}", display_config, why),
            Ok(_file_config) => _file_config,
        };

        let config: String = "CORE.OUT".to_string();

        match file_config.write_all(config.as_bytes()) {
            Err(why) => panic!("Could not write to {}: {}", display_config, why),
            Ok(_) => println!("Successfully wrote to {}", display_config),
        }

        // }

        // SOURCE {

        let formatted_source_path = format!("{}/main.lsmx", source_file);

        let path_source: &Path = Path::new(&formatted_source_path);
        let display_source = path_source.display();

        let mut file_source: File = match File::create(format!("{}/main.lsmx", source_file)) {
            Err(why) => panic!("Could not create {}: {}", display_source, why),
            Ok(_file_source) => _file_source,
        };

        let source: String = "fn main() {\n    println(\"Hello World\");\n}".to_string();

        match file_source.write_all(source.as_bytes()) {
            Err(why) => panic!("Could not write to {}: {}", display_source, why),
            Ok(_) => println!("Successfully wrote to {}", display_source),
        }

        std::process::abort();
        // }
    }
    // }

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

    let mut file_contents: String = String::new();
    match file_source.read_to_string(&mut file_contents) {
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

    // IMPORTS {

    let mut imported: String = "".to_owned();
    let mut aditlibs: String = "".to_owned();
    let mut libs: String = "".to_owned();
    let mut ext: String = "".to_owned();

    let mut uselmd: bool = false;
    let mut usedsk: bool = false;
    let mut usescan: bool = false;
    let mut useequ: bool = false;

    if !skip {
        for import in to_import {
            if import.len() > 6 {
                if import == "None" {
                    break;
                } else if import == "CORE > IN > STR" {
                    let inlib: String = read_to_string("STD/CORE/in_str.ryx")
                        .expect("CORE > INPUT > STRING Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "CORE > IN > INT" {
                    let inlib: String = read_to_string("CORE/in_int.ryx")
                        .expect("CORE > INPUT > INTEGER Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "CORE > IN > FLO" {
                    let inlib: String = read_to_string("STD/CORE/in_flo.ryx")
                        .expect("CORE > INPUT > FLOAT Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "CORE > IN > BYT" {
                    let inlib: String = read_to_string("STD/CORE/in_byt.ryx")
                        .expect("CORE > INPUT > BYTE Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "CORE > IN > SHO" {
                    let inlib: String = read_to_string("STD/CORE/in_sho.ryx")
                        .expect("CORE > INPUT > SHORT Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "CORE > IN > BOO" {
                    let inlib: String = read_to_string("STD/CORE/in_boo.ryx")
                        .expect("CORE > INPUT > BOOL Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "CORE > IN > CHAR" {
                    let inlib: String = read_to_string("STD/CORE/in_cha.ryx")
                        .expect("CORE > INPUT > CHAR Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "CORE > IN > DOU" {
                    let inlib: String = read_to_string("STD/CORE/in_dou.ryx")
                        .expect("CORE > INPUT > DOUBLE Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "CORE > IN > LON" {
                    let inlib: String = read_to_string("STD/CORE/in_lon.ryx")
                        .expect("CORE > INPUT > LONG Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "CORE > CASE" {
                    let caselib: String =
                        read_to_string("STD/CORE/case.ryx").expect("CORE > CASE Library Missing");
                    aditlibs.push_str(&caselib);
                } else if import == "CORE > SLEEP" {
                    let waitlib: String =
                        read_to_string("STD/CORE/sleep.ryx").expect("CORE > SLEEP Library Missing");
                    aditlibs.push_str(&waitlib);
                } else if import == "CORE > DATETIME" {
                    let datetimelib: String = read_to_string("STD/CORE/datetime.ryx")
                        .expect("CORE > DATETIME Library Missing");
                    aditlibs.push_str(&datetimelib);
                    let str: String = "\nimport java.text.DateFormat;\nimport java.text.ParseException;\nimport java.text.SimpleDateFormat;\nimport java.util.Date;".to_string();
                    imported.push_str(&str);
                } else if import == "CORE > OUT" {
                    let outlib: String =
                        read_to_string("STD/CORE/out.ryx").expect("CORE > OUTPUT Library Missing");
                    aditlibs.push_str(&outlib);
                } else if import == "CORE > REGEX" {
                    let rgxlib: String =
                        read_to_string("STD/CORE/regex.ryx").expect("CORE > REGEX Library Missing");
                    aditlibs.push_str(&rgxlib);
                    let str: String =
                        "\nimport java.util.regex.Matcher;\nimport java.util.regex.Pattern;"
                            .to_string();
                    imported.push_str(&str);
                } else if import == "CORE > SHELL" {
                    let shelllib: String =
                        read_to_string("STD/shell.ryx").expect("CORE > SHELL Library Missing");
                    aditlibs.push_str(&shelllib);
                    let str: String = "\nimport java.lang.Process;\nimport java.io.InputStream;\nimport java.util.Scanner;\nimport java.text.SimpleDateFormat;\nimport java.util.Date;".to_string();
                    imported.push_str(&str);
                } else if import == "CORE > FILEIO" {
                    let fileiolib: String = read_to_string("STD/CORE/fileio.ryx")
                        .expect("CORE > FILEIO Library Missing");
                    aditlibs.push_str(&fileiolib);
                    let str: String = "\nimport java.io.File;\nimport java.io.FileReader;\nimport java.io.BufferedReader;\nimport java.io.IOException;\nimport java.nio.file.Files;\nimport java.nio.file.Path;\nimport java.nio.file.StandardOpenOption;\nimport java.nio.file.Paths;".to_string();
                    imported.push_str(&str);
                } else if import == "CORE > HASH" {
                    let hashlib: String =
                        read_to_string("STD/CORE/hash.ryx").expect("CORE > HASH Library Missing");
                    aditlibs.push_str(&hashlib);
                } else if import == "CORE > FORMAT" {
                    let str: String = "\nimport java.text.MessageFormat;".to_string();
                    imported.push_str(&str);
                } else if import == "FUNC > RELAY" {
                    let relaylib: String =
                        read_to_string("STD/FUNC/relay.ryx").expect("FUNC > RELAY Library Missing");
                    aditlibs.push_str(&relaylib);
                } else if import == "FUNC > LAMBDA" {
                    uselmd = true;
                    let lmdlib: String = read_to_string("STD/FUNC/lambda.ryx")
                        .expect("FUNC > LAMBDA Library Missing");
                    libs.push_str(&lmdlib);
                } else if import == "STAT > DESK" {
                    usedsk = true;
                    let desklib: String =
                        read_to_string("STD/STAT/desk.ryx").expect("STAT > DESK Library Missing");
                    libs.push_str(&desklib);
                    let str: String = "\nimport java.util.ArrayList;\nimport java.util.Arrays;\nimport java.util.Collections;\nimport java.util.List;".to_string();
                    imported.push_str(&str);
                } else if import == "MATH > EQU" {
                    let mathlib: String =
                        read_to_string("STD/MATH/equations.ryx").expect("MATH.EQU Library Missing");
                    libs.push_str(&mathlib);
                    let str: String =
                        "\nimport java.util.ArrayList;\nimport java.util.HashMap;".to_string();
                    imported.push_str(&str);
                    useequ = true;
                } else if import == "MATH > SQRT" {
                    let sqrtlib: String =
                        read_to_string("STD/MATH/sqrts.ryx").expect("MATH > SQRT Library Missing");
                    aditlibs.push_str(&sqrtlib);
                } else if import == "MATH > RANDOM" {
                    let randomlib: String = read_to_string("STD/MATH/random.ryx")
                        .expect("MATH > RANDOM Library Missing");
                    aditlibs.push_str(&randomlib);
                    let str: String = "\nimport java.util.Random;".to_string();
                    imported.push_str(&str);
                } else {
                    let splitted_import: Vec<&str> = import.split(".").collect();
                    if splitted_import[0] == "java" || splitted_import[0] == "javax" {
                        let str: String = format!("import {};", import);
                        imported.push_str(&str);
                    } else if splitted_import[1] == "ryx" {
                        let error: String = format!("Unable to read import: {}", import);
                        let importfile: String = read_to_string(import).expect(&error);
                        aditlibs.push_str(&importfile);
                    } else if splitted_import[1] == "lsmx" {
                        let error: String = format!("Unable to read import: {}", import);
                        let importfile: String = read_to_string(import).expect(&error);
                        ext.push_str(&importfile);
                    } else {
                        panic!(
                            "Imported file must end in \".ryx\", \".lsmx\", be a Java import or be part of the ALNOOR library."
                        );
                    }
                }
            }
        }
        if usescan {
            let str: String = "\nimport java.util.Scanner;".to_string();
            imported.push_str(&str);
        }
    }

    // }

    // TRANSPARSING {

    let self_dund = format!("var _self = new {}();", namef);

    let to_write: String = format!(
        "{imports}\n{libraries}\npublic class {name} {{\n{adit}\n{code}\n\n\n}}",
        name = namef,
        code = file_contents
            .replace("fn main()", "public static void main(String[] args)")
            .replace("_fn ", "private static ")
            .replace("fn ", "public static ")
            .replace("ret ", "return ")
            .replace("bool ", "boolean ")
            .replace("_match ", "switch ")
            .replace("elif ", "else if ")
            .replace("const ", "final ")
            .replace("#def ", "private static ")
            .replace("new_self!", &self_dund)
            .replace("exit!", "System.exit(0);")
            .replace("abort!", "System.exit(1);")
            .replace("print!", "System.out.print")
            .replace("println!", "System.out.println")
            .replace("format!", "MessageFormat.format")
            .replace("args!", "args")
            .replace("exit(", "System.exit(")
            .replace("exit (", "System.exit (")
            .replace("_class", "} class")
            .replace("_construct", "public")
            .replace("_catch;", "catch(Exception e) { e.printStackTrace(); }")
            .replace("=>", "{")
            .replace("$.", "this.")
            .replace("l>", "->"),
        imports = imported,
        adit = aditlibs,
        libraries = libs,
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

    if uselmd {
        remove_file("Void.class").expect("VOID delete failed");
        remove_file("Function.class").expect("FUNCTION delete failed");
    }

    if usedsk {
        remove_file("Desk.class").expect("DESK delete failed");
    }

    if useequ {
        remove_file("Equ.class").expect("Equ delete failed");
        remove_file("Equ$MathFunction.class").expect("EquMathFunction delete failed");
        remove_file("Equ$MathParsingExeption.class").expect("EquMathParsingExeption delete failed");
    }

    Ok(())
}
