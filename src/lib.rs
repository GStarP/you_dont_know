use std::{error::Error, fs, io};
mod encrypt;

pub fn ask_to_crypt_one_file() -> Result<(), Box<dyn Error>> {
    // input file path
    println!("Input complete file path (press ctrl+c to exit):");
    let mut filepath = String::new();
    io::stdin().read_line(&mut filepath)?;
    // read_line will bring an extra `\n`, if you dont trim it, filepath will be incorrect
    // @ref https://stackoverflow.com/questions/74679915/fsread-to-stringfile-path-error-123-invalidfilename-works-when-hardcoded-bu
    let filepath = filepath.trim();

    // extract file info
    let ss = &filepath.split('\\').collect::<Vec<&str>>();
    let filename = *ss.last().unwrap();
    // filepath ends with '.ydn' => decrypt
    let is_encrypt = &filepath[(&filepath.len() - 4)..] != ".ydn";
    let op = if is_encrypt { "Encrypt" } else { "Decrypt" };

    // input password
    println!("\nInput password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();

    // do crypt
    println!("\n{}ing...\n", op);
    let raw_bytes = fs::read(&filepath).unwrap();
    let processed_bytes = if is_encrypt {
        encrypt::encrypt_bytes(&raw_bytes, &password)
    } else {
        encrypt::decrypt_bytes(&raw_bytes, &password)
    };

    // output file
    let output_filename = if is_encrypt {
        String::from(filename) + ".ydn"
    } else {
        String::from(&filename[..filename.len() - 4])
    };
    fs::write(output_filename, &processed_bytes)?;
    println!("{} success!\n", op);

    Ok(())
}
