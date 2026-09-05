### The Shunting-Yard Algorithm

To construct an Abstract Syntax Tree (AST) using the [Shunting-Yard algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm), you must modify Dijkstra's original design by replacing the standard output queue with an operand stack that stores tree nodes.

Instead of writing text tokens directly to a stream, you wrap incoming numbers into leaf nodes, and when processing operators, you pop child trees off the operand stack to link them beneath a new operator node.


#### Shunting Yard Youtube Videos:

- [Fundamental Algorithms in Scala: Problem Explanation](https://www.youtube.com/watch?v=A-SSrZUHYSk)
- [CS 235 Lab 3 Overview - The Shunting Yard](https://www.youtube.com/watch?v=HJOnJU77EUs)

------------------------------
## 🛠️ Core Components

* Operator Stack: Temporarily stores operators (+, -, *, etc.) and parentheses to manage order of operations.
* Operand Stack: Stores fully constructed AST sub-trees (nodes) instead of simple numbers.
* Node Representation: Each node contains a value/operator, a reference to a left child, and a reference to a right child. [1, 3] 

------------------------------
## ⚙️ Step-by-Step Algorithm Rules
Read your infix expression from left to right, token by token:

   1. If the token is a Number (Operand):
   * Create a leaf node containing the number.
      * Push this node onto the operand stack.
   2. If the token is a Left Parenthesis (:
   * Push it directly onto the operator stack.
   3. If the token is an Operator (op₁):
   * While there is an operator (op₂) at the top of the operator stack, and op₂ has higher or equal precedence than op₁ (and op₁ is left-associative), pop op₂ and execute a Tree Build Step (defined below).
      * Push op₁ onto the operator stack.
   4. If the token is a Right Parenthesis ):
   * While the top of the operator stack is not a left parenthesis (, pop operators off the stack and execute a Tree Build Step for each.
      * Pop and discard the matching left parenthesis (.
   5. At the End of the Expression:
   * While operators remain on the operator stack, pop them one by one and execute a Tree Build Step.
      * The single remaining node on the operand stack is the root of your complete AST.
   
------------------------------
## 🏗️ The "Tree Build Step"
Whenever an operator is popped from the operator stack, do the following to merge sub-trees:

   1. Pop the top node from the operand stack (this becomes the Right Child).
   2. Pop the next top node from the operand stack (this becomes the Left Child).
   3. Create a new parent node with the popped operator.
   4. Assign the Left and Right Children to this new operator node.
   5. Push the new operator node back onto the operand stack.

------------------------------
## 📊 Comprehensive Example Walkthrough
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

