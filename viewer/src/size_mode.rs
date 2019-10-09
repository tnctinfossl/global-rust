#[derive(Debug,PartialOrd, PartialEq,Copy,Clone)]
pub enum SizeMode {
    Smallest,
    Small,
    Normal,
    Large,
    Largest,
}

impl Default for SizeMode{
    fn default()->SizeMode{
        SizeMode::Normal
    }
}

impl SizeMode{
    pub fn next (&self)->Option<Self>{
        match self{
            SizeMode::Smallest=>Some(SizeMode::Small),
            SizeMode::Small=>Some(SizeMode::Normal),
            SizeMode::Normal=>Some(SizeMode::Large),
            SizeMode::Large=>Some(SizeMode::Largest),
            SizeMode::Largest=>None
        }
    }
    pub fn back (&self)->Option<Self>{
        match self{
            SizeMode::Smallest=>None,
            SizeMode::Small=>Some(SizeMode::Smallest),
            SizeMode::Normal=>Some(SizeMode::Small),
            SizeMode::Large=>Some(SizeMode::Normal),
            SizeMode::Largest=>Some(SizeMode::Large)
        }
    }
    pub fn size(&self)->(i32,i32){
        match self{
            SizeMode::Smallest=>(640,480),
            SizeMode::Small=>(800,600),
            SizeMode::Normal=>(1366,768),
            SizeMode::Large=>(1920,1080),
            SizeMode::Largest=>(2560,1440)
        }
    }
}
