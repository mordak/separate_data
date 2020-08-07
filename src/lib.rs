mod data;
use crate::data::DATA;

pub fn do_stuff() -> Result<(), String> {
    for x in DATA {
        println!("{}", x);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::data::DATA;
    #[test]
    fn it_works() {
        assert!(DATA.len() > 0);
    }
}
