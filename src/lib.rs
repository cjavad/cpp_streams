use std::{ops::Shl, ops::Shr, fmt::Display, io::{self, Read}};

#[allow(non_camel_case_types)]
pub struct cout;

#[allow(non_camel_case_types)]
pub struct cin;

#[allow(non_camel_case_types)]
pub struct endl;

impl Display for endl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)
    }    
}


impl<T> Shl<T> for cout
where
    T: Display,
{
    type Output = Self;

    fn shl(self, rhs: T) -> Self::Output {
        print!("{}", rhs);
        self
    }

}

impl Shr<&mut String> for cin {
    type Output = Self;

    fn shr(self, lhs: &mut String) -> Self::Output {
        io::stdin().read_line(lhs).unwrap();
        lhs.truncate(lhs.trim_end().len()); // To remove trailing newline
        self
    }
}
