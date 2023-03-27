use std::error::Error;
use std::io::Read;
use log::info;
use xml::reader::{EventReader, XmlEvent};

use networking::request::*;

use crate::database::connect::establish_connection;
use crate::models::session::registration_data::RegistrationData;
use crate::parser::util::parse_attributes;

use crate::networking;



pub async fn get_registration_data(registration_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
    info!("Downloading registration data: {}", registration_id);
    
    let link = format!("https://apps.lrs.lt/sip/p2b.ad_sp_registracijos_rezultatai?registracijos_id={}", registration_id);
    let xmlstring = url_request(&link).await?;
    parse_registration_data(EventReader::from_str(&xmlstring));
    Ok(())
}



fn parse_registration_data<R: Read> (eventreader: EventReader<R>) {
    let conn = &mut establish_connection();
    let mut registration_id: Option<i32> = None;

    for e in eventreader {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "SeimoNariųRegistracija" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        registration_id = keyvaluepairs["registracijos_id"].parse().ok();
                    }
                    "IndividualusRegistracijosRezultatas" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        let person_id: i32 = keyvaluepairs["asmens_id"].parse().unwrap();
                        let registered: Option<bool> = match keyvaluepairs["ar_registravosi"].as_str() {
                            "Ne" => {
                                Some(false)
                            },
                            "Taip" => {
                                Some(true)
                            },
                            _ => {
                                None
                            }
                        };

                        let registration_data = RegistrationData{ 
                            id: registration_id.unwrap(),
                            person_id: person_id,
                            registered: registered,
                        };
                        registration_data.save(conn).unwrap();
                    }
                    _ => {

                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    "SeimoNariųRegistracija" => {
                        registration_id = None;
                    },
                    _ => {

                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
