use std::error::Error;

use log::{error, debug};
use models::session::{meetings::Meetings, sessions::Sessions};
use networking::download::{get_protocol_document, get_stenogram_document};
use parser::session::meetings::get_meetings;

use crate::{models::{self, session::{meeting_data::{Vote, Registration}, parliament::Parliament, voting_data::VoteData, registration_data::RegistrationData}}, networking, parser::{self, politicians::politician, session::{meeting_data, voting_data, registration_data, parliaments, sessions, meetings}}};

#[macro_export]
macro_rules! asyncrun {
    ( $functions:expr,$task_count:expr ) => {
        {

            while $functions.len() > 0 {
                let mut download_futures = Vec::new();
                for _ in 0..$task_count {
                    match $functions.pop() {
                        Some(function) => download_futures.push(tokio::spawn(function)),
                        None => break,
                    }
                }
                
                for download_future in download_futures {
                    match download_future.await {
                        Ok(_) => {}
                        Err(error) => {
                            error!("{:?}", error);
                        }
                    }
                }
            }
        }
    };
}


pub async fn download_parliaments() -> Result<(), Box<dyn Error + Send + Sync>> {
    debug!("Downloading parliaments...");
    parliaments::get_parliaments().await
}

pub async fn download_politicians() -> Result<(), Box<dyn Error + Send + Sync>> {
    debug!("Downloading politicians...");
    let mut functions = Vec::new();

    for parliament_id in Parliament::get_parliaments_ids()? {
        functions.push(politician::get_politicians(parliament_id));
    }

    asyncrun!(functions, 16);

    Ok(())
}

pub async fn download_meetings_documents(parliament_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
    debug!("Downloading meetings documents parliament {}...", parliament_id);
    let mut functions = Vec::new();
    let mut functions2 = Vec::new();

    for session in Sessions::get_sessions_per_parliament(parliament_id)? {
        if Meetings::get_session_count(session)? < 1 {
            get_meetings(session).await?;
        }
        for protocol in Meetings::get_protocols_per_session(session)? {
            if let (session_id, meeting_num, Some(protocol_url_unwrapped)) = protocol {
                functions.push(get_protocol_document(protocol_url_unwrapped, session_id, meeting_num));
            }
        }
        for stenogram in Meetings::get_stenograms_per_session(session)? {
            if let (session_id, meeting_num, Some(stenogram_url_unwrapped)) = stenogram {
                functions2.push(get_stenogram_document(stenogram_url_unwrapped, session_id, meeting_num));
            }
        }
    }

    asyncrun!(functions, 8);
    asyncrun!(functions2, 8);

    Ok(())
}

pub async fn download_all_meetings_documents() -> Result<(), Box<dyn Error + Send + Sync>> {
    debug!("Downloading all meetings documents...");

    for parliament_id in Parliament::get_parliaments_ids()? {
        download_meetings_documents(parliament_id).await?;
    }

    Ok(())
}

pub async fn download_sessions() -> Result<(), Box<dyn Error + Send + Sync>> {
    debug!("Downloading sessions...");
    let mut functions = Vec::new();

    for parliament_id in Parliament::get_parliaments_ids()? {
        functions.push(sessions::get_sessions(parliament_id));
    }
    
    asyncrun!(functions, 16);

    Ok(())
}

pub async fn download_meetings() -> Result<(), Box<dyn Error + Send + Sync>> {
    debug!("Downloading meetings...");
    let mut functions = Vec::new();

    for parliament_id in Parliament::get_parliaments_ids()? {
        for session in  Sessions::get_sessions_per_parliament(parliament_id)? {
            functions.push(meetings::get_meetings(session));
        }
    }
    
    asyncrun!(functions, 16);

    Ok(())
}


pub async fn download_meeting_data() -> Result<(), Box<dyn Error + Send + Sync>> {
    debug!("Downloading meeting data...");
    let mut functions = Vec::new();

    for parliament_id in Parliament::get_parliaments_ids()? {
        for session in  Sessions::get_sessions_per_parliament(parliament_id)? {
            for meeting_id in Meetings::get_meetings_ids(session)? {
                functions.push(meeting_data::get_meeting_data(meeting_id));
            }
        }
    }
    
    asyncrun!(functions, 16);

    Ok(())
}

pub async fn download_voting_data() -> Result<(), Box<dyn Error + Send + Sync>> {
    debug!("Downloading voting data...");
    let mut functions = Vec::new();

    for vote_id in Vote::get_vote_ids()? {
        functions.push(voting_data::get_voting_data(vote_id));
    }

    asyncrun!(functions, 16);

    Ok(())
}

pub async fn download_registration_data() -> Result<(), Box<dyn Error + Send + Sync>> {
    debug!("Downloading registration data...");
    let mut functions = Vec::new();

    for registration_id in Registration::get_registration_ids()? {
        functions.push(registration_data::get_registration_data(registration_id));
    }

    asyncrun!(functions, 16);

    Ok(())
}

pub async fn download_all_parliament(parliament_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
    debug!("Downloading sessions...");
    sessions::get_sessions(parliament_id).await?;

    debug!("Downloading meetings...");
    let mut functions = Vec::new();

    for session in  Sessions::get_sessions_per_parliament(parliament_id)? {
        functions.push(meetings::get_meetings(session));
    }
    
    asyncrun!(functions, 16);

    debug!("Downloading meeting data...");
    let mut functions = Vec::new();

    for meeting_id in Meetings::get_missing_meeting_ids()? {
        functions.push(meeting_data::get_meeting_data(meeting_id));
    }
    
    asyncrun!(functions, 16);

    debug!("Downloading voting data...");
    let mut functions = Vec::new();

    for vote_id in VoteData::get_missing_vote_ids()? {
        functions.push(voting_data::get_voting_data(vote_id));
    }

    asyncrun!(functions, 16);

    debug!("Downloading registration data...");
    let mut functions = Vec::new();

    for registration_id in RegistrationData::get_missing_registration_ids()? {
        functions.push(registration_data::get_registration_data(registration_id));
    }

    asyncrun!(functions, 16);

    download_meetings_documents(parliament_id).await?;

    Ok(())
}

pub async fn download_all() -> Result<(), Box<dyn Error + Send + Sync>> {
    download_parliaments().await?;
    download_politicians().await?;
    download_sessions().await?;
    download_meetings().await?;
    download_meeting_data().await?;
    download_voting_data().await?;
    download_registration_data().await?;
    download_all_meetings_documents().await?;
    Ok(())
}
