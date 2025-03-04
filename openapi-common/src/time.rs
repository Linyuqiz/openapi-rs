use std::time;

pub fn current_timestamp() -> anyhow::Result<String> {
    Ok(time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)?
        .as_secs()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_timestamp() -> anyhow::Result<()> {
        let timestamp = current_timestamp()?;
        dbg!(&timestamp);
        assert!(timestamp.parse::<i64>().is_ok());
        Ok(())
    }
}
