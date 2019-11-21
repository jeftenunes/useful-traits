use std::fmt;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct USD(i32);

impl Display for USD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = (self.0 as f32) / 100.;
        if r < 0. {
            return write!(f,"-${:.2}",-r);
        }

        write!(f,"${:.2}",r)
    }
}

impl Clone for USD {
    fn clone(&self) -> USD {
        USD(self.0 + 1)
    }
}

impl Copy for USD { }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let usd = USD(230);
        let clone_usd = usd;

        assert_eq!(usd.to_string(), "$2.30".to_string());
        assert_eq!(usd, clone_usd);
    }
}
