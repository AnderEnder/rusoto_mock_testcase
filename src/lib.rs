#[cfg(test)]
mod tests {
    use rusoto_mock::{MockCredentialsProvider, MockRequestDispatcher};
    use rusoto_s3::S3Client;

    #[test]
    fn test_smoke_mock() {
        let client = S3Client::new_with(
            MockRequestDispatcher::default(),
            MockCredentialsProvider,
            Default::default(),
        );
    }
}
