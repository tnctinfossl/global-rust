use std::collections::HashMap;
use std::rc::Rc;




pub fn parse()->HashMap<String,Option<String>>{
    let mut result:HashMap<String,Option<String>> = HashMap::new();
    let vec :Vec<String>= std::env::args().collect();
    let mut iter= vec.iter();
    iter.next();
    while let Some(text) =iter.next(){
        if text.starts_with("--"){
            let key= text[2..].to_string();
            if let Some(value)= iter.clone().next(){
                if !value.starts_with("--"){
                    result.insert(key,Some(value.to_string()));
                    iter.next();
                }else{
                    result.insert(key,None);
                }
            }else{
                result.insert(key,None);
            }
        }else {
            
        }
    }
    result
}
   



