use std::borrow::Cow;

pub type AppResult<T> = Result<T, AppUiError>;

#[derive(Debug)]
pub enum AppActionError {
    Connection(anyhow::Error),
    Bind(rusmppc::error::Error),
    SubmitSm(rusmppc::error::Error),
    Unbind(rusmppc::error::Error),
    Close(rusmppc::error::Error),
    /// rusmppc event stream background error
    Background(rusmppc::error::Error),
}

#[derive(Debug, Clone)]
pub enum AppUiError {
    Url(SmppUrlError),
    Field(SmppFieldError),
    MultiPart(MultiPartError),
}

impl AppUiError {
    pub fn display_message(&self) -> Cow<'static, str> {
        match self {
            AppUiError::Field(err) => err.display_message(),
            AppUiError::Url(err) => err.display_message(),
            AppUiError::MultiPart(err) => err.display_message(),
        }
    }

    pub const fn invalid_system_id() -> Self {
        Self::Field(SmppFieldError::SystemId)
    }

    pub const fn invalid_password() -> Self {
        Self::Field(SmppFieldError::Password)
    }

    pub const fn invalid_system_type() -> Self {
        Self::Field(SmppFieldError::SystemType)
    }

    pub const fn invalid_service_type() -> Self {
        Self::Field(SmppFieldError::ServiceType)
    }

    pub const fn invalid_source_addr() -> Self {
        Self::Field(SmppFieldError::SourceAddr)
    }

    pub const fn invalid_destination_addr() -> Self {
        Self::Field(SmppFieldError::DestinationAddr)
    }
}

#[derive(Debug, Clone)]
pub enum SmppFieldError {
    /// Invalid System ID
    SystemId,
    /// Invalid Password
    Password,
    /// Invalid System Type
    SystemType,
    /// Invalid Service Type
    ServiceType,
    /// Invalid Source Address
    SourceAddr,
    /// Invalid Destination Address
    DestinationAddr,
}

impl SmppFieldError {
    pub fn display_message(&self) -> Cow<'static, str> {
        match self {
            SmppFieldError::SystemId => "System ID must be 0-15 ascii octets long.".into(),
            SmppFieldError::Password => "Password must be 0-8 ascii octets long.".into(),
            SmppFieldError::SystemType => "System Type must be 0-12 ascii octets long.".into(),
            SmppFieldError::ServiceType => "Service Type must be 0-5 ascii octets long.".into(),
            SmppFieldError::SourceAddr => "Source Address must be 0-20 ascii octets long.".into(),
            SmppFieldError::DestinationAddr => {
                "Destination Address must be 0-20 ascii octets long.".into()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SmppUrlError {
    /// URL Parse Error
    Parse,
    /// Invalid URL Schema
    Schema,
    /// URL Host Missing
    Host,
}

impl SmppUrlError {
    pub fn display_message(&self) -> Cow<'static, str> {
        match self {
            SmppUrlError::Parse => "Invalid URL".into(),
            SmppUrlError::Schema => "URL schema must be smpp, ssmpp or smpps".into(),
            SmppUrlError::Host => "URL host is missing".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MultiPartError {
    // TODO
    Todo,
}

impl MultiPartError {
    pub fn display_message(&self) -> Cow<'static, str> {
        // TODO
        "".into()
    }
}
