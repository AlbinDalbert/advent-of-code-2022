use crate::reading;

pub fn day4() {
    let s = reading::get_content_from_path("./input/day4.txt");
    let lines = s.lines();
    let lines2 = lines.clone();

    let mut complete_overlap = 0;
    let mut partial_overlap = 0;

    for li in lines {

        let (a, b) = li.split_once(',').unwrap();
        let (a1, a2) = a.split_once('-').unwrap();
        let (b1, b2) = b.split_once('-').unwrap();

        let mut a1 = a1.parse::<i32>().unwrap();
        let mut a2 = a2.parse::<i32>().unwrap();
        let mut b1 = b1.parse::<i32>().unwrap();
        let mut b2 = b2.parse::<i32>().unwrap();

        if  a1 <= b1 && a2 >= b2
        {
            complete_overlap += 1;
        } else 
        if  b1 <= a1 && b2 >= a2
        {
            complete_overlap += 1;
        };

        if  a1 <= b2 && a2 >= b1
        {
            partial_overlap += 1;
        } else 
        if  b1 <= a2 && b2 >= a1
        {
            partial_overlap += 1;
        };
        println!("--{} - {} , {} - {}", a1, a2, b1 ,b2);

    }

    println!("Part 1 Answer: {}", complete_overlap);
    println!("Part 2 Answer: {}", partial_overlap);
}