use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

pub fn is_local_recipient(email: &str, domain_name: &str) -> bool { email.ends_with(&format!("@{}", domain_name)) }

pub fn lookup_mx_record(email: &str) -> Result<String, Box<dyn std::error::Error>> {
    let domain      = email.split("@").nth(1).ok_or("Invalid email format")?;
    let resolver    = Resolver::new(ResolverConfig::default(), ResolverOpts::default())?;
    let response    = resolver.mx_lookup(domain)?;
    let mx          = response.iter()
        .min_by_key(|record| record.preference())
        .ok_or("No MX record found")?;
    Ok(format!("{}:25", mx.exchange()))
}
