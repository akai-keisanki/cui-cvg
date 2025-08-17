pub trait IGroup
{
    fn new (label : &String) -> Self;
    fn add (&mut self, ch : &String) -> ();
    fn get_chars (&self) -> &Vec::<String>;
    fn clear (&mut self) -> ();
}

pub struct Group
{
    label : String,
    chars : Vec::<String>
}

impl IGroup for Group
{
    fn new (label : &String) -> Self
    {
        Self {
            label: label.clone(),
            chars: Vec::<String>::new()
        }
    }

    fn add (&mut self, ch : &String) -> ()
    {
        self.chars.push(ch.clone());
    }

    fn get_chars (&self) -> &Vec::<String>
    {
        &self.chars
    }

    fn clear (&mut self) -> ()
    {
        self.chars.clear();
    }
}


pub trait IGroupManager
{
    fn new () -> Self;
    fn group (&mut self, label : &String) -> Option<&mut Group>;
    fn add (&mut self, label : &String) -> ();
}

pub struct GroupManager
{
    groups : Vec::<Group>
}

impl IGroupManager for GroupManager
{
    fn new () -> Self
    {
        Self {
            groups: Vec::<Group>::new()
        }
    }

    fn group (&mut self, label : &String) -> Option<&mut Group>
    {
        self.groups.iter_mut().find(|group| group.label == *label)
    }

    fn add (&mut self, label : &String) -> ()
    {
        if let None = self.group(label)
        {
            self.groups.push(Group::new(label));
        }
    }
}
