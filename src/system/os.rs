use std::path::Path;

pub fn is_a_container() -> bool 
{
    return Path::new(".asukadocker").exists()
}
