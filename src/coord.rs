use super::*;

#[derive(Debug)]
enum CoordRepr<'a> {
    Num(usize),
    Alg(&'a str),
}

#[derive(Debug)]
pub struct InvalidCoordinateError<'a> {
    coord: CoordRepr<'a>,
}

impl<'a> InvalidCoordinateError<'a> {
    fn new(coord: usize) -> Self {
        Self {
            coord: CoordRepr::Num(coord),
        }
    }

    fn new_alg(alg: &'a str) -> Self {
        Self {
            coord: CoordRepr::Alg(alg),
        }
    }
}

impl<'a> Display for InvalidCoordinateError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.coord {
            CoordRepr::Num(coord) => write!(f, "Invalid Coordinate: {coord:#x}"),
            CoordRepr::Alg(alg) => write!(f, "Invalid Coordinate: {alg}"),
        }
    }
}

impl<'a> Error for InvalidCoordinateError<'a> {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coord(pub u8);

impl<'a> Coord {
    pub fn from_rf(rank: usize, file: usize) -> Result<Self, InvalidCoordinateError<'a>> {
        let val = ((rank) << 4) + (file);
        if val & 0x88 != 0 || val >= 128 {
            return Err(InvalidCoordinateError::new(val));
        }
        Ok(Self(val as u8))
    }

    pub fn to_rf(self) -> (usize, usize) {
        ((self.0 >> 4) as usize, (self.0 & 7) as usize)
    }

    pub fn from_alg(alg: &str) -> Result<Self, InvalidCoordinateError<'_>> {
        if alg.len() != 2 {
            return Err(InvalidCoordinateError::new_alg(alg));
        }
        let mut chars = alg.chars();
        let file = (chars.next().unwrap() as usize) - ('a' as usize);
        let rank = (chars.next().unwrap() as usize) - ('1' as usize);

        Self::from_rf(rank, file)
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self::from_rf(value.0, value.1).unwrap()
    }
}

impl From<Coord> for (usize, usize) {
    fn from(value: Coord) -> (usize, usize) {
        value.to_rf()
    }
}
