use clap::{App, Arg};
use std::fs::File;
use std::io::{Write, Read, BufReader, BufWriter};

use std::fs::{OpenOptions};
use std::io::{self, Seek, SeekFrom,  Cursor};
use std::path::Path;

struct ArHeader {
    name: String,
    timestamp: String,
    owner_id: String,
    group_id: String,
    mode: String,
    size: usize,
}

fn main() {
    let matches = App::new("Rust ar")
        .version("0.1.0")
        .author("J Dumont")
        .about("Implements an ar command in Rust")
        .arg(Arg::new("archive")
             .takes_value(true)
             .required(true)
             .help("The archive file to operate on"))
        .arg(Arg::new("files")
             .takes_value(true)
             .multiple_values(true)
             .help("Files to process"))
       .arg(Arg::new("positionbefore")
             .short('b')
             .takes_value(false)
             .help("Position new files in the archive before the file named by the posname operand. "))
       .arg(Arg::new("positionafter")
             .short('a')
             .takes_value(false)
             .help("Position new files in the archive after the file named by the posname operand."))
         .arg(Arg::new("verbose")
             .short('v')
             .long("verbose")
             .takes_value(false)
             .help("Give verbose output. When used with the option characters -d, -r, or -x, write a detailed file-by-file description of the archive creation and maintenance activity, as described in the STDOUT section.             When used with -p, write the name of the file in the archive to the standard output before writing the file in the archive itself to the standard output, as described in the STDOUT section.              When used with -t, include a long listing of information about the files in the archive, as described in the STDOUT section."))     
        .arg(Arg::new("create")
             .short('c')
             .long("create")
             .takes_value(false)
             .help("Suppress the diagnostic message that is written to standard error by default when the archive archive is created."))
        .arg(Arg::new("preventnamereplacement")
             .short('C')
             .takes_value(false)
             .help("Prevent extracted files from replacing like-named files in the file system. This option is useful when -T is also used, to prevent truncated filenames from replacing files with the same prefix."))
        .arg(Arg::new("list")
             .short('t')
             .takes_value(false)
             .help("Write a table of contents of archive to the standard output. Only the files specified by the file operands shall be included in the written list. If no file operands are specified, all files in archive shall be included in the order of the archive."))
        .arg(Arg::new("trucate")
             .short('T')
             .takes_value(false)
             .help("Allow filename truncation of extracted files whose archive names are longer than the file system can support. By default, extracting a file with a name that is too long shall be an error; a diagnostic message shall be written and the file shall not be extracted."))
        .arg(Arg::new("extract")
             .short('x')
             .long("extract")
             .takes_value(false)
             .help("Extract the files in the archive named by the file operands from archive. The contents of the archive shall not be changed. If no file operands are given, all files in the archive shall be extracted. The modification time of each file extracted shall be set to the time the file is extracted from the archive."))
        .arg(Arg::new("positionbeforeinarchive")
             .short('i')
             .takes_value(false)
             .help("Position new files in the archive before the file in the archive named by the posname operand (equivalent to -b)"))
        .arg(Arg::new("move")
            .short('m')
            .takes_value(false)
            .help("Move the named files in the archive. The -a, -b, or -i options with the posname operand indicate the position; otherwise, move the names files in the archive to the end of the archive."))
        .arg(Arg::new("delete")
             .short('d')
             .long("delete")
             .takes_value(false)
             .help("Delete one or more files from archive."))
         .arg(Arg::new("append")
             .short('q')
             .long("append")
             .takes_value(false)
             .help("Append the named files to the end of the archive. In this case ar does not check whether the added files are already in the archive. This is useful to bypass the searching otherwise done when creating a large archive piece by piece."))
         .arg(Arg::new("writeoperands")
             .short('p')
             .takes_value(false)
             .help("Write the contents of the files in the archive named by file operands from archive to the standard output. If no file operands are specified, the contents of all files in the archive shall be written in the order of the archive."))
        .arg(Arg::new("update")
             .short('u')
             .long("update")
             .takes_value(false)
             .help("Update older files in the archive. When used with the -r option, files in the archive shall be replaced only if the corresponding file has a modification time that is at least as new as the modification time of the file in the archive."))
         .arg(Arg::new("replace")
             .short('r')
             .takes_value(false)
             .help("Replace or add files to archive. If the archive named by archive does not exist, a new archive shall be created and a diagnostic message shall be written to standard error (unless the -c option is specified). If no files are specified and the archive exists, the results are undefined. Files that replace existing files in the archive shall not change the order of the archive. Files that do not replace existing files in the archive shall be appended to the archive [XSI] [Option Start]  unless a -a, -b, or -i option specifies another position."))
        .arg(Arg::new("forceregeneration")
             .short('s')
             .takes_value(false)
             .help("Force the regeneration of the archive symbol table even if ar is not invoked with an option that modifies the archive contents. This option is useful to restore the archive symbol table after it has been stripped;"))
        .get_matches();
    
    // Parsing logic for each option
    if matches.is_present("list") {
        list_contents(&matches.value_of("archive").unwrap());
    }else if matches.is_present("archive") {
    
    }else if matches.is_present("replace") {
        let archive_path = matches.value_of("archive").unwrap();
        let files: Vec<&Path> = matches.values_of("files")
                                       .unwrap()
                                       .map(Path::new)
                                       .collect();
         println!("Replace or add ");

        replace_or_add_files(Path::new(archive_path), files).unwrap();
    }else if matches.is_present("update") {
    
    }else if matches.is_present("writeoperands") {
    
    }else if matches.is_present("append") {
    
    }else if matches.is_present("delete") {
    
    }else if matches.is_present("move") {
        movef(&matches.value_of("archive").unwrap());
    }else if matches.is_present("positionbeforeinarchive") {
    
    }else if matches.is_present("extract") {
    
    }else if matches.is_present("positionbefore") {
    
    }else if matches.is_present("positionafter") {
    
    } else if matches.is_present("verbose") {
    
    } 

    // Add further logic here for other commands
}

fn list_contents(archive_path: &str) {
    // This function would implement listing archive contents
    println!("Listing contents of archive: {}", archive_path);
}

fn movef(archive_path: &str) {
    // This function would implement listing archive contents
    println!("Move contents of archive: {}", archive_path);
}

fn replace_or_add_files(archive_path: &Path, files: Vec<&Path>) -> io::Result<()> {
    let mut archive = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(archive_path)?;

    // Try to read the existing entries
    let mut existing_files = read_archive_entries(&archive)?;
    
    // Determine which files are new and which need to be replaced
    let mut to_add = Vec::new();
    for file_path in files {
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        if existing_files.contains_key(file_name) {
            // Replace the existing file in the archive (mock function call)
            replace_file_in_archive(&mut archive, file_path, existing_files.get(file_name).unwrap())?;
        } else {
            // Add to list of files to be added
            to_add.push(file_path);
        }
    }

    // Append new files
    append_files_to_archive(&mut archive, &to_add)?;

    Ok(())
}

fn read_archive_entries(archive: &File) -> io::Result<std::collections::HashMap<String, usize>> {
    // Mock-up: Read the archive and return a map of file names to their positions or offsets
    let mut entries = std::collections::HashMap::new();
    // Logic to read and parse the archive file to find existing entries
    Ok(entries)
}

fn replace_file_in_archive(archive: &mut File, new_file: &Path, offset: &usize) -> io::Result<()> {
    // Actual implementation needed here
    Ok(())
}

fn append_files_to_archive(archive: &mut File, files: &[&Path]) -> io::Result<()> {
    for file in files {
        let mut file_handle = File::open(file)?;
        let mut buffer = Vec::new();
        file_handle.read_to_end(&mut buffer)?;

        // Mock-up: Append file content directly to the archive
        archive.write_all(&buffer)?;
    }
    Ok(())
}


fn read_ar_header(cursor: &mut Cursor<Vec<u8>>) -> io::Result<ArHeader> {
    let mut name = vec![0u8; 16];
    let mut timestamp = vec![0u8; 12];
    let mut owner_id = vec![0u8; 6];
    let mut group_id = vec![0u8; 6];
    let mut mode = vec![0u8; 8];
    let mut size = vec![0u8; 10];
    let mut end_chars = vec![0u8; 2];

    cursor.read_exact(&mut name)?;
    cursor.read_exact(&mut timestamp)?;
    cursor.read_exact(&mut owner_id)?;
    cursor.read_exact(&mut group_id)?;
    cursor.read_exact(&mut mode)?;
    cursor.read_exact(&mut size)?;
    cursor.read_exact(&mut end_chars)?;

    Ok(ArHeader {
        name: String::from_utf8(name).unwrap().trim().to_string(),
        timestamp: String::from_utf8(timestamp).unwrap(),
        owner_id: String::from_utf8(owner_id).unwrap(),
        group_id: String::from_utf8(group_id).unwrap(),
        mode: String::from_utf8(mode).unwrap(),
        size: String::from_utf8(size).unwrap().trim().parse().unwrap(),
    })
}

fn move_file_in_archive(archive_path: &Path, filename: &str, position: Option<&str>, before: bool) -> io::Result<()> {
    let mut file = File::open(archive_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut cursor = Cursor::new(buffer);
    read_ar_header(&mut cursor)?;

    // You will continue implementing this logic to move files as specified
    // This includes reading all entries, finding the specified file, and rearranging entries
    // according to the `position` and `before` flags.
    
    Ok(())
}

fn create_archive(output: &str, files: Vec<&str>) {
    let mut archive = BufWriter::new(File::create(output).expect("Could not create file"));

    for file in files {
        let mut content = Vec::new();
        let mut f = BufReader::new(File::open(file).expect("Could not open file"));
        f.read_to_end(&mut content).expect("Could not read file");

        // Write file metadata and content to the archive
        archive.write_all(&content).expect("Could not write to archive");
    }
}