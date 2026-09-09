@external(erlang, "dynamic_provider", "Token")
pub type Token

@external(erlang, "dynamic_provider", "token")
pub fn token(value: String) -> Token

@external(erlang, "dynamic_provider", "identity_token")
pub fn identity_token(value: Token) -> Token

@external(erlang, "dynamic_provider", "identity_token_pair")
pub fn identity_token_pair(value: Token) -> #(Token, Bool)



@external(erlang, "dynamic_provider", "first_token")
pub fn first_token(values: List(Token)) -> Token
