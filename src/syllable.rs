use group::{Group, IGroup};


pub enum OptionalGroup
{
    Required(Group),
    Optional(Vec::<OptionalGroup>)
}


pub trait ISyllable
{
    fn new (label : &String) -> Self;
    fn set_pattern (&mut self, pattern &Vec::<OptionalGroup>) -> ();
    fn get_pattern (&self) -> &Vec::<OptionalGroup>;
}

pub struct Syllable
{
    label : String,
    pattern : Vec::<OptionalGroup>
}

impl ISyllable for Syllable
{
    fn new (label : &String) -> Self
    {
        Self {
            label: label.clone(),
            pattern: Vec::<OptionalGroup>::new()
        }
    }

    fn set_pattern (&mut self, pattern &Vec::<OptionalGroup>) -> ()
    {
        self.pattern = pattern.clone();
    }

    fn get_pattern () -> &Vec::<OptionalGroup>
    {
        &self.pattern
    }
}


pub trait ISyllableManager
{
    fn new () -> Self;
    fn add (&mut self, label : &String) -> ();
    fn list (&self) -> &Vec::<Syllable>;
}

pub struct SyllableManager
{
    syllables : Vec::<Syllable>
}

impl ISyllableManager for SyllableManager
{
    fn new () -> Self
    {
        Self {
            syllables: Vec::<Syllable>::new()
        }
    }

    fn add (&mut self, label : &String) -> ()
    {
        self.syllables.push(label.clone());
    }

    fn list (&self) -> &Vec::<Syllable>
    {
        &self.syllables
    }
}
