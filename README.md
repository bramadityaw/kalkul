# kalkul: An Infix Arithmetic Expression Evaluator

Based on Samuelsen & Bauer's algorithm.

# The Algorithm
This algorithm requires maintaining two stacks: one for numbers,
and another for operators.

- Parse the expression sequentially
- If the token is a number, place the token in the number stack
- If the token is an operator, evaluate operators until either
    + The operator stack is empty
    + The top of the operator is an open parenthesis
    + The precedence of the operator at the top of the operator
      stack is lower than the current operator
    + Then push the operator onto the operator stack
- If the token is an open parenthesis, push the token onto the operator stack
- If the token is a close parenthesis
    + Evaluate operators until an open parenthesis is at the
      top of the operator stack
    + Pop the open parenthesis from the operator stack
- If there are no more tokens to parse, evaluate the remaining operators
