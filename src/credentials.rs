use crate::xenon;

///
/// 
///
#[derive(Clone)]
pub enum Credential {
    Certificate(CertificateCredential),
    Password(PasswordCredential)
}

///
/// 
/// 
#[derive(Clone)]
pub struct CertificateCredential {
    pub certificate: String,
    pub passphrase: String,
    pub username: String,
}

impl CertificateCredential {
    ///
    /// 
    /// 
    pub fn new(
        certificate: String,
        username: String,
        passphrase: String,
    ) -> Credential {
        Credential::Certificate(
            CertificateCredential {
                certificate,
                passphrase,
                username,
            }
        )
    }

    ///
    /// 
    /// 
    pub(crate) fn proto(
        self
    ) -> xenon::CertificateCredential {
        let mut credential = xenon::CertificateCredential::new();
        credential.set_certfile(self.certificate);
        credential.set_passphrase(self.passphrase);
        credential.set_username(self.username);

        credential
    }
}

///
/// 
/// 
#[derive(Clone)]
pub struct PasswordCredential {
    pub password: String,
    pub username: String,
}

impl PasswordCredential {
    ///
    /// 
    /// 
    pub fn new(
        username: String,
        password: String,
    ) -> Credential {
        Credential::Password(
            PasswordCredential {
                password,
                username
            }
        )
    }

    ///
    /// 
    /// 
    pub(crate) fn proto(
        self
    ) -> xenon::PasswordCredential {
        let mut credential = xenon::PasswordCredential::new();
        credential.set_username(self.username);
        credential.set_password(self.password);

        credential
    }
}
