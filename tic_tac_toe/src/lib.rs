pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if horizontal('X', table) || vertical('X', table) || diagonals('X', table) {
        return "player X won".to_string();
    } else if horizontal('O', table) || vertical('O', table) || diagonals('O', table) {
        return "player O won".to_string();
    }
    return "tie".to_string();
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let main = table[0][0] == player && table[1][1] == player && table[2][0] == player;
    let anti = table[0][2] == player && table[1][1] == player && table[0][2] == player;
    main || anti
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for _ in 0..3 {
        if table[0][0] == player && table[0][1] == player && table[0][2] == player {
            return true;
        }
    }
    return false;
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for _ in 0..3 {
        if table[0][0] == player && table[1][0] == player && table[2][0] == player {
            return true;
        }
    }
    return false;
}
