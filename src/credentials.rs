use crate::xenon as x;

/// Credential used to access a remote location.
#[derive(Clone)]
pub enum Credential {
    Certificate(CertificateCredential),
    Password(PasswordCredential),
}

impl Credential {
    ///
    ///
    ///
    pub fn new_password<S1, S2>(
        username: S1,
        password: S2,
    ) -> Credential
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let password = PasswordCredential::new(username, password);
        Credential::Password(password)
    }

    ///
    ///
    ///
    pub fn new_certificate<S1, S2, S3>(
        certificate: S1,
        username: S2,
        passphrase: S3,
    ) -> Credential
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
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

/// Credential variant for certificate-based access.
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
    pub fn new<S1, S2, S3>(
        certificate: S1,
        username: S2,
        passphrase: S3,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        CertificateCredential {
            certificate: certificate.into(),
            passphrase: passphrase.into(),
            username: username.into(),
        }
    }

    ///
    ///
    ///
    pub(crate) fn proto(self) -> x::CertificateCredential {
        x::CertificateCredential {
            username: self.username,
            passphrase: self.passphrase,
            certfile: self.certificate,
        }
    }
}

/// Credential variant for username/password-based access.
#[derive(Clone)]
pub struct PasswordCredential {
    pub password: String,
    pub username: String,
}

impl PasswordCredential {
    ///
    ///
    ///
    pub fn new<S1, S2>(
        username: S1,
        password: S2,
    ) -> PasswordCredential
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        PasswordCredential {
            password: password.into(),
            username: username.into(),
        }
    }

    ///
    ///
    ///
    pub(crate) fn proto(self) -> x::PasswordCredential {
        x::PasswordCredential {
            username: self.username,
            password: self.password,
        }
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

            assert_eq!(cc_proto.certfile, certificate);
            assert_eq!(cc_proto.username, username);
            assert_eq!(cc_proto.passphrase, passphrase);
        }
    }

    #[test]
    fn password_proto_ok() {
        let username = String::from("username");
        let password = String::from("password");

        let credential = Credential::new_password(&username, &password);

        if let Credential::Password(cp) = credential {
            let cp_proto = cp.proto();

            assert_eq!(cp_proto.username, username);
            assert_eq!(cp_proto.password, password);
        }
    }
}
