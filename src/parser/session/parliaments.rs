use std::error::Error;
use std::io::Read;
use chrono::NaiveDate;
use log::{error, info};
use xml::reader::{EventReader, XmlEvent};

use networking::request::*;
use models::session::{parliament::Parliament};

use crate::database::connect::establish_connection;
use crate::parser::util::parse_attributes;
use crate::{networking, models};



pub async fn get_parliaments() -> Result<(), Box<dyn Error + Send + Sync>> {
    info!("Downloading parliaments");
    let link = format!("http://apps.lrs.lt/sip/p2b.ad_seimo_kadencijos");

    let xmlstring = url_request(&link).await?;
    parse_parliaments(EventReader::from_str(&xmlstring));
    Ok(())
}

fn parse_parliaments<R: Read> (eventreader: EventReader<R>) {
    let conn = &mut establish_connection();
    for e in eventreader {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "SeimoKadencija" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        let id: i32 = keyvaluepairs["kadencijos_id"].parse().unwrap();
                        let name: Option<String> = keyvaluepairs["pavadinimas"].parse().ok();
                        let from: Option<NaiveDate> = NaiveDate::parse_from_str(&keyvaluepairs["data_nuo"], "%Y-%m-%d").ok();
                        let to: Option<NaiveDate> = NaiveDate::parse_from_str(&keyvaluepairs["data_iki"], "%Y-%m-%d").ok();

                        let parliament = Parliament {
                            id,
                            name,
                            from,
                            to,
                        };
                        parliament.save(conn).unwrap();
                    },
                    _ => {

                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    _ => {

                    }
                }
            }
            Err(e) => {
                error!("{:?}", e);
            }
            _ => {}
        }
    }
}
