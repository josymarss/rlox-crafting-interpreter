mod token_type;  
use token_type::*;

struct Token {
  type_of:TokenType,
  lexeme:String, 
  literal:String,
  line:i64, 
}

impl Token {
  fn init(&self, type_of:TokenType, lexeme: String, line: i64, literal: String) {
    self.type_of = type_of; 
    self.lexeme = lexeme;
    self.literal = literal;
    self.line = line;
  }

  fn to_string () -> String {
    type_of + " " + lexeme + " " + literal;
  }
}