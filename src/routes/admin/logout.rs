use crate::{session_state::TypedSession, utils::see_other};
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    session.logout();
    FlashMessage::info("You have successfully logged out.").send();
    Ok(see_other("/login"))
}
