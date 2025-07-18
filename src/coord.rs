use super::*;

#[derive(Debug)]
enum CoordRepr {
    Empty,
    Num(usize),
    Alg(String),
}

#[derive(Debug)]
pub struct InvalidCoordinateError {
    coord: CoordRepr,
}

impl InvalidCoordinateError {
    fn new() -> Self {
        Self {
            coord: CoordRepr::Empty,
        }
    }
    fn new_num(coord: usize) -> Self {
        Self {
            coord: CoordRepr::Num(coord),
        }
    }

    fn new_alg(alg: &str) -> Self {
        Self {
            coord: CoordRepr::Alg(String::from(alg)),
        }
    }
}

impl Display for InvalidCoordinateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.coord {
            CoordRepr::Empty => write!(f, "Invalid Coordinate"),
            CoordRepr::Num(coord) => write!(f, "Invalid Coordinate: {coord:#x}"),
            CoordRepr::Alg(alg) => write!(f, "Invalid Coordinate: {alg}"),
        }
    }
}

impl Error for InvalidCoordinateError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord(pub u8);

impl Coord {
    pub fn from_rf(rank: usize, file: usize) -> Result<Self, InvalidCoordinateError> {
        let val = ((rank) << 4) + file;
        if val & 0x88 != 0 || val >= 128 {
            return Err(InvalidCoordinateError::new_num(val));
        }
        Ok(Self(val as u8))
    }

    pub fn to_rf(self) -> (usize, usize) {
        (self.rank() as usize, self.file() as usize)
    }

    pub fn rank(self) -> u8 {
        self.0 >> 4
    }

    pub fn file(self) -> u8 {
        self.0 & 7
    }

    pub fn from_alg(alg: &str) -> Result<Self, InvalidCoordinateError> {
        if alg.len() != 2 {
            return Err(InvalidCoordinateError::new_alg(alg));
        }
        let mut chars = alg.chars();
        let file = (chars.next().unwrap() as usize) - ('a' as usize);
        let rank = (chars.next().unwrap() as usize) - ('1' as usize);

        Self::from_rf(rank, file)
    }

    pub fn to_alg(self) -> String {
        let mut alg = String::new();

        alg.push((self.file() + ('a' as u8)) as char);
        alg.push((self.rank() + ('1' as u8)) as char);

        alg
    }

    pub fn add(self, b: (isize, isize)) -> Result<Self, InvalidCoordinateError> {
        let mut rf = self.to_rf();
        rf.0 =
            rf.0.checked_add_signed(b.0)
                .ok_or(InvalidCoordinateError::new())?;
        rf.1 =
            rf.1.checked_add_signed(b.1)
                .ok_or(InvalidCoordinateError::new())?;
        rf.try_into()
    }
}

impl TryFrom<(usize, usize)> for Coord {
    type Error = InvalidCoordinateError;
    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        Self::from_rf(value.0, value.1)
    }
}

impl From<Coord> for (usize, usize) {
    fn from(value: Coord) -> (usize, usize) {
        value.to_rf()
    }
}
