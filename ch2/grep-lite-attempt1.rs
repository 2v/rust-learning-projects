fn main() {
    let ctx_lines: usize = 3; // the amount of lines surrounding the matched text that we want
    let needle = "test";

    let haystack = "\
So shaken as we are, so wan with care,
Find we a time for frighted peace to pant,
And breathe short-winded accents of new broils
To be commenced in strands afar remote.
No more the thirsty entrance of this soil
Shall daub her lips with her own children's blood;
Nor more shall trenching war channel her fields,
Nor bruise her flowerets with the armed hoofs
Of hostile paces: those opposed eyes,
Which, like the meteors of a troubled heaven,
All of one nature, of one substance bred,
Did lately meet in the intestine shock
And furious close of civil butchery
Shall now, in mutual well-beseeming ranks,
March all one way and be no more opposed
Against acquaintance, kindred and allies:
The edge of war, like an ill-sheathed knife,
No more shall cut his master. Therefore, friends,
As far as to the sepulchre of Christ,
Whose soldier now, under whose blessed cross
We are impressed and engaged to fight,
Forthwith a power of English shall we levy;
Whose arms were moulded in their mothers' womb
To chase these pagans in those holy fields
Over whose acres walk'd those blessed feet
Which fourteen hundred years ago were nail'd
For our advantage on the bitter cross.
But this our purpose now is twelve month old,
And bootless 'tis to tell you we will go:
Therefore we meet not now. Then let me hear
Of you, my gentle cousin Westmoreland,
What yesternight our council did decree
In forwarding this dear expedience.";



    let mut target_lines: Vec<usize> = vec![];
    // iterate once through the lines, and add the line numbers which have a match to a vector.
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            println!("{}: {:?}", i, line);
            target_lines.push(i+1);
        }
    } 

    let mut it = haystack.lines();
    for index in target_lines {
        let lower_bound = index.saturating_sub(ctx_lines);
        let upper_bound = index.saturating_add(ctx_lines);
        println!("{}: {:?}", lower_bound, it.nth(lower_bound));
        for i in lower_bound+1..upper_bound {
            println!("{}, {:?}", i, it.next());
        }
        
    }
    // iterate again, this time adding the lines within the range we have specified to a new vector of vectors
    // output this vector of vectors with an informative message
}
