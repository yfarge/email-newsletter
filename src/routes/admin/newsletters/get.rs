use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn newsletter_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut message_html = String::new();
    for m in flash_messages.iter() {
        writeln!(message_html, "<p><i>{}</i></p>", m.content()).unwrap()
    }

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!doctype html>
            <html>
                <head>
                    <title>Send a newsletter</title>
                    <meta http-equiv="content-type" content="text/html; charset=utf-8" />
                </head>
                <body>
                    {message_html}
                    <form action="/admin/newsletters" method="POST">
                        <label>Title
                            <input type="text" placeholder="Enter title" name="title" />
                        </label>
                        <br />
                        <label>Plain Text
                            <textarea placeholder="Enter plain text" name="text"></textarea>
                        </label>
                        <br />
                        <label>Html Text
                            <textarea placeholder="Enter html" name="html"></textarea>
                        </label>
                        <br />
                        <button type="submit">Submit</button>
                    </form>
                </body>
            </html>
        "#,
        ))
}
