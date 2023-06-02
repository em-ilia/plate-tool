use serde::{Deserialize, Serialize};

use super::plate::Plate;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Region {
    Rect((u8, u8), (u8, u8)),
    Point((u8, u8)),
}
impl Default for Region {
    fn default() -> Self {
        Region::Point((1, 1))
    }
}
impl TryFrom<Region> for ((u8, u8), (u8, u8)) {
    type Error = &'static str;
    fn try_from(region: Region) -> Result<Self, Self::Error> {
        if let Region::Rect(c1, c2) = region {
            Ok((c1, c2))
        } else {
            // Should consider returning a degenerate rectangle here instead
            Err("Cannot convert this region to a rectangle, it was a point.")
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct TransferRegion {
    pub source_plate: Plate,
    pub source_region: Region, // Even if it is just a point, we don't want corners.
    pub dest_plate: Plate,
    pub dest_region: Region,
    pub interleave_source: (i8, i8),
    pub interleave_dest: (i8, i8),
}

impl Default for TransferRegion {
    fn default() -> Self {
        TransferRegion {
            source_plate: Plate::default(),
            source_region: Region::default(),
            dest_plate: Plate::default(),
            dest_region: Region::default(),
            interleave_source: (1, 1),
            interleave_dest: (1, 1),
        }
    }
}

impl TransferRegion {
    pub fn get_source_wells(&self) -> Vec<(u8, u8)> {
        match self.source_region {
            Region::Rect(c1, c2) => {
                let mut wells = Vec::<(u8, u8)>::new();
                let (ul, br) = standardize_rectangle(&c1, &c2);
                let (interleave_i, interleave_j) = self.interleave_source;
                // NOTE: This will panic if either is 0!
                // We'll reassign these values (still not mutable) just in case.
                // This behaviour shouldn't be replicated for destination wells
                // because a zero step permits pooling.
                let (interleave_i, interleave_j) =
                    (i8::max(interleave_i, 1), i8::max(interleave_j, 1));

                for i in (ul.0..=br.0).step_by(i8::abs(interleave_i) as usize) {
                    for j in (ul.1..=br.1).step_by(i8::abs(interleave_j) as usize) {
                        // NOTE: It looks like we're ignoring negative interleaves,
                        // because it wouldn't make a difference here---the same
                        // wells will still be involved in the transfer.
                        wells.push((i, j))
                    }
                }
                return wells;
            }
            Region::Point(p) => return vec![p],
        }
    }

    pub fn get_destination_wells(&self) -> Vec<(u8, u8)> {
        let map = self.calculate_map();
        let source_wells = self.get_source_wells();

        let mut wells = Vec::<(u8, u8)>::new();

        // log::debug!("GDW:");
        for well in source_wells {
            if let Some(mut dest_wells) = map(well) {
                // log::debug!("Map {:?} to {:?}", well, dest_wells);
                wells.append(&mut dest_wells);
            }
        }
        // log::debug!("GDW END.");

        return wells;
    }

    pub fn calculate_map(&self) -> Box<dyn Fn((u8, u8)) -> Option<Vec<(u8, u8)>> + '_> {
        // By validating first, we have a stronger guarantee that
        // this function will not panic. :)
        // log::debug!("Validating: {:?}", self.validate());
        if let Err(msg) = self.validate() {
            eprintln!("{}", msg);
            eprintln!("This transfer will be empty.");
            return Box::new(|(_, _)| None);
        }

        // log::debug!("What is ild? {:?}", self);
        let source_wells = self.get_source_wells();
        let il_dest = self.interleave_dest;
        let il_source = self.interleave_source;

        let source_corners: ((u8, u8), (u8, u8)) = match self.source_region {
            Region::Point((x, y)) => ((x, y), (x, y)),
            Region::Rect(c1, c2) => (c1, c2),
        };
        let (source_ul, _) = standardize_rectangle(&source_corners.0, &source_corners.1);
        // This map is not necessarily injective or surjective,
        // but we will have these properties in certain cases.
        // If the transfer is not a pooling transfer (interleave == 0)
        // and simple then we *will* have injectivity.

        // Non-replicate transfers:
        match self.dest_region {
            Region::Point((x, y)) => {
                return Box::new(move |(i, j)| {
                    if source_wells.contains(&(i, j)) {
                        // Validity here already checked by self.validate()
                        Some(vec![(
                            x + i
                                .checked_sub(source_ul.0)
                                .expect("Point cannot have been less than UL")
                                .checked_div(il_source.0.abs() as u8)
                                .expect("Source interleave cannot be 0")
                                .mul(il_dest.0.abs() as u8),
                            y + j
                                .checked_sub(source_ul.1)
                                .expect("Point cannot have been less than UL")
                                .checked_div(il_source.1.abs() as u8)
                                .expect("Source interleave cannot be 0")
                                .mul(il_dest.1.abs() as u8),
                        )])
                    } else {
                        None
                    }
                });
            }
            Region::Rect(c1, c2) => {
                return Box::new(move |(i, j)| {
                    if source_wells.contains(&(i, j)) {
                        let possible_destination_wells = create_dense_rectangle(&c1, &c2);
                        let (d_ul, d_br) = standardize_rectangle(&c1, &c2);
                        let (s_ul, s_br) =
                            standardize_rectangle(&source_corners.0, &source_corners.1);
                        let s_dims = (
                            s_br.0.checked_sub(s_ul.0).unwrap() + 1,
                            s_br.1.checked_sub(s_ul.1).unwrap() + 1,
                        );
                        let d_dims = (
                            d_br.0.checked_sub(d_ul.0).unwrap() + 1,
                            d_br.1.checked_sub(d_ul.1).unwrap() + 1,
                        );
                        let N_s = (
                            // Number of used source wells
                            (s_dims.0 + il_source.0.abs() as u8 - 1)
                                .div_euclid(il_source.0.abs() as u8),
                            (s_dims.1 + il_source.1.abs() as u8 - 1)
                                .div_euclid(il_source.1.abs() as u8),
                        );
                        let D_per_replicate = (
                            // How many wells are used per replicate?
                            (N_s.0 * (il_dest.0.abs() as u8)),
                            (N_s.1 * (il_dest.1.abs() as u8)),
                        );
                        let count = (
                            // How many times can we replicate?
                            (1..)
                                .position(|n| {
                                    n * N_s.0 * il_dest.0.abs() as u8 - il_dest.0.abs() as u8 + 1
                                        > d_dims.0
                                })
                                .unwrap() as u8,
                            (1..)
                                .position(|n| {
                                    n * N_s.1 * il_dest.1.abs() as u8 - il_dest.1.abs() as u8 + 1
                                        > d_dims.1
                                })
                                .unwrap() as u8,
                        );
                        let i = i
                            .saturating_sub(s_ul.0)
                            .saturating_div(il_source.0.abs() as u8);
                        let j = j
                            .saturating_sub(s_ul.1)
                            .saturating_div(il_source.1.abs() as u8);
                        // log::debug!("N_s: {:?}, d_dims: {:?}, D_per: {:?}, count: {:?}", N_s, d_dims, D_per_replicate, count);

                        Some(
                            possible_destination_wells
                                .into_iter()
                                .filter(|(x, _)| {
                                    x.checked_sub(d_ul.0).unwrap()
                                        % (N_s.0 * il_dest.0.abs() as u8) // Counter along x
                                    == ((il_dest.0.abs() as u8 *i))
                                        % (N_s.0 * il_dest.0.abs() as u8)
                                })
                                .filter(|(_, y)| {
                                    y.checked_sub(d_ul.1).unwrap()
                                        % (N_s.1 * il_dest.1.abs() as u8) // Counter along u
                                    == ((il_dest.1.abs() as u8 *j))
                                        % (N_s.1 * il_dest.1.abs() as u8)
                                })
                                .filter(|(x, y)| {
                                    // How many times have we replicated? < How many are we allowed
                                    // to replicate?
                                    x.checked_sub(d_ul.0)
                                        .unwrap()
                                        .div_euclid(N_s.0 * il_dest.0.abs() as u8)
                                        < count.0
                                        && y.checked_sub(d_ul.1)
                                            .unwrap()
                                            .div_euclid(N_s.1 * il_dest.1.abs() as u8)
                                            < count.1
                                })
                                .collect(),
                        )
                    } else {
                        None
                    }
                });
            }
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        // Checks if the region does anything suspect
        //
        // If validation fails, we pass a string to show to the user.
        //
        // We check:
        //     - Are the wells in the source really there?
        //     - In a replication region, do the source lengths divide the destination lengths?
        //     - Are the interleaves valid?
        let il_source = self.interleave_source;
        let il_dest = self.interleave_dest;

        match self.source_region {
            Region::Point(_) => return Ok(()), // Should make sure it's actually in the plate, leave for
            // later
            Region::Rect(s1, s2) => {
                // Check if all source wells exist:
                if s1.0 == 0 || s1.1 == 0 || s2.0 == 0 || s2.1 == 0 {
                    return Err("Source region is out-of-bounds! (Too small)");
                }
                // Sufficient to check if the corners are in-bounds
                let source_max = self.source_plate.size();
                if s1.0 > source_max.0 || s2.0 > source_max.0 {
                    return Err("Source region is out-of-bounds! (Too tall)");
                }
                if s1.1 > source_max.1 || s2.1 > source_max.1 {
                    // log::debug!("s1.1: {}, max.1: {}", s1.1, source_max.1);
                    return Err("Source region is out-of-bounds! (Too wide)");
                }
                // Check that source lengths divide destination lengths
                /* This section is disabled because it's not
                 * strictly necessary to have this property (divisibility)
                match &self.dest_region {
                    Region::Point(_) => (),
                    Region::Rect(d1, d2) => {
                        // If we consider interleaves, it's slightly more
                        // complicated to compute the true dimensions of
                        // each region.
                        // (dim)*(il) - (il - 1)
                        let dest_dim_i = u8::abs_diff(d1.0, d2.0)+1;
                        let dest_dim_j = u8::abs_diff(d1.1, d2.1)+1;
                        let source_dim_i = ((il_source.0.abs() as u8) * u8::abs_diff(s1.0, s2.0))
                            .checked_sub(il_source.0.abs() as u8 - 1)
                            .expect("Dimension is somehow negative?")
                            + 1;
                        let source_dim_j = ((il_source.1.abs() as u8) * u8::abs_diff(s1.1, s2.1))
                            .checked_sub(il_source.1.abs() as u8 - 1)
                            .expect("Dimension is somehow negative?")
                            + 1;

                        if dest_dim_i % (source_dim_i+il_dest.0.abs() as u8) != 0 {
                            eprintln!("{} % {} = {}", dest_dim_i,
                                source_dim_i+il_dest.0.abs() as u8,
                                dest_dim_i % (source_dim_i+il_dest.0.abs() as u8));
                            return Err("Replicate region has indivisible height!");
                        }
                        if dest_dim_j % (source_dim_j+il_dest.1.abs() as u8) != 0 {
                            eprintln!("{} {}", source_dim_j, source_dim_j);
                            return Err("Replicate region has indivisible width!");
                        }
                    }
                }
                */
            }
        }

        if il_source.0 == 0 || il_dest.1 == 0 {
            return Err("Source interleave cannot be zero!");
        }

        // Check if all destination wells exist:
        // NOT IMPLEMENTED
        // Should *not* happen in this function---otherwise
        // we'd get a nasty recursive loop.

        return Ok(());
    }
}

fn in_region(pt: (u8, u8), r: &Region) -> bool {
    match r {
        Region::Rect(c1, c2) => {
            pt.0 <= u8::max(c1.0, c2.0)
                && pt.0 >= u8::min(c1.0, c2.0)
                && pt.1 <= u8::max(c1.1, c2.1)
                && pt.1 >= u8::min(c1.1, c2.1)
        }
        Region::Point((i, j)) => pt.0 == *i && pt.1 == *j,
    }
}

fn create_dense_rectangle(c1: &(u8, u8), c2: &(u8, u8)) -> Vec<(u8, u8)> {
    // Creates a vector of every point between two corners
    let (c1, c2) = standardize_rectangle(c1, c2);

    let mut points = Vec::<(u8, u8)>::new();
    for i in c1.0..=c2.0 {
        for j in c1.1..=c2.1 {
            points.push((i, j));
        }
    }

    return points;
}

fn standardize_rectangle(c1: &(u8, u8), c2: &(u8, u8)) -> ((u8, u8), (u8, u8)) {
    let upper_left_i = u8::min(c1.0, c2.0);
    let upper_left_j = u8::min(c1.1, c2.1);
    let bottom_right_i = u8::max(c1.0, c2.0);
    let bottom_right_j = u8::max(c1.1, c2.1);
    return (
        (upper_left_i, upper_left_j),
        (bottom_right_i, bottom_right_j),
    );
}

#[cfg(debug_assertions)]
use std::fmt;
use std::ops::Mul;

#[cfg(debug_assertions)] // There should be no reason to print a transfer otherwise
impl fmt::Display for TransferRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Source Plate:")?;
        let source_dims = self.source_plate.size();
        let source_wells = self.get_source_wells();
        let mut source_string = String::new();
        for i in 1..=source_dims.0 {
            for j in 1..=source_dims.1 {
                if source_wells.contains(&(i, j)) {
                    source_string.push_str("x")
                } else {
                    source_string.push_str(".")
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
                if dest_wells.contains(&(i, j)) {
                    dest_string.push_str("x")
                } else {
                    dest_string.push_str(".")
                }
            }
            dest_string.push_str("\n");
        }
        write!(f, "{}", dest_string)
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use crate::data::plate::*;
    use crate::data::transfer_region::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_simple_transfer() {
        let source = Plate::new(PlateType::Source, PlateFormat::W96);
        let destination = Plate::new(PlateType::Destination, PlateFormat::W384);

        let transfer1 = TransferRegion {
            source_plate: source,
            source_region: Region::Rect((1, 1), (3, 3)),
            dest_plate: destination,
            dest_region: Region::Point((3, 3)),
            interleave_source: (1, 1),
            interleave_dest: (1, 1),
        };
        let transfer1_map = transfer1.calculate_map();
        assert_eq!(
            transfer1_map((1, 1)),
            Some(vec! {(3,3)}),
            "Failed basic shift transfer 1"
        );
        assert_eq!(
            transfer1_map((1, 2)),
            Some(vec! {(3,4)}),
            "Failed basic shift transfer 2"
        );
        assert_eq!(
            transfer1_map((2, 2)),
            Some(vec! {(4,4)}),
            "Failed basic shift transfer 3"
        );

        let transfer2 = TransferRegion {
            source_plate: source,
            source_region: Region::Rect((1, 1), (3, 3)),
            dest_plate: destination,
            dest_region: Region::Point((3, 3)),
            interleave_source: (2, 2),
            interleave_dest: (1, 1),
        };
        let transfer2_map = transfer2.calculate_map();
        assert_eq!(
            transfer2_map((1, 1)),
            Some(vec! {(3,3)}),
            "Failed source interleave, type simple 1"
        );
        assert_eq!(
            transfer2_map((1, 2)),
            None,
            "Failed source interleave, type simple 2"
        );
        assert_eq!(
            transfer2_map((2, 2)),
            None,
            "Failed source interleave, type simple 3"
        );
        assert_eq!(
            transfer2_map((3, 3)),
            Some(vec! {(4,4)}),
            "Failed source interleave, type simple 4"
        );

        let transfer3 = TransferRegion {
            source_plate: source,
            source_region: Region::Rect((1, 1), (3, 3)),
            dest_plate: destination,
            dest_region: Region::Point((3, 3)),
            interleave_source: (1, 1),
            interleave_dest: (2, 3),
        };
        let transfer3_map = transfer3.calculate_map();
        assert_eq!(
            transfer3_map((1, 1)),
            Some(vec! {(3,3)}),
            "Failed destination interleave, type simple 1"
        );
        assert_eq!(
            transfer3_map((2, 1)),
            Some(vec! {(5,3)}),
            "Failed destination interleave, type simple 2"
        );
        assert_eq!(
            transfer3_map((1, 2)),
            Some(vec! {(3,6)}),
            "Failed destination interleave, type simple 3"
        );
        assert_eq!(
            transfer3_map((2, 2)),
            Some(vec! {(5,6)}),
            "Failed destination interleave, type simple 4"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_replicate_transfer() {
        let source = Plate::new(PlateType::Source, PlateFormat::W96);
        let destination = Plate::new(PlateType::Destination, PlateFormat::W384);

        let transfer1 = TransferRegion {
            source_plate: source,
            source_region: Region::Rect((1, 1), (2, 2)),
            dest_plate: destination,
            dest_region: Region::Rect((2, 2), (11, 11)),
            interleave_source: (1, 1),
            interleave_dest: (3, 3),
        };
        let transfer1_map = transfer1.calculate_map();
        assert_eq!(
            transfer1_map((1, 1)),
            Some(vec! {(2, 2), (2, 8), (8, 2), (8, 8)}),
            "Failed type replicate 1"
        );
        assert_eq!(
            transfer1_map((2, 1)),
            Some(vec! {(5, 2), (5, 8), (11, 2), (11, 8)}),
            "Failed type replicate 1"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_pooling_transfer() {}
}
