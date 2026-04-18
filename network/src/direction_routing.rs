use ipv8_core::address::IPv8Address;
use rand::Rng;

#[derive(Debug)]
pub struct DirectionalRoute {
    pub forward_path: Vec<IPv8Address>,
    pub reverse_path: Vec<IPv8Address>,
}

pub struct DirectionRouter;

impl DirectionRouter {
    pub fn generate(source: &IPv8Address, destination: &IPv8Address) -> DirectionalRoute {
        let mut rng = rand::thread_rng();

        let mut forward = vec![
            source.clone(),
            IPv8Address::new_random(),
            IPv8Address::new_random(),
            destination.clone(),
        ];

        let mut reverse = vec![
            destination.clone(),
            IPv8Address::new_random(),
            IPv8Address::new_random(),
            source.clone(),
        ];

        // Randomize paths slightly
        if rng.gen_bool(0.5) {
            forward.swap(1, 2);
        }

        if rng.gen_bool(0.5) {
            reverse.swap(1, 2);
        }

        DirectionalRoute {
            forward_path: forward,
            reverse_path: reverse,
        }
    }
}
