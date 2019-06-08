# Chessmaster Whitepaper

For this whitepaper, I'd like to go over a few of Rust's idiosyncrasies 
that I encountered while writing Chessmaster. I have previously written 
minichess in Java. In turn, this have given me a unique opportunity to 
directly compare how each language handles the construction of minichess. 
Below are some main observations that stuck during Chessmaster's development. 

#### Absence of Do-While loops 
While Rust has while and for loops like any other language, it notable does 
not implement do-while loops. For example take the following pseudocode 
(taken directly from Bart's guide) for finding all the chessmoves for a 
piece in a given direction: 
```
    x ← x0
    y ← y0
    c ← color of piece at x, y
    moves ← ∅
    repeat
        x ← x + dx
        y ← y + dy
        if x or y is not in bounds
            break
        if there is a piece p at x, y
            if the color of p is c
                break
            if capture = false
                break
            stop-short ← true
        else if capture = only
            break
        insert move from <x0, y0> to <x, y> into moves
    until stop-short
    return moves
```

Here, we need the repeat block to always execute once. Otherwise pawns 
and kings would never return any moves since stop-short is always set to 
True. In Java, this calls for a do-while loop. However, In Rust we implement
it through a 'loop' loop that has the check condition at the end of the loop:
```$xslt
       'outer: loop {
            x = x + dx as i32;
            y = y + dy as i32;
            if !Self::in_bounds(x, y) {
                break;
            }

            // Do logic for determining if we need to add the move here...

            // Add the move

            // Rust has no do while loops, so we use loop and check the condition at the end.
            if stop_short {
                break;
            }
        }
```

#### Array indices must be usize

While developing Chessmaster I exclusively used usize for all integer types. 
However, this led to underflow issues as symmscan negates board coordinates. 
After changing everything to i32, my array indexing wouldn't compile as indices 
only take usize. Casting all indices to usize throughout the program is 
inelegant though I understand why Rust requires this. In retrospect, I should 
have thoughtfully picked integer types, and casted them to usize outside of array 
indices. 

#### Parallel testing

Another feature that caused a brief issue was that Rust spins up multiple threads
for testing. For my testing strategy, I decided to output all possible moves for 
each piece and then visually check them. While these are not 'true' tests, it was 
an efficient way to sanity check my move generator algorithm. However, when initially
printing, Chessmaster printed out garbage as all the IO was run in parallel. Specifying
the number of threads fixed the problem: 
```$xslt
test -- --test-threads=1
```

#### Rustfmt

I developed Chessmaster in Intellij which was not as useful as I originally thought 
it would be. While it natively supports code reformatting for Java, there was no option
for Rust. Instead I used rustfmt, which gave some less than desirable results: 

```        
    match target {
        Some(GamePiece {
            piece: Piece::King,
            color,
        }) => {
            moves.append(&mut Self::movescan(&self, x0, y0, 0, 1, color, Capture::True, true));
```

==> rustfmt ==> 

```$xslt
        match target {
            Some(GamePiece {
                piece: Piece::King,
                color,
            }) => {
                moves.append(&mut Self::movescan(
                    &self,
                    x0,
                    y0,
                    0,
                    1,
                    color,
                    Capture::True,
                    true,
                ));
```

This is much less readable. Before, the containing function took around ~20 lines of 
code. Calling rustfmt bloated it up into an unreadable mess taking up nearly 150 lines. 

#### Optionals are a bad idea

In my design I choose to use optionals to represent gamepieces on the minichess 
board. This ended up being a terrible decision. I ended up having to add logic 
everywhere to check if a square contained a piece or not, in many cases where it
wasn't even possible to have a None value. Instead, I should have added a 'Blank'
value to my Piece Enum, to make handling pieces much easier. 


#### Object-oriented design vs access modifiers

Coming from Java, I tried to make my Rust minichess implementation as object-oriented 
as possible. This was a mistake. After all, Rust is not an object-oriented language. 
The more I developed, the more I had to declare structs, enums, and functions public.
Overall, I unintentionally overruled safety aspects that might have been better handled 
all in a single file.  
