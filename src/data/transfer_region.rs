use super::plate::Plate;

pub enum Region {
    Rect((u8,u8),(u8,u8)),
    Point(u8,u8)
}

pub struct TransferRegion<'a> {
    pub source_plate: &'a Plate,
    pub source_region: Region, // Even if it is just a point, we don't want corners.
    pub dest_plate: &'a Plate,
    pub dest_region: Region,
    pub offset: Option<(i8,i8)>,
}

impl TransferRegion<'_> {
    pub fn validate(&self) -> Result<(), String> {
        // Checks if the region does anything suspect
        //
        // If validation fails, we pass a string to show to the user.
        //
        // We check:
        //     - Are the wells in the source really there?
        //     - Are the wells in the destination there? (Sometimes running OOB is okay though?)
        //     - In a replication region, do the source lengths divide the destination lengths?

        // Easy checks:
        match self.source_region {
            Region::Point(_, _) => return Err("Source region should not be a point!".to_string()),
            Region::Rect(c1, c2) => {
                // Check if all source wells exist:
                if c1.0 == 0 || c1.1 == 0
                    || c2.0 == 0 || c2.1 == 0 {
                        return Err("Source region is out-of-bounds! (Too small)".to_string())
                    }
                // Sufficient to check if the corners are in-bounds
                let source_max = self.source_plate.size();
                if c1.0 > source_max.0 ||
                    c2.0 > source_max.0 {
                        return Err("Source region is out-of-bounds! (Too tall)".to_string())
                    }
                if c1.1 > source_max.1 ||
                    c2.1 > source_max.1 {
                        return Err("Source region is out-of-bounds! (Too wide)".to_string())
                    }
                // Check that source lengths divide destination lengths
                match &self.dest_region {
                    Region::Point(_,_) => (),
                    Region::Rect(c1, c2) => {
                        let dest_diff_i = u8::abs_diff(c1.0, c2.0);
                        let dest_diff_j = u8::abs_diff(c1.1, c2.1);

                        let source_diff_i = u8::abs_diff(c1.0, c2.0);
                        let source_diff_j = u8::abs_diff(c1.1, c2.1);

                        if source_diff_i % dest_diff_i != 0 {
                            return Err("Replicate region has indivisible height!".to_string())
                        }
                        if source_diff_j % dest_diff_j != 0 {
                            return Err("Replicate region has indivisible width!".to_string())
                        }
                    }
                }
            }
        }


        // Check if all destination wells exist:
        // NOT IMPLEMENTED


        return Ok(())
    }
}

fn in_region(pt: (u8,u8), r: &Region) -> bool {
    match r {
        Region::Rect(c1, c2) => {
            pt.0 <= u8::max(c1.0, c2.0)
            && pt.0 >= u8::min(c1.0, c2.0)
            && pt.1 <= u8::max(c1.1, c2.1)
            && pt.1 >= u8::min(c1.1, c2.1)
        },
        Region::Point(i, j) => {
            pt.0 == *i && pt.1 == *j
        }
    }
}

#[cfg(debug_assertions)]
use std::fmt;

#[cfg(debug_assertions)]
impl fmt::Display for TransferRegion<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Source Plate:")?;
        let source_dims = self.source_plate.size();
        let mut source_string = String::new();
        for i in 1..=source_dims.0 {
            for j in 1..=source_dims.1 {
                if in_region((i,j), &self.source_region) {
                    source_string.push_str("x")
                } else {
                    source_string.push_str("o")
                }
            }
            source_string.push_str("\n");
        }
        write!(f, "{}", source_string)
    }
}
