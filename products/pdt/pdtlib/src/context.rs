use anyhow::{anyhow, Result};
/** Context for downloads
 */
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::operation::get_object::GetObjectOutput;
use aws_sdk_s3::operation::get_object_attributes::GetObjectAttributesOutput;
use aws_sdk_s3::{config::Region, Client};

// Locations in the bucket.

pub const PERSISTENCE_SNAPSHOT_NAME: &str = "blockchain-data";
pub const INCREMENTAL_NAME: &str = "incremental";
pub const STATEDELTA_NAME: &str = "statedelta";

pub struct Context {
    /// The S3 client to fetch with.
    pub client: Client,

    /// The bucket name
    pub bucket_name: String,

    /// Network name
    pub network_name: String,

    /// Where we download persistence to.
    pub target_path: String,
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub key: String,
    pub e_tag: Option<String>,
    pub size: i64,
}

impl Context {
    pub async fn duplicate(old: &Context) -> Result<Self> {
        Context::new(&old.bucket_name, &old.network_name, &old.target_path).await
    }
    pub async fn new(bucket_name: &str, network_name: &str, target_path: &str) -> Result<Self> {
        let region_provider = RegionProviderChain::first_try(Region::new("us-west-2")); // FIXME
        let config_loader = aws_config::from_env()
            .no_credentials()
            .region(region_provider);
        // Temporary whilst migrations are being carried out
        let config_loader = if network_name.starts_with("testnet-") {
            config_loader.endpoint_url("https://storage.googleapis.com")
        } else {
            config_loader
        };

        let config = config_loader.load().await;

        let client = aws_sdk_s3::Client::new(&config);
        Ok(Context {
            client,
            bucket_name: bucket_name.to_string(),
            network_name: network_name.to_string(),
            target_path: target_path.to_string(),
        })
    }

    /// Because of the way we do our bucket permissions, this needs to list the
    /// object and then assert there is only one of it.
    pub async fn maybe_list_object(&self, key: &str) -> Result<Option<Entry>> {
        let listing = self.list_objects(key).await?;
        match listing.len() {
            0 => Ok(None),
            1 => Ok(Some(listing[0].clone())),
            _ => Err(anyhow!(
                "More than one possibility ({}) for key {}",
                listing.len(),
                key
            )),
        }
    }

    pub async fn list_object(&self, key: &str) -> Result<Entry> {
        self.maybe_list_object(key)
            .await?
            .ok_or(anyhow!("No object for key {}", key))
    }

    pub async fn list_objects(&self, prefix: &str) -> Result<Vec<Entry>> {
        const ENTRIES_PER_REQUEST: i32 = std::i32::MAX;
        let mut result = Vec::new();
        // Because AWS, you can't seem to get more than 1000 keys in a list, so
        // tediously continue it until it's finished.
        let mut token: Option<String> = None;
        let caller = self
            .client
            .list_objects_v2()
            .bucket(self.bucket_name.clone())
            .prefix(prefix.to_string())
            .max_keys(ENTRIES_PER_REQUEST);

        loop {
            let caller = caller.clone().set_continuation_token(token);
            let res = caller.send().await?;
            println!(
                " {} objects with trunc {} next {:?}",
                res.key_count, res.is_truncated, res.next_continuation_token
            );

            if let Some(objects) = res.contents {
                let valid_entries = objects.iter().filter_map(|object| {
                    // Getting objects with valid keys
                    object.key().map(|key| Entry {
                        key: key.to_string(),
                        e_tag: object.e_tag().map(|x| x.to_string()),
                        size: object.size(),
                    })
                });

                result.extend(valid_entries);
            }

            if res.is_truncated {
                // Need to go around again.
                token = res.next_continuation_token.clone();
            } else {
                break;
            }
        }
        Ok(result)
    }

    pub async fn get_attributes(&self, object_key: &str) -> Result<GetObjectAttributesOutput> {
        let res = self
            .client
            .get_object_attributes()
            .bucket(self.bucket_name.clone())
            .key(object_key)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn get_object(&self, object_key: &str) -> Result<GetObjectOutput> {
        let res = self
            .client
            .get_object()
            .bucket(self.bucket_name.clone())
            .key(object_key)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn get_key_as_string(&self, object_key: &str) -> Result<String> {
        let the_object = self.get_object(object_key).await?;
        let the_bytes = the_object.body.collect().await?.into_bytes();
        Ok(std::str::from_utf8(&the_bytes)?.to_string())
    }

    pub async fn get_byte_range(
        &self,
        object_key: &str,
        offset: i64,
        nr_bytes: i64,
    ) -> Result<GetObjectOutput> {
        let res = self
            .client
            .get_object()
            .bucket(self.bucket_name.clone())
            .key(object_key)
            .range(format!("bytes={}-{}", offset, offset + nr_bytes - 1))
            .send()
            .await?;
        Ok(res)
    }
}
