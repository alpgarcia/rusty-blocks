# Technical design document for the project
**Project Name**: Rusty Blocks

## Project Description
Rusty Blocks is a hobby project to learn Rust. It is inspired by Tetris and developed from scratch.

## Shapes
The game consists of 7 shapes, each made up of 4 blocks. The shapes are:
- I
- J
- L
- O
- S
- T
- Z

Each shape is represented by a vector of variable length, depending on the shape and the rotation system. Each vector position is an integer number, being:
* 0: empty space
* non-zero: block (can be used to represent the color of the block, for instance)

> [!IMPORTANT]  
> 99 is reserved for playfield edges, useful to calculate the collision of the shape with the playfield borders

## Rotation System
The game implements the following rotation systems:
* [SRS (Standard Rotation System)](https://harddrop.com/wiki/SRS)
* [NRS (Nintendo Rotation System - Right Handed Version)](https://harddrop.com/wiki/Nintendo_Rotation_System)

## Shift Delay System
The game implements a simple shift delay system when a key remains pressed. It is a time delay between the moves of the shape. The delay is set to 0.1 seconds by default, except for the first key press, which immediately moves the shape 1 space. The delay is then applied to the following shape moves until the key is released.

## Playfield
The playfield is represented by an array of 276 `usize` integers. It represents a 10x22 grid surrounded by two edges at the sides and one at the bottom, making a total of 12x23 positions, including edges or borders. 

**The edges are represented by the value 99**.

|  | Row Number | Border |
|:--|--:| --:|
| Top row | 0 | No (except cols 0 and 11) |
| Bottom row | 22 | Yes |

|  | Col Number | Border |
|:--|--:| --:|
| Left edge | 0 | Yes |
| Right edge | 11 | Yes |

The playfield is represented as follows:

```
[99,  0,  0,  0,  0,  0,  0,  0,  0,  0, 99] <== row  0 (hidden row)
[99,  0,  0,  0,  0,  0,  0,  0,  0,  0, 99] <== row  1 (hidden row)
[99,  0,  0,  0,  0,  0,  0,  0,  0,  0, 99] <== row  2
...
[99,  0,  0,  0,  0,  0,  0,  0,  0,  0, 99] <== row 21
[99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99] <== row 22
```


## Collision Detection
The collision detection is done by checking if there is any non-zero cell in the shape that is being placed in a non-zero playfield cell. Must be checked prior to move or rotate the shape.

The collision detection follows two rules:
* If a shape block is empty (0) and is going to be placed in a playfield column 
  smaller than 0, it is ignored. Empty blocks must not be checked because they
  never cause a collision, however they can reach out of bounds column positions when the shapes are placed against the left wall and have empty columns at their left. Thus, not checking them is not only more efficient but also prevents
  out of bounds errors [^1].
* There are no individual blocks of the shape placed in a playfield position
with a value different from 0 (edges or cells with blocks already placed).



[^1]: If there are empty blocks of the shape that goes under 0 column (left
playfield edge), ignore them. This may happen when a given shape is rotated
and the blocks are placed against the left edge of the playfield iff the
rotated shape has one or more empty columns at the left. In that case, the shape
can be moved farther than to the column number 1 (0 is the left edge) because
empty colums don't produce any collision. Once an empty shape column is placed
in the column 0 of the playfield, trying to check collisions when a new move to 
the left action is performed would result in an out of bounds error if the algorithm does not ignore the empty blocks. 
