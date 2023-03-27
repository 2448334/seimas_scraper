use std::error::Error;
use std::io::Read;
use log::{error, info};
use xml::reader::{EventReader, XmlEvent};

use networking::request::*;

use crate::database::connect::establish_connection;
use crate::models::session::voting_data::{VoteData, VoteType};
use crate::parser::util::parse_attributes;

use crate::networking;



pub async fn get_voting_data(voting_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
    info!("Downloading voting data: {}", voting_id);
    
    let link = format!("https://apps.lrs.lt/sip/p2b.ad_sp_balsavimo_rezultatai?balsavimo_id={}", voting_id);
    let xmlstring = url_request(&link).await?;
    parse_voting_data(EventReader::from_str(&xmlstring));
    Ok(())
}



fn parse_voting_data<R: Read> (eventreader: EventReader<R>) {
    let conn = &mut establish_connection();
    let mut voting_id: Option<i32> = None;

    for e in eventreader {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "SeimoNariųBalsavimas" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        voting_id = keyvaluepairs["balsavimo_id"].parse().ok();
                    }
                    "IndividualusBalsavimoRezultatas" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        let person_id: i32 = keyvaluepairs["asmens_id"].parse().unwrap();
                        let vote: Option<VoteType> = match keyvaluepairs["kaip_balsavo"].as_str() {
                            "Už" => {
                                Some(VoteType::For)
                            },
                            "Prieš" => {
                                Some(VoteType::Against)
                            },
                            "Susilaikė" => {
                                Some(VoteType::Abstain)
                            },
                            _ => {
                                None
                            }
                        };

                        let voting_data = VoteData{ 
                            id: voting_id.unwrap(),
                            person_id: person_id,
                            vote: vote,
                        };
                        voting_data.save(conn).unwrap();
                    }
                    _ => {

                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    "SeimoNariųBalsavimas" => {
                        voting_id = None;
                    },
                    _ => {

                    }
                }
            }
            Err(e) => {
                error!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
