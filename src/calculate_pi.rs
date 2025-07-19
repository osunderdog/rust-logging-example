
use log::{debug, info};

pub fn get_pi_const() -> f64 {
    debug!("Retrieving constant for pi...");
    std::f64::consts::PI
}

pub fn get_pi_derive() -> f64 {
    //AI generated
    // Derive pi using Archimedes' method of inscribing and circumscribing
    // polygons in a circle. We start with a hexagon and double the number of
    // sides four times to reach a 96-sided polygon, just as Archimedes did.
    debug!("Calculating pi using Archimedes' method...");

    let mut num_sides = 6.0;
    // For a unit circle (radius = 1):
    // Perimeter of inscribed hexagon is 6 * r = 6
    let mut inscribed_perimeter = 6.0;
    // Perimeter of circumscribed hexagon is 6 * (2/sqrt(3)) * r = 4 * sqrt(3)
    let mut circumscribed_perimeter = 4.0 * 3.0_f64.sqrt();

    // We will perform 4 iterations to go from 6 -> 12 -> 24 -> 48 -> 96 sides.
    for i in 0..4 {
        // These are the recursive formulas to find the perimeters of a polygon
        // with 2n sides from a polygon with n sides.
        circumscribed_perimeter = (2.0 * inscribed_perimeter * circumscribed_perimeter)
            / (inscribed_perimeter + circumscribed_perimeter);
        inscribed_perimeter = (inscribed_perimeter * circumscribed_perimeter).sqrt();

        num_sides *= 2.0;

        debug!(
            "Iteration {}: sides={}, π lower bound ≈ {:.8}, π upper bound ≈ {:.8}",
            i + 1,
            num_sides,
            inscribed_perimeter / 2.0, // π ≈ perimeter / diameter, and r=1, d=2
            circumscribed_perimeter / 2.0,
        );
    }

    // The final approximation is the average of the lower and upper bounds.
    let pi_approximation = (inscribed_perimeter + circumscribed_perimeter) / 4.0;

    info!("Derived π with {}-sided polygon: {}", num_sides, pi_approximation);

    pi_approximation
}