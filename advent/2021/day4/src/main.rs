use anyhow::anyhow;

#[derive(Copy, Clone, Debug)]
struct Square {
    marked: bool,
    number: u32
}

type Board = [[Square; 5]; 5];


fn parse_board(s: &str) -> anyhow::Result<Board> {
    let mut board = [[Square {
        marked: false,
        number: 0,
    }; 5]; 5];

    for (y, l) in s.lines().enumerate() {
        for (x, n) in l.split_whitespace().enumerate() {
            let n = n.parse()?;
            board[y][x].number = n;
        }
    }
    Ok(board)
}

fn parse_input(s: &str) -> anyhow::Result<(Vec<u32>, Vec<Board>)> {
    let mut sections = s.split("\n\n");
    let first = sections.next().ok_or(anyhow!("no first section"))?;
    let numbers = first.split(',').map(|n| n.parse()).collect::<Result<Vec<u32>, _>>()?;
    let boards = sections.map(|s| parse_board(s)).collect::<Result<Vec<Board>, _>>()?;

    Ok((numbers, boards))
}

fn is_winner(board: &Board) -> bool {
    //check rows
    for row in board {
        if row.iter().all(|s| s.marked) {
            return true;
        }
    }
    //check cols
    for x in 0..5 {
        let mut marked = true;
        for y in 0..5 {
            marked &= board[y][x].marked;
        }
        if marked {
            return true;
        }
    }

    return false;
}

fn mark(board: &mut Board, n: u32) {
    for row in board {
        for s in row {
            if s.number == n {
                s.marked = true;
            }
        }
    }
}

fn score(board: &Board) -> u32 {
    let mut score = 0;
    for row in board {
        for s in row {
            if !s.marked {
                score += s.number;
            }
        }
    }
    score
}

fn play(boards: &mut Vec<Board>, numbers: &[u32]) -> u32 {
    for n in numbers {
        for board in &mut *boards {
            mark(board, *n);
            if is_winner(board) {
                return score(board) * *n;
            }
        }
    }
    unreachable!()
}

#[test]
fn test_play() {
    let (numbers, mut boards) = parse_input("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7").unwrap();
    assert_eq!(play(&mut boards, &numbers), 4512);
}


fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (numbers, mut boards) = parse_input(&input)?;
    let score = play(&mut boards, &numbers);

    println!("{}", score);

    Ok(())
}
