use std::error::Error;
use std::io::Read;
use chrono::NaiveDateTime;
use log::info;
use xml::reader::{EventReader, XmlEvent};

use networking::request::*;
use models::session::meetings::Meetings;

use crate::database::connect::establish_connection;
use crate::parser::util::parse_attributes;

use crate::{networking, models};



pub async fn get_meetings(session_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
    info!("Downloading meetings: {}", session_id);
    
    let link = format!("https://apps.lrs.lt/sip/p2b.ad_seimo_posedziai?sesijos_id={}", session_id);
    let xmlstring = url_request(&link).await?;
    parse_meetings(EventReader::from_str(&xmlstring));
    Ok(())
}



fn parse_meetings<R: Read> (eventreader: EventReader<R>) {
    let conn = &mut establish_connection();
    let mut session_id: Option<i32> = None;
    let mut meeting: Option<Meetings> = None;
    for e in eventreader {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "SeimoPosėdis" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        let meeting_id: i32 = keyvaluepairs["posėdžio_id"].parse().unwrap();
                        let meeting_num: i32 = keyvaluepairs["numeris"].parse().unwrap();
                        let meeting_type: String = keyvaluepairs["tipas"].to_owned();
                        let meeting_from: Option<NaiveDateTime> = NaiveDateTime::parse_from_str(&keyvaluepairs["pradžia"], "%Y-%m-%d %H:%M").ok();
                        let meeting_to: Option<NaiveDateTime> = NaiveDateTime::parse_from_str(&keyvaluepairs["pabaiga"], "%Y-%m-%d %H:%M").ok();

                        meeting = Some(Meetings{ 
                            id: meeting_id,
                            num: meeting_num,
                            meeting_type: meeting_type,
                            from: meeting_from,
                            to: meeting_to,
                            session: session_id.unwrap(),
                            protocol_link: None,
                            stenogram_link: None,
                            video_comment: None,
                            video_link: None,
                        });
                    },
                    "SeimoSesija" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        session_id = keyvaluepairs["sesijos_id"].parse().ok();
                    }
                    "Protokolas" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        if let Some(unwrapped_meeting) = &mut meeting {
                            unwrapped_meeting.protocol_link = keyvaluepairs["protokolo_nuoroda"].parse().ok();
                        }
                    }
                    "Stenograma" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        if let Some(unwrapped_meeting) = &mut meeting {
                            unwrapped_meeting.stenogram_link = keyvaluepairs["stenogramos_nuoroda"].parse().ok();
                        }
                    }
                    "VaizdoĮrašas" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        if let Some(unwrapped_meeting) = &mut meeting {
                            unwrapped_meeting.video_comment = keyvaluepairs["komentaras"].parse().ok();
                            unwrapped_meeting.video_link = keyvaluepairs["vaizdo_įrašo_nuoroda"].parse().ok();
                        }
                    }
                    _ => {

                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    "SeimoSesija" => {
                        session_id = None;
                    },
                    "SeimoPosėdis" => {
                        if let Some(unwrapped_meeting) = &mut meeting {
                            unwrapped_meeting.save(conn).unwrap();
                        }
                        meeting = None;
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
