mod group;
use group::{Group, IGroup, GroupManager, IGroupManager};

fn input_commands () -> Vec::<String>
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed reading input!");
    input.split_whitespace().map(|x| x.to_string()).collect()
}

fn expect_next_command (commands : &Vec::<String>, command_index : &mut u16) -> Result::<(), Box::<dyn std::error::Error>>
{
    *command_index += 1;

    if commands.len() <= *command_index as usize
    {
        return Err("Expected command".into())
    }

    Ok(())
}

fn main () -> ()
{
    let mut group_manager = group::GroupManager::new();
    let mut commands = Vec::<String>::new();
    let mut command_index : u16 = 0;

    println!("--- Character User Interface Conlang Vocabulary Generator ---");

    loop
    {
        commands = input_commands();
        command_index = 0;

        if commands.len() == 0
        {
            continue;
        }

        match commands[command_index as usize].as_str()
        {
        "exit" => break,
        "group" =>
        {
            if let Err(err) = expect_next_command(&commands, &mut command_index)
            { println!("{}", err.to_string()); }
            match commands[command_index as usize].as_str()
            {
            "add" =>
            {
                if let Err(err) = expect_next_command(&commands, &mut command_index)
                { println!("{}", err.to_string()); }
                group_manager.add(&commands[command_index as usize]);
            },
            "list" =>
            {
                for group in group_manager.list()
                {
                    print!("{} : ", group.get_label());
                    for ch in group.get_chars()
                    {
                        print!("{ch} ");
                    }
                    println!("");
                }
            },
            "label" =>
            {
                if let Err(err) = expect_next_command(&commands, &mut command_index)
                { println!("{}", err.to_string()); }

                if let Some(group) = group_manager.group(&commands[command_index as usize])
                {

                    if let Err(err) = expect_next_command(&commands, &mut command_index)
                    { println!("{}", err.to_string()); }

                    match commands[command_index as usize].as_str()
                    {
                    "add" =>
                    {
                        for ch in &commands[(command_index+1) as usize..commands.len()]
                        { group.add(ch); }
                    },
                    "clear" =>
                    {
                        group.clear();
                    },
                    "list" =>
                    {
                        for ch in group.get_chars()
                        {
                            print!("{ch} ");
                        }
                        println!("");
                    }
                    command => println!("Group command {command} not found!")
                    }
                }
                else
                {
                    println!("Group not found!");
                }
            }
            "clear" =>
            {
                group_manager.clear();
            }
            command => println!("Group command {command} not found!")
            }
        }
        command => println!("Command {command} not found!")
        }

    }
}
