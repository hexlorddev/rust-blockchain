use libp2p::{
    identity, 
    ping,
    swarm::{Swarm, SwarmEvent},
    Multiaddr,
    PeerId,
    futures::StreamExt,
};
use std::error::Error;

pub struct P2PNetwork {
    swarm: Swarm<ping::Behaviour>,
}

impl P2PNetwork {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        let transport = libp2p::development_transport(local_key)?;
        let ping_behavior = ping::Behaviour::default();
        
        let mut swarm = Swarm::new(transport, ping_behavior, local_peer_id);
        
        // Listen on all interfaces and random port
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        
        Ok(P2PNetwork { swarm })
    }

    pub async fn run(&mut self) {
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {}", address);
                }
                SwarmEvent::Behaviour(event) => {
                    println!("Event: {:?}", event);
                }
                _ => {}
            }
        }
    }

    pub fn add_peer(&mut self, addr: Multiaddr) -> Result<(), Box<dyn Error>> {
        self.swarm.dial(addr)?;
        Ok(())
    }
}