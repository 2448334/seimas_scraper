use std::error::Error;
use std::io::Read;
use chrono::NaiveDateTime;
use log::{info, debug, error};
use xml::reader::{EventReader, XmlEvent};

use networking::request::*;


use crate::database::connect::establish_connection;
use crate::models::session::meeting_data::{AgendaItem, Vote, Speech, Registration, MeetingData};
use crate::parser::util::parse_attributes;
use crate::networking;



pub async fn get_meeting_data(meeting_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
    info!("Downloading meeting data: {}", meeting_id);    
    
    let link = format!("https://apps.lrs.lt/sip/p2b.ad_seimo_posedzio_eiga_full?posedzio_id={}", meeting_id);
    let xmlstring = url_request(&link).await?;
    parse_meeting_data(EventReader::from_str(&xmlstring));
    debug!("Done getting meeting data {}", meeting_id);
    Ok(())
}

fn parse_meeting_data<R: Read> (eventreader: EventReader<R>) {
    let conn = &mut establish_connection();
    let mut meeting_id: Option<i32> = None;

    let mut agenda: Vec<Option<i32>> = Vec::new();
    let mut registrations: Vec<Option<i32>> = Vec::new();

    let mut speeches: Vec<Option<i32>> = Vec::new();
    let mut voting: Vec<Option<i32>> = Vec::new();

    let mut current_element: Option<String> = None;
    let mut current_parent_element: Option<String> = None;

    let mut meeting_item: Option<MeetingData> = None;
    let mut agenda_item: Option<AgendaItem> = None;
    let mut vote_item: Option<Vote> = None;
    let mut speech_item: Option<Speech> = None;
    let mut registration_item: Option<Registration> = None;

    for e in eventreader {
        match e {
            Ok(XmlEvent::Characters(text)) => {                
                if let Some(cpe) = &current_parent_element {
                    match cpe.as_str() {
                        "meeting_item" => {
                            if let Some(ce) = &current_element {
                                match ce.as_str() {
                                    "pradzia" => {
                                        if let Some(unwrapped_item) = &mut meeting_item {
                                            unwrapped_item.from = NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S").ok();
                                        }
                                    }
                                    "pabaiga" => {
                                        if let Some(unwrapped_item) = &mut meeting_item {
                                            unwrapped_item.to = NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S").ok();
                                        }
                                    }
                                    _ => {
                                        //error!("Unrecognized meeting item: {}", text);
                                    }
                                }
                            }
                        },

                        "agenda_item" => {
                            if let Some(ce) = &current_element {
                                match ce.as_str() {
                                    "nr" => {
                                        if let Some(unwrapped_item) = &mut agenda_item {
                                            unwrapped_item.nr = Some(text);
                                        }
                                    }
                                    "pavadinimas" => {
                                        if let Some(unwrapped_item) = &mut agenda_item {
                                            unwrapped_item.name = Some(text);
                                        }
                                    }
                                    "stadija" => {
                                        if let Some(unwrapped_item) = &mut agenda_item {
                                            unwrapped_item.state = Some(text);
                                        }
                                    }
                                    "tipas" => {
                                        if let Some(unwrapped_item) = &mut agenda_item {
                                            unwrapped_item.agenda_type = Some(text);
                                        }
                                    }
                                    "nuo" => {
                                        if let Some(unwrapped_item) = &mut agenda_item {
                                            unwrapped_item.from = NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S").ok();
                                        }
                                    }
                                    "iki" => {
                                        if let Some(unwrapped_item) = &mut agenda_item {
                                            unwrapped_item.to = NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S").ok();
                                        }
                                    }
                                    _ => {
                                        error!("Unrecognized agenda item: {}", text);
                                    }
                                }
                            }
                        },

                        "vote_item" => {
                            if let Some(ce) = &current_element {
                                match ce.as_str() {
                                    "aprasas" => {
                                        if let Some(unwrapped_item) = &mut vote_item {
                                            unwrapped_item.summary = Some(text);
                                        }
                                    }
                                    "antraste" => {
                                        if let Some(unwrapped_item) = &mut vote_item {
                                            unwrapped_item.result = Some(text);
                                        }
                                    }
                                    "nuo" => {
                                        if let Some(unwrapped_item) = &mut vote_item {
                                            unwrapped_item.from = NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S").ok();
                                        }
                                    }
                                    "iki" => {
                                        if let Some(unwrapped_item) = &mut vote_item {
                                            unwrapped_item.to = NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S").ok();
                                        }
                                    }
                                    _ => {
                                        error!("Unrecognized vote item: {}", text);
                                    }
                                }
                            }
                        },

                        "speech_item" => {
                            if let Some(ce) = &current_element {
                                match ce.as_str() {
                                    "asmuo" => {
                                        if let Some(unwrapped_item) = &mut speech_item {
                                            unwrapped_item.person = Some(text);
                                        }
                                    }
                                    "pareigos" => {
                                        if let Some(unwrapped_item) = &mut speech_item {
                                            unwrapped_item.office = Some(text);
                                        }
                                    }
                                    "nuo" => {
                                        if let Some(unwrapped_item) = &mut speech_item {
                                            unwrapped_item.from = NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S").ok();
                                        }
                                    }
                                    "iki" => {
                                        if let Some(unwrapped_item) = &mut speech_item {
                                            unwrapped_item.to = NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S").ok();
                                        }
                                    }
                                    _ => {
                                        error!("Unrecognized speech item: {}", text);
                                    }
                                }
                            }
                        }

                        "registration_item" => {
                            if let Some(ce) = &current_element {
                                match ce.as_str() {
                                    "antraste" => {
                                        if let Some(unwrapped_item) = &mut registration_item {
                                            unwrapped_item.result = Some(text);
                                        }
                                    }
                                    "nuo" => {
                                        if let Some(unwrapped_item) = &mut registration_item {
                                            unwrapped_item.from = NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S").ok();
                                        }
                                    }
                                    "iki" => {
                                        if let Some(unwrapped_item) = &mut registration_item {
                                            unwrapped_item.to = NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S").ok();
                                        }
                                    }
                                    _ => {
                                        error!("Unrecognized vote item: {}", text);
                                    }
                                }
                            }
                        },

                        _ => {}
                    }
                }
            }
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "posedis" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        meeting_id = keyvaluepairs["pos_id"].parse().ok();
                        meeting_item = Some(MeetingData {
                            id: meeting_id.unwrap(),
                            from: None,
                            to: None,
                            agenda: vec![],
                            registrations: vec![],
                        });
                        current_parent_element = Some("meeting_item".to_owned());
                    },
                    "darbotvarkes-klausimas" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        let id: i32 = keyvaluepairs["svarst_kl_stad_id"].parse().unwrap();
                        let agenda_state_id: Option<i32> = keyvaluepairs
                            .get("kl_stad_id").and_then(|x| x.parse().ok());
                        let agenda_group_id: Option<i32> = keyvaluepairs
                            .get("kl_gr_id").and_then(|x| x.parse().ok());
                        let document_key: Option<i32> = keyvaluepairs
                            .get("dok_key").and_then(|x| x.parse().ok());
                        agenda_item = Some(AgendaItem {
                            id,
                            agenda_state_id,
                            agenda_group_id,
                            document_key,
                            nr: None,
                            name: None,
                            state: None,
                            agenda_type:None,
                            from: None,
                            to: None,
                            speeches: vec![],
                            voting: vec![],
                        });
                        current_parent_element = Some("agenda_item".to_owned());
                        agenda.push(Some(id));
                    },
                    "balsavimas" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        let id: i32 = keyvaluepairs["bals_id"].parse().unwrap();
                        vote_item = Some(Vote {
                            id,
                            summary: None,
                            result: None,
                            from: None,
                            to: None,
                        });
                        current_parent_element = Some("vote_item".to_owned());
                        voting.push(Some(id));
                    },
                    "kalbetojas" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        let id: i32 = keyvaluepairs["klb_id"].parse().unwrap();
                        let discussion_id: Option<i32> = keyvaluepairs["diskus_id"].parse().ok();

                        let person_id: Option<i32> = if keyvaluepairs.contains_key("asm_id") {
                            keyvaluepairs["asm_id"].parse().ok()
                        } else {
                            keyvaluepairs["pran_id"].parse().ok()
                        };
                        
                        speech_item = Some(Speech {
                            id,
                            discussion_id,
                            person_id,
                            person: None,
                            office: None,
                            from: None,
                            to: None,
                        });
                        current_parent_element = Some("speech_item".to_owned());
                        speeches.push(Some(id));
                    },
                    "registracija" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        let id: i32 = keyvaluepairs["reg_id"].parse().unwrap();

                        registration_item = Some(Registration {
                            id,
                            result: None,                            
                            from: None,
                            to: None,
                        });
                        current_parent_element = Some("registration_item".to_owned());
                        registrations.push(Some(id));
                    },
                    _ => {

                    }
                }
                current_element = Some(name.local_name);
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    "posedis" => {
                        if let Some(unwrapped_item) = &mut meeting_item {
                            unwrapped_item.agenda = agenda;
                            unwrapped_item.registrations = registrations;
                            unwrapped_item.save(conn).unwrap();
                        }
                        
                        registrations = vec![];
                        agenda = vec![];
                    },
                    "darbotvarkes-klausimas" => {
                        if let Some(unwrapped_item) = &mut agenda_item {
                            unwrapped_item.speeches = speeches;
                            unwrapped_item.voting = voting;
                            unwrapped_item.save(conn).unwrap();
                        }
                        speeches = vec![];
                        voting = vec![];
                    },
                    "balsavimas" => {
                        if let Some(unwrapped_item) = &mut vote_item {
                            unwrapped_item.save(conn).unwrap();
                        }
                    },
                    "kalbetojas" => {
                        if let Some(unwrapped_item) = &mut speech_item {
                            unwrapped_item.save(conn).unwrap();
                        }
                    },
                    "registracija" => {
                        if let Some(unwrapped_item) = &mut registration_item {
                            unwrapped_item.save(conn).unwrap();
                        }
                    },
                    _ => {

                    }
                }
                current_element = None;
            }
            Err(e) => {
                error!("{:?}", e);
                break;
            }
            _ => {}
        }
    }
}
