use anyhow::Result;
use pdtdb::{utils::ProcessCoordinates, zqproj::ZilliqaDBProject};
use pdtlib::exporter::Exporter;

use crate::importer::Importer;

/// Returns true if we're done, and false if we're not
pub async fn import<T: Importer, P: ZilliqaDBProject + std::marker::Sync>(
    imp: &mut T,
    exporter: &Exporter,
    project: &P,
    in_coords: &ProcessCoordinates,
    nr_batches: Option<i64>,
    start_block: Option<i64>,
) -> Result<bool> {
    let client_id = format!(
        "{}{}_{}",
        imp.get_id(),
        in_coords.machine_id,
        in_coords.nr_machines
    );
    imp.set_client_id(&client_id);

    let coords = in_coords.with_client_id(&client_id);
    println!("{:?}", coords);
    let mut batch = 0;
    let mut last_max = start_block.unwrap_or_default();
    while match nr_batches {
        None => true,
        Some(val) => batch < val,
    } {
        println!("{}: requesting a block .. ", coords.client_id);
        let maybe_range = imp.maybe_range(project, last_max).await?;
        match maybe_range {
            None => {
                println!("{}: range fetched. All done.", coords.client_id);
                return Ok(true);
            }
            Some(range) => {
                // Curses! Work to do..
                println!("{}: work to do at {:?}", coords.client_id, range);
                imp.extract_start(project, exporter).await?;
                imp.extract_range(project, exporter, &range).await?;
                println!("{}: inserting records.. ", coords.client_id);
                imp.extract_done(project, exporter).await?;
                last_max = range.end;
            }
        }
        batch += 1;
    }
    Ok(false)
}
