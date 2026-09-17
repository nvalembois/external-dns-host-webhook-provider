use std::net::IpAddr;
use tracing::{debug, info};

use crate::model::records::{Endpoint, RecordType};

/// Une entrée (host, ip) telle que stockée dans /etc/avahi/hosts (https://github.com/avahi/avahi avahi-daemon/static-hosts.c)
#[derive(Debug, Clone)]
pub struct Host {
    pub host: String,
    pub ip: IpAddr,
    iteration: usize,
}

/// Cache des Hosts et lien entre webhook et configmap
#[derive(Debug, Clone)]
pub struct HostsCache{
    iteration: usize,
    hosts: Vec<Host>,
}

impl HostsCache {

    pub fn update_from_file(&mut self, data: &str) {
        // début de nouvelle itération
        self.iteration += 1;
        info!("start HostsCache update from file content (iteration: {0})", self.iteration);

        // ajout des enregistrements 
        self.insert_from_file(data);

        // netoyage des host/ip obsolétes
        self.hosts.retain(|h| h.iteration == self.iteration);
    }

    fn insert_from_file(&mut self, data: &str) {
        // itération sur chaque ligne
        for mut line in data.lines() {
            // suppression des espaces et des tabulations au début et à la fin
            line = line.trim_matches(is_space);

            // ignore les lignes vides
            if line.is_empty() { continue }
            // ignore les lignes de commentaire
            if line.starts_with('#') { continue }

            // séparation après le premier espace ou tabulation
            let (ip, host) = match line.split_once(is_space) {
                Some((ip, host)) => (ip, host),
                None =>  {
                    info!("Skip unparseable line : {line}");
                    continue
                }
            };

            // vérification qu'ip est une adresse ip
            let ip: IpAddr = match ip.parse() {
                Ok(v) => v,
                Err(_) => {
                    info!("Skip line with unparseable ip : {line}");
                    continue
                }
            };

            // nettoyage host
            let host = host.trim_matches(is_space);

            // ignore si plusieurs hosts
            if host.contains(is_space) { continue }
            
            // ajoute host/ip dans le cache
            self.insert(host, ip);
        }
    }

    pub fn insert(&mut self, host: &str, ip: IpAddr) -> bool
    {
        for h in &mut self.hosts {
            if h.host.eq_ignore_ascii_case(host) && h.ip.eq(&ip){
                debug!("already in cache {ip} {host}");
                h.iteration = self.iteration;
                return false;
            }
        }
        // ajoute nouvel host/ip
        debug!("add in cache {ip} {host}");
        let host_entry = Host{
            host: host.to_string(),
            ip: ip,
            iteration: self.iteration
        };
        self.hosts.push(host_entry);
        true
    }

    pub fn remove(&mut self, host: &str, ip: &IpAddr) -> bool
    {
        for (i, h) in &mut self.hosts.iter().enumerate() {
            if h.host.eq_ignore_ascii_case(host) && h.ip.eq(ip){
                self.hosts.swap_remove(i);
                debug!("remove from cache {ip} {host}");
                return true;
            }
        }
        false
    }

    pub fn into_endpoints(&self) -> Vec<Endpoint> {
        self.hosts
            .iter()
            .map(|host| Endpoint {
                dns_name: host.host.clone(),
                record_type: match host.ip {
                    IpAddr::V4(_) => RecordType::A,
                    IpAddr::V6(_) => RecordType::AAAA,
                },
                targets: vec![host.ip.to_string()],
                set_identifier: None,
                record_t_t_l: None,
                labels: None,
                provider_specific: None
            }).collect()
    }

}

impl ToString for HostsCache {
    fn to_string(&self) -> String {
        self.hosts
            .iter()
            .fold(String::new(),|acc, h| format!("{acc}{0}\t{1}\n",h.ip,h.host))
    }
}

impl Into<Vec<Endpoint>> for HostsCache {
    fn into(self) -> Vec<Endpoint> {
        self.into_endpoints()
    }
}

impl From<&str> for HostsCache {
    fn from(value: &str) -> Self {
        info!("init HostsCache from file content");

        let mut s = Self {
            iteration: 0,
            hosts: vec![]
        };
        
        // ajout des enregistrements 
        s.insert_from_file(value);
        
        s
    }
}

#[inline]
fn is_space(c: char) -> bool {
    c.is_whitespace() || c == '\t'
}
