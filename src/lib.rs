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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    InProgress,
    Check,
    WaitingOnPromotionChoice,
    GameOver,
}

/// Enum for the colours of the board. Is implemented as an auxiliary state for by e.g. Piece and Game.
///
/// Contains the variants `White` and `Black`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Colour {
    White,
    Black,
}

impl Colour {
    /// A function that returns the opposite colour
    fn opposite(colour: Colour) -> Colour {
        if colour == Colour::White {
            return Colour::Black;
        } else {
            return Colour::White;
        }
    }
}

/// Enum for the type of piece referenced. Implements a value per piece for comparative calculations. Is implemented by e.g. `Piece`.
///
/// Contains the variants `King`, `Queen`, `Rook`, `Knight`, `Bishop`, `Pawn`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Struct for some Piece.
///
/// Is used in the engine as an Option<Piece>-structure implementing None where there are no pieces and Some(Piece) where there are pieces.
///
/// Contains the fields piece_type of type PieceType and colour of type Colour.
pub struct Piece {
    piece_type: PieceType,
    colour: Colour,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
        if row > 8 || col > 8 {
            let error = format!(
                "Invalid row: {} or col: {} input. Input should be between 0-7.",
                row, col
            );
            return Err(error);
        }

        return Ok(Position {
            row,
            col,
            idx: row * 8 + col,
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
            row: idx / 8,
            col: idx % 8,
            idx,
        });
    }

    /// Init-function that parses some position on the chessboard from a two character String on the format `XF` where `X` is a character a-h and `F` is a number 0-7. Performs trimming and caps-handling.
    ///
    /// Returns an `Ok(Position)`,
    /// or an `Err(&str)` describing the error if the input does not represent some part of the chess board.
    pub fn parse_str(str: &str) -> Result<Position, String> {
        let str_lowercase = str.to_lowercase(); // Performed to permit uppercase inputs. Saved in a memory to permit safe borrowing.
        let chars: Vec<char> = str_lowercase
            .trim() // Removes potential whitespaces passed to the function
            .chars()
            .collect(); // Creates the vector

        if chars.len() != 2 {
            return Err(String::from(format!("Input {} is of invalid length.", str)));
        }

        // parses the first character: the column; throws an error if the character is not a character between a-h
        let col: usize = match chars[0] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => {
                let error = format!(
                    "First character '{}' of string invalid, should be some character between a-h",
                    chars[0]
                );
                return Err(error);
            }
        };

        // parses the second character: the row; throws an error if the character is not a number between 1-8
        // the function's return statement is nested within these if-statements
        let row: usize = match chars[1] {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => {
                let error = format!(
                    "Second character '{}' of string invalid, should be some number between 1-8",
                    chars[1]
                );
                return Err(error);
            }
        };

        return Position::new(row, col);
    }

    /// Function that modifies self by offset, given as a tuple (row-offset, col-offset)
    pub fn offset_self(&mut self, offset: (i32, i32)) -> Result<bool, String> {
        let row_result: i32 = self.row as i32 + offset.0;
        let col_result: i32 = self.col as i32 + offset.1;

        if row_result < 0 || row_result > 7 || col_result < 0 || col_result > 7 {
            return Err(String::from("New position not on board."));
        }

        // We are fine and complete the addition
        self.row = row_result as usize;
        self.col = col_result as usize;
        self.idx = self.row * 8 + self.col;
        return Ok(true);
    }
}

/// The game! The struct contains our accessible fields and functions.
///
/// * `new()` which instantiates the game.
/// * `make_move(from_str, to_str)` which, if legal, makes a move from some pos XF to some pos XF and returns the resulting error or new GameState.
/// * `get_game_state()` returns the state of the game.
/// * `get_active_colour()` returns the active colour.
/// * `get_board()` returns the board.
/// * `get_possible_moves(position, recursion_order)` returns a list of all possible moves for the piece at position.
/// * `set_promotion(piece)` should be called if the game is in GameState::WaitingOnPromotionChoice to indicate what piece to promote the last moved pawn to.
///
/// Also contains the constant `MAX_RECURSIONS` which defines how many orders of check-recursion should be checked by `get_possible_moves`.
#[derive(Clone)] // The clone derivation is necessary as it is used by try_move
pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    active_colour: Colour,
    board: [Option<Piece>; 8 * 8],
    last_moved_to: Position,
}

/// Here we implement the main functions of our game.
impl Game {
    /// This is a constant used in the function `try_move` that specifies how far the engine should check for Check-states.
    /// The value 2 should do since after 2 recursions, we have checked each user making the next move. In this time, we should discover all relevant Check-states.
    const MAX_RECURSIONS: i32 = 2;

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
        let board_init = [
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
            last_moved_to: Position::new(0, 0).unwrap(), // arbitrary position, is updated before it is used
        }
    }

    /// If the current game state is InProgress or Check and the move is legal,
    /// move a piece and return the resulting state of the game. Performs trimmming and caps-handling.
    ///
    /// Updates all fields.
    pub fn make_move(&mut self, from_str: &str, to_str: &str) -> Result<GameState, String> {
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

        // check that the the piece is not None and is of the right colour
        match self.board[from_pos.idx] {
            None => {
                return Err(String::from(
                    "There is no piece on the square you are trying to move from.",
                ))
            }
            Some(piece) => {
                if piece.colour != self.active_colour {
                    return Err(String::from("It is not this colour's turn!"));
                }
            }
        }

        // Generates a list of all the legal moves that the piece in question can be performed.
        let possible_moves = self.get_possible_moves(from_pos, 0);

        if !possible_moves
            .iter() // Creates an iterable of positions.
            .any(|pos| pos == &to_pos)
        // Checks if our position is equal to some position in the list of possible moves. We use .any() since the objects may be different instances.
        {
            // eprintln!("Possible moves are {:?}", possible_moves); // DEBUG
            return Err(String::from("Illegal move. (This might mean that this piece cannot move this way, or that it puts your king in check!)"));
        } else {
            // We move the piece!
            self.board[to_pos.idx] = self.board[from_pos.idx];
            self.board[from_pos.idx] = None;
            // and save this movement for future reference
            self.last_moved_to = to_pos;
            // and update the active colour (NEEDS TO BE DONE BEFORE update_game_state()!)
            self.active_colour = Colour::opposite(self.active_colour);
            // and update the game state (to some variant of GameState)
            self.update_game_state();

            return Ok(self.state);
        }
    }

    /// Checks the current game state for the player of the active_colour and updates it. Expects the active colour to be updated to the next player's colour.
    ///
    /// Updates only the field `state`.
    ///
    /// SHOULD ONLY BE CALLED BY INTERNAL FUNCTIONS.
    fn update_game_state(&mut self) {
        /*
        If there is a pawn that needs to be promoted (is at the end of the board),
        the method will put the game into GameState::WaitingOnPromotionChoice and skip the rest of the state-checking.
        This is safe because the promotion method set_promotion will call this method again at the end to set the state to one of the below values.
        */
        if self.state != GameState::GameOver {
            // Check if the user needs to promote a pawn by checking the piece at `last_moved_to`
            let last_moved_piece = self.board[self.last_moved_to.idx].unwrap(); // unwrap is safe due since last_moved_to is well-defined.
            if last_moved_piece.piece_type == PieceType::Pawn {
                // We only care for pawns of the active colour.
                // Unwrapping piece is safe here since it is not none.
                // Unwrapping Position::new_from_idx(i) is safe here since the board is well defined.
                if last_moved_piece.colour == Colour::White && self.last_moved_to.row == 7 {
                    self.state = GameState::WaitingOnPromotionChoice;
                    return;
                } else if last_moved_piece.colour == Colour::Black && self.last_moved_to.row == 0 {
                    self.state = GameState::WaitingOnPromotionChoice;
                    return;
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
        if self.is_in_check(self.active_colour, 0) {
            if self.can_make_legal_move(self.active_colour) {
                self.state = GameState::Check;
            } else {
                self.state = GameState::GameOver;
            }
        } else {
            if self.can_make_legal_move(self.active_colour) {
                // We have a stalemate
                self.state = GameState::InProgress;
            } else {
                self.state = GameState::GameOver;
            }
        }
    }

    /// Checks whether the king of colour `colour` is in check and returns a boolean. `recursion_order` should be set to 0 unless you know what you're doing.
    /// This is done by iterating over every piece of the opposite colour and checking whether it can move to the king.
    ///
    /// SHOULD ONLY BE CALLED BY INTERNAL FUNCTIONS. If you are wondering whether the game is in state Check, please use `get_game_state` instead.
    /// This function is public such that it can be called by `get_possible_moves` on cloned instances.
    ///
    /// Note that this function calls `get_possible_moves` again which calls this function.
    /// To avoid infinite recursion, we pass the variable `recursion_order` which is incremented by `get_possible_moves`.
    fn is_in_check(&self, colour: Colour, recursion_order: i32) -> bool {
        let king_pos = self.find_king_pos(colour);

        // eprintln!("Assessing game variant for king of colour {:?}:\n{}\n", colour, self); // DEBUG

        for (i, piece) in self.board.iter().enumerate() {
            if piece.is_none() {
                // Do nothing
            } else if piece.unwrap().colour != colour {
                // eprintln!("Possible moves assessed for {:?} at pos {:?}.", piece, Position::new_from_idx(i)); // DEBUG
                // Unwrapping piece is safe here since it is not none.
                // Unwrapping Position::new_from_idx(i) is safe here since the board is well defined.
                let possible_moves =
                    self.get_possible_moves(Position::new_from_idx(i).unwrap(), recursion_order);

                // if piece.unwrap().piece_type == PieceType::Queen { eprintln!("Found moves for the queen are: {:?}", possible_moves); } // DEBUG
                if possible_moves
                    .iter() // Creates an iterable of positions.
                    .any(|pos| pos == &king_pos)
                // Checks if our position is equal to the list of possible moves. We use .any() since the objects may be different instances.
                {
                    // eprintln!("King is in check"); // DEBUG
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
    ///
    /// SHOULD ONLY BE CALLED BY INTERNAL FUNCTIONS.
    fn can_make_legal_move(&self, colour: Colour) -> bool {
        for (i, piece) in self.board.iter().enumerate() {
            if piece.is_none() {
                // Do nothing
            } else if piece.unwrap().colour == colour {
                // Unwrapping piece is safe here since it is not none.
                // Unwrapping Position::new_from_idx(i) is safe here since the board is well defined.
                let possible_moves = self.get_possible_moves(Position::new_from_idx(i).unwrap(), 0);
                // eprintln!("Possible moves found for piece {:?} are: {:?}", piece, possible_moves); // DEBUG
                if possible_moves.len() > 0 {
                    // We have found at least one possible move and return true
                    return true;
                }
            }
        }

        // We have, after iterating over every piece, found no possible move and return false
        return false;
    }

    /// Finds the king of colour `colour`'s position and returns it as a Position
    ///
    /// SHOULD ONLY BE CALLED BY INTERNAL FUNCTIONS.
    fn find_king_pos(&self, colour: Colour) -> Position {
        for (i, piece) in self.board.iter().enumerate() {
            if piece.is_none() {
                // Do nothing
            } else if piece.unwrap().piece_type == PieceType::King
                && piece.unwrap().colour == colour
            {
                // Unwrapping piece is safe here since it is not none.
                // Unwrapping Position::new_from_idx(i) is safe here since the board is well defined.
                return Position::new_from_idx(i).unwrap();
            }
        }
        panic!("The king is not on the board! Something is wrong.");
    }

    /// Set the piece type that a peasant becames following a promotion. Performs trimming and caps-handling.
    ///
    /// Uses the field `last_moved_to` due to expected use of the library. Will break if used to promote a piece which was not just moved.
    pub fn set_promotion(&mut self, piece: String) -> Result<GameState, String> {
        if self.state != GameState::WaitingOnPromotionChoice {
            return Err(String::from(format!(
                "The game is not currently waiting on a promotion. Currently, the state is {:?}.",
                self.state
            )));
        }
        let piece_lowercase = piece.to_lowercase();

        let piece_type = match piece_lowercase.trim() {
            "queen" => PieceType::Queen,
            "rook" => PieceType::Rook,
            "bishop" => PieceType::Bishop,
            "knight" => PieceType::Knight,
            "king" => return Err(String::from("You can't promote a pawn to a king!")),
            "pawn" => return Err(String::from("You can't promote a pawn to a pawn!")),
            _ => {
                return Err(String::from(format!(
                    "Invalid input '{}'.",
                    piece_lowercase
                )))
            }
        };

        self.board[self.last_moved_to.idx] = Some(Piece {
            piece_type,
            colour: self.board[self.last_moved_to.idx].unwrap().colour,
        });

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

    /// Get the current game state.
    pub fn get_active_colour(&self) -> Colour {
        self.active_colour
    }

    pub fn get_board(&self) -> &[Option<Piece>; 8 * 8] {
        return &self.board;
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// Takes the arguments `pos` of type Position and `recursion_order`. Put `recursion_order` to 0 if you do not know what you are doing.
    /// `recursion_order` is an auxiliary variable that prevents the function from checking for potential Check-states further in the future than MAX_RECURSIONS.
    ///
    /// Note: en passent and castling not implemented. TODO.
    pub fn get_possible_moves(&self, pos: Position, mut recursion_order: i32) -> Vec<Position> {
        // Increment recursion_order. See docstring for details.
        recursion_order += 1;

        // Get piece. If it is None, it cannot move so return an empty vector.
        let piece: Piece = match self.board[pos.idx] {
            None => return vec![],
            Some(piece) => piece,
        };

        // Start listing possible moves.
        let mut possible_moves: Vec<Position> = Vec::with_capacity(60);

        // For each piece_type, follow some set of rules.
        /* Design philosophy:
            For every direction that a piece should move in, generate an offset or a set of offsets for that direction.
            Then, iterate over every direction using the function try_move (see the function for details) which returns two booleans:
                legal_move - if the move is legal; then it is added to the possible_moves vector
                engine_should_continue - bool describing if the move should cause this method to halt further movement in the same direction
            So, we iterate over each offset in every direction until we reach a point where engine_should_continue is false, and then we change direction.

            Note that the pawn implementation is hacked! Pawns do not work the same way, but their behavior abides to the checks performed by the above booleans.
            See their specific implementation for details.

            Note that trial.0 refers to legal_move and trial.1 refers to engine_should_continue.
        */
        match piece.piece_type {
            PieceType::King => {
                // Kings can move all directions but only one distance.
                // See the comment above the match-case for details on the implementation.
                for offset in [
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (0, 1),
                    (0, -1),
                    (-1, 1),
                    (-1, 0),
                    (-1, -1),
                ] {
                    let trial = self.try_move(pos, offset, recursion_order);
                    if trial.0 {
                        let mut ok_pos = pos.clone();
                        ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                        possible_moves.push(ok_pos);
                    }
                }
            }
            PieceType::Queen => {
                // Queens can move all directions and however far they like. (The board is size 8.)
                // See the comment above the match-case for details on the implementation.
                for dir in [
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (0, 1),
                    (0, -1),
                    (-1, 1),
                    (-1, 0),
                    (-1, -1),
                ] {
                    for len in 1..8 {
                        let offset = (dir.0 * len, dir.1 * len);
                        let trial = self.try_move(pos, offset, recursion_order);
                        if trial.0 {
                            let mut ok_pos = pos.clone();
                            ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                            possible_moves.push(ok_pos);
                        }

                        if !trial.1 {
                            break;
                        }
                    }
                }
            }
            PieceType::Bishop => {
                // Bishops can move all diagonal directions and however far they like. (The board is size 8.)
                // See the comment above the match-case for details on the implementation.
                for dir in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                    for len in 1..8 {
                        let offset = (dir.0 * len, dir.1 * len);
                        let trial = self.try_move(pos, offset, recursion_order);
                        if trial.0 {
                            let mut ok_pos = pos.clone();
                            ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                            possible_moves.push(ok_pos);
                        }

                        if !trial.1 {
                            break;
                        }
                    }
                }
            }
            PieceType::Knight => {
                // Knight can move according to eight movesets.
                // See the comment above the match-case for details on the implementation.
                for offset in [
                    (2, 1),
                    (2, -1),
                    (1, 2),
                    (1, -2),
                    (-1, 2),
                    (-1, -2),
                    (-2, 1),
                    (2, -1),
                ] {
                    let trial = self.try_move(pos, offset, recursion_order);
                    if trial.0 {
                        let mut ok_pos = pos.clone();
                        ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                        possible_moves.push(ok_pos);
                    }

                    if !trial.1 {
                        break;
                    }
                }
            }
            PieceType::Rook => {
                // Rooks can move all non-diagonal directions and however far they like. (The board is size 8.)
                // See the comment above the match-case for details on the implementation.
                for dir in [(1, 0), (0, 1), (0, -1), (-1, 0)] {
                    for len in 1..8 {
                        let offset = (dir.0 * len, dir.1 * len);
                        let trial = self.try_move(pos, offset, recursion_order);
                        if trial.0 {
                            let mut ok_pos = pos.clone();
                            ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                            possible_moves.push(ok_pos);
                        }

                        if !trial.1 {
                            break;
                        }
                    }
                }
            }
            PieceType::Pawn => {
                /* This pawn-implementation is hacky :)
                    We find the direction (positive or negative) and then iterate
                    i) forward in that direction
                    ii) to the sides

                    In the forward direction we allow all moves which don't return a false boolean engine_should_continue from try_move (trial.1 in the code).
                    This is because that indicates that we either i) have run into the end of the board or ii) have run into a piece.
                    The first option isn't relevant for pawns, and the second the method try_move thinks is legal but actually isn't, since pawns can't capture forward.

                    For double-step-checking, we break the loop after the first iteration here if there is a piece on the way or if the piece is not on the second row.


                    In the diagonal direction we do the opposite! We ONLY allow moves for which try_move returns a false boolean engine_should_continue,
                    with the same methodology. If engine_should_continue is false, we would be capturing a piece.

                    See the docstring above the match-case for context.
                */

                let dir: i32;
                let mut on_first_row = false;
                if piece.colour == Colour::White {
                    dir = 1;
                    if pos.row == 1 {
                        on_first_row = true;
                    }
                } else {
                    dir = -1;
                    if pos.row == 6 {
                        on_first_row = true;
                    }
                }

                // forward direction
                for (i, j) in [(1, 0), (2, 0)] {
                    let offset: (i32, i32) = (i * dir, j);
                    let trial = self.try_move(pos, offset, recursion_order);
                    if trial.0 && trial.1 {
                        let mut ok_pos = pos.clone();
                        ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                        possible_moves.push(ok_pos);
                    }
                    if !on_first_row || !trial.1 {
                        // break if it is not on the first row or if there was a piece in the way
                        break;
                    }
                }

                // diagonal direction
                for (i, j) in [(1, 1), (1, -1)] {
                    let offset: (i32, i32) = (i * dir, j);
                    let trial = self.try_move(pos, offset, recursion_order);
                    if trial.0 && !trial.1 {
                        let mut ok_pos = pos.clone();
                        ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                        possible_moves.push(ok_pos);
                    }
                }
            }
        }
        return possible_moves;
    }

    /// This function tries to move a piece from old_pos to the offset (i32, i32). Does not check whether pieces are in the way for this move, but it does
    /// check whether it puts the own king in check.
    /// Takes as input `recursion_order` too, which is an integer describing which order in the recursion this iteration of try_move is.
    /// If the iteration is higher than MAX_RECURSIONS, this function will not check whether a move implies putting the king in check.
    ///
    /// Returns two booleans, one bool indicating whether the move was legal (internally legal_move)
    /// and another bool indicating whether the engine should continue checking for legal moves in the same direction (internally engine_should_continue)
    ///
    /// SHOULD ONLY BE CALLED BY INTERNAL FUNCTIONS.
    fn try_move(
        &self,
        old_pos: Position,
        offset: (i32, i32),
        recursion_order: i32,
    ) -> (bool, bool) {
        if self.board[old_pos.idx].is_none() {
            panic!(
                "try_move was called trying to move a piece from a tile where there is no piece!"
            );
        }

        /* The philosophy for this function is that we generate a clone of the own game, perform the move in that game and see where that takes us.
            We also perform error-handling for the offset (if it is off the board) and check whether there is a piece in the way.
            If there is a piece in the way, we check that it is of the opposite color (a.k.a. capture-able)
            and in that case return that the engine should not continue.

            If a move is found to be almost legal, a.k.a. moves to an empty piece or a piece of the opposite color, this function will check whether
            the move puts the own king in check by calling is_check on the new board. This step is skipped if the recursion order is greater than
            MAX_RECURSIONS.

            There are comments guiding you through the if-clauses below if you need to read the code.
        */

        // Unwrapping is safe since it is not none.
        let player_colour = self.board[old_pos.idx].unwrap().colour;

        // Generate new position and check if it is in the board
        let mut new_pos = old_pos.clone();
        match new_pos.offset_self(offset) {
            Err(_) => return (false, false), // If the new position is outside of the board, it is not valid and the engine should change direction.
            _ => (),                         // continue
        };

        // eprintln!("Trying to move {:?} from {:?} to {:?}", self.board[old_pos.idx], old_pos, new_pos); // DEBUG

        // Clone into a new game to try the movement in that game
        let mut game_after_movement = self.clone();
        game_after_movement.board[new_pos.idx] = game_after_movement.board[old_pos.idx];
        game_after_movement.board[old_pos.idx] = None;
        game_after_movement.active_colour = Colour::opposite(game_after_movement.active_colour);

        // Check piece movement on the new board
        let legal_move: bool;
        let engine_should_continue: bool;
        match self.board[new_pos.idx] {
            // If there is no piece in the new slot, return false if the king is in check after movement or else true. Return true that the engine should keep checking the same direction.
            None => {
                engine_should_continue = true;
                if recursion_order < Game::MAX_RECURSIONS {
                    legal_move = !game_after_movement.is_in_check(player_colour, recursion_order);
                } else {
                    legal_move = true;
                }
                ();
            }
            // If there is a piece in the new slot, the engine should not keep checking the same direction...
            Some(piece) => {
                engine_should_continue = false;
                // ... and the move is not legal if the piece is of the player's colour
                if piece.colour == player_colour {
                    legal_move = false;
                }
                // ... else the move is legal if the king is not in check after movement
                else {
                    if recursion_order < Game::MAX_RECURSIONS {
                        legal_move = !game_after_movement.is_in_check(player_colour, recursion_order);
                    } else {
                        legal_move = true;
                    }
                }
            }
        }

        // eprintln!("Legal? {}. Engine should continue? {}", legal_move, engine_should_continue); // DEBUG
        return (legal_move, engine_should_continue);
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

impl fmt::Debug for Game {
    // Make the formatter print game the same in debug mode as outside of debug mode.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Colour {
    // Make the formatter print colours fancily outside of debug mode.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;
    use super::Position;

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
    fn position_checking_works() {
        let possible_moves = vec![Position::new(0, 0).unwrap()];
        let other_position = Position::new(0, 0).unwrap();
        assert!(possible_moves
            .iter() // Creates an iterable of positions.
            .any(|pos| pos == &other_position)); // Checks if our position is equal to the list of possible moves. We use .any() since the objects may be different instances.
    }

    // check that game state is check after the queen attacks the king
    #[test]
    fn game_enters_check() {
        let mut game = Game::new();
        let moves: Vec<&str> = "d2 d3
        d7 d6
        e1 b4
        d6 d5
        b4 d6"
            .split_whitespace()
            .collect();

        for i in 0..(moves.len() / 2) {
            let result = game.make_move(moves[2 * i], moves[2 * i + 1]);
            assert!(result.is_ok());
        }

        assert_eq!(game.get_game_state(), GameState::Check);
    }

    // check that the game state is checkmate after "skolmatt"
    // due to the nature of the library, this also verifies that stalemate-checking will work
    #[test]
    fn game_enters_checkmate() {
        let mut game = Game::new();
        let moves: Vec<&str> = "d2 d3
        d7 d6
        e1 c3
        d6 d5
        c1 f4
        d5 d4
        c3 c7"
            .split_whitespace()
            .collect();

        for i in 0..(moves.len() / 2) {
            let result = game.make_move(moves[2 * i], moves[2 * i + 1]);
            assert!(result.is_ok());
        }

        eprintln!("{}", game);
        assert_eq!(game.get_game_state(), GameState::GameOver);
    }

    // verify that the game enters the state waitingonpromotionchoice if a pawn should be promoted
    #[test]
    fn game_enters_waitingonpromitionchoice() {
        let mut game = Game::new();
        let moves: Vec<&str> = "e2 e3
        d7 d6
        e3 e4
        d6 d5
        e4 d5
        d8 d7
        d5 d6
        d7 c6
        d6 d7
        c6 c5
        d7 d8"
            .split_whitespace()
            .collect();

        for i in 0..(moves.len() / 2) {
            let result = game.make_move(moves[2 * i], moves[2 * i + 1]);
            eprintln!(
                "{} {}: {:?}",
                moves[2 * i],
                moves[2 * i + 1],
                result.unwrap()
            );
        }

        assert_eq!(game.get_game_state(), GameState::WaitingOnPromotionChoice);
    }

    // verify that a pawn can be promoted
    #[test]
    fn game_promotes_correctly() {
        let mut game = Game::new();
        let moves: Vec<&str> = "e2 e3
        d7 d6
        e3 e4
        d6 d5
        e4 d5
        d8 d7
        d5 d6
        d7 c6
        d6 d7
        c6 c5
        d7 d8"
            .split_whitespace()
            .collect();

        for i in 0..(moves.len() / 2) {
            let result = game.make_move(moves[2 * i], moves[2 * i + 1]);
            eprintln!(
                "{} {}: {:?}",
                moves[2 * i],
                moves[2 * i + 1],
                result.unwrap()
            );
        }

        assert_eq!(game.get_game_state(), GameState::WaitingOnPromotionChoice);
        assert!(game.set_promotion(String::from("queen")).is_ok());
        assert_eq!(game.get_game_state(), GameState::InProgress);
        eprintln!("{}", game);
    }

    // verify that the output is accurate
    #[test]
    fn output_accurate() {
        let game = Game::new();

        assert_eq!(
            format!("{}", game),
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
