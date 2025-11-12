use std::str::FromStr;
use teloxide::{prelude::*, types::MessageEntityKind};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Link {
    X(Url),
    Instagram(Url),
    TikTok(Url),
    Unsupported,
}

impl FromStr for Link {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Url::from_str(s)?.into())
    }
}

impl From<Url> for Link {
    fn from(value: Url) -> Self {
        let host = value.host().map(|host| host.to_string());
        match host.as_deref() {
            Some("twitter.com" | "x.com" | "www.twitter.com" | "www.x.com") => Self::X(value),
            Some("instagram.com" | "www.instagram.com") => Self::Instagram(value),
            Some("tiktok.com" | "www.tiktok.com" | "vm.tiktok.com" | "vt.tiktok.com") => Self::TikTok(value),
            _ => Self::Unsupported,
        }
    }
}

impl Link {
    /// Changes the host of the Url to one of the mapping alternatives
    pub fn into_mapped_url(self) -> Option<Url> {
        match self {
            Self::X(mut url) => {
                url.set_host(Some("fixupx.com")).unwrap();
                url.set_query(None);
                Some(url)
            }
            Self::Instagram(mut url) => {
                url.set_host(Some("kkinstagram.com")).unwrap();
                url.set_query(None);
                Some(url)
            }
            Self::TikTok(mut url) => {
                url.set_host(Some("vm.vxtiktok.com")).unwrap();
                url.set_query(None);
                Some(url)
            }
            Self::Unsupported => None,
        }
    }
}

/// Return a better version of the link for all links in the message
pub fn map_links(msg: &Message) -> Vec<String> {
    let links = links_from_msg(msg);
    if !links.is_empty() {
        let user = msg.from().map_or(&None, |user| &user.username);
        println!("New message: {user:?}: {:?}", msg.text());
    }

    links
        .into_iter()
        .filter_map(Link::into_mapped_url)
        .map(|url| {
            let new_url = url.to_string();
            println!("New link: {new_url:?}");
            new_url
        })
        .collect()
}

/// Extract all links from a Teloxide's [`Message`].
fn links_from_msg(msg: &Message) -> Vec<Link> {
    let entities = msg.parse_entities().unwrap_or_default();
    entities
        .iter()
        .filter_map(|entity| match entity.kind() {
            MessageEntityKind::Url => entity.text().parse().ok(),
            MessageEntityKind::TextLink { url, .. } => Some(url.clone().into()),
            // Other message kinds are ignored
            _ => None,
        })
        .collect()
}
