use reqwest::Error;
use super::kata::Kata;

pub async fn kata(code: &str) -> Result<Kata, Error> {
    let url = "https://www.codewars.com/api/v1/code-challenges/".to_string() + code;
    let res = reqwest::get(url).await.expect("Unrecognized URL");
    let kata: Kata = res.json().await?;
    Ok(kata)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn kata_hash_map() -> HashMap<&'static str, &'static str> {
        let mut katas = HashMap::new();
        katas.insert("Invert values", "5899dc03bc95b1bf1b0000ad");
        katas.insert("Name Shuffler", "559ac78160f0be07c200005a");
        katas.insert("8kyu interpreters: HQ9+", "591588d49f4056e13f000001");
        katas.insert("Sequence convergence", "59971e64bfccc70748000068");
        katas
    }

    #[tokio::test]
    async fn test_kata_data() {
        for (name, code) in kata_hash_map() {
            let kata = kata(code).await.unwrap();
            println!("{}", kata.name);
            assert_eq!(name, kata.name);
        }
    }

    #[tokio::test]
    async fn test_kata_train_url() {
        let katas: HashMap<&str, &str> = kata_hash_map();
        for (_, code) in katas {
            let kata = kata(code).await;
            assert!(kata.is_ok());
            let kata = kata.unwrap();
            println!("{}: {:?}", kata.name, kata.train_url().unwrap());
        }
    }

    #[tokio::test]
    async fn test_kata_rank_parse() {
        let katas: HashMap<&str, &str> = kata_hash_map();
        for (_, code) in katas {
            let kata = kata(code).await.unwrap();
            println!("{}: {:?}", kata.name, kata.rank());
        }
    }
}
