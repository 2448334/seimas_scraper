use std::error::Error;
use std::io::Read;
use chrono::NaiveDate;
use log::{error, info};
use xml::reader::{EventReader, XmlEvent};

use networking::request::*;
use models::session::sessions::Sessions;

use crate::database::connect::establish_connection;
use crate::parser::util::parse_attributes;
use crate::{networking, models};



pub async fn get_sessions(parliament_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
    info!("Downloading sessions: {}", parliament_id);
    
    let link = format!("https://apps.lrs.lt/sip/p2b.ad_seimo_sesijos?kadencijos_id={}", parliament_id);

    let xmlstring = url_request(&link).await?;
    parse_sessions(EventReader::from_str(&xmlstring));
    Ok(())
}


fn parse_sessions<R: Read> (eventreader: EventReader<R>) {
    let conn = &mut establish_connection();
    let mut parliament_id: Option<i32> = None;

    for e in eventreader {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "SeimoKadencija" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        parliament_id = keyvaluepairs["kadencijos_id"].parse().ok();
                    },
                    "SeimoSesija" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        let session_id: i32 = keyvaluepairs["sesijos_id"].parse().unwrap();
                        let session_num: i32 = keyvaluepairs["numeris"].parse().unwrap();
                        let session_name: String = keyvaluepairs["pavadinimas"].to_owned();
                        let session_from: Option<NaiveDate> = NaiveDate::parse_from_str(&keyvaluepairs["data_nuo"], "%Y-%m-%d").ok();
                        let session_to: Option<NaiveDate> = NaiveDate::parse_from_str(&keyvaluepairs["data_iki"], "%Y-%m-%d").ok();
                        
                        let session = Sessions {
                            id: session_id,
                            num: session_num,
                            name: session_name,
                            from: session_from,
                            to: session_to,
                            parliament: parliament_id.unwrap(),
                        };
                        
                        session.save(conn).unwrap();
                    }
                    _ => {

                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    "SeimoKadencija" => {
                        parliament_id = None;
                    },
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
