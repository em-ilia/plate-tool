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
    pub interleave_source: Option<(i8,i8)>,
    pub interleave_dest: Option<(i8,i8)>,
}

impl TransferRegion<'_> {
    pub fn get_source_wells(&self) -> Vec<(u8,u8)> {
        if let Region::Rect(c1, c2) = self.source_region {
            let mut wells = Vec::<(u8,u8)>::new();
            let (ul, br) = standardize_rectangle(&c1, &c2);
            let (interleave_i, interleave_j) = self.interleave_source.unwrap_or((1,1));
            // NOTE: This will panic if either is 0!
            // We'll reassign these values (still not mutable) just in case.
            // This behaviour shouldn't be replicated for destination wells
            // because a zero step permits pooling.
            let (interleave_i, interleave_j) = (i8::max(interleave_i, 1), i8::max(interleave_j, 1));

            for i in (ul.0..=br.0).step_by(i8::abs(interleave_i) as usize) {
                for j in (ul.0..=br.0).step_by(i8::abs(interleave_j) as usize) {
            // NOTE: It looks like we're ignoring negative interleaves,
            // because it wouldn't make a difference here---the same
            // wells will still be involved in the transfer.
                    wells.push((i,j))
                }
            }
            return wells;
        } else { panic!("Source region is just a point!") }
    }

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

fn standardize_rectangle(c1: &(u8,u8), c2: &(u8,u8)) -> ((u8,u8),(u8,u8)) {
    let upper_left_i = u8::min(c1.0, c2.0);
    let upper_left_j = u8::min(c1.1, c2.1);
    let bottom_right_i = u8::max(c1.0, c2.0);
    let bottom_right_j = u8::max(c1.1, c2.1);
    return ((upper_left_i,upper_left_j),(bottom_right_i,bottom_right_j));
}

#[cfg(debug_assertions)]
use std::fmt;

#[cfg(debug_assertions)]
impl fmt::Display for TransferRegion<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Source Plate:")?;
        let source_dims = self.source_plate.size();
        let source_wells = self.get_source_wells();
        let mut source_string = String::new();
        for i in 1..=source_dims.0 {
            for j in 1..=source_dims.1 {
                if source_wells.contains(&(i,j)) {
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
