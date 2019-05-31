# Chessmaster

Chessmaster is a chess game for chess enthusiasts. 
Players can play chess against an AI after connecting to a central server. 
More specifically, players may start a new game and input moves to the game. 
The game is responsible for setting up a new game, keeping track of past moves 
and pieces taken, and making informed moves against the player. Additionally, 
the program supports a console interface that displays the board and accepts input.


##Usage:

There are two types of accepted player input, creating a new game and making a move. 
If the player is making a pawn promotion, they must enter in the piece they would 
like to promote to a long with the move coordinates.

```
Input:
“new game”
Response:

New game created

  ABCDEFGH

8 rnbqkbnr
7 pppppppp
6 ........
5 ........
4 ........
3 ........
2 PPPPPPPP
1 RNBQKBNR

  ABCDEFGH

Input:
Move
e2 e4
Response:

  ABCDEFGH

8 rnbqkbnr
7 pppppppp
6 ........
5 ........
4 ....P...
3 ........
2 PPPP.PPP
1 RNBQKBNR

  ABCDEFGH

  ABCDEFGH

8 r.bqkbnr
7 pppppppp
6 n.......
5 ........
4 ....P...
3 ........
2 PPPP.PPP
1 RNBQKBNR

  ABCDEFGH

Input:
Move
a7 a8
Queen
Response:


  ABCDEFGH

8 Q......k
7 ........
6 ........
5 ........
4 ........
3 ........
2 ........
1 K.......

  ABCDEFGH

  ABCDEFGH

8 Q.......
7 .......k
6 ........
5 ........
4 ........
3 ........
2 ........
1 K.......

  ABCDEFGH
```
