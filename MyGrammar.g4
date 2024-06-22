grammar MyGrammar;

tokens { INTEGER, IDENTIFIER }

lexerRule
    : INTEGER
    | IDENTIFIER
    ;

INTEGER
    : [0-9]+
    ;

IDENTIFIER
    : [a-zA-Z_][a-zA-Z_0-9]*
    ;
