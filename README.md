# Regex Parser
This is a recursive descent parser written in Rust for regular expresssions. Given a regular expression as input, it outputs an AST. It follows this very simple LL(1) grammar:

```
<regex> ::= <term> '|' <regex>
         |  <term>

<term> ::= { <factor> }

<factor> ::= <base> { '*' }

<base> ::= <char>
        |  '(' <regex> ')'
```

## Usage

```
let mut p = RegExParser::new("((a|b*)|a*)|aab".to_string());
let res = Ok(Or(
    Box::new(Sequence(vec![Box::new(Or(
        Box::new(Sequence(
            vec![Box::new(Or(Box::new(Sequence(vec![Box::new(Terminal('a'))])),
                             Box::new(Sequence(
                                 vec![Box::new(Repetition(Box::new(Terminal('b'))))]))))])),
        Box::new(Sequence(vec![Box::new(Repetition(Box::new(Terminal('a'))))]))))])),
    Box::new(Sequence(vec![Box::new(Terminal('a')), Box::new(Terminal('a')),
                           Box::new(Terminal('b'))]))));
assert_eq!(p.parse(), res);
 ```
