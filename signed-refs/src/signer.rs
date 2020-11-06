use std::{io, path::Path};

use radicle_keystore::{crypto, pinentry, FileStorage, Keystore};

use librad::{
    keys::{self, PublicKey, SecretKey},
    signer::{BoxedSigner, SomeSigner},
};

const SECRET_KEY_FILE: &str = "librad.key";

pub fn get_signer(keys_dir: &Path) -> anyhow::Result<BoxedSigner> {
    let file = keys_dir.join(SECRET_KEY_FILE);
    let prompt = crypto::Pwhash::new(
        pinentry::Prompt::new("Please enter your passphrase: "),
        *crypto::KDF_PARAMS_PROD,
    );
    let keystore = FileStorage::<_, PublicKey, _, _>::new(&file, prompt);
    let key: SecretKey = keystore.get_key().map(|keypair| keypair.secret_key)?;

    Ok(SomeSigner { signer: key }.into())
}

pub fn from_std_in<R: io::Read>(mut r: R) -> anyhow::Result<BoxedSigner> {
    use radicle_keystore::SecretKeyExt;

    let mut bytes = Vec::new();
    r.read_to_end(&mut bytes)?;

    let sbytes: keys::SecStr = bytes.into();
    match keys::SecretKey::from_bytes_and_meta(sbytes, &()) {
        Ok(key) => Ok(SomeSigner { signer: key }.into()),
        Err(err) => Err(io::Error::new(io::ErrorKind::InvalidData, err).into()),
    }
}
