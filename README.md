### The Shunting-Yard Algorithm

To construct an Abstract Syntax Tree (AST) using the [Shunting-Yard algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm), you must modify Dijkstra's original design by replacing the standard output queue with an operand stack that stores tree nodes.

Instead of writing text tokens directly to a stream, you wrap incoming numbers into leaf nodes, and when processing operators, you pop child trees off the operand stack to link them beneath a new operator node.


#### Shunting Yard Youtube Videos:

- [Fundamental Algorithms in Scala: Problem Explanation](https://www.youtube.com/watch?v=A-SSrZUHYSk)
- [CS 235 Lab 3 Overview - The Shunting Yard](https://www.youtube.com/watch?v=HJOnJU77EUs)

### Conceptual Diagram

```
       INFIX INPUT
  [ 3 ] [ + ] [ 4 ] [ * ] [ 2 ]  <-- (Tokens read from left to right)
                │
                ▼
      ┌──────────────────┐
      │   SWITCH TRACK   │
      └─────────┬────────┘
                │
        Is it a │ Is it an
        Number? │ Operator?
                │
        ┌───────┴───────┐
        │               │
        ▼               ▼
  ┌───────────┐   ┌───────────┐
  │  OUTPUT   │   │ OPERATOR  │
  │   QUEUE   │   │   STACK   │  <-- (Holds operators based on precedence.
  └─────┬─────┘   └─────┬─────┘       Higher precedence stays on top.)
        │               │
        ▼               ▼
  [ 3 4 2 * + ]   [   ] (Empty at end)
  
     POSTFIX
   (FINAL RPN)
```


### 🔄 The Token Routing Rules
When a token travels down the track, it follows a simple set of switching
rules:

1. **Numbers/Operands**: Go straight to the Output Queue.
2. **Operators (+, -, , /, etc.)**: Look at the top of the Operator Stack:
   - If the stack is empty, or the top operator has lower precedence, push the new operator onto the stack.
   - If the top operator has higher or equal precedence, pop it off the stack to the Output Queue before pushing the new one.
3. **Left Parenthesis "("**: Always push onto the Operator Stack.
4. **Right Parenthesis ")"**: Pop operators from the stack to the Output Queue until a ( is reached. Then, discard both parentheses.
5. **End of Input**: Pop all remaining operators from the stack onto the Output Queue.

### 📊 Comprehensive Example Walkthrough

Let's parse the expression: 3 + 4 * 2

| Token | Operator Stack | Operand Stack (Visual Nodes) | Action / Description |
|---|---|---|---|
| 3 | [] | [[3]] | Token is a number. Push leaf node 3. |
| + | [+] | [[3]] | Stack is empty. Push operator +. |
| 4 | [+] | [[3], [4]] | Token is a number. Push leaf node 4. |
| * | [+, *] | [[3], [4]] | * has higher precedence than +. Push * onto stack. |
| 2 | [+, *] | [[3], [4], [2]] | Token is a number. Push leaf node 2. |
| End | [+] | [[3], [* -> Left: 4, Right: 2]] | Input empty. Pop *. Pop 2 (Right), pop 4 (Left). Connect under *. Push node back. |
| End | [] | [[+ -> Left: 3, Right: (* -> 4, 2)]] | Pop +. Pop the * tree (Right), pop 3 (Left). Connect under +. |

Final Constructed AST Hierarchy:

```
```result:

    +
   / \
  3   *
     / \
    4   2
```

### 🛠️ Core Components

* Operator Stack: Temporarily stores operators (+, -, *, etc.) and parentheses to manage order of operations.
* Operand Stack: Stores fully constructed AST sub-trees (nodes) instead of simple numbers.
* Node Representation: Each node contains a value/operator, a reference to a left child, and a reference to a right child. [1, 3] 

### 🏗️ The "Tree Build Step"

Whenever an operator is popped from the operator stack, do the following to
merge sub-trees:

   1. Pop the top node from the operand stack (this becomes the Right Child).
   2. Pop the next top node from the operand stack (this becomes the Left Child).
   3. Create a new parent node with the popped operator.
   4. Assign the Left and Right Children to this new operator node.
   5. Push the new operator node back onto the operand stack.

