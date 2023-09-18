use std::{
    fs::File,
    io::{BufReader, BufRead, Write},
};
use anyhow::anyhow;
use colored::*;

pub struct Xcode {

}
impl Xcode {
    pub fn ios_bump_version(base_path: &str, version_part: &str) -> Result<(), anyhow::Error> {
        // Open the project.pbxproj file
        let path = format!("{}/App/App.xcodeproj/project.pbxproj", base_path);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
    
        // Read the file line by line and modify MARKETING_VERSION where necessary
        let mut lines = reader.lines();
        let mut output = String::new();
        while let Some(line) = lines.next() {
            let line = line?;
            if line.contains("MARKETING_VERSION") {
                let line = line.trim_end_matches(';');
                let (prefix, suffix) = line.split_once("=").unwrap();
                let version = suffix.trim().trim_matches('"').to_string();
                let version_parts = version.split('.').collect::<Vec<&str>>();
                let major = version_parts.get(0).ok_or(anyhow!("Unable to parse major version"))?.to_string();
                let minor = version_parts.get(1).ok_or(anyhow!("Unable to parse minor version"))?.to_string();
                let patch = version_parts.get(2).ok_or(anyhow!("Unable to parse patch version"))?.to_string();
    
                let (new_major, new_minor, new_patch) = match version_part {
                    "major" => ((major.parse::<u32>().map_err(|_| anyhow!("Unable to parse major version"))? + 1).to_string(), "0".to_string(), "0".to_string()),
                    "minor" => (major, (minor.parse::<u32>().map_err(|_| anyhow!("Unable to parse minor version"))? + 1).to_string(), "0".to_string()),
                    "patch" => (major, minor, (patch.parse::<u32>().map_err(|_| anyhow!("Unable to parse patch version"))? + 1).to_string()),
                    _ => return Err(anyhow::anyhow!("Invalid version part specified")),
                };
    
                let new_version = format!("{}.{}.{}", new_major, new_minor, new_patch);
                output.push_str(&format!("{} = \"{}\";\n", prefix, new_version));
            } else {
                output.push_str(&format!("{}\n", line));
            }
        }
    
        // Write the modified file back to disk
        println!("updating {} version", version_part.blue());
        let mut file = File::create(&path)?;
        file.write_all(output.as_bytes())?;
    
        Ok(())
    }
    
    pub fn ios_get_version(base_path: &str) -> Result<String, anyhow::Error> {
        let path = format!("{}/App/App.xcodeproj/project.pbxproj", base_path);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
    
        for line in reader.lines() {
            let line = line?;
            if line.contains("MARKETING_VERSION") {
                let line = line.trim_end_matches(';');
                let (_, version) = line.split_once("=").unwrap();
                let version = version.trim().trim_matches('"').to_string();
                return Ok(version);
            }
        }
    
        Err(anyhow!("Failed to find version in project.pbxproj"))
    }
    
}
