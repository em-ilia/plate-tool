use super::plate::Plate;

enum Region {
    Rect((u8,u8),(u8,u8)),
    Point(u8,u8)
}

struct TransferRegion {
    source_plate: &Plate,
    source_region: Region::Rect, // Even if it is just a point, we don't want corners.
    dest_plate: &Plate,
    dest_region: Region,
    offset: Option<(u8,u8)>,
}

impl TransferRegion {
    pub fn validate(&self) -> Result<(), String> {
        /// Checks if the region does anything suspect
        ///
        /// If validation fails, we pass a string to show to the user.
        ///
        /// We check:
        ///     - Are the wells in the source really there?
        ///     - Are the wells in the destination there? (Sometimes running OOB is okay though?)
        ///     - In a replication region, do the source lengths divide the destination lengths?

        // Easy checks:
        if self.source_region.0.0 == 0 || self.source_region.0.1 == 0
            || self.source_region.1.0 == 0 || self.source_region.1.1 == 0 {
                Err("Source region is out-of-bounds! (Too small)")
            }

        // Check if all source wells exist:
        // Sufficient to check if the corners are in-bounds
        let source_max = self.source_plate.size();
        if self.source_region.0.0 > source_max.0 ||
            self.source_region.1.0 > source_max.0 {
                Err("Source region is out-of-bounds! (Too tall)")
            }
        if self.source_region.0.1 > source_max.1 ||
            self.source_region.1.1 > source_max.1 {
                Err("Source region is out-of-bounds! (Too wide)")
            }

        // Check if all destination wells exist:
        // NOT IMPLEMENTED

        // Check that source lengths divide destination lengths
        match self.dest_region {
            Point => (),
            Region::Rect(c1, c2) => {
                let dest_diff_i = u8::abs_diff(c1.0, c2.0);
                let dest_diff_j = u8::abs_diff(c1.1, c2.1);

                let source_diff_i = u8::abs_diff(self.source_region.0.0, self.source_region.1.0);
                let source_diff_j = u8::abs_diff(self.source_region.0.1, self.source_region.1.1);

                if source_diff_i % dest_diff_i != 0 {
                    Err("Replicate region has indivisible height!")
                }
                if source_diff_j % dest_diff_j != 0 {
                    Err("Replicate region has indivisible width!")
                }
            }
        }

        return Ok(())
    }
}
