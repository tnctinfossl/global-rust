pub struct SubProgram {
    pub name: String,
    pub param: String,
    pub usage:String,
    pub program: Box<dyn Fn(&[String])>,
}

pub type SubPrograms = Vec<SubProgram>;

impl SubProgram{
    pub fn new(name:String,param:String,usage:String,program:Box<dyn Fn(&[String])>)->SubProgram{
        SubProgram{
            name:name,
            param:param,
            usage:usage,
            program:program
        }
    }
}

pub trait SubProgramsTrait {
    fn run(&self, args: &[String]);
    fn help(&self);
}

impl SubProgramsTrait for SubPrograms {
    fn run(&self, args: &[String]) {
        if let Some(sub) = self.iter().find(|&cmp| cmp.name == args[0]) {
            (sub.program)(&args);
        } else {
            self.help();
        }
    }
    fn help(&self) {
        println!("usage");
        for item in self {
            println!("\t{} {}:{}", item.name, item.param, item.usage);
        }
    }
}