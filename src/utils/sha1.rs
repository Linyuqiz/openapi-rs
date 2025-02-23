use md5::Digest;
use sha1::Sha1;

pub fn sha1(content: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let result = hasher.finalize();
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1() {
        assert_eq!(
            sha1("hello world"),
            "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"
        );
        assert_eq!(sha1("abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");
        assert_eq!(sha1(""), "da39a3ee5e6b4b0d3255bfef95601890afd80709")
    }
}
