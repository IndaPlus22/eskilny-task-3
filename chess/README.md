Welcome to my chess library!
The recommended use of this library is that you interact with the structs Game and Position. For an example of how to interact with the library, see main.rs which is the interface I used for playing the game while debugging it. The tests in lib.rs may also provide some insight.


Game is the game library! See the specific docstring for Game for details. 
Game is implemented fully except for castling and en-passent. It also doesn't keep track of which moves have been performed or anything,
it just provides you the current state of the game. It implements stalemate and checkmate checking.


Position is an auxiliary struct that provides nice parsing methods for working with the row and column of some position interchangably with the corresponding index.
It has nice initialization methods:
- `Position::new(row,col)` from the row or column on the format 0-7
- `Position::new_from_idx(idx)` from the index on the format 0-63 (great if you're iterating over the board)
- `Position::parse_str(str)` from a string on the format XF where X is a character a-h and F is a number 1-8

This library stores the chess board as an array of Piece:s, which are structs containing the nums PieceType and Colour. 
If you want to represent the state of the board in some way, learn to work with this array! As I said, working via Position is recommended.
You can get the current board via the function `Game::get_board()`.


Good luck!

Report any bugs you find to Eskil Nyberg on Discord plz <3