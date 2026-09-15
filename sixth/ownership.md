Leasson: 

 * What is Ownership
 * Stack and the Heap
 * Demo: Variable Scope
 * The String Type
 * Memory Allocation and Drop
 * Move
 * Clone
 * Copy


Memory Handling:

 * All languages must manage how they use memory while running.
  
    - Approach 1: Having a garbage collector that regularly looks for no longer used memory as the program runs: Simple but doesn't efficient
    - Approach 2: Explicitly allocate/free memory (C, C++, Assembly, ...): this has a cost is harder to code 

 * Rust is different!
    - Memory is managed through a set of rules that the compiler check if any rule is violated, the program won't compile


Ownership: 

- Is a new concept for many programmers: it might take some time to get used to it.

- It's a way to develop code that is safe and efficient naturally

- We will focus on examples using a specific data structure: Strings.



The Stack and the Heap:

 - Many languages don't require you think about the stack and heap often.

 - In Rust (Systems language), whether a value is on the stack or the heap affects how the language behaves and why you must make certain decisions 

 - Both the stack and the heap are parts of memory avaliable to your code to use at runtime, but they are structured in different ways.

    * Description: If a type/variable is into a stack or heap these change the behavior of the language.
  
  STACK: Order all in a sort pile tower and we start from the bottom if you want put something onto the stack there's operation called popping or pop to remove data there's in the top of stack weight: 10 kilobytes and also fixed sides these

  HEAP: is less organized when you put data on the heap, you request some space

  the memory allocator:

    1. Finds a bog enough empty spot in the heap 
    2. Mark it as being in use
    3. Returns a poniter: the location's address

    This process is called allocating 

    You can stro the pointers on the stack 
    to get the data, follow the pointer

Ownership's purpose 

The primary purpose of ownershipe is to maanage heap data

  * Kepping track of what code is using what data on the heap 
  * Minimizing the amount of duplicate data on the heap 
  * Cleaning up unused data on the heap

Ownership Rules

1. each value has an owner.
2. There can only be one owner at a time.
3. when the owner goes out of scope, the value will be dropped.

 