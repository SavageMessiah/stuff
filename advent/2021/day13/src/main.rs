use std::collections::HashSet;

use anyhow::anyhow;

type Dot = [usize; 2];

static X: usize = 0;
static Y: usize = 1;

type Sheet = HashSet<Dot>;

fn bounds(sheet: &Sheet) -> Dot {
    [sheet.iter().map(|d| d[0]).max().unwrap(),
     sheet.iter().map(|d| d[1]).max().unwrap()]
}

fn print_sheet(sheet: &Sheet) {
    let bounds = bounds(sheet);
    for y in 0..=bounds[Y] {
        for x in 0..=bounds[X] {
            print!("{}", if sheet.contains(&[x, y]) {
                '#'
            } else {
                '.'
            });
        }
        println!("");
    }
}

#[derive(Debug)]
struct Fold {
    axis: usize,
    n: usize
}

impl Fold {
    fn fold(&self, sheet: &Sheet) -> Sheet {
        let mut folded = HashSet::new();
        for dot in sheet {
            let mut folded_dot = dot.clone();
            if folded_dot[self.axis] > self.n {
                folded_dot[self.axis] = self.n - (folded_dot[self.axis] - self.n);
            }
            folded.insert(folded_dot);
        }

        folded
    }
}

fn parse_input(input: &str) -> anyhow::Result<(Sheet, Vec<Fold>)> {
    let (dots, folds) = input.split_once("\n\n").ok_or(anyhow!("missing fold section"))?;
    let dots = dots.lines().map(|l| {
        let (x, y) = l.split_once(',').ok_or(anyhow!("bad dot"))?;
        Ok([x.parse()?, y.parse()?])
    }).collect::<anyhow::Result<Sheet>>()?;
    let folds = folds.lines().map(|l| {
        let fold = l.strip_prefix("fold along ").ok_or(anyhow!("bad fold"))?;
        let (axis, n) = fold.split_once('=').ok_or(anyhow!("bad fold spec {}", fold))?;
        Ok(Fold {
            axis: match axis {
                "x" => X,
                "y" => Y,
                s => return Err(anyhow!("bad axis {}", s))
            },
            n: n.parse()?
        })
    }).collect::<anyhow::Result<Vec<_>>>()?;
    Ok((dots, folds))
}

#[test]
fn test_parse_and_answer() {
    let (mut sheet, folds) = parse_input("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5").unwrap();


    print_sheet(&sheet);
    println!("");

    sheet = folds[0].fold(&sheet);

    print_sheet(&sheet);

    assert_eq!(sheet.len(), 17);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (mut sheet, folds) = parse_input(&input)?;
    sheet = folds[0].fold(&sheet);

    println!("{}", sheet.len());

    Ok(())
}
