use std::env;
use std::fs::read_to_string;
use std::fs::remove_file;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

fn main() {
    let arguments: Vec<String> = env::args().collect(); // Command Line Arguments

    let filename = &arguments[1];
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

    let stdlib = read_to_string("STD/.std").expect("STD Library Missing");
    let mathlib = read_to_string("STD/.math").expect("MATH Library Missing");
    let inlib = read_to_string("STD/.input").expect("INPUT Library Missing");

    // }

    // file.ext => file

    let iter = filename.split(".");
    let splitted: Vec<&str> = iter.collect();
    let namef = splitted[0];

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

    let j = format!("{}.java", namef);
    let pathj = Path::new(&j);
    let displayj = pathj.display();

    let mut filew = match File::create(&pathj) {
        Err(why) => panic!("Could not create {}: {}", displayj, why),
        Ok(filew) => filew,
    };

    let mut imported: String = "".to_owned();
    let mut aditlibs: String = "".to_owned();

    for import in to_import {
        if import == "None" {
            break;
        } else if import == "STD.MATH" {
            let str = format!("{}", mathlib);
            aditlibs.push_str(&str);
        } else if import == "STD.IN" {
            let str = format!("{}", inlib);
            aditlibs.push_str(&str);
        } else if import == "STD.STD" {
            let str = format!("{}", stdlib);
            aditlibs.push_str(&str);
        } else {
            let str = format!("import {};", import);
            imported.push_str(&str);
        }
    }

    let tw = format!(
        "{imports}\npublic class {name} {{\n{adit}\npublic static void main(String[] args){{{code}}}\n}}",
        name = namef,
        code = s,
        imports = imported,
        adit = aditlibs,
    );
    match filew.write_all(tw.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", display, why),
        Ok(_) => println!("Successfully wrote to {}", display),
    }

    // }

    // COMPILATION {

    let outputc = Command::new("javac") // Compile to .class
        .arg(format!("{}.java", namef))
        .output()
        .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));

    let jar = format!("{f}.jar", f = namef);
    let class = format!("{f}.class", f = namef);

    if outputc.status.success() {
        let outputj = Command::new("jar") // Pack into .jar
            .args(["cvfm", &jar, "Manifest.txt", &class])
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

    let namej = format!("{}", namef);

    let cmd = format!("@echo off\njava {}\npause>nul\nexit", namej);

    let cmd_path = Path::new("Command.cmd"); // Command.cmd
    let display_cmd = cmd_path.display();

    let mut file_c = match File::create(&cmd_path) {
        Err(why) => panic!("Could not create {}: {}", display_cmd, why),
        Ok(file_c) => file_c,
    };

    match file_c.write_all(cmd.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", display_cmd, why),
        Ok(_) => println!("Successfully wrote to {}", display_cmd),
    }

    let outputr = Command::new("cmd") // Start Command.cmd
        .args(["/C", "start Command.cmd"])
        .output()
        .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));
    if outputr.status.success() {
        let s = String::from_utf8_lossy(&outputr.stdout);

        println!("JAVA succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&outputr.stderr);

        println!("JAVA failed and stderr was:\n{}", s);
    }
    remove_file("Command.cmd").expect("Command file delete failed");

    // }

    // COMAND LINE ARGS

    if arg == "-a" {
        remove_file("Manifest.txt").expect("Manifest delete failed");
        remove_file(format!("{}.java", namef)).expect("Java delete failed");
        remove_file(format!("{}.class", namef)).expect("Class delete failed");
    } else if arg == "-m" {
        remove_file("Manifest.txt").expect("Manifest delete failed");
    } else if arg == "-c" {
        remove_file(format!("{}.class", namef)).expect("Class delete failed");
    } else if arg == "-j" {
        remove_file(format!("{}.java", namef)).expect("Java delete failed");
    } else if arg == "-mc" {
        remove_file("Manifest.txt").expect("Manifest delete failed");
        remove_file(format!("{}.class", namef)).expect("Class delete failed");
    } else if arg == "-cj" {
        remove_file(format!("{}.class", namef)).expect("Class delete failed");
        remove_file(format!("{}.java", namef)).expect("Java delete failed");
    } else if arg == "-jm" {
        remove_file(format!("{}.java", namef)).expect("Java delete failed");
        remove_file("Manifest.txt").expect("Manifest delete failed");
    } else if arg == "-cm" {
        remove_file("Manifest.txt").expect("Manifest delete failed");
        remove_file(format!("{}.class", namef)).expect("Class delete failed");
    } else if arg == "-jc" {
        remove_file(format!("{}.class", namef)).expect("Class delete failed");
        remove_file(format!("{}.java", namef)).expect("Java delete failed");
    } else if arg == "-mj" {
        remove_file(format!("{}.java", namef)).expect("Java delete failed");
        remove_file("Manifest.txt").expect("Manifest delete failed");
    } else if arg == "-n" {
    }
}
