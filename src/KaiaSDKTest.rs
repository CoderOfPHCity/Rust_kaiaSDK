use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Mock};
    use serde_json::json;



    fn create_mock_response<T>(data: T, code: i32, msg: &str) -> String 
    where
        T: Serialize,
    {
        json!({
            "code": code,
            "data": data,
            "msg": msg
        }).to_string()
    }

    #[tokio::test]
  //  #[test]
    async fn test_get_fungible_token() {
        let mut server = mockito::Server::new();
        let mock_token = json!({
            "contractType": "ERC20",
            "name": "TestToken",
            "symbol": "TT",
            "icon": "https://example.com/icon.png",
            "decimal": 18,
            "totalSupply": 1000000.0,
            "totalTransfers": 500,
            "officialSite": "https://example.com",
            "burnAmount": 1000.0,
            "totalBurns": 10
        });

        let _m = mock("GET", "/api/v1/tokens")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(create_mock_response(mock_token, 0, "Success"))
            .create();

        let client = KaiaScan::new().unwrap();
        let result = client.get_fungible_token(Address::new("0x1234567890abcdef")).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.code, 0);
        assert_eq!(response.data.name, "TestToken");
    }

  
}