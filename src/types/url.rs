use url::Url;

use crate::result::SmppUrlError;

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone)]
pub struct WasmSmppUrl {
    pub ssl: bool,
    pub domain: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct SmppUrl {
    url: Url,
    #[cfg(target_arch = "wasm32")]
    wasm: WasmSmppUrl,
}

impl SmppUrl {
    pub fn new(url: &str) -> Result<Self, SmppUrlError> {
        let url = Url::parse(url).map_err(|_| SmppUrlError::Parse)?;

        let _ssl = match url.scheme() {
            "smpp" => false,
            "smpps" | "ssmpp" => true,
            _ => return Err(SmppUrlError::Schema),
        };

        let _domain = match url.host_str() {
            Some(domain) => domain.to_string(),
            None => return Err(SmppUrlError::Host),
        };

        let _port = url.port().unwrap_or(if _ssl { 2776 } else { 2775 });

        Ok(SmppUrl {
            url,
            #[cfg(target_arch = "wasm32")]
            wasm: WasmSmppUrl {
                ssl: _ssl,
                domain: _domain,
                port: _port,
            },
        })
    }

    #[cfg(target_arch = "wasm32")]
    pub fn into_wasm(self) -> WasmSmppUrl {
        self.wasm
    }
}

impl std::fmt::Display for SmppUrl {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.url, formatter)
    }
}
