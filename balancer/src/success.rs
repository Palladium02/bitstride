/// A success tracker that keeps track of the success rate using exponential decay
/// to also keep track of historic data that eventually fades out.
#[derive(Debug, Clone)]
pub struct SuccessTracker {
    weighted_success: f32,
    weighted_total: f32,
    lambda: f32,
}

impl SuccessTracker {
    pub fn new(lambda: f32) -> Self {
        assert!(lambda > 0.0 && lambda < 1.0);
        SuccessTracker {
            weighted_success: 0.0,
            weighted_total: 0.0,
            lambda,
        }
    }

    #[allow(unused)] // TODO: remove later, currently here to satisfy the CI pipeline (specifically clippy)
    pub fn update(&mut self, is_success: bool) {
        let success = match is_success {
            true => 1.0,
            false => 0.0,
        };
        self.weighted_success = self.lambda * self.weighted_success + (1.0 - self.lambda) * success;
        self.weighted_total = self.lambda * self.weighted_total + (1.0 - self.lambda);
    }

    pub fn ratio(&self) -> f32 {
        if self.weighted_total > 0.0 {
            self.weighted_success / self.weighted_total
        } else {
            1.0
        }
    }
}