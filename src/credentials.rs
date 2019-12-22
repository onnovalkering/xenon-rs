use std::path::PathBuf;

///
/// 
/// 
pub struct CertificateCredential {
    pub certificate: PathBuf,
    pub passphrase: String,
    pub username: String,
}

impl CertificateCredential {
    ///
    /// 
    /// 
    pub fn new(
        certificate: PathBuf,
        passphrase: String,
        username: String,
    ) -> CertificateCredential {
        CertificateCredential {
            certificate,
            passphrase,
            username,
        }        
    }
}

///
/// 
/// 
pub struct PasswordCredential {
    pub password: String,
    pub username: String,
}

impl PasswordCredential {
    ///
    /// 
    /// 
    pub fn new(
        password: String,
        username: String,
    ) -> PasswordCredential {
        PasswordCredential {
            password,
            username
        }
    }
}
