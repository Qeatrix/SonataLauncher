use surf;

pub async fn download_in_json(url: String) -> Result<serde_json::Value, String> {
    match surf::get(url).await {
        Ok(mut response) => {
            match response.body_json::<serde_json::Value>().await {
                Ok(data) => Ok(data),
                Err(e) => Err(e.to_string())
            }
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn download(url: String) -> Result<Vec<u8>, String> {
    match surf::get(url).recv_bytes().await {
        Ok(response) => {
            return Ok(response)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}
