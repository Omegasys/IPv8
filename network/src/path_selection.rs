use ipv8_core::address::IPv8Address;
use rand::seq::SliceRandom;

pub struct PathSelector;

impl PathSelector {
    pub fn random_path(source: &IPv8Address, destination: &IPv8Address) -> Vec<IPv8Address> {
        let mut path = vec![
            source.clone(),
            IPv8Address::new_random(),
            IPv8Address::new_random(),
            destination.clone(),
        ];

        // Shuffle intermediate hops (not endpoints)
        let mut middle = path[1..path.len() - 1].to_vec();
        middle.shuffle(&mut rand::thread_rng());

        path.splice(1..path.len() - 1, middle);

        path
    }
}
