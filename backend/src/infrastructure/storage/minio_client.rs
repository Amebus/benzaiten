use anyhow::Result;
use bytes::Bytes;

/// Client per l'interazione con MinIO (storage S3-compatibile)
pub struct MinioClient {
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}

impl MinioClient {
    pub fn new(endpoint: String, bucket: String, access_key: String, secret_key: String) -> Self {
        Self {
            endpoint,
            bucket,
            access_key,
            secret_key,
        }
    }

    /// Carica un file su MinIO e restituisce la chiave S3
    pub async fn upload(&self, key: &str, data: Bytes, content_type: &str) -> Result<String> {
        let url = format!("{}/{}/{}", self.endpoint, self.bucket, key);

        let client = reqwest::Client::new();
        let response = client
            .put(&url)
            .header("Content-Type", content_type)
            .header("Content-Length", data.len())
            .body(data)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Errore upload su MinIO: {}", response.status());
        }

        Ok(key.to_string())
    }

    /// Elimina un file da MinIO
    pub async fn delete(&self, key: &str) -> Result<()> {
        let url = format!("{}/{}/{}", self.endpoint, self.bucket, key);

        let client = reqwest::Client::new();
        let response = client.delete(&url).send().await?;

        if !response.status().is_success() && response.status().as_u16() != 404 {
            anyhow::bail!("Errore eliminazione da MinIO: {}", response.status());
        }

        Ok(())
    }

    /// Restituisce l'URL pubblico di un file
    pub fn public_url(&self, key: &str) -> String {
        format!("{}/{}/{}", self.endpoint, self.bucket, key)
    }
}
