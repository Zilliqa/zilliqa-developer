// Here
use gcp_bigquery_client::model::table::Table;
use gcp_bigquery_client::Client;

#[derive(Clone, Debug)]
pub struct BigQueryDatasetLocation {
    pub project_id: String,
    pub dataset_id: String,
}

impl BigQueryDatasetLocation {
    pub fn get_dataset_desc(&self) -> String {
        format!("{}.{}", self.project_id, self.dataset_id)
    }

    pub fn with_table_id(&self, table_id: &str) -> BigQueryTableLocation {
        BigQueryTableLocation {
            dataset: self.clone(),
            table_id: table_id.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BigQueryTableLocation {
    pub dataset: BigQueryDatasetLocation,
    pub table_id: String,
}

impl BigQueryTableLocation {
    pub fn new(bq: &BigQueryDatasetLocation, table_id: &str) -> Self {
        BigQueryTableLocation {
            dataset: bq.clone(),
            table_id: table_id.to_string(),
        }
    }

    pub fn to_meta(&self) -> BigQueryTableLocation {
        BigQueryTableLocation {
            dataset: self.dataset.clone(),
            table_id: format!("{}_meta", self.table_id),
        }
    }

    pub fn get_table_desc(&self) -> String {
        format!("{}.{}", self.dataset.get_dataset_desc(), self.table_id)
    }

    pub async fn find_table(&self, client: &Client) -> Option<Table> {
        client
            .table()
            .get(
                &self.dataset.project_id,
                &self.dataset.dataset_id,
                &self.table_id,
                Option::None,
            )
            .await
            .ok()
    }
}
