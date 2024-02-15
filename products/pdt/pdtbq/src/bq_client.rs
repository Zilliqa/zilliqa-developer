use anyhow::{anyhow, Result};
use gcp_bigquery_client::Client;
use std::{env, fmt};

#[derive(Clone, Debug, PartialEq)]
pub enum DefaultCredentialSource {
    EnvVar,
    DefaultSecretsFile,
    MetadataServer,
}

pub struct DefaultCredentials {
    // The client we created
    pub client: Client,
    pub source: DefaultCredentialSource,
    pub file_name: Option<String>,
}

impl fmt::Debug for DefaultCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "DefaultCredentials[ type = {0:?} ", self.source)?;
        if let Some(the_fn) = &self.file_name {
            write!(f, "file_name = {0}", the_fn)?;
        }
        write!(f, "]")
    }
}

/// Apply google's rules to generate a bigquery client:
///
///  - if GOOGLE_APPLICATION_CREDENTIALS is defined, try that first.
///  - otherwise, have a go with the default gcloud config file (~/.config/gcloud/application_default_credentials.json)
///  - if that fails, try the metadata server.
pub async fn client_from_default_credentials() -> Result<DefaultCredentials> {
    async fn try_credentials(where_from: &str) -> Result<Client> {
        Ok(Client::from_authorized_user_secret(where_from).await?)
    }
    if let Ok(val) = env::var("GOOGLE_APPLICATION_CREDENTIALS") {
        if let Ok(result) = try_credentials(&val).await {
            return Ok(DefaultCredentials {
                client: result,
                source: DefaultCredentialSource::EnvVar,
                file_name: Some(val),
            });
        }
    }
    // If we can't find your home directory, we will simply continue ..
    if let Some(home_dir) = home::home_dir() {
        let home_as_string = home_dir.into_os_string().into_string().or(Err(anyhow!(
            "Your home directory could not be represented as a string"
        )))?;
        let default_file = format!(
            "{0}/.config/gcloud/application_default_credentials.json",
            home_as_string
        );
        if let Ok(result) = try_credentials(&default_file).await {
            return Ok(DefaultCredentials {
                client: result,
                source: DefaultCredentialSource::DefaultSecretsFile,
                file_name: Some(default_file.to_string()),
            });
        }
    }
    // OK. We got here ..
    let result = Client::from_application_default_credentials().await?;
    Ok(DefaultCredentials {
        client: result,
        source: DefaultCredentialSource::MetadataServer,
        file_name: None,
    })
}
