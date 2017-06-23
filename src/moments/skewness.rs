/// Estimate the arithmetic mean, the variance and the skewness of a sequence of
/// numbers ("population").
///
/// This can be used to estimate the standard error of the mean.
#[derive(Debug, Clone)]
pub struct Skewness {
    /// Estimator of mean and variance.
    avg: MeanWithError,
    /// Intermediate sum of cubes for calculating the skewness.
    sum_3: f64,
}

impl Skewness {
    /// Create a new skewness estimator.
    #[inline]
    pub fn new() -> Skewness {
        Skewness {
            avg: MeanWithError::new(),
            sum_3: 0.,
        }
    }

    /// Add an observation sampled from the population.
    #[inline]
    pub fn add(&mut self, x: f64) {
        let delta = x - self.mean();
        self.increment();
        let n = f64::approx_from(self.len()).unwrap();
        self.add_inner(delta, delta/n);
    }

    /// Increment the sample size.
    ///
    /// This does not update anything else.
    #[inline]
    fn increment(&mut self) {
        self.avg.increment();
    }

    /// Add an observation given an already calculated difference from the mean
    /// divided by the number of samples, assuming the inner count of the sample
    /// size was already updated.
    ///
    /// This is useful for avoiding unnecessary divisions in the inner loop.
    #[inline]
    fn add_inner(&mut self, delta: f64, delta_n: f64) {
        // This algorithm was suggested by Terriberry.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        let n = f64::approx_from(self.len()).unwrap();
        let term = delta * delta_n * (n - 1.);
        self.sum_3 += term * delta_n * (n - 2.)
            - 3.*delta_n * self.avg.sum_2;
        self.avg.add_inner(delta_n);
    }

    /// Determine whether the sample is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.avg.is_empty()
    }

    /// Estimate the mean of the population.
    ///
    /// Returns 0 for an empty sample.
    #[inline]
    pub fn mean(&self) -> f64 {
        self.avg.mean()
    }

    /// Return the sample size.
    #[inline]
    pub fn len(&self) -> u64 {
        self.avg.len()
    }

    /// Calculate the sample variance.
    ///
    /// This is an unbiased estimator of the variance of the population.
    #[inline]
    pub fn sample_variance(&self) -> f64 {
        self.avg.sample_variance()
    }

    /// Calculate the population variance of the sample.
    ///
    /// This is a biased estimator of the variance of the population.
    #[inline]
    pub fn population_variance(&self) -> f64 {
        self.avg.population_variance()
    }

    /// Estimate the standard error of the mean of the population.
    #[inline]
    pub fn error_mean(&self) -> f64 {
        self.avg.error()
    }

    /// Estimate the skewness of the population.
    #[inline]
    pub fn skewness(&self) -> f64 {
        if self.sum_3 == 0. {
            return 0.;
        }
        let n = f64::approx_from(self.len()).unwrap();
        let sum_2 = self.avg.sum_2;
        debug_assert_ne!(sum_2, 0.);
        n.sqrt() * self.sum_3 / (sum_2*sum_2*sum_2).sqrt()
    }

    /// Merge another sample into this one.
    #[inline]
    pub fn merge(&mut self, other: &Skewness) {
        let len_self = f64::approx_from(self.len()).unwrap();
        let len_other = f64::approx_from(other.len()).unwrap();
        let len_total = len_self + len_other;
        let delta = other.mean() - self.mean();
        let delta_n = delta / len_total;
        self.sum_3 += other.sum_3
            + delta*delta_n*delta_n * len_self*len_other*(len_self - len_other)
            + 3.*delta_n * (len_self * other.avg.sum_2 - len_other * self.avg.sum_2);
        self.avg.merge(&other.avg);
    }
}

impl_from_iterator!(Skewness);
