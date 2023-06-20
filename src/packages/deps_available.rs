use rpkg::debversion;
use crate::Packages;
use crate::packages::Dependency;

impl Packages {
    /// Gets the dependencies of package_name, and prints out whether they are satisfied (and by which library/version) or not.
    pub fn deps_available(&self, package_name: &str) {
        if !self.package_exists(package_name) {
            println!("no such package {}", package_name);
            return;
        }
        println!("Package {}:", package_name);

        let package_num = self.get_package_num(package_name);
        let package_deps = self.dependencies.get(package_num).unwrap();

        for package_dep in package_deps {
            let package_ver = package_dep.first().unwrap();
            println!("{}", self.rel2str(package_ver));
            if let Some(debver) = self.installed_debvers.get(&package_ver.package_num) {
                println!("+ {} satisfied by installed version {}", self.get_package_name(package_ver.package_num), debver);
                // some sort of for loop...
            } else {
                println!("-> not satisfied");
            }
        }
    }

    /// Returns Some(package) which satisfies dependency dd, or None if not satisfied.
    pub fn dep_is_satisfied(&self, dd:&Dependency) -> Option<&str> {
        // presumably you should loop on dd
        return None;
    }

    /// Returns a Vec of packages which would satisfy dependency dd but for the version.
    /// Used by the how-to-install command, which calls compute_how_to_install().
    pub fn dep_satisfied_by_wrong_version(&self, dd:&Dependency) -> Vec<&str> {
        assert! (self.dep_is_satisfied(dd).is_none());
        let mut result = vec![];
        // another loop on dd
        return result;
    }
}

