// Author: Eskil Nyberg
// Based on IndaPlus22/task-3/chess_template by Viola Söderlund, modified by Isak Larsson

use std::fmt;

/// Enum for the current state of the game.
///
/// ### States
/// - `InProgress` describes that the game is initialized and playable. The game starts in this state.
/// This is the general state of the game unless the game is in check.
/// - `Check` describes that the game is currently in a check state that needs to be corrected.
/// In this state, `get_possible_moves()` returns a limited list of moves.
/// - `WaitingOnPromotionChoice` describes that the game is waiting for the user to choose which piece
/// the recently moved pawn should be promoted to.
/// - `GameOver` describes a finished game. All state-altering functions will not work in this state.
/// This state is reached either by reaching a checkmate, stalemate or by a user-submitted defeat.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    WaitingOnPromotionChoice,
    GameOver
}

/// Enum for the colours of the board. Is implemented as an auxiliary state for by e.g. Piece and Game.
///
/// Contains the variants `White` and `Black`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour {
    White,
    Black,
}

/// Enum for the type of piece referenced. Implements a value per piece for comparative calculations. Is implemented by e.g. `Piece`.
///
/// Contains the variants (and values) `King` (0), `Queen` (1), `Rook` (2), `Knight` (3), `Bishop` (4), `Pawn` (5).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    King = 0,
    Queen = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Pawn = 5,
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// Struct for some Piece. 
/// 
/// Is used in the engine as an Option<Piece>-structure implementing None where there are no pieces and Some(Piece) where there are pieces.
/// 
/// Contains the fields piece_type of type PieceType and colour of type Colour.
pub struct Piece {
    piece_type: PieceType,
    colour: Colour,
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// Struct for some position. Contains the fields `row` and `col` corresponding to the row and col represented, individually,
/// as well as the field `idx` corresponding to the index of the position in the board array. 
/// 
/// Note the implementations of `Position::new(row, col)`, `Position::new_from_idx(idx)` and `Position::parse_str(str)` perform error handling.
/// Every instance of Position should represent a legal position, or else the functions will return an Err Result.
pub struct Position {
    row: usize,
    col: usize,
    idx: usize,
}

impl Position {
    /// Init-function that parses some position on the chessboard from the corresponding row and col as indices 0-7.
    /// 
    /// Returns an `Ok(Position)`,
    /// or an `Err(&str)` describing the error if the input does not represent some part of the chess board. 
    pub fn new(row: usize, col: usize) -> Result<Position, String> {
        if row > 8 ||  col > 8 {
            let error = format!("Invalid row: {} or col: {} input. Input should be between 0-7.", row, col);
            return Err(error);
        }
        
        return Ok(Position {
            row,
            col,
            idx: row * 8 + col
        });
    }

    /// Init-function that parses some position on the chessboard from the corresponding array index 0-63.
    /// 
    /// Returns an `Ok(Position)`,
    /// or an `Err(&str)` describing the error if the input does not represent some part of the chess board. 
    pub fn new_from_idx(idx: usize) -> Result<Position, String> {
        if idx > 63 {
            let error = format!("Invalid idx: {} input. Input should be between 0-63.", idx);
            return Err(error);
        } 
        
        return Ok(Position {
            row: idx % 8,
            col: idx - (idx % 8)*8,
            idx
        });
    }

    /// Init-function that parses some position on the chessboard from a two character String on the format `XF` where `X` is a character a-h and `F` is a number 0-7.
    ///
    /// Returns an `Ok(Position)`,
    /// or an `Err(&str)` describing the error if the input does not represent some part of the chess board. 
    pub fn parse_str(str: &str) -> Result<Position, String> {
        let str_lowercase = str.to_lowercase();
        let str_vec: Vec<&str> = str_lowercase // Performed to permit uppercase inputs
            .trim() // Removes potential whitespaces passed to the function
            .split("").collect(); // Creates the vector

        if str_vec.len() != 2 {
            return Err(String::from("Input has invalid length"));
        };

        // parses the first character: the column; throws an error if the character is not a character between a-h
        let col: usize = match str_vec[0] {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _ => {
                let error = format!("First character '{}' of string invalid, should be some character between a-h", str_vec[0]);
                return Err(error);
            }
        };

        // parses the second character: the row; throws an error if the character is not a number between 1-8
        // the function's return statement is nested within these if-statements
        let row_result = str_vec[1].parse::<usize>();
        if row_result.is_err() {
            let error = format!("Second character '{}' of string invalid, should be some number between 1-8", str_vec[1]);
            return Err(error);
        } else {
            let mut row = row_result.unwrap(); // note that the array index should be corrected to be 0-indexed, but this is done later to prevent underflow
            if row < 1 || row > 8 {
                let error = format!("Second character '{}' of string invalid, should be some number between 1-8", str_vec[1]);
                return Err(error);
            } else {
                row -= 1; // This operation is performed here to prevent underflow in usize.
                return Position::new(row, col); // This will not return an error since we have already performed error handling.
            }
        }
    }
}

/// The game! The struct contains our accessible fields.
/// 
/// `new()` which instantiates the game.
/// `make_move(from_str, to_str)` which, if legal, makes a move from some pos XF to some pos XF and returns the resulting error or new GameState.
pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    active_colour: Colour,
    board: [Option<Piece>; 8 * 8],
    last_moved_to: Position
}

/// Here we implement the main functions of our game. 
impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        // generate the pieces
        let w_king = Some(Piece {
            colour: Colour::White,
            piece_type: PieceType::King,
        });
        let w_queen = Some(Piece {
            colour: Colour::White,
            piece_type: PieceType::Queen,
        });
        let w_rook = Some(Piece {
            colour: Colour::White,
            piece_type: PieceType::Rook,
        });
        let w_knight = Some(Piece {
            colour: Colour::White,
            piece_type: PieceType::Knight,
        });
        let w_bishop = Some(Piece {
            colour: Colour::White,
            piece_type: PieceType::Bishop,
        });
        let w_pawn = Some(Piece {
            colour: Colour::White,
            piece_type: PieceType::Pawn,
        });

        let b_king = Some(Piece {
            colour: Colour::Black,
            piece_type: PieceType::King,
        });
        let b_queen = Some(Piece {
            colour: Colour::Black,
            piece_type: PieceType::Queen,
        });
        let b_rook = Some(Piece {
            colour: Colour::Black,
            piece_type: PieceType::Rook,
        });
        let b_knight = Some(Piece {
            colour: Colour::Black,
            piece_type: PieceType::Knight,
        });
        let b_bishop = Some(Piece {
            colour: Colour::Black,
            piece_type: PieceType::Bishop,
        });
        let b_pawn = Some(Piece {
            colour: Colour::Black,
            piece_type: PieceType::Pawn,
        });

        // initializing board array
        let mut board_init = [
            w_rook, w_knight, w_bishop, w_king, w_queen, w_bishop, w_knight, w_rook, w_pawn,
            w_pawn, w_pawn, w_pawn, w_pawn, w_pawn, w_pawn, w_pawn, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, b_pawn,
            b_pawn, b_pawn, b_pawn, b_pawn, b_pawn, b_pawn, b_pawn, b_rook, b_knight, b_bishop,
            b_king, b_queen, b_bishop, b_knight, b_rook,
        ];

        Game {
            /* initialise board, set active colour to white and state to in progress */
            state: GameState::InProgress,
            active_colour: Colour::White,
            board: board_init,
            last_moved_to: Position::new(0,0).unwrap() // arbitrary position, is updated before it is used
        }
    }

    /// If the current game state is InProgress or Check and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from_str: String, to_str: String) -> Result<GameState, String> {
        // Checks that the game state is InProgress or Check, else throws an error.
        if !(self.state == GameState::InProgress || self.state == GameState::Check) {
            let error = format!("The game is not in a state where a move can be made. Currently, the state is {:?}.", self.state);
            return Err(error);
        }
        
        // parse from_str
        let from_pos = match Position::parse_str(&from_str) {
            Ok(result) => result,
            Err(string) => return Err(string),
        };

        // parse to_str
        let to_pos = match Position::parse_str(&to_str) {
            Ok(result) => result,
            Err(string) => return Err(string),
        };

        let possible_moves = self.get_possible_moves(from_pos);

        if !possible_moves.iter() // Creates an iterable of positions.
            .any(|pos| pos == &to_pos) // Checks if our position is equal to the list of possible moves. We use .any() since the objects may be different instances.
        {
            return Err(String::from("Invalid move."));
        } else {
            // We move the piece!
            self.set_piece(to_pos, self.get_piece(from_pos));
            self.set_piece(from_pos, None);
            // update game state (to some variant of GameState)
            self.update_game_state();
            
            // If the user has to choose a promotion, wait for that before updating the active colour.
            if self.state != GameState::WaitingOnPromotionChoice {
                // update active colour
                if self.active_colour == Colour::Black {
                    self.active_colour = Colour::White;
                } else {
                    self.active_colour = Colour::Black;
                }
            }
            
            return Ok(self.state);
        }
    }

    /// Checks the current game state and updates it.
    ///
    /// Only iterates over the active colour due to expected use of the library.
    fn update_game_state(&mut self) {
        /*
        If there is a pawn that needs to be promoted (is at the end of the board),
        the method will put the game into GameState::WaitingOnPromotionChoice and skip the rest of the state-checking.
        This is safe because the promotion method set_promotion will call this methodagain at the end to set the state to one of the below values.
        */ 
        if self.state != GameState::GameOver {
            // Check if the user needs to promote a pawn by iterating over every pawn of the correct colour and checking whether they are at the end of the board.
            for (i, piece) in self.board.iter().enumerate() {
                if piece.is_none() {
                    // Do nothing
                } else if piece.unwrap().colour == self.active_colour && piece.unwrap().piece_type == PieceType::Pawn {
                    // We only care for pawns of the active colour.
                    // Unwrapping piece is safe here since it is not none.
                    // Unwrapping Position::new_from_idx(i) is safe here since the board is well defined.
                    if self.active_colour == Colour::White && i % 8 == 7 {
                        self.state = GameState::WaitingOnPromotionChoice;
                        return;
                    } else if self.active_colour == Colour::Black && i % 8 == 0 {
                        self.state = GameState::WaitingOnPromotionChoice;
                        return;
                    }
                }
            }
        }
        /* If the next thing to happen is not a promotion:
        If the king is in check and no correcting move can be made, the game is in checkmate with GameState::GameOver.
        If the king is in check and a correcting move can be made, the game is in check with GameState::Check.
        If the king is not in check yet no move can be made, the game is in stalemate with GameState::GameOver.
        If the king is not in check and some move can be made, the game is simply in progress with GameState::InProgress.

        Note that the method `can_make_legal_move` primarily uses the function `get_possible_moves` which checks whether
        some move puts the king in check when it is performed. A "possible" or "legal" move is thus defined as a move that
        can be performed without putting the king at risk. 
        */
        if self.is_check(self.active_colour) {
            if self.can_make_legal_move(self.active_colour) {
                self.state = GameState::Check;
            } else {
                self.state = GameState::GameOver;
            }
            
        } else {
            if self.can_make_legal_move(self.active_colour) {
                // We have a stalemate
                self.state = GameState::GameOver;
            } else {
                self.state = GameState::InProgress;
            }
        }
    }
    
    /// Checks whether the king of colour `colour` is in check and returns a boolean.
    /// 
    /// This is done by iterating over every piece of the opposite colour and checking whether it can move to the king. 
    fn is_check(&self, colour: Colour) -> bool {
        let king_pos = self.find_king_pos(colour);

        for (i, piece) in self.board.iter().enumerate() {
            if piece.is_none() {
                // Do nothing
            } else if piece.unwrap().colour != colour {
                // Unwrapping piece is safe here since it is not none.
                // Unwrapping Position::new_from_idx(i) is safe here since the board is well defined.
                let possible_moves = self.get_possible_moves(Position::new_from_idx(i).unwrap()); 

                if possible_moves.contains(&king_pos) {
                    return true;
                }
            } else {
                // Do nothing
            }
        }

        // If we have found no cases where the king is in check, the king is not in check.
        return false;
    }

    /// Checks whether the colour of parameter `colour` has some legal move it can make and returns a boolean.
    /// 
    /// This primarily relies on the function `get_possible_moves` which implements checking whether some move would put the king in check.
    /// Is implemented in checkmate-checking.
    fn can_make_legal_move(&self, colour: Colour) -> bool{
        let mut moveable_piece_count = 0;

        for (i, piece) in self.board.iter().enumerate() {
            if piece.is_none() {
                // Do nothing
            } else if piece.unwrap().colour == colour {
                // Unwrapping piece is safe here since it is not none.
                // Unwrapping Position::new_from_idx(i) is safe here since the board is well defined.
                let possible_moves = self.get_possible_moves(Position::new_from_idx(i).unwrap());
                if possible_moves.len() > 0 {
                    moveable_piece_count += 1;
                }
            }
        }

        if moveable_piece_count == 0 { return true; } else { return false; }
    }

    /// Finds the king's position and returns it as a Position
    fn find_king_pos(&self, colour: Colour) -> Position {
        for (i, piece) in self.board.iter().enumerate() {
            if piece.is_none() {
                // Do nothing
            } else if piece.unwrap().piece_type == PieceType::King && piece.unwrap().colour == colour {
                // Unwrapping piece is safe here since it is not none.
                // Unwrapping Position::new_from_idx(i) is safe here since the board is well defined.
                return Position::new_from_idx(i).unwrap();
            }
        }
        panic!("The king is not on the board! Something is wrong.");
    }

    /// Sets the piece at Position pos. 
    fn set_piece(&mut self, pos: Position, piece: Option<Piece>) {
        self.board[pos.idx] = piece;
    }

    /// Gets the piece at Position pos, returning either None or Some(piece)
    fn get_piece(&self, pos: Position) -> Option<Piece> {
        return self.board[pos.idx];
    }

    /// Set the piece type that a peasant becames following a promotion.
    /// 
    /// Uses the field `last_moved_to` due to expected use of the library. Will break if used to promote a piece which was not just moved.
    pub fn set_promotion(&mut self, piece: String) -> Result<GameState, String> {
        let piece_lowercase = piece.to_lowercase();

        let piece_type = match piece_lowercase.trim() {
            "queen" => PieceType::Queen,
            "rook" => PieceType::Rook,
            "bishop" => PieceType::Bishop,
            "knight" => PieceType::Knight,
            "king" => return Err(String::from("You can't promote a pawn to a king!")),
            "pawn" => return Err(String::from("You can't promote a pawn to a pawn!")),
            _ => return Err(String::from(format!("Invalid input '{}'.", piece_lowercase)))
        };

        self.set_piece(self.last_moved_to,
            Some(Piece {piece_type, colour: self.active_colour})
        );

        // update active colour
        if self.active_colour == Colour::Black {
            self.active_colour = Colour::White;
        } else {
            self.active_colour = Colour::Black;
        }

        self.update_game_state();
        return Ok(self.state);
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, position: Position) -> Vec<Position> {
        let mut possible_moves: Vec<Position> = Vec::with_capacity(60);

        return vec![];
    }
}

/// Implement print routine for Game.
///
/// Output example:
/// |:------------------------------:|
/// | wR  wKn wB  wK  wQ  wB  wKn wR |
/// | wP  wP  wP  wP  wP  wP  wP  wP |
/// | *   *   *   *   *   *   *   *  |
/// | *   *   *   *   *   *   *   *  |
/// | *   *   *   *   *   *   *   *  |
/// | *   *   *   *   *   *   *   *  |
/// | bP  bP  bP  bP  bP  bP  bP  bP |
/// | bR  bKn bB  bK  bQ  bB  bKn bR |
/// |:------------------------------:|
/// 
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // init output, the string we'll be coding our format to
        let mut output = String::new();

        // start with the top row
        output.push_str("|:------------------------------:|\n");

        // for every Option<piece> in board, print a representation. Also, for every beginning of a row i % 8 == 0 and end of a row i & 8 == 7 add corresponding slices.
        for (i, piece) in self.board.iter().enumerate() {
            if i % 8 == 0 {
                output.push_str("|");
            }

            if piece.is_none() {
                output.push_str(" *  "); // there is no piece here, add an asterisk
            } else {
                // from here, unwrapping is safe since the piece is not None
                // add initial spacing
                output.push_str(" ");

                // match dict for Colour representation
                output.push_str(match piece.unwrap().colour {
                    Colour::White => "w",
                    Colour::Black => "b",
                });

                // match dict for PieceType representation
                output.push_str(match piece.unwrap().piece_type {
                    PieceType::King => "K ",
                    PieceType::Queen => "Q ",
                    PieceType::Bishop => "B ",
                    PieceType::Knight => "Kn",
                    PieceType::Rook => "R ",
                    PieceType::Pawn => "P ",
                });
            }
            
            if i % 8 == 7 {
                output.push_str("|\n");
            }
        }

        // end with the bottom row
        output.push_str("|:------------------------------:|");

        write!(f, "{}", output)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    #[test]
    fn output_accurate() {
        let game = Game::new();

        assert_eq!(format!("{}", game),
"|:------------------------------:|
| wR  wKn wB  wK  wQ  wB  wKn wR |
| wP  wP  wP  wP  wP  wP  wP  wP |
| *   *   *   *   *   *   *   *  |
| *   *   *   *   *   *   *   *  |
| *   *   *   *   *   *   *   *  |
| *   *   *   *   *   *   *   *  |
| bP  bP  bP  bP  bP  bP  bP  bP |
| bR  bKn bB  bK  bQ  bB  bKn bR |
|:------------------------------:|"
        );
    }
}
