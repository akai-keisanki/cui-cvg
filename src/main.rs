mod group;
use group::IGroupManager;

fn main () -> ()
{
    println!("Hello, world!");

    let mut group_manager = group::GroupManager::new();
    // example: group_manager.add(&String::from("Hello, World!"));
}
