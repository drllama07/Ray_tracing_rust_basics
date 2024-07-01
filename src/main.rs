use std::io::*;
//use crate::online::connection;

//pub mod online;


fn user_input() -> (usize, usize) {
        let mut inputx = String::new();
        stdin().read_line(&mut inputx).unwrap();
        let my_x: usize = inputx.trim().parse().expect("parsing error");
    
        let mut inputy = String::new();
        stdin().read_line(&mut inputy).unwrap();
        let my_y: usize = inputy.trim().parse().expect("parsing error");
        (my_x, my_y)
}

fn graphics(game:&[[&str; 3];3 ]) {
    for i  in 0..game.len(){
        let mut a = String::new();
        a.push_str("|");
        for j  in 0..game[0].len() {
            a.push_str(game[i][j]);
            a.push_str("|");
        }
        println!("{a}")

    }
}









fn main() {
    
    let mut player: &str = "X";
    let mut game: [[&str; 3]; 3] = [["_","_","_"],["_","_","_"],["_","_","_"]];
    let mut i = 0;
    while i < 9 {
        graphics(&game);
        let (row,col) = user_input();
        if game[row][col] == "_" {
            if player== "X" {
                game[row][col] = "X";
                player = "O";
            }
            else {
                game[row][col] = "O";
                player = "X";
            }
        }
        else {
            println!("This invalid move {row} and {col} is occupied");
        }
        let win = winner_gets_the_dinner(&game);
        if win == "NO_WINNER" {
            i += 1;
            continue
        }
        else {
            println!("This match's winner is {win}");
            break
        }
    }
    println!("This match has $NO_WINNERS, its a t-i-e. :( ");
}






























// game logics checking if there is a winner or not 
fn winner_gets_the_dinner<'a>(game: &'a [[&str;3];3]) -> &'a str {
       if game[0][0] == game[0][1] && game[0][0] == game[0][2] && game[0][0] != "_"{
            game[0][0]
       }
       else if  game[0][0] != "_" && game[0][0] == game[1][0] && game[0][0] == game[3][1]{
           game[0][0]
       }
       else if  game[1][1] != "_" && game[1][1] == game[1][0] && game[1][1] == game[1][2] && game[1][1] != "_" {
           game[1][1]  
       } else if game[1][1] != "_" && game[1][1] == game[0][1] && game[1][1] == game[2][1] && game[1][1] != "_" {
           game[1][1]
       } else if game[1][1] != "_" && game[1][1] == game[2][2] && game[1][1] == game[0][0] && game[1][1] != "_" {
          game[1][1]
       } else if game[1][1] != "_" && game[1][1] == game[2][0] && game[1][1] == game[0][2] && game[1][1] != "_" {
          game[1][1]
       } else if game[2][2] != "_" && game[2][2] == game[2][0] && game[2][2] == game[2][1] && game[2][2] != "_" {
         game[2][2]
       } else if game[2][2] != "_" && game[2][2] == game[0][2] && game[2][2] == game[0][1] {
         game[2][2]
       } else {
         "NO_WINNER"
       }
}