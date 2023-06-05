use crate::data::transfer::Transfer;
use crate::components::transfer_menu::num_to_letters;
use crate::components::states::MainState;

use std::{error::Error};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct TransferRecord {
    #[serde(rename = "Source Plate Barcode")]
    source_plate: String,
    #[serde(rename = "Source Well")]
    source_well: String,
    #[serde(rename = "Destination Plate Barcode")]
    destination_plate: String,
    #[serde(rename = "Destination Well")]
    destination_well: String,
    #[serde(rename = "Volume")]
    volume: f64,
    #[serde(rename = "Concentration")]
    concentration: Option<f64>,
}

pub fn state_to_csv(state: &MainState) -> Result<String, Box<dyn Error>> {
    let mut records: Vec<TransferRecord> = Vec::new();
    for transfer in &state.transfers {
        let src_barcode = state.source_plates.iter().find(|spi| spi.get_uuid() == transfer.source_id)
                                             .ok_or("Found unpurged transfer")?;
        let dest_barcode = state.destination_plates.iter().find(|dpi| dpi.get_uuid() == transfer.dest_id)
                                             .ok_or("Found unpurged transfer")?;
        records.append(&mut transfer_to_records(transfer, &src_barcode.name, &dest_barcode.name))
    }
    return records_to_csv(records)
}

fn transfer_to_records(
    tr: &Transfer,
    src_barcode: &str,
    dest_barcode: &str,
) -> Vec<TransferRecord> {
    let source_wells = tr.transfer_region.get_source_wells();
    let map = tr.transfer_region.calculate_map();

    let mut records: Vec<TransferRecord> = vec![];

    for s_well in source_wells {
        let dest_wells = map(s_well);
        if let Some(dest_wells) = dest_wells {
            for d_well in dest_wells {
                records.push(TransferRecord { 
                    source_plate: src_barcode.to_string(),
                    source_well: format!("{}{}", num_to_letters(s_well.0).unwrap(), s_well.1),
                    destination_plate: dest_barcode.to_string(),
                    destination_well: format!("{}{}", num_to_letters(d_well.0).unwrap(), d_well.1),
                    volume: 2.5, // Default value since not yet implemented
                    concentration: None })
            }
        }
    }
    return records
}

fn records_to_csv(trs: Vec<TransferRecord>) -> Result<String, Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new().from_writer(vec![]);
    for record in trs {
        wtr.serialize(record)?
    }
    let data = String::from_utf8(wtr.into_inner()?)?;
    return Ok(data)
}
