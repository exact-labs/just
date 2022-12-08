use crate::project;

pub struct DependencyManager;
impl DependencyManager {
    pub fn install() {
        println!("install")
    }
    pub fn add(name: &String) {
        println!("{name}")
    }
    pub fn remove(name: &String) {
        println!("{name}")
    }
    pub fn clean() {
        println!("clean")
    }
}

pub fn publish() {
    println!("{}", project::package::read().info.name);
    println!("{}", project::package::read().info.version);
    println!("{}", project::package::read().info.author);
    println!("{}", project::package::read().registry.public);
    println!("{}", project::package::read().info.license);
    println!("{}", project::package::read().info.url);
    println!("{}", project::package::read().info.repository);
    println!("{}", project::package::read().info.description);
    println!("{:#?}", project::package::read().dependencies);
}
