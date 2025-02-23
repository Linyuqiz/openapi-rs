use std::time;

pub fn current_timestamp() -> anyhow::Result<String> {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap_or(time::Duration::from_secs(0))
        .as_secs()
        .to_string()
        .parse()
        .map_err(|e| anyhow::Error::new(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_timestamp() {
        let timestamp = current_timestamp().unwrap();
        dbg!("timestamp: {}", &timestamp);
        assert!(timestamp.parse::<i64>().is_ok());
    }
}
