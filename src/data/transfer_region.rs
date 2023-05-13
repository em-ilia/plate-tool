use super::plate::Plate;

#[derive(Clone, Copy)]
pub enum Region {
    Rect((u8,u8),(u8,u8)),
    Point((u8,u8))
}
impl TryFrom<Region> for ((u8,u8),(u8,u8)) {
    type Error = &'static str;
    fn try_from(region: Region) -> Result<Self, Self::Error> {
        if let Region::Rect(c1, c2) = region {
            Ok((c1,c2))
        } else {
            // Should consider returning a degenerate rectangle here instead
            Err("Cannot convert this region to a rectangle, it was a point.")
        }
    }
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
    pub fn get_destination_wells(&self) -> Vec<(u8,u8)> {
        let map = self.calculate_map();
        let source_wells = self.get_source_wells();

        let mut wells = Vec::<(u8,u8)>::new();
        
        for well in source_wells {
            if let Some(dest_well) = map(well) {
                wells.push(dest_well)
            }
        }

        return wells;
    }

    pub fn calculate_map(&self) -> Box<dyn Fn((u8,u8)) -> Option<(u8,u8)> + '_> {
        // By validating first, we have a stronger guarantee that
        // this function will not panic. :)
        if let Err(msg) = self.validate() {
            eprintln!("{}", msg);
            eprintln!("This transfer will be empty.");
            return Box::new(|(_,_)| None)
        }

        let source_wells = self.get_source_wells();
        let il_dest = self.interleave_dest.unwrap_or((1,1));


        let source_corners: ((u8,u8),(u8,u8)) = self.source_region.try_into()
                                                   .expect("Source region should not be a point");
        let (source_ul, _) = standardize_rectangle(&source_corners.0, &source_corners.1);
        // This map is not necessarily injective or surjective,
        // but we will have these properties in certain cases.
        // If the transfer is not a pooling transfer (interleave == 0)
        // then we *will* have injectivity.

        // Non-replicate transfers:
        match self.dest_region {
            Region::Point((x,y)) => return Box::new(move |(i,j)| {
                if source_wells.contains(&(i,j)) {
                    let il_source = self.interleave_source.unwrap_or((1,1));
                    // Validity here already checked by self.validate()
                    Some((
                            x + i.checked_sub(source_ul.0).expect("Point cannot have been less than UL")
                                .checked_div(il_source.0.abs() as u8).expect("Source interleave cannot be 0")
                                    .mul(il_dest.0.abs() as u8),
                            y + j.checked_sub(source_ul.1).expect("Point cannot have been less than UL")
                                .checked_div(il_source.1.abs() as u8).expect("Source interleave cannot be 0")
                                    .mul(il_dest.1.abs() as u8),
                            ))
                } else { None }
            }),
            Region::Rect(c1, c2) => return Box::new(move |(i,j)| {
                None
            })
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        // Checks if the region does anything suspect
        //
        // If validation fails, we pass a string to show to the user.
        //
        // We check:
        //     - Are the wells in the source really there?
        //     - Are the wells in the destination there? (Sometimes running OOB is okay though?)
        //     - In a replication region, do the source lengths divide the destination lengths?
        //     - Are the interleaves valid?
        let il_source = self.interleave_source.unwrap_or((1,1));
        let il_dest = self.interleave_dest.unwrap_or((1,1));

        match self.source_region {
            Region::Point(_) => return Err("Source region should not be a point!"),
            Region::Rect(s1, s2) => {
                // Check if all source wells exist:
                if s1.0 == 0 || s1.1 == 0
                    || s2.0 == 0 || s2.1 == 0 {
                        return Err("Source region is out-of-bounds! (Too small)")
                    }
                // Sufficient to check if the corners are in-bounds
                let source_max = self.source_plate.size();
                if s1.0 > source_max.0 ||
                    s2.0 > source_max.0 {
                        return Err("Source region is out-of-bounds! (Too tall)")
                    }
                if s1.1 > source_max.1 ||
                    s2.1 > source_max.1 {
                        return Err("Source region is out-of-bounds! (Too wide)")
                    }
                // Check that source lengths divide destination lengths
                match &self.dest_region {
                    Region::Point(_) => (),
                    Region::Rect(d1, d2) => {
                        // If we consider interleaves, it's slightly more
                        // complicated to compute the true dimensions of
                        // each region.
                        // (dim)*(il) - (il - 1)
                        let dest_diff_i = ((il_dest.0.abs() as u8)*u8::abs_diff(d1.0, d2.0))
                                            .checked_sub(il_dest.0.abs() as u8 - 1)
                                            .expect("Dimension is somehow negative?");
                        let dest_diff_j = ((il_dest.1.abs() as u8)*u8::abs_diff(d1.1, d2.1))
                                            .checked_sub(il_dest.1.abs() as u8 - 1)
                                            .expect("Dimension is somehow negative?");
                        let source_diff_i = ((il_source.0.abs() as u8)*u8::abs_diff(s1.0, s2.0))
                                            .checked_sub(il_source.0.abs() as u8 - 1)
                                            .expect("Dimension is somehow negative?");
                        let source_diff_j = ((il_source.1.abs() as u8)*u8::abs_diff(s1.1, s2.1))
                                            .checked_sub(il_source.1.abs() as u8 - 1)
                                            .expect("Dimension is somehow negative?");


                        if source_diff_i % dest_diff_i != 0 {
                            return Err("Replicate region has indivisible height!")
                        }
                        if source_diff_j % dest_diff_j != 0 {
                            return Err("Replicate region has indivisible width!")
                        }
                    }
                }
            }
        }

        if let Some(source_il) = self.interleave_source {
            if source_il.0 == 0 || source_il.1 == 0 {
                return Err("Source interleave cannot be zero!")
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
        Region::Point((i, j)) => {
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
use std::ops::Mul;

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
        write!(f, "{}", source_string)?;

        writeln!(f, "Dest Plate:")?;
        let dest_dims = self.dest_plate.size();
        let dest_wells = self.get_destination_wells();
        let mut dest_string = String::new();
        for i in 1..=dest_dims.0 {
            for j in 1..=dest_dims.1 {
                if dest_wells.contains(&(i,j)) {
                    dest_string.push_str("x")
                } else {
                    dest_string.push_str("o")
                }
            }
            dest_string.push_str("\n");
        }
        write!(f, "{}", dest_string)
    }
}
