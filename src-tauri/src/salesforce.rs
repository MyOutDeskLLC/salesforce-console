use rustforce::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::state::{AppState};
use rustforce::response::{QueryResponse};
use crate::{State};




#[derive(Serialize, Debug)]
struct Response {
    status: String,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct SfErrorResponse {
    message: String,
    error_code: String,
    fields: Vec<String>
}

#[tauri::command]
pub async fn login(app_state: State<'_, AppState>, client_id: String, client_secret: String, username: String, password: String) -> Result<String, String> {
    match connect_to_sf(client_id, client_secret, username, password).await {
        Ok(client) => {
            app_state.0.lock().unwrap().sf_client = client;
            Ok("Logged in".to_string())
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(format!("{}", e))
        }
    }
}

#[tauri::command]
pub async fn query(app_state: State<'_, AppState>, query: String) -> Result<String, String> {

    let sf_client = {
        let app_data = app_state.0.lock().unwrap();
        app_data.sf_client.clone()
    };
    let result: Result<QueryResponse<Value>, Error> = sf_client.query(query.as_str()).await;
    match result {
        Ok(result) => {
            Ok(serde_json::to_string(&result.records).unwrap())
        }
        Err(Error::NotLoggedIn) => {
            Err("You are not logged in to Salesforce. Please login and try again.".to_string())
        }
        Err(Error::TokenError(resp)) => {
            Err(format!("Invalid token received from Salesforce: {:?}", resp))
        }
        Err(Error::HTTPError(resp)) => {
            Err(format!("Failed to make an HTTP request to Salesforce: {}", resp))
        }
        Err(Error::DeserializeError(resp)) => {
            Err(format!("Failed to deserialize response from Salesforce: {}", resp))
        }
        Err(Error::ErrorResponses(resp)) => {
            println!("Error response from Salesforce: {:?}", resp);
            let mut errors : Vec<SfErrorResponse> = vec![];
            for error in resp {
                let mut error_fields : Vec<String> = vec![];
                if let Some(fields) = error.fields {
                    for field in fields {
                        error_fields.push(field);
                    }
                }
                errors.push(SfErrorResponse {
                    message: error.message,
                    error_code: error.error_code,
                    fields: error_fields
                });
            }
            Err(serde_json::to_string(&errors).unwrap())
        }
        Err(Error::DescribeError(resp)) => {
            Err(format!("Error completing describe in Salesforce: {:?}", resp))
        }
        Err(Error::LoginError(resp)) => {
            Err(format!("Error logging in to Salesforce: {:?}", resp))
        }
    }
}


async fn connect_to_sf(client_id: String, client_secret: String, username: String, password: String) -> Result<Client, Error> {
    let mut client = Client::new(Some(client_id), Some(client_secret));
    if username.contains(".fullsb") {
        client.set_login_endpoint("https://myoutdeskllc--fullsb.my.salesforce.com");
    } else {
        client.set_login_endpoint("https://myoutdeskllc.my.salesforce.com");
    }

    match client.login_with_credential(username, password).await {
        Ok(_) => {
            println!("Logged in");
            Ok(client)
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        }
    }
}
