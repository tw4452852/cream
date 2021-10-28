use moon::*;
use shared::*;

async fn frontend() -> Frontend {
    Frontend::new().title("Caddy")
}

async fn get_resp(res: reqwest::Response) -> Result<String, String> {
    let status = res.status();
    let body = res.text().await.unwrap_or_else(|e| e.to_string());
    if status != reqwest::StatusCode::OK {
        Err(body)
    } else {
        Ok(body)
    }
}

async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    println!("{:?}", req);

    let UpMsgRequest {
        up_msg,
        cor_id,
        session_id,
        ..
    } = req;

    let down_msg = match up_msg {
        UpMsg::Connect(ip, port) => {
            match reqwest::get(format!("http://{}:{}/config/", ip, port)).await {
                Ok(resp) => match get_resp(resp).await {
                    Ok(config) => DownMsg::ConnectResult(Ok(config)),
                    Err(e) => DownMsg::ConnectResult(Err(e.to_string())),
                },
                Err(e) => DownMsg::ConnectResult(Err(e.to_string())),
            }
        }
        UpMsg::Config(url, method) => match method {
            Config::Get => match reqwest::get(url).await {
                Ok(resp) => match get_resp(resp).await {
                    Ok(config) => DownMsg::ConfigResult(Ok(config)),
                    Err(e) => DownMsg::ConfigResult(Err(e.to_string())),
                },
                Err(e) => DownMsg::ConfigResult(Err(e.to_string())),
            },
            Config::Del => match reqwest::Client::new().delete(url).send().await {
                Ok(resp) => match get_resp(resp).await {
                    Ok(config) => DownMsg::ConfigResult(Ok(config)),
                    Err(e) => DownMsg::ConfigResult(Err(e.to_string())),
                },
                Err(e) => DownMsg::ConfigResult(Err(e.to_string())),
            },
            Config::New(s) => match reqwest::Client::new()
                .put(url)
                .body(s)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .await
            {
                Ok(resp) => match get_resp(resp).await {
                    Ok(config) => DownMsg::ConfigResult(Ok(config)),
                    Err(e) => DownMsg::ConfigResult(Err(e.to_string())),
                },
                Err(e) => DownMsg::ConfigResult(Err(e.to_string())),
            },
            Config::SetOrReplace(s) => match reqwest::Client::new()
                .post(url)
                .body(s)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .await
            {
                Ok(resp) => match get_resp(resp).await {
                    Ok(config) => DownMsg::ConfigResult(Ok(config)),
                    Err(e) => DownMsg::ConfigResult(Err(e.to_string())),
                },
                Err(e) => DownMsg::ConfigResult(Err(e.to_string())),
            },
        },
    };

    if let Some(session) = sessions::by_session_id().wait_for(session_id).await {
        session.send_down_msg(&down_msg, cor_id).await;
    } else {
        eprintln!("cannot find the session with id `{}`", session_id);
    }
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
