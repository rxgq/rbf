from enum import Enum


class TokenType(Enum):
    LParen = 1
    RParen = 2
    Identifier = 3
    Number = 4
    String = 5
    Define = 6
    Lambda = 7
    If = 8
    Bool_True = 9
    Bool_False = 10
    Atom = 11
    List = 12
    Print = 13
    Let = 14
    DefVar = 15
    DefParameter = 16
    DefConstant = 17
    Defun = 18
    Loop = 19
    DefMacro = 20

KEYWORDS = {
    "define": TokenType.Define,
    "lambda": TokenType.Lambda,
    "if": TokenType.If,
    "true": TokenType.Bool_True,
    "false": TokenType.Bool_False,
    "atom": TokenType.Atom,
    "list": TokenType.List,
    "print": TokenType.Print,
    "let": TokenType.Let,
    "defvar": TokenType.DefVar,
    "defparameter": TokenType.DefParameter,
    "defconstant": TokenType.DefConstant,
    "defun": TokenType.Defun,
    "loop": TokenType.Loop,
    "defmacro": TokenType.DefMacro,
}

class Token:
    def __init__(self, type, value) -> None:
        self.Type = type
        self.value = value

    def __str__(self) -> str:
        type_width = 10
        value_width = 10
        
        type_str = f'{self.Type.name:<{type_width}}'
        value_str = f'{self.value:<{value_width}}'
        
        return f'[{type_str} {value_str}]'
