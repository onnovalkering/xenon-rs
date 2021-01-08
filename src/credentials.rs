use crate::xenon as x;

///
///
///
#[derive(Clone)]
pub enum Credential {
    Certificate(CertificateCredential),
    Password(PasswordCredential),
}

impl Credential {
    ///
    ///
    ///
    pub fn new_password(
        username: String,
        password: String,
    ) -> Credential {
        let password = PasswordCredential::new(username, password);
        Credential::Password(password)
    }

    ///
    ///
    ///
    pub fn new_certificate(
        certificate: String,
        username: String,
        passphrase: String,
    ) -> Credential {
        let certificate = CertificateCredential::new(certificate, username, passphrase);
        Credential::Certificate(certificate)
    }
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
    ) -> CertificateCredential {
        CertificateCredential {
            certificate,
            passphrase,
            username,
        }
    }

    ///
    ///
    ///
    pub(crate) fn proto(self) -> x::CertificateCredential {
        let mut credential = x::CertificateCredential::new();
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
    ) -> PasswordCredential {
        PasswordCredential { password, username }
    }

    ///
    ///
    ///
    pub(crate) fn proto(self) -> x::PasswordCredential {
        let mut credential = x::PasswordCredential::new();
        credential.set_username(self.username);
        credential.set_password(self.password);

        credential
    }
}
