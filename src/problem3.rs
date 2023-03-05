#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

pub type Move = (Peg, Peg);

pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    if num_discs == 1 {
        moves.push((src, dst));
    } else {
        moves.append(&mut hanoi(num_discs - 1, src, aux, dst));
        moves.append(&mut hanoi(num_discs - 1, aux, dst, src));
    }

    moves
}