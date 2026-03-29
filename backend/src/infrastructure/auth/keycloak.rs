use anyhow::Result;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

/// Claims JWT di Keycloak
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
    pub realm_access: Option<RealmAccess>,
}

/// Ruoli nel realm Keycloak
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

/// Validatore JWT per Keycloak
pub struct KeycloakValidator {
    pub keycloak_url: String,
    pub realm: String,
}

impl KeycloakValidator {
    pub fn new(keycloak_url: String, realm: String) -> Self {
        Self { keycloak_url, realm }
    }

    /// URL del discovery endpoint di Keycloak
    pub fn discovery_url(&self) -> String {
        format!(
            "{}/realms/{}/.well-known/openid-configuration",
            self.keycloak_url, self.realm
        )
    }

    /// URL del JWKS endpoint di Keycloak
    pub fn jwks_url(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/certs",
            self.keycloak_url, self.realm
        )
    }

    /// Valida il token JWT. In produzione recupera le chiavi JWKS da Keycloak.
    /// In questa implementazione semplificata decodifica senza verificare la firma.
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        // Disabilita la verifica della firma per questa implementazione semplificata
        let mut validation = Validation::new(Algorithm::RS256);
        validation.insecure_disable_signature_validation();
        validation.validate_exp = true;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&[]),
            &validation,
        )
        .map_err(|e| anyhow::anyhow!("Token JWT non valido: {}", e))?;

        Ok(token_data.claims)
    }

    /// Estrae il sub (user ID) dal token senza validare la firma
    pub fn extract_user_id(&self, token: &str) -> Result<String> {
        let claims = self.validate_token(token)?;
        Ok(claims.sub)
    }
}
