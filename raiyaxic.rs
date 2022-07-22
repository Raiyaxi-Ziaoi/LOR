use std::env;
use std::fs::read_to_string;
use std::fs::remove_file;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

fn main() {
    let arguments: Vec<String> = env::args().collect(); // COMMAND LINE ARGUMENTS

    let filename = &arguments[1];

    // OTHER ARGS {
    if filename == "-v" || filename == "--version" {
        println!(
            "{}",
            read_to_string("STD/.version").expect("Version Missing")
        );
        std::process::abort();
    } else if filename == "-h" || filename == "--help" || filename == "-?" {
        println!("(./)raiyaxic(.exe) [Cleanup Mode] [Filename] [Libraries]");
        println!("Cleanup modes:");
        println!("-a : All");
        println!("-m : Manifest");
        println!("-j : .java");
        println!("-c : .class");
        println!("-n : None");
        std::process::abort();
    }

    // }

    let arg: &String = &arguments[2];
    let to_import: &[String] = &arguments[3..arguments.len()];

    let path = Path::new(filename);

    // FILE {

    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could not read {}: {}", display, why),
        Ok(_) => println!(""),
    }

    // }

    // LIBRARIES {

    let stdlib = read_to_string("STD/output.ryx").expect("OUTPUT Library Missing");
    let mathlib = read_to_string("STD/math.ryx").expect("MATH Library Missing");
    let inlib = read_to_string("STD/input.ryx").expect("INPUT Library Missing");

    // }

    // FILEEXT & FILENAME

    let iter = filename.split(".");
    let splitted: Vec<&str> = iter.collect();
    let endf: Vec<&str> = splitted[0].split("/").collect();
    let namef = endf[endf.len() - 1];

    let javaf = format!("{}.java", namef);
    let classf = format!("{}.class", namef);
    let jarf = format!("{}.jar", namef);

    if splitted[1] != ".lsmx" || splitted[1] != ".ryx" {
        panic!("Wrong filetype! Please ensure that the file ends with \".ryx\" or \".lsmx\"");
    }

    // MANIFEST {

    let mut filem = match File::create("Manifest.txt") {
        Err(why) => panic!("Could not create {}: {}", display, why),
        Ok(filem) => filem,
    };

    let manifest = format!("Main-Class: {}", namef);
    let pathm = Path::new("Manifest.txt");
    let displaym = pathm.display();
    match filem.write_all(manifest.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", displaym, why),
        Ok(_) => println!("Successfully wrote to {}", displaym),
    }

    // }

    // JAVA {

    let pathj = Path::new(&javaf);
    let displayj = pathj.display();

    let mut filew = match File::create(&pathj) {
        Err(why) => panic!("Could not create {}: {}", displayj, why),
        Ok(filew) => filew,
    };

    // }

    let mut imported: String = "".to_owned();
    let mut aditlibs: String = "".to_owned();

    // IMPORTS {

    for import in to_import {
        if import == "None" {
            break;
        } else if import == "STD.MATH" {
            let str = format!("{}", mathlib);
            aditlibs.push_str(&str);
            let math = "import java.math.*;\nimport java.util.Random;";
            imported.push_str(&math);
        } else if import == "STD.IN" {
            let str = format!("{}", inlib);
            aditlibs.push_str(&str);
            let scan = "import java.util.Scanner;";
            imported.push_str(&scan);
        } else if import == "STD.STD" {
            let str = format!("{}", stdlib);
            aditlibs.push_str(&str);
        } else {
            let str = format!("import {};", import);
            imported.push_str(&str);
        }
    }

    // }

    // TRANSPARSING {

    let tw = format!(
        "{imports}\npublic class {name} {{\n{adit}\npublic static void main(String[] args){{{code}}}\n}}",
        name = namef,
        code = s.replace("|> int", "|> private static int").replace("|> String", "|> private static String").replace("|> double", "|> private static double").replace("|> float", "|> private static float").replace("|> float", "|> private static float").replace("|> byte", "|> private static byte").replace("|> long", "|> private static long").replace("|> short", "|> private static short").replace("|> char", "|> private static char").replace("|>", "}").replace(">|", "{").replace("void", "private static void").replace("ret","return"),
        imports = imported,
        adit = aditlibs,
    );
    match filew.write_all(tw.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", display, why),
        Ok(_) => println!("Successfully wrote to {}", display),
    }

    // }

    // COMPILATION {

    let outputc = Command::new("javac") // TO .class
        .arg(javaf)
        .output()
        .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));

    if outputc.status.success() {
        let outputj = Command::new("jar") // TO .jar
            .args(["cvfm", &jarf, "Manifest.txt", &classf])
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));

        if outputj.status.success() {
            println!("JAR succeeded\n");
        } else {
            let s = String::from_utf8_lossy(&outputj.stderr);

            println!("JAR failed and stderr was:\n{}", s);
        }
    } else {
        let s = String::from_utf8_lossy(&outputc.stderr);

        println!("JAVAC failed and stderr was:\n{}", s);
    }

    let os = env::consts::OS;

    if os == "windows" {
        let cmd = format!("@echo off\njava {}\npause>nul\nexit", namef);
        let cmd_path = Path::new("Command.cmd"); // CREATE Command.cmd
        let display_cmd = cmd_path.display();

        let mut file_c = match File::create(&cmd_path) {
            Err(why) => panic!("Could not create {}: {}", display_cmd, why),
            Ok(file_c) => file_c,
        };

        match file_c.write_all(cmd.as_bytes()) {
            Err(why) => panic!("Could not write to {}: {}", display_cmd, why),
            Ok(_) => println!("Successfully wrote to {}", display_cmd),
        }

        let outputr = Command::new("cmd") // START Command.cmd
            .args(["/C", "start Command.cmd"])
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));
        if outputr.status.success() {
            println!("JAVA succeeded");
        } else {
            let s = String::from_utf8_lossy(&outputr.stderr);

            println!("JAVA failed and stderr was:\n{}", s);
        }
        remove_file("Command.cmd").expect("Command file delete failed");
    } else {
        let cmd = format!(
            "java {}\nread -rsp $'Press enter to continue...\n'\nexit",
            namef
        );
        let cmd_path = Path::new("Command.sh"); // CREATE Command.sh
        let display_cmd = cmd_path.display();

        let mut file_c = match File::create(&cmd_path) {
            Err(why) => panic!("Could not create {}: {}", display_cmd, why),
            Ok(file_c) => file_c,
        };

        match file_c.write_all(cmd.as_bytes()) {
            Err(why) => panic!("Could not write to {}: {}", display_cmd, why),
            Ok(_) => println!("Successfully wrote to {}", display_cmd),
        }

        let outputr = Command::new("./") // START Command.sh
            .arg("Command.sh")
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));
        if outputr.status.success() {
            let s = String::from_utf8_lossy(&outputr.stdout);

            println!("JAVA succeeded and stdout was:\n{}", s);
        } else {
            let s = String::from_utf8_lossy(&outputr.stderr);

            println!("JAVA failed and stderr was:\n{}", s);
        }
        remove_file("Command.sh").expect("Command file delete failed");
    }

    // }

    // COMAND LINE ARGS

    let javaf = format!("{}.java", namef);
    let classf = format!("{}.class", namef);

    if arg == "-a" {
        remove_file("Manifest.txt").expect("Manifest delete failed");
        remove_file(javaf).expect("Java delete failed");
        remove_file(classf).expect("Class delete failed");
    } else if arg == "-m" {
        remove_file("Manifest.txt").expect("Manifest delete failed");
    } else if arg == "-c" {
        remove_file(classf).expect("Class delete failed");
    } else if arg == "-j" {
        remove_file(javaf).expect("Java delete failed");
    } else if arg == "-mc" || arg == "-cm" {
        remove_file("Manifest.txt").expect("Manifest delete failed");
        remove_file(classf).expect("Class delete failed");
    } else if arg == "-cj" || arg == "-jc" {
        remove_file(classf).expect("Class delete failed");
        remove_file(javaf).expect("Java delete failed");
    } else if arg == "-jm" || arg == "-mj" {
        remove_file(javaf).expect("Java delete failed");
        remove_file("Manifest.txt").expect("Manifest delete failed");
    } else if arg == "-n" {
    }
}
