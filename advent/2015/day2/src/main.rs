use anyhow::{anyhow, Result};

struct Package {
    l: u32,
    w: u32,
    h: u32,
}

type Side = (u32, u32);

fn area(side: &Side) -> u32 {
    side.0 * side.1
}

fn perimeter(side: &Side) -> u32 {
    2 * (side.0 + side.1)
}

impl Package {
    fn sides(&self) -> Vec<Side> {
        [(self.l, self.w),
         (self.l, self.w),
         (self.l, self.h),
         (self.l, self.h),
         (self.w, self.h),
         (self.w, self.h)].to_vec()
    }

    fn paper_needed(&self) -> u32 {
        self.sides().iter().map(area).sum::<u32>() +
            self.sides().iter().map(area).min().unwrap_or(0)
    }

    fn ribbon_needed(&self) -> u32 {
        self.sides().iter().map(perimeter).min().unwrap_or(0) +
            (self.l * self.w * self.h)
    }
}

#[test]
fn package_needed() {
    assert_eq!(Package{l: 2, w: 3, h: 4}.paper_needed(), 58);
    assert_eq!(Package{l: 2, w: 3, h: 4}.ribbon_needed(), 34);
    assert_eq!(Package{l: 1, w: 1, h: 10}.ribbon_needed(), 14);

}

fn parse_package(line: &str) -> Result<Package> {
    let dims : Vec<u32> = line.split("x")
                              .map(|d| d.parse::<u32>())
                              .collect::<Result<_, _>>()?;
    if dims.len() != 3 {
        return Err(anyhow!("Expected 3 dimensions, got {}", dims.len()))
    }

    Ok(Package {l: dims[0], w: dims[1], h: dims[2]})
}

fn main() -> anyhow::Result<()> {
    let packages = include_str!("input.txt")
        .split("\n")
        .map(parse_package)
        .collect::<Result<Vec<Package>, _>>()?;

    let paper_needed: u32 = packages.iter().map(Package::paper_needed).sum();
    let ribbon_needed: u32 = packages.iter().map(Package::ribbon_needed).sum();

    println!("Paper needed: {} sq ft", paper_needed);
    println!("Ribbon needed: {} ft", ribbon_needed);

    Ok(())
}
