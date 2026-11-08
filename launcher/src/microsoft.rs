//! Microsoft sign-in for the launcher.
//!
//! The flow is the one Microsoft documents for public clients: device code, Xbox Live, XSTS,
//! Minecraft services, ownership and profile. Tokens are kept in the launcher's keychain store
//! under `online:<uuid>`; `accounts.json` keeps only the public profile.
//!
//! A build needs Scoria's own Azure app registration. `SCORIA_MSA_CLIENT_ID` at build time, or
//! at run time to override it, is that client ID. Without one, sign-in is unavailable and
//! offline play keeps working.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

use crate::account;

/// The keychain key prefix for one online account's secrets.
const SECRET_PREFIX: &str = "online:";
const SCOPE: &str = "XboxLive.signin offline_access";
pub const NOT_CONFIGURED: &str =
    "This build has no Microsoft sign-in (SCORIA_MSA_CLIENT_ID). Offline play still works.";

/// The configured Azure app, if any: run time wins over build time, empty means none.
pub fn client_id() -> Option<String> {
    std::env::var("SCORIA_MSA_CLIENT_ID")
        .ok()
        .or_else(|| option_env!("SCORIA_MSA_CLIENT_ID").map(str::to_owned))
        .map(|id| id.trim().to_owned())
        .filter(|id| !id.is_empty())
}

/// Where requests go. Tests and local demos can override the Microsoft endpoint.
#[derive(Clone, Debug)]
pub struct Endpoints {
    pub microsoft: String,
    pub xbox_user: String,
    pub xsts: String,
    pub minecraft: String,
}

impl Endpoints {
    pub fn live() -> Self {
        let microsoft = std::env::var("SCORIA_MSA_MICROSOFT_ENDPOINT")
            .ok()
            .filter(|_| cfg!(debug_assertions))
            .unwrap_or_else(|| "https://login.microsoftonline.com/consumers/oauth2/v2.0".into());
        Self {
            microsoft,
            xbox_user: "https://user.auth.xboxlive.com/user/authenticate".into(),
            xsts: "https://xsts.auth.xboxlive.com/xsts/authorize".into(),
            minecraft: "https://api.minecraftservices.com".into(),
        }
    }
}

fn request_to(method: &str, url: &str) -> ureq::Request {
    crate::proxy::agent(
        ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(20))
            .user_agent(concat!("scoria-launcher/", env!("CARGO_PKG_VERSION"))),
        url,
    )
    .build()
    .request(method, url)
}

enum Body<'a> {
    Form(Vec<(&'a str, &'a str)>),
    Json(Value),
}

fn send(request: ureq::Request, body: Option<Body>, service: &str) -> Result<(u16, Value), String> {
    let result = match body {
        None => request.call(),
        Some(Body::Form(fields)) => request.send_form(&fields),
        Some(Body::Json(value)) => request.set("Accept", "application/json").send_json(value),
    };
    let response = match result {
        Ok(response) => response,
        Err(ureq::Error::Status(_, response)) => response,
        Err(ureq::Error::Transport(_)) => {
            return Err(format!(
                "Could not reach {service}. Check your internet connection."
            ));
        }
    };
    let status = response.status();
    let text = response.into_string().unwrap_or_default();
    Ok((status, serde_json::from_str(&text).unwrap_or(Value::Null)))
}

fn field(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_owned)
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}

/// What the player types at the verification page.
#[derive(Clone, PartialEq)]
pub struct DeviceCode {
    pub user_code: String,
    pub verification_uri: String,
    pub interval: u64,
    pub expires_at: u64,
    pub(crate) device_code: String,
}

impl std::fmt::Debug for DeviceCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceCode")
            .field("user_code", &self.user_code)
            .field("verification_uri", &self.verification_uri)
            .finish_non_exhaustive()
    }
}

/// The Microsoft tokens; the access token only lasts for the Xbox step.
pub struct MsTokens {
    access: String,
    refresh: String,
}

impl std::fmt::Debug for MsTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MsTokens(<hidden>)")
    }
}

pub enum Poll {
    Pending,
    SlowDown,
    Done(MsTokens),
}

fn oauth_error(value: &Value) -> String {
    match value.get("error").and_then(Value::as_str) {
        Some("authorization_declined") => "Sign-in was declined in the browser.".into(),
        Some("expired_token") => "The sign-in code expired. Start again.".into(),
        Some("bad_verification_code") => "The sign-in code was not recognised. Start again.".into(),
        Some("invalid_grant") => "Your Microsoft sign-in expired. Sign in again.".into(),
        Some("invalid_client" | "unauthorized_client") => {
            "Microsoft refused this launcher's app registration (client ID).".into()
        }
        Some(other) => format!("Microsoft sign-in failed ({other})."),
        None => "Microsoft sign-in failed.".into(),
    }
}

pub fn device_code(endpoints: &Endpoints, client_id: &str) -> Result<DeviceCode, String> {
    let (status, value) = send(
        request_to("POST", &format!("{}/devicecode", endpoints.microsoft)),
        Some(Body::Form(vec![("client_id", client_id), ("scope", SCOPE)])),
        "Microsoft",
    )?;
    match (
        status,
        field(&value, "user_code"),
        field(&value, "device_code"),
    ) {
        (200, Some(user_code), Some(device_code)) => Ok(DeviceCode {
            user_code,
            verification_uri: field(&value, "verification_uri")
                .unwrap_or_else(|| "https://www.microsoft.com/link".into()),
            interval: value
                .get("interval")
                .and_then(Value::as_u64)
                .unwrap_or(5)
                .max(1),
            expires_at: now()
                + value
                    .get("expires_in")
                    .and_then(Value::as_u64)
                    .unwrap_or(900),
            device_code,
        }),
        _ => Err(oauth_error(&value)),
    }
}

pub fn poll(endpoints: &Endpoints, client_id: &str, code: &DeviceCode) -> Result<Poll, String> {
    let (status, value) = send(
        request_to("POST", &format!("{}/token", endpoints.microsoft)),
        Some(Body::Form(vec![
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ("client_id", client_id),
            ("device_code", &code.device_code),
        ])),
        "Microsoft",
    )?;
    match value.get("error").and_then(Value::as_str) {
        Some("authorization_pending") => return Ok(Poll::Pending),
        Some("slow_down") => return Ok(Poll::SlowDown),
        _ => {}
    }
    ms_tokens(status, &value).map(Poll::Done)
}

fn ms_tokens(status: u16, value: &Value) -> Result<MsTokens, String> {
    match (
        status,
        field(value, "access_token"),
        field(value, "refresh_token"),
    ) {
        (200, Some(access), Some(refresh)) => Ok(MsTokens { access, refresh }),
        _ => Err(oauth_error(value)),
    }
}

/// Wait for the player in the browser until the code expires or `cancelled` says stop.
pub fn wait_for_sign_in(
    endpoints: &Endpoints,
    client_id: &str,
    code: &DeviceCode,
    cancelled: &dyn Fn() -> bool,
) -> Result<MsTokens, String> {
    let mut interval = code.interval;
    loop {
        let until = std::time::Instant::now() + Duration::from_secs(interval);
        while std::time::Instant::now() < until {
            if cancelled() {
                return Err("Sign-in cancelled.".into());
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        if now() > code.expires_at {
            return Err("The sign-in code expired. Start again.".into());
        }
        match poll(endpoints, client_id, code)? {
            Poll::Pending => {}
            Poll::SlowDown => interval += 5,
            Poll::Done(tokens) => return Ok(tokens),
        }
    }
}

/// A public Minecraft profile, as `accounts.json` keeps it.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OnlineAccount {
    /// The profile UUID without dashes.
    pub uuid: String,
    pub name: String,
    /// The active skin's URL and model (`classic` / `slim`).
    #[serde(default)]
    pub skin: Option<(String, String)>,
    #[serde(default)]
    pub cape: Option<String>,
}

/// The keychain's copy for one account. Never serialized into `accounts.json`.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Secrets {
    pub(crate) refresh_token: String,
    pub(crate) minecraft_token: String,
    pub(crate) minecraft_expires_at: u64,
    #[serde(default)]
    pub(crate) xuid: Option<String>,
}

impl std::fmt::Debug for Secrets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Secrets(<hidden>)")
    }
}

/// Progress the UI can show; no credentials.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Microsoft,
    Xbox,
    Minecraft,
    Ownership,
    Profile,
    Saving,
}

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Self::Microsoft => "Waiting for Microsoft…",
            Self::Xbox => "Signing in to Xbox Live…",
            Self::Minecraft => "Getting a Minecraft session…",
            Self::Ownership => "Checking Minecraft ownership…",
            Self::Profile => "Loading the Minecraft profile…",
            Self::Saving => "Saving sign-in…",
        }
    }
}

fn xsts_error(code: u64) -> String {
    match code {
        2148916233 => "This Microsoft account has no Xbox profile yet. Sign in once at xbox.com to create one, then try again.".into(),
        2148916235 => "Xbox Live is not available in your country or region.".into(),
        2148916236 | 2148916237 => "This account needs adult verification on xbox.com before it can play.".into(),
        2148916238 => "This is a child account: an adult has to add it to a Microsoft family first.".into(),
        other => format!("Xbox Live refused the sign-in (error {other})."),
    }
}

/// From Microsoft tokens to a ready online profile.
pub fn finish_sign_in(
    endpoints: &Endpoints,
    ms: MsTokens,
    progress: &dyn Fn(Phase),
) -> Result<(OnlineAccount, Secrets), String> {
    progress(Phase::Xbox);
    let (status, xbl) = send(
        request_to("POST", &endpoints.xbox_user),
        Some(Body::Json(json!({
            "Properties": {
                "AuthMethod": "RPS",
                "SiteName": "user.auth.xboxlive.com",
                "RpsTicket": format!("d={}", ms.access),
            },
            "RelyingParty": "http://auth.xboxlive.com",
            "TokenType": "JWT",
        }))),
        "Xbox Live",
    )?;
    let xbl_token = field(&xbl, "Token")
        .filter(|_| status == 200)
        .ok_or("Xbox Live refused the sign-in.")?;
    let (status, xsts) = send(
        request_to("POST", &endpoints.xsts),
        Some(Body::Json(json!({
            "Properties": { "SandboxId": "RETAIL", "UserTokens": [xbl_token] },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT",
        }))),
        "Xbox Live",
    )?;
    if let Some(code) = xsts.get("XErr").and_then(Value::as_u64) {
        return Err(xsts_error(code));
    }
    let claims = xsts.pointer("/DisplayClaims/xui/0");
    let (Some(xsts_token), Some(uhs)) = (
        field(&xsts, "Token").filter(|_| status == 200),
        claims.and_then(|claim| field(claim, "uhs")),
    ) else {
        return Err("Xbox Live refused the sign-in.".into());
    };
    let xuid = claims.and_then(|claim| field(claim, "xid"));
    progress(Phase::Minecraft);
    let (token, expires_in) = minecraft_token(endpoints, &format!("XBL3.0 x={uhs};{xsts_token}"))?;
    progress(Phase::Ownership);
    if !owns_game(endpoints, &token)? {
        return Err("This account doesn't own Minecraft: Java Edition.".into());
    }
    progress(Phase::Profile);
    let profile = profile(endpoints, &token)?
        .ok_or("This account has no Minecraft profile yet. Create one at minecraft.net first.")?;
    Ok((
        profile,
        Secrets {
            refresh_token: ms.refresh,
            minecraft_token: token,
            minecraft_expires_at: now() + expires_in,
            xuid,
        },
    ))
}

fn minecraft_token(endpoints: &Endpoints, token: &str) -> Result<(String, u64), String> {
    let lifetime = |value: &Value| {
        value
            .get("expires_in")
            .and_then(Value::as_u64)
            .unwrap_or(86400)
    };
    let launcher = send(
        request_to("POST", &format!("{}/launcher/login", endpoints.minecraft)),
        Some(Body::Json(
            json!({ "xtoken": token, "platform": "PC_LAUNCHER" }),
        )),
        "the Minecraft services",
    );
    if let Ok((200, value)) = &launcher {
        if let Some(token) = field(value, "access_token") {
            return Ok((token, lifetime(value)));
        }
    }
    let (status, value) = send(
        request_to(
            "POST",
            &format!("{}/authentication/login_with_xbox", endpoints.minecraft),
        ),
        Some(Body::Json(json!({ "identityToken": token }))),
        "the Minecraft services",
    )?;
    match (status, field(&value, "access_token")) {
        (200, Some(token)) => Ok((token, lifetime(&value))),
        (403, _) => Err(
            "The Minecraft services refused this launcher: its app is not approved by Mojang yet."
                .into(),
        ),
        (429, _) => Err("Too many sign-ins in a short time. Wait a minute and try again.".into()),
        _ => Err("The Minecraft services refused the sign-in.".into()),
    }
}

fn owns_game(endpoints: &Endpoints, token: &str) -> Result<bool, String> {
    let (status, value) = send(
        request_to(
            "GET",
            &format!("{}/entitlements/mcstore", endpoints.minecraft),
        )
        .set("Authorization", &format!("Bearer {token}")),
        None,
        "the Minecraft services",
    )?;
    if status != 200 {
        return Err("Could not check which games this account owns.".into());
    }
    Ok(value
        .get("items")
        .and_then(Value::as_array)
        .is_some_and(|items| {
            items.iter().any(|item| {
                matches!(
                    item.get("name").and_then(Value::as_str),
                    Some("game_minecraft" | "product_minecraft")
                )
            })
        }))
}

fn profile(endpoints: &Endpoints, token: &str) -> Result<Option<OnlineAccount>, String> {
    let (status, value) = send(
        request_to("GET", &format!("{}/minecraft/profile", endpoints.minecraft))
            .set("Authorization", &format!("Bearer {token}")),
        None,
        "the Minecraft services",
    )?;
    if status == 404 {
        return Ok(None);
    }
    let (Some(uuid), Some(name)) = (field(&value, "id"), field(&value, "name")) else {
        return Err("Could not read the Minecraft profile.".into());
    };
    let active = |key: &str| {
        value
            .get(key)
            .and_then(Value::as_array)
            .and_then(|list| {
                list.iter()
                    .find(|item| item.get("state").and_then(Value::as_str) == Some("ACTIVE"))
            })
            .cloned()
    };
    Ok(Some(OnlineAccount {
        uuid: uuid.replace('-', "").to_lowercase(),
        name,
        skin: active("skins").and_then(|skin| {
            Some((
                field(&skin, "url")?,
                field(&skin, "variant")
                    .unwrap_or_else(|| "CLASSIC".into())
                    .to_lowercase(),
            ))
        }),
        cape: active("capes").and_then(|cape| field(&cape, "url")),
    }))
}

fn secret_key(uuid: &str) -> String {
    format!("{SECRET_PREFIX}{uuid}")
}

pub fn save_secrets(uuid: &str, secrets: &Secrets) -> Result<(), String> {
    let text = serde_json::to_string(secrets).map_err(|error| error.to_string())?;
    account::store().set(&secret_key(uuid), &text)
}

pub fn load_secrets(uuid: &str) -> Result<Option<Secrets>, String> {
    account::store()
        .get(&secret_key(uuid))?
        .map(|text| serde_json::from_str(&text).map_err(|_| "The saved sign-in is damaged.".into()))
        .transpose()
}

pub fn forget_secrets(uuid: &str) -> Result<(), String> {
    account::store().delete(&secret_key(uuid))
}

/// Ensure a fresh session and write the file the game reads once. The caller is the launcher;
/// the file contains a token and lives only under the game's `sessions/`.
pub fn session_for_launch(
    game_home: &std::path::Path,
    account: &OnlineAccount,
) -> Result<std::path::PathBuf, String> {
    let mut secrets =
        load_secrets(&account.uuid)?.ok_or("Your Microsoft sign-in is missing. Sign in again.")?;
    if secrets.minecraft_expires_at <= now() + 3600 {
        secrets = refresh(&Endpoints::live(), &secrets)?;
        save_secrets(&account.uuid, &secrets)?;
    }
    let dir = game_home.join("sessions");
    std::fs::create_dir_all(&dir).map_err(|error| format!("{}: {error}", dir.display()))?;
    let name = format!("session-{}-{}.json", std::process::id(), now());
    let path = dir.join(name);
    let json = json!({
        "name": account.name,
        "uuid": account.uuid,
        "access_token": secrets.minecraft_token,
        "user_type": "msa",
        "xuid": secrets.xuid,
    });
    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)
        .map_err(|error| format!("{}: {error}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(std::fs::Permissions::from_mode(0o600))
            .map_err(|error| format!("{}: {error}", path.display()))?;
    }
    use std::io::Write;
    file.write_all(json.to_string().as_bytes())
        .and_then(|()| file.sync_all())
        .map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(path)
}

/// Renew the Minecraft token from the Microsoft refresh token. The caller saves the result.
pub fn refresh(endpoints: &Endpoints, secrets: &Secrets) -> Result<Secrets, String> {
    let client_id = client_id().ok_or_else(|| NOT_CONFIGURED.to_string())?;
    let (status, value) = send(
        request_to("POST", &format!("{}/token", endpoints.microsoft)),
        Some(Body::Form(vec![
            ("grant_type", "refresh_token"),
            ("client_id", &client_id),
            ("refresh_token", &secrets.refresh_token),
            ("scope", SCOPE),
        ])),
        "Microsoft",
    )?;
    let ms = ms_tokens(status, &value)?;
    let (_, renewed) = finish_sign_in(endpoints, ms, &|_| {})?;
    Ok(renewed)
}
