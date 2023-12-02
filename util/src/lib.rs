use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

/// Represents input data loaded from a file, stored as lines.
///
/// This struct is used to hold the contents of a file, where each line
/// of the file is stored as a separate string in a vector.
pub struct Input {
    lines: Vec<String>,
}

impl Input {
    /// Loads input from a file located at the specified path.
    ///
    /// This function reads the file line by line and stores each line
    /// as a string in a vector.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that holds the path to the file.
    ///
    /// # Returns
    ///
    /// Returns `Input` instance containing the lines of the file, or an `io::Error`.
    pub fn load(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
        Ok(Self { lines })
    }

    /// Creates an `Input` instance from an array of lines.
    ///
    /// # Arguments
    ///
    /// * `lines` - An array of string slices representing the lines of input.
    ///
    /// # Returns
    ///
    /// Returns `Input` instance containing the provided lines.
    pub fn from_lines(lines: &[&str]) -> Self {
        Self {
            lines: lines.iter().map(|&line| line.to_string()).collect(),
        }
    }

    /// Provides a reference to the vector of lines stored in the Input struct.
    ///
    /// # Returns
    ///
    /// Returns a reference to a vector of strings, where each string is a line of input.
    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }
}

/// Writes data to a specified file.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file.
/// * `data` - A string slice that holds the data to be written.
///
/// # Returns
///
/// The functions returns an `io::Result<()>`. On success, it returns `Ok(())`,
/// and on failure, it returns an `io::Error`.
pub fn write_file(file_path: &str, data: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(data.as_bytes())
}

/// Reads and returns the contents of a specified file.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file.
///
/// # Returns
///
/// The functions returns an `io::Result<(String)>`. On success, it returns `Ok(String)`
/// containing the file contents, and on failure, it returns an `io::Error`.
pub fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content: String = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use tempfile::{tempdir, TempDir};

    // Helper function that creates a temp directory and file
    fn setup_temp_file(file_name: &str) -> io::Result<(TempDir, String)> {
        let temp_dir: TempDir = tempdir()?;
        let file_path_buf = temp_dir.path().join(file_name);
        let file_path: String = file_path_buf.to_str().unwrap().to_string();
        Ok((temp_dir, file_path))
    }

    // Helper function that creates a temp directory and file with content
    fn setup_temp_file_with_content(
        file_name: &str,
        content: &str,
    ) -> io::Result<(TempDir, String)> {
        let (temp_dir, file_path) = setup_temp_file(file_name)?;
        let mut file = File::create(&file_path)?;
        file.write_all(content.as_bytes())?;
        Ok((temp_dir, file_path))
    }

    mod test_input {
        use crate::test::setup_temp_file_with_content;
        use crate::Input;

        #[test]
        pub fn test_load() -> anyhow::Result<()> {
            // Given a path to a file that contains multiple lines
            let file_contents = "Line 1\nLine 2\nLine 3";
            let (temp_dir, file_path) = setup_temp_file_with_content("test.txt", file_contents)?;

            // When Input is loaded from this file
            let input = Input::load(&file_path)?;

            // Then it should contain each line of the file in its lines vector
            assert_eq!(
                input.lines,
                vec!["Line 1", "Line 2", "Line 3"],
                "Input should contain [\"Line 1\", \"Line 2\", \"Line 3\"] \
                   in its lines vector"
            );
            drop(temp_dir);
            Ok(())
        }

        #[test]
        pub fn test_from_lines() -> anyhow::Result<()> {
            // Given an array of lines
            let lines = ["Line 1", "Line 2", "Line 3"];

            // When Input is created from these lines
            let input = Input::from_lines(&lines);

            // Then it should contain the lines
            assert_eq!(
                input.lines(),
                &vec!["Line 1", "Line 2", "Line 3"],
                "Input.lines() should return [\"Line 1\", \"Line 2\", \"Line 3\"]"
            );
            Ok(())
        }
    }

    #[test]
    pub fn test_write_file() -> Result<()> {
        // Given a file path and content data
        let file_contents: &str = "test data";
        let (temp_dir, file_path) = setup_temp_file("test.txt")?;

        // When write_file is called
        write_file(&file_path, file_contents)?;

        // Then it writes the data to the specified file path
        let mut file = File::open(file_path)?;
        let mut actual: String = String::new();
        file.read_to_string(&mut actual)?;

        assert_eq!(
            actual, file_contents,
            "write_file should write \"${file_contents}\" to specified file."
        );
        drop(temp_dir);
        Ok(())
    }

    #[test]
    pub fn test_read_file() -> Result<()> {
        // Given a path to file containing data
        let file_contents: &str = "test data";
        let (temp_dir, file_path) = setup_temp_file_with_content("test.txt", file_contents)?;

        // When read_file is called
        let actual: String = read_file(&file_path)?;

        // Then it returns the contents of the file
        assert_eq!(
            actual, file_contents,
            "read_file should return \"${file_contents}\"."
        );
        drop(temp_dir);
        Ok(())
    }
}
