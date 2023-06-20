use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::Packages;
use crate::packages::{RelVersionedPackageNum, Dependency};

use rpkg::debversion;

const KEYVAL_REGEX : &str = r"(?P<key>(\w|-)+): (?P<value>.+)";
const PKGNAME_AND_VERSION_REGEX : &str = r"(?P<pkg>(\w|\.|\+|-)+)( \((?P<op>(<|=|>)(<|=|>)?) (?P<ver>.*)\))?";

fn kv_captures(line: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref kv_regexp: Regex = Regex::new(KEYVAL_REGEX).unwrap();
    }

    kv_regexp.captures(line)
}

fn version_captures(line: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref version_regexp: Regex = Regex::new(PKGNAME_AND_VERSION_REGEX).unwrap();
    }

    version_regexp.captures(line)
}

impl Packages {
    /// Loads packages and version numbers from a file, calling get_package_num_inserting on the package name
    /// and inserting the appropriate value into the installed_debvers map with the parsed version number.
    pub fn parse_installed(&mut self, filename: &str) {        
        let mut current_package_num = 0;
        let lines = read_lines(filename);
        match lines {
            Ok(lines) => {
                for line in lines {
                    match kv_captures(line.unwrap().as_str()) {
                        None => (),
                        Some(caps) => {
                            let (key, value) = (caps.name("key").unwrap().as_str(),
                                                caps.name("value").unwrap().as_str());
                            match key {
                                "Package" => {
                                    current_package_num = self.get_package_num_inserting(&value);
                                },
                                "Version" => {
                                    let debver = value.trim().parse::<debversion::DebianVersionNum>().unwrap();
                                    self.installed_debvers.insert(current_package_num, debver);
                                },
                                &_ => {},
                            }
                        }
                    }
                }
            },
            Err(err) => {
                println!("Failed to read {filename}: {err}");
                return;
            },
        }
        println!("Packages installed: {}", self.installed_debvers.keys().len());
    }

    /// Loads packages, version numbers, dependencies, and md5sums from a file, calling get_package_num_inserting on the package name
    /// and inserting the appropriate values into the dependencies, md5sum, and available_debvers maps.
    pub fn parse_packages(&mut self, filename: &str) {
        let mut current_package_num = 0;
        let lines = read_lines(filename);
        match lines {
            Ok(lines) => {
                for line in lines {
                    match kv_captures(line.unwrap().as_str()) {
                        None => (),
                        Some(caps) => {
                            let (key, value) = (caps.name("key").unwrap().as_str(),
                                                caps.name("value").unwrap().as_str());
                            match key {
                                "Package" => {
                                    current_package_num = self.get_package_num_inserting(&value);
                                },
                                "Version" => {
                                    let debver = value.trim().parse::<debversion::DebianVersionNum>().unwrap();
                                    self.available_debvers.insert(current_package_num, debver);
                                },
                                "Depends" => {
                                    self.parse_depends(current_package_num, value);
                                }
                                &_ => {},
                            }
                        }
                    }
                }
            },
            Err(err) => {
                println!("Failed to read {filename}: {err}");
                return;
            },
        }
        println!("Packages available: {}", self.available_debvers.keys().len());
    }

    pub fn parse_depends(&mut self, current_package_num: i32, value: &str) {
        let rel_package_strs = value.split(",");

        for rel_package_str in rel_package_strs {
            let caps = version_captures(rel_package_str).unwrap();
            let package_num = self.get_package_num_inserting(caps.name("pkg").unwrap().as_str());
            
            let op = caps.name("op"); //.unwrap().as_str()           

            let mut package_deps = self.dependencies.get_mut(&current_package_num);
            
            if package_deps.is_none() {
                self.dependencies.insert(current_package_num, Vec::new());
                package_deps = self.dependencies.get_mut(&current_package_num);
                // So we can assume there is always a Vec<RelVersionedPackageNum>
                //package_deps.unwrap().push(Dependency::new());
            }

            let rel_version = match op {
                Some(op) => {
                    let ver = caps.name("ver"); //.unwrap().as_str()
                    Some((op.as_str().parse::<debversion::VersionRelation>().unwrap(), 
                        ver.unwrap().as_str().to_string()))
                },
                None => None 
            };

            package_deps.unwrap().push(vec![
                RelVersionedPackageNum{
                    package_num, 
                    rel_version : rel_version
                }
            ]);
/* 
            // Check last package in Vec<Dependency>
            let mut last_dep_package = package_deps.unwrap().last();

            if last_dep_package.is_none() {
            } else {
                // Check last rel_version
                if let Some(last_rel_version) = last_dep_package.unwrap().last() {
                    if last_rel_version.package_num == package_num {
                        last_dep_package.unwrap().push(
                            RelVersionedPackageNum{
                                package_num, 
                                rel_version : Some((op.parse::<debversion::VersionRelation>().unwrap(), ver.to_string()))
                            }    
                        )
                    } else {
                        // Different package so start a new list
                        // Package does not match so start a new Vec
                        package_deps.unwrap().push(Vec::new());
                        package_deps.unwrap().last().unwrap().push(
                            RelVersionedPackageNum{
                                package_num, 
                                rel_version : Some((op.parse::<debversion::VersionRelation>().unwrap(), ver.to_string()))
                            }
                        );
                    }
                }
            } else {
                package_deps.unwrap().push
            }

            package_deps.unwrap().push(
            );
*/
        }
    }
}


// standard template code downloaded from the Internet somewhere
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
