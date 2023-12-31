use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::{fs, io};

use rand::Rng;
use regex::Regex;
use serde_json::json;

/// # errors
/// 1. parents of dest dir not exists
/// 2. source dir not exists
/// 3. source dir is not a dir
pub fn copy_directory(source_dir: &Path, dest_dir: &Path) -> std::io::Result<()> {
    //may have TOCTOU problem
    if !dest_dir.exists() {
        fs::create_dir_all(dest_dir)?;
    }

    for entry in fs::read_dir(source_dir)? {
        println!("hhh");
        let entry = entry?;
        let dest_file = dest_dir.join(entry.file_name());

        if entry.path().is_dir() {
            copy_directory(&entry.path(), &dest_file)?;
        } else {
            fs::copy(entry.path(), dest_file)?;
        }
    }
    Ok(())
}

/// convert a non-integral string to a double
/// its behavior is different from value.parse::<f64>().unwrap()
pub fn convert_to_double(value: &str) -> f64 {
    let mut ret = 0;
    for c in value.chars() {
        ret = ret * 10 + (c as u32);
    }

    ret as f64
}

/// get the last part after . of a string
pub fn get_simple_name(name: &str) -> &str {
    match name.rfind('.') {
        Some(index) => &name[index + 1..],
        None => name,
    }
}

/// get the last part after / of a string
pub fn get_simple_file_name(name: &str) -> &str {
    match name.rfind('/') {
        Some(index) => &name[index + 1..],
        None => name,
    }
}

pub fn random_json_car_data() -> String {
    let mut rng = rand::thread_rng();
    let max = 120 as f64;
    json!(
        {
            "front": rng.gen::<f64>() * max ,
            "back": rng.gen::<f64>() * max,
            "left": rng.gen::<f64>() * max,
            "right": rng.gen::<f64>() * max,
        }
    )
    .to_string()
}

/// 正则表达式要求：
/// 1. 前缀任意多字符，最多有一个/
/// 2. 后接任意字母数字下划线点号，最少一个字符
/// 3. 后接-line至少一个数字
/// 4. -grp至少一个数字
/// 5. 后接.至少一个字符
pub fn is_trace_file(file_name: &str) -> bool {
    let re = Regex::new(r".*/?[a-zA-Z0-9._]+-line\d+-grp\d+\.[a-zA-Z]+$").unwrap();
    re.is_match(file_name)
}

/// get app name, line number and group from file name
/// #warning:
/// file_name should match the pattern
pub fn get_app_name_line_number_group(file_name: &str) -> (String, i32, i32) {
    let index0 = file_name.rfind('/');
    let index0 = match index0 {
        Some(index) => index as i32,
        None => -1,
    };

    let index1 = file_name.find('-').unwrap();
    let index2 = file_name[index1 + 1..].find('-').unwrap() + index1 + 1;
    let index3 = file_name.rfind('.').unwrap();

    let app_name = &file_name[(index0 + 1) as usize..index1];
    let line_number: i32 = file_name[index1 + 5..index2].parse().unwrap();
    let group: i32 = file_name[index2 + 4..index3].parse().unwrap();

    (app_name.to_string(), line_number, group)
}

/// get the max value of a hashmap
pub fn get_max_value<K: Ord + Hash, V>(m: &HashMap<K, V>) -> Option<&V> {
    m.iter().max_by_key(|(k, _)| *k).map(|(_, v)| v)
}

/// get the max key of a hashmap
pub fn get_max_key<K: Ord + Hash, V>(m: &HashMap<K, V>) -> Option<&K> {
    m.iter().max_by_key(|(k, _)| *k).map(|(k, _)| k)
}

/// transform a hashmap to a vector of key-value pairs
pub fn map_list_to_list_list<K, V>(m: &HashMap<K, Vec<V>>) -> Vec<Vec<V>>
where
    K: Eq + Hash,
    V: Clone,
{
    m.values().cloned().collect()
}

//below only implemented used functions
//TODO: add useless functions

/// delete a directory with all contents or a file
pub fn delete_dir(dir: &Path) -> std::io::Result<()> {
    if dir.is_dir() {
        fs::remove_dir_all(dir)
    } else {
        fs::remove_file(dir)
    }
}

/// read a file and replace " with replace_quota
pub fn read_file_content(file_name: &str, replace_quota: Option<&str>) -> io::Result<String> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let replace_quota = replace_quota.unwrap_or("\"");
    let mut content = String::new();
    for line in reader.lines() {
        let line = line?;
        content.push_str(&line.replace("\"", replace_quota));
        content.push('\n');
    }

    Ok(content)
}

/// write content to a file and replace replace_quota with "
pub fn write_file_content(
    dir: &str,
    name: &str,
    content: &str,
    replace_quota: Option<&str>,
) -> io::Result<()> {
    let dir_path = Path::new(dir);
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }
    let file_name = dir_path.join(name);

    let replace_quota = replace_quota.unwrap_or("\"");
    let content = content.replace(replace_quota, "\"");

    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

/// compute distance of two double vectors
/// #warning:
/// v1 and v2 should have the same length
pub fn distance(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    let mut ret = 0.0;
    for i in 0..v1.len() {
        ret += (v1[i] - v2[i]).powf(2.0);
    }
    ret.sqrt()
}

//TODO: add log functions like java: createNewLog4jProperties

///should test by one thread for file system
#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    #[test]
    fn test_copy_directory() {
        //. means carte directory
        let source_dir = Path::new("./test_dir");
        println!("{:?}", source_dir);
        let dest_dir = Path::new("./test_dir_copy");
        let _ = fs::remove_dir_all(dest_dir);
        copy_directory(source_dir, dest_dir).unwrap();
        assert!(dest_dir.exists());
        assert!(dest_dir.is_dir());
        assert!(dest_dir.join("test.txt").exists());
    }

    #[test]
    fn test_convert_to_double() {
        let value = "abc";
        let ret = convert_to_double(value);
        println!("{}", ret);
    }

    #[test]
    fn test_get_simple_name() {
        let name = "abc.ss";
        let ret = get_simple_name(name);
        assert_eq!(ret, "ss")
    }

    #[test]
    fn test_get_simple_file_name() {
        let name = "abc/def.ss";
        let ret = get_simple_file_name(name);
        assert_eq!(ret, "def.ss")
    }

    #[test]
    fn test_random_json_car_data() {
        let json_str = random_json_car_data();
        println!("{}", json_str);
    }

    #[test]
    fn test_is_trace_file() {
        let file_name = "./../abc-line1-grp1.txt";
        assert!(is_trace_file(file_name));
        let file_name = "abc-line1-grp1";
        assert!(!is_trace_file(file_name));
    }

    #[test]
    fn test_get_app_name_line_number_group() {
        let file_name = "/abc-line1-grp1.txt";
        let (app_name, line_number, group) = get_app_name_line_number_group(file_name);
        assert_eq!(app_name, "abc");
        assert_eq!(line_number, 1);
        assert_eq!(group, 1);
    }

    #[test]
    fn test_get_max_value() {
        let mut m = HashMap::new();
        m.insert(1, 1);
        m.insert(2, 2);
        m.insert(3, 3);
        let ret = get_max_value(&m);
        assert_eq!(ret, Some(&3));
        println!("{:?}", m);
    }

    #[test]
    fn test_get_max_key() {
        let mut m = HashMap::new();
        m.insert(1, 1);
        m.insert(2, 2);
        m.insert(4, 3);
        let ret = get_max_key(&m);
        assert_eq!(ret, Some(&4));
        println!("{:?}", m);
    }

    #[test]
    fn test_map_list_to_list_list() {
        let mut m = HashMap::new();
        m.insert(1, vec![1, 2, 3]);
        m.insert(2, vec![4, 5, 6]);
        m.insert(3, vec![7, 8, 9]);
        let ret = map_list_to_list_list(&m);
        println!("{:?}", ret);
    }

    #[test]
    fn test_delete_dir() {
        let dir = Path::new("./test_dir_copy_1");
        println!("{:?}", delete_dir(dir));
        assert!(!dir.exists());
    }

    #[test]
    fn test_read_file_content() {
        let file_name = "./test_dir/test.txt";
        let content = read_file_content(file_name, Some("'")).unwrap();
        println!("{}", content);
        let content = read_file_content(file_name, None).unwrap();
        println!("{}", content);
    }

    #[test]
    fn test_write_file_content() {
        let dir = "./test_dir_copy";
        let name = "test.txt";
        let content = "abc";
        write_file_content(dir, name, content, Some("'")).unwrap();
        let content = read_file_content(&format!("{}/{}", dir, name), None).unwrap();
        println!("{}", content);
    }
}
