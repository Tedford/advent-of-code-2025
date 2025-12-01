    use std::fs;
    use std::path::PathBuf;

    /// Retrieves the session ID from the `.session` file.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - If the session ID is read successfully.
    /// * `Err(String)` - If there is an error reading the session ID.
    pub fn get_session_id(root: &PathBuf) -> Option<String> {
        // check if .session file exists
        let session_path = root.join(".session");
        return if fs::metadata(&session_path).is_ok() {
            Some(fs::read_to_string(&session_path).unwrap())
        } else {
            None
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use std::{path::PathBuf, sync::LazyLock};
        pub static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| get_data_dir());

        fn get_data_dir() -> PathBuf {
            std::env::temp_dir().join("aoc_session_test")
        }

        #[test]
        fn test_get_session_id() {
            create_session_file();
            let session_id = get_session_id(&DATA_DIR.clone());
            assert_eq!(session_id, Some("fake_session_id".to_string()));
        }

        #[test]
        fn when_no_session_file_exists() {
            clear_session_file();

            let session_id = get_session_id(&DATA_DIR.clone());
            assert_eq!(session_id, None);
        }

        fn clear_session_file() {
            
            let session_path = DATA_DIR.join(".session");
            if fs::metadata(&session_path).is_ok() {
                fs::remove_file(&session_path).unwrap();
            }
        }

        fn create_session_file() {
            let session_path = DATA_DIR.join(".session");
            println!("Creating test file at {:?}", session_path);
            fs::write(&session_path, "fake_session_id").unwrap();
        }

    }
