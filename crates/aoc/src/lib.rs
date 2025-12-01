pub mod session;
mod stats;

use reqwest::{Client, Url};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

pub struct Context {
    pub session_id: String,
    pub data_dir: PathBuf,
    pub url: Url,
}

/// Fetches the input for the specified year and day.
///
/// # Arguments
///
/// * `year` - A string slice that holds the year of the event.
/// * `day` - A string slice that holds the day of the event.
/// * `context` - Defines the context to interact with the Advent of Code website.
///
/// # Returns
///
/// * `Ok(Vec<String>)` - If the input is fetched and parsed successfully.
/// * `Err(String)` - If there is an error fetching or parsing the input.
pub async fn get_input(year: &i32, day: &i32, context: &Context) -> Result<Vec<String>, String> {
    println!("Fetching input from for AOC {} Day {}", year, day);

    let body = match get_input_from_cache(&year, &day, &context) {
        Some(body) => body,
        None => {
            let body = get_input_from_site(year, day, &context).await?;
            add_to_cache(year, day, &body, &context)?;
            body
        }
    };

    let result: Vec<String> = body.split("\n").map(|s| s.to_string()).collect();

    Ok(result)
}

/// Builds the file name for the input data based on the year and day.
///
/// # Arguments
///
/// * `year` - A string slice that holds the year of the event.
/// * `day` - A string slice that holds the day of the event.
///
/// # Returns
///
/// * `String` - The constructed file name in the format `year.day.dat`.
fn build_file_name(year: &i32, day: &i32) -> String {
    format!("{}.day{}.dat", year, day)
}

/// Retrieves the input from the cache if it exists.
///
/// # Arguments
///
/// * `year` - A string slice that holds the year of the event.
/// * `day` - A string slice that holds the day of the event.
///
/// # Returns
///
/// * `Ok(Some(String))` - If the input file exists and is read successfully.
/// * `Ok(None)` - If the input file does not exist.
/// * `Err(String)` - If there is an error reading the input file or creating the directory.
pub fn get_input_from_cache(year: &i32, day: &i32, context: &Context) -> Option<String> {
    let input_dir = std::env::current_dir().unwrap().join(&context.data_dir);
    if !input_dir.exists() {
        fs::create_dir(&input_dir).unwrap();
    }

    let input_file = input_dir.join(build_file_name(year, day));
    return if input_file.exists() {
        println!("Cache hit");
        let body = fs::read_to_string(input_file).unwrap();
        Some(body)
    } else {
        None
    };
}
/// Cache the input data for later recall.
///
/// # Arguments
///
/// * `year` - A string slice that holds the year of the event.
/// * `day` - A string slice that holds the day of the event.
/// * `body` - A string slice that holds the input data to be cached.
///
/// # Returns
///
/// * `Ok(())` - If the input is successfully written to the cache.
/// * `Err(String)` - If there is an error writing the input to the cache.
pub fn add_to_cache(year: &i32, day: &i32, body: &str, context: &Context) -> Result<(), String> {
    let input_dir = std::env::current_dir().unwrap().join(&context.data_dir);
    if !input_dir.exists() {
        fs::create_dir(&input_dir).map_err(|e| e.to_string())?;
    }

    let input_file = input_dir.join(build_file_name(year, day));
    fs::write(input_file, body).map_err(|e| e.to_string())
}

/// Fetches the input from the site for the specified year and day.
///
/// # Arguments
///
/// * `year` - A string slice that holds the year of the event.
/// * `day` - A string slice that holds the day of the event.
/// * `session_id` - A string slice that holds the session ID for authentication.
///
/// # Returns
///
/// * `Ok(String)` - If the input is fetched successfully.
/// * `Err(String)` - If there is an error fetching the input.
pub async fn get_input_from_site(
    year: &i32,
    day: &i32,
    context: &Context,
) -> Result<String, String> {
    let path = format!("{}{}/day/{}/input", context.url, year, day);
    println!("Fetching input from {}", path);

    if path.is_empty() {
        return Err("Path is empty".to_string());
    }

    let jar = reqwest::cookie::Jar::default();

    // the value of domain needs to be extracted from context.url with the schema removed
    let domain = context.url.host_str().unwrap();
    jar.add_cookie_str(
        format!("session={}; Domain={}; Path=/", context.session_id, domain).as_str(),
        &context.url,
    );

    let client = Client::builder()
        .cookie_provider(Arc::new(jar))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client.get(&path).send().await.map_err(|e| e.to_string())?;
    let status = &response.status();
    let body = response.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!(
            "Failed to fetch input from {}.  Response: {}",
            path, body
        ));
    }

    Ok(body.trim().to_string())
}

pub fn pretty_print<T: std::fmt::Debug>(matrix: &Vec<Vec<T>>) {
    for row in matrix {
        println!("{:#?}", row.iter().map(|elem| format!("{:?}", elem)).collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctor::ctor;
    use std::sync::LazyLock;
    pub static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| get_data_dir());

    fn get_data_dir() -> PathBuf {
        std::env::temp_dir().join("aoc_test")
    }

    #[ctor]
    fn init() {
        // Delete all files from the Data directory
        if DATA_DIR.exists() {
            for entry in fs::read_dir(DATA_DIR.clone()).unwrap() {
                let entry = entry.unwrap();
                if entry.path().is_file() {
                    fs::remove_file(entry.path()).unwrap();
                }
            }
        } else {
            fs::create_dir(DATA_DIR.clone()).unwrap();
        }
    }

    // this is not using tokio::test due to runtime block_on collisions when using mockito
    #[test]
    fn test_get_input_from_site() {
        let mut server = mockito::Server::new();

        let _m = server
            .mock("GET", "/2023/day/1/input")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("mocked input")
            .create();

        let context = Context {
            session_id: "fake_session_id".to_string(),
            data_dir: DATA_DIR.clone(),
            url: Url::parse(server.url().as_str()).unwrap(),
        };

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result = runtime.block_on(get_input_from_site(&2023, &1, &context));

        println!("{:?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "mocked input");
    }



    #[test]
    fn cache_hit() {
        let year = 1013;
        let day = 8;
        let body = "test input";

        let context = Context {
            session_id: "fake_session_id".to_string(),
            data_dir: DATA_DIR.clone(),
            url: Url::parse("https://adventofcode.com").unwrap(),
        };

        let result = add_to_cache(&year, &day, body, &context);
        assert!(result.is_ok());

        let result = get_input_from_cache(&year, &day, &context);
        assert_eq!(result, Some(body.to_string()));
    }

    #[test]
    fn cache_miss() {
        let year = 9999;
        let day = 1;

        let context = Context {
            session_id: "fake_session_id".to_string(),
            data_dir: DATA_DIR.clone(),
            url: Url::parse("https://adventofcode.com").unwrap(),
        };

        let result = get_input_from_cache(&year, &day, &context);
        assert_eq!(result, None);
    }

    #[test]
    fn populate_cache() {
        let year = 1013;
        let day = 2;
        let body = "test input";

        let context = Context {
            session_id: "fake_session_id".to_string(),
            data_dir: DATA_DIR.clone(),
            url: Url::parse("https://adventofcode.com").unwrap(),
        };

        let result = add_to_cache(&year, &day, body, &context);
        assert!(result.is_ok());

        let cached_file = context.data_dir.join(build_file_name(&year, &day));
        let cached_body = fs::read_to_string(&cached_file).unwrap();
        assert_eq!(cached_body, body);
    }

    #[test]
    fn overwrite_cached_file() {
        let year = 1013;
        let day = 7;

        let context = Context {
            session_id: "fake_session_id".to_string(),
            data_dir: DATA_DIR.clone(),
            url: Url::parse("https://adventofcode.com").unwrap(),
        };

        let result = add_to_cache(&year, &day, "original", &context);
        assert!(result.is_ok());
        let body = "altered";
        let result = add_to_cache(&year, &day, body, &context);
        assert!(result.is_ok());

        let cached_file = context.data_dir.join(build_file_name(&year, &day));
        let cached_body = fs::read_to_string(&cached_file).unwrap();
        assert_eq!(cached_body, body);
    }

    #[test]
    fn test_build_file_name() {
        let year = 1013;
        let day = 7;
        let result = build_file_name(&year, &day);
        assert_eq!(result, "1013.day7.dat");
    }
}
