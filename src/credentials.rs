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
    pub fn new_password<S: Into<String>>(
        username: S,
        password: S,
    ) -> Credential {
        let password = PasswordCredential::new(username, password);
        Credential::Password(password)
    }

    ///
    ///
    ///
    pub fn new_certificate<S: Into<String>>(
        certificate: S,
        username: S,
        passphrase: Option<S>,
    ) -> Credential {
        let certificate = CertificateCredential::new(certificate, username, passphrase);
        Credential::Certificate(certificate)
    }

    ///
    ///
    ///
    pub fn is_password(&self) -> bool {
        match self {
            Credential::Certificate(_) => false,
            Credential::Password(_) => true,
        }
    }

    ///
    ///
    ///
    pub fn is_certificate(&self) -> bool {
        match self {
            Credential::Certificate(_) => true,
            Credential::Password(_) => false,
        }
    }
}

///
///
///
#[derive(Clone)]
pub struct CertificateCredential {
    pub certificate: String,
    pub passphrase: Option<String>,
    pub username: String,
}

impl CertificateCredential {
    ///
    ///
    ///
    pub fn new<S: Into<String>>(
        certificate: S,
        username: S,
        passphrase: Option<S>,
    ) -> Self {
        CertificateCredential {
            certificate: certificate.into(),
            passphrase: passphrase.map(S::into),
            username: username.into(),
        }
    }

    ///
    ///
    ///
    pub(crate) fn proto(self) -> x::CertificateCredential {
        let mut credential = x::CertificateCredential::new();
        credential.set_username(self.username);
        credential.set_certfile(self.certificate);
        if let Some(passphrase) = self.passphrase {
            credential.set_passphrase(passphrase);
        }

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
    pub fn new<S: Into<String>>(
        username: S,
        password: S,
    ) -> PasswordCredential {
        PasswordCredential {
            password: password.into(),
            username: username.into(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credential_newpassword_variant() {
        let username = String::from("username");
        let password = String::from("password");

        let credential = Credential::new_password(username, password);
        assert!(credential.is_password());
        assert!(!credential.is_certificate());
    }

    #[test]
    fn credential_newcertificate_variant() {
        let certificate = String::from("certificate");
        let username = String::from("username");
        let passphrase = String::from("passphrase");

        let credential = Credential::new_certificate(certificate, username, passphrase);
        assert!(!credential.is_password());
        assert!(credential.is_certificate());
    }

    #[test]
    fn certificate_proto_ok() {
        let certificate = String::from("certificate");
        let username = String::from("username");
        let passphrase = String::from("passphrase");

        let credential = Credential::new_certificate(&certificate, &username, &passphrase);

        if let Credential::Certificate(cc) = credential {
            let cc_proto = cc.proto();

            assert_eq!(cc_proto.get_certfile(), &certificate);
            assert_eq!(cc_proto.get_username(), &username);
            assert_eq!(cc_proto.get_passphrase(), &passphrase);
        }
    }

    #[test]
    fn password_proto_ok() {
        let username = String::from("username");
        let password = String::from("password");

        let credential = Credential::new_password(&username, &password);

        if let Credential::Password(cp) = credential {
            let cp_proto = cp.proto();

            assert_eq!(cp_proto.get_username(), &username);
            assert_eq!(cp_proto.get_password(), &password);
        }
    }
}
