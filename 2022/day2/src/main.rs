use std::{ fs, collections::HashMap };

fn main() {
    
    let strat_guide_encrypted = match fs::read_to_string("input.txt") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("biffed it reading the encrypted strategy guide {:?}", e);
            std::process::exit(1)
        }
    };

    let d = 3; // DRAW
    let l = 0; // LOSE
    let w = 6; // WIN
    let r = 1; // ROCK
    let p = 2; // PAPER
    let s = 3; // SISSORS
    let p1_points = vec![
        // enemy ->
        //   A=r, B=p, C=s
        vec![r+d, r+l, r+w], // me = X (Rock = 1);
        vec![p+w, p+d, p+l], // me = Y (Paper = 2)
        vec![s+l, s+w, s+d], // me = Z (Sissors = 3)
    ];

    let p2_points = vec![
        // enemy ->
        //   A=r, B=p, C=s
        vec![l+s, l+r, l+p], // me = LOSE
        vec![d+r, d+p, d+s], // me = DRAW
        vec![w+p, w+s, w+r], // me = WIN
    ];



    // these are the indexes in the points map ...
    let mut decryp_to_pts_idx = HashMap::new();
    decryp_to_pts_idx.insert("A", 0);
    decryp_to_pts_idx.insert("B", 1);
    decryp_to_pts_idx.insert("C", 2);
    decryp_to_pts_idx.insert("X", 0);
    decryp_to_pts_idx.insert("Y", 1);
    decryp_to_pts_idx.insert("Z", 2);

    let mut p1_total_points = 0;
    let mut p2_total_points = 0;

    for line in strat_guide_encrypted.split("\n") {
        let mut plays = line.split(" ");
        let en_play_encrypted = plays.next().unwrap();
        let me_play_encrypted = plays.next().unwrap();

        let en_play = *decryp_to_pts_idx.get(en_play_encrypted).unwrap();
        let me_play = *decryp_to_pts_idx.get(me_play_encrypted).unwrap();
        
        let p1_pts = p1_points[me_play][en_play];
        let p2_pts = p2_points[me_play][en_play];
        println!("P1: for enemy={:?} me={:?} i scored {:?}", en_play_encrypted, me_play_encrypted, p1_pts);
        println!("P2: for enemy={:?} me={:?} i scored {:?}", en_play_encrypted, me_play_encrypted, p2_pts);
        p1_total_points += p1_pts;
        p2_total_points += p2_pts;
    }
    
    println!("PART 1 = at the end of the elf RPS tournament I won {:?}", p1_total_points);
    println!("PART 2 = using the real docding I won this many pts {:?}", p2_total_points);
    
}
