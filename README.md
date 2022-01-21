### Loop in Python

#### `while` loop

When we run into a `SETUP_LOOP` byte code, it means we need set a loop. The argument of the byte code is the address of the instruction we need to run after breaking the loop.

As we may break the loop at any time, we need to know how many items are stored in the current stack, we need an object to store some relative information.

Then we define a `Block` struct, it's with three members:

- _type_: the type of the loop, currently we only have one type.
- _target_: the target instruction address we need to enter in.
- _level_: the level of the stack before entering the loop, when out of the loop, we need to keep popping items from the stack until the level of the stack equal to *level*.

Every `Block` instance represents a loop, as we may set up a new loop in the middle of a loop. So we need a stack to store these nested loops, which is `_loop_stack`.

When we run into a `continue`, we jump to the start of the loop. When we run into a `break`, we pop the loop `Block` from `_loop_stack` and all items of the loop from the stack, then set `pc` as target value stored in `Block`.

As the `for` loop in Python is a lot different from it in other languages like C, we'll cover `for` loop later.

### Klass-Oop Concept

When we implement the `add` method for `Integer` type, to be consistent with the method in `Object` trait, we have to keep the `_rhs` argument as `*const dyn Object` type. So we are not actually sure whether the passed argument is the type we expect.

One way of solving the problem is to set a type id for each type and define a `get_typeid` method in `Object` trait, so we can distinguish different types. But that's not elegant enough. 

We define a `klass` type which implement singleton design pattern, which means only one instance exists for each `klass` type. Every instance of `Integer` carries a reference of this instance. So we can distinguish different types by just compare references!

### Function Object

#### Frame

When we call a function, we actually enter a new local environment, this environment essentially has no differences with the environment in which the function is called. 

We call such an environment as a frame, when we call a function, we create a frame and enter it. Also, we need to store the frame that calls the new frame, so we can know which frame to return to when exit from the current frame.

A frame should contain the following members:

- stack: for operations
- loop stack: for setting up loops
- locals: `HashMap`, key: string pointer, value: variable pointer.
- codes: `CodeObject` of the function
- pc: program counter
- sender: `Box<Frame>`, the frame that called this frame. 

#### Function Object

As we already have `CodeObject`, why would we need a `FunctionObject` type?

That's because a function has many dynamic information like parameters. These can't be stored in bytecodes, so we need to dynamically pass them in. 

When we run into the `MAKE_FUNCTION` operation code, we just connect the function name with the function's bytecodes. The actual `FuntionObject` and the new frame are not created until with the `CALL_FUNCTION` operation code.

A function object should contain the following members:

- func code: CodeObject of the function
- func name
- flags: for latter usage

