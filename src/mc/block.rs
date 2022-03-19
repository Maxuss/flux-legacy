use std::str::FromStr;

pub trait ToCoord {
    fn to_coord(&self) -> Coordinate;
}

impl<C> ToCoord for C where C: Into<Coordinate> + Clone {
    fn to_coord(&self) -> Coordinate {
        self.to_owned().into()
    }
}

impl<C> From<&C> for Coordinate where C: Into<Coordinate> + Clone {
    fn from(c: &C) -> Self {
        Clone::clone(c).into()
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Location {
    x: Coordinate,
    y: Coordinate,
    z: Coordinate
}

impl Location {
    pub fn new<C>(x: C, y: C, z: C) -> Self where C: ToCoord {
        Self {
            x: x.to_coord(),
            y: y.to_coord(),
            z: z.to_coord()
        }
    }

    pub fn relative(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: Coordinate::relative(x),
            y: Coordinate::relative(y),
            z: Coordinate::relative(z)
        }
    }

    pub fn local(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: Coordinate::local(x),
            y: Coordinate::local(y),
            z: Coordinate::local(z)
        }
    }
}

impl ToString for Location {
    fn to_string(&self) -> String {
        let mut buf = String::new();
        buf.push_str(&format!("{} ", self.x.to_string()));
        buf.push_str(&format!("{} ", self.x.to_string()));
        buf.push_str(&self.x.to_string());
        buf
    }
}

impl FromStr for Location {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" ").collect::<Vec<&str>>();
        let x = Coordinate::from_str(split[0])?;
        let y = Coordinate::from_str(split[1])?;
        let z = Coordinate::from_str(split[2])?;
        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Coordinate {
    pos: i32,
    relative: bool,
    local: bool
}

impl Coordinate {
    pub fn new(pos: i32) -> Self {
        Self {
            pos,
            relative: false,
            local: false
        }
    }

    pub fn relative(pos: i32) -> Self {
        Self {
            pos,
            relative: true,
            local: false
        }
    }

    pub fn local(pos: i32) -> Self {
        Self {
            pos,
            relative: false,
            local: true
        }
    }
}

impl ToString for Coordinate {
    fn to_string(&self) -> String {
        let mut s = String::new();
        if self.local {
            s.push_str("^");
        } else if self.relative {
            s.push_str("~");
        };
        if self.pos == 0 {
            s.to_string()
        } else {
            s.push_str(self.pos.to_string().as_str());
            s.to_string()
        }
    }
}

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter = s.clone().chars();
        let mut relative: bool = false;
        let mut local: bool = false;
        let mut out = String::new();
        iter.for_each(|c| {
            match c {
                '~' => relative = true,
                '^' => local = true,
                _ => out.push(c)
            };
        });
        if out.is_empty() {
            Ok(Self {
                pos: 0,
                relative,
                local
            })
        } else {
            Ok(Self {
                pos: i32::from_str(&out).unwrap(),
                relative,
                local
            })
        }
    }
}