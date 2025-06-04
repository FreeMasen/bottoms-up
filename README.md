# Bottoms Up

A programming language read from the bottom up.

> This repository is an entry to a programming language jam so all aspects of this project are
> considered in progress and/or may never be completed.


## Language Specification

### Module

Each file will define a discrete module.

A module consists of a sequence of statements which will be executed from the last line of the file
up to the first line.

Some statements can be prefixed with a keyword indicating it should be consumable by other modules.

#### Statements

##### Constant Declarations

Defining a module level constant, the scope of these constants is global to the module but
cannot be used outside of the module it is defined in.

Constants represent data that cannot be re-assigned or mutated during the lifetime of the
application.

Constants may only appear at the top level of a module.

##### Variable Declarations

Define an identifier that represents a piece of data and can be mutated.

The scope of a variable is dependant on where it is defined.

> TODO: define scope boundaries

##### Function Declarations

Define an identifier that represents a sequence of [statements](#statements) that may be executed on
demand using that identifier.


### Expressions

Expressions are a chain of variable names or literal values separated by operators and are evaluated
eagerly from right to left (no mathematic operator precedence is used).

#### Literals

- Decimal Integers
- Double Quoted String 

#### Operators

##### Integer Operators

- `+` add
- `-` difference
- `*` multiply
- `/` divide
- `%` modulo
- `<<` bit shift left
- `>>` bit shift right

##### String Operators

- `><` concatenate two strings
