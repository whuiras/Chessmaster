# Chessmaster

Will Huiras (c) 2019

Chessmaster is a chess program where users can play minichess against eachother. 
As this project was done in part to learn the idiosyncocies of Rust, there are a 
few changes to teh rules of minichess: 
* Pawns may only ever move one square at a time 
(e.i. no moving two squares if previously unmoved)
    * In turn, no en passant
* No pawn promotions
* No castling

The game is responsible for setting up a new game, keeping track of past moves 
and pieces taken, and making players moves. Additionally, the program supports 
a console interface that displays the board and accepts input. It was written 
following Bart Massey's how-to guide for minichess. It can be found here:
http://wiki.cs.pdx.edu/mc-howto/index.html


##Usage:

Chessmaster automatically creates a new game on startup. Moves are inputted into the 
game through traditional chess notation (e.g. a2-a3). Example use is as follows: 

```
Welcome to Chessmaster
Starting a new game: 

6 kqbnr
5 ppppp
4 .....
3 .....
2 PPPPP
1 KQBNR

  abcde

Make a move

a2-a3

6 kqbnr
5 ppppp
4 .....
3 P....
2 .PPPP
1 KQBNR

  abcde

Make a move

a5-a4

6 kqbnr
5 .pppp
4 p....
3 P....
2 .PPPP
1 KQBNR

  abcde

Make a move

b2-b3

6 kqbnr
5 .pppp
4 p....
3 PP...
2 ..PPP
1 KQBNR

  abcde

Make a move

a4-b3

6 kqbnr
5 .pppp
4 .....
3 Pp...
2 ..PPP
1 KQBNR

  abcde

Make a move

e2-e3

6 kqbnr
5 .pppp
4 .....
3 Pp..P
2 ..PP.
1 KQBNR

  abcde

Make a move

fae
Invalid input

Make a move

c2-c3
Illegal move

Make a move

b3-b2

6 kqbnr
5 .pppp
4 .....
3 P...P
2 .pPP.
1 KQBNR

  abcde

Make a move

c2-c3

6 kqbnr
5 .pppp
4 .....
3 P.P.P
2 .p.P.
1 KQBNR

  abcde

Make a move

b2-a1

6 kqbnr
5 .pppp
4 .....
3 P.P.P
2 ...P.
1 pQBNR

  abcde
  
Game Over!
```

whuiras@pdx.edu
