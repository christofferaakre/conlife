//! Module exposing the API for creating custom objects or starting configurations that can be
//! loaded onto a grid.
/// Struct representing objects that can be loaded onto the grid.
/// You can for example load just one object, and then that object represents your entire
/// initial starting state for the grid, or you can for example have one object that represents
/// a glider, and load two gliders onto the grid at different positions.
#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub coordinates: Vec<(usize, usize)>,
}

/// Enum describing the possible errors that can happen when trying to load an object.
#[derive(Debug, PartialEq)]
pub enum LoadObjectError {
    /// The parser failed to parse your input into any coordinates
    NoCoordinatesFound,
    /// Your input contained duplicate coordinates
    DuplicateCoordinate,
    /// The input provided was bad
    BadInput,
}

impl Object {
    /// Load an object from a file, usually with a `.life` extension, but this is not required.
    /// [`Self::from_string`] calls this function under the hood,
    /// so you can refer to its documentation to see what the format of the string should be.
    /// Sample files defining various objects can be found at <https://github.com/christofferaakre/conlife/tree/master/objects>.
    pub fn from_file(filename: &str) -> Result<Object, LoadObjectError> {
        let file_contents = std::fs::read_to_string(filename).expect("Failed to read file");
        Self::from_string(file_contents.as_str())
    }

    /// Load an object from a string. The string should contain ordered (x,y) coordinate pairs, separated by whitespace.
    /// Below is an example that defines a glider:
    /// `(0,2) (1,2) (2,2) (1,0) (2,1)`
    pub fn from_string(buffer: &str) -> Result<Object, LoadObjectError> {
        let mut coordinates = vec![];
        let buffer = buffer.replace("(", "");
        let buffer = buffer.replace(")", "");
        // Ignore whitespace before or after commas
        let buffer = buffer.replace(", ", ",");
        let buffer = buffer.replace(" ,", ",");

        let coords = buffer.split_whitespace();
        for coord in coords {
            let mut coord_vals = coord.split(",");
            let x: usize = coord_vals
                .next()
                .expect("Failed to get next value in comma split")
                .parse()
                .expect("Failed to parse coordinate to usize");
            let y: usize = coord_vals
                .next()
                .expect("Failed to get next value in comma split")
                .parse()
                .expect("Failed to parse coordinate to usize");
            let pair = (x, y);

            // handling duplicate coordinates
            if coordinates.contains(&pair) {
                return Err(LoadObjectError::DuplicateCoordinate);
            }
            coordinates.push(pair);
        }
        if coordinates.is_empty() {
            return Err(LoadObjectError::NoCoordinatesFound);
        }
        Ok(Self { coordinates })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_from_empty_string_throws_error() {
        let expected = Err(LoadObjectError::NoCoordinatesFound);
        let actual = Object::from_string("");
        assert_eq!(expected, actual);
    }

    #[test]
    fn newlines_no_error() {
        let glider_str = "(0,2)\n(1,2)\n(2,2)\n(1,0)\n(2,1)";
        let glider = Object::from_string(glider_str);
        assert!(glider.is_ok());
        let glider = glider.unwrap();
        assert_eq!(
            glider.coordinates,
            vec![(0, 2), (1, 2), (2, 2), (1, 0), (2, 1)]
        );
    }

    #[test]
    fn whitespace_around_comma() {
        let object_str = "(0, 2) (1,3)";
        let object = Object::from_string(object_str);
        assert!(object.is_ok());
        let object = object.unwrap();
        assert_eq!(object.coordinates, vec![(0, 2), (1, 3)]);
    }

    #[test]
    fn repeated_coordinates() {
        let object_str = "(0,2) (3,4) (0,2)";
        let object = Object::from_string(object_str);
        assert_eq!(Err(LoadObjectError::DuplicateCoordinate), object);
    }

    #[test]
    fn load_glider() {
        let glider = Object::from_file("objects/glider.life");
        assert!(glider.is_ok());
        assert_eq!(
            glider.unwrap().coordinates,
            vec![(0, 2), (1, 2), (2, 2), (1, 0), (2, 1)]
        );
    }
}
