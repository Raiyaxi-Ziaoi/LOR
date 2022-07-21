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
    let args: &[String] = &arguments[2..arguments.len()];

    let path = Path::new(filename);

    // FILE {

    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!(""),
    }

    // }

    // LIBRARY {

    let strlib = read_to_string(".library").expect("Library Missing");

    // }

    // file.ext => file

    let iter = filename.split(".");
    let splitted: Vec<&str> = iter.collect();
    let namef = splitted[0];

    // MANIFEST {

    let mut filem = match File::create("Manifest.txt") {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(filem) => filem,
    };

    let manifest = format!("Main-Class: {}", namef);
    let pathm = Path::new("Manifest.txt");
    let displaym = pathm.display();
    match filem.write_all(manifest.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", displaym, why),
        Ok(_) => println!("successfully wrote to {}", displaym),
    }

    // }

    // JAVA {

    let j = format!("{}.java", namef);
    let pathj = Path::new(&j);
    let display = pathj.display();

    let mut filew = match File::create(&pathj) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(filew) => filew,
    };

    let tw = format!(
        "public class {name} {{{lib}public static void main(String[] args) {{{code}}}}}",
        name = namef,
        code = s,
        lib = strlib
    );
    match filew.write_all(tw.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    // }

    // COMPILATION {

    let outputc = Command::new("javac")
        .arg(format!("{}.java", namef))
        .output()
        .unwrap_or_else(|e| panic!(" failed to execute process: {}", e));

    let jar = format!("{f}.jar", f = namef);
    let class = format!("{f}.class", f = namef);

    if outputc.status.success() {
        let outputj = Command::new("jar")
            .args(["cvfm", &jar, "Manifest.txt", &class])
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        if outputj.status.success() {
            print!("jar succeeded\n");
        } else {
            let s = String::from_utf8_lossy(&outputj.stderr);

            print!("jar failed and stderr was:\n{}", s);
        }
    } else {
        let s = String::from_utf8_lossy(&outputc.stderr);

        print!("javac failed and stderr was:\n{}", s);
    }

    let namej = format!("{}", namef);

    let outputr = Command::new("java")
        .arg(&namej)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    if outputr.status.success() {
        let s = String::from_utf8_lossy(&outputr.stdout);

        print!("java succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&outputr.stderr);

        print!("java failed and stderr was:\n{}", s);
    }

    // }

    // COMAND LINE ARGS

    for arg in args.iter() {
        if arg == "-a" {
            remove_file("Manifest.txt").expect("Manifest delete failed");
            remove_file(format!("{}.java", namef)).expect("Java delete failed");
            remove_file(format!("{}.class", namef)).expect("Class delete failed");
            break;
        } else if arg == "-m" {
            remove_file("Manifest.txt").expect("Manifest delete failed");
        } else if arg == "-c" {
            remove_file(format!("{}.class", namef)).expect("Class delete failed");
        } else if arg == "-j" {
            remove_file(format!("{}.java", namef)).expect("Java delete failed");
        }
    }
}
