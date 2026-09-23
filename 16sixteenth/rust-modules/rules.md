## MODULES RULES

1. Start from the create root 

    - The compiler first looks in the crate root file (src/lib.rs for a library create or src/main.rs for a binary crate) for code to compile

2. How to declare a module 

   - In the crate root file (main.rs), you can declare new module.
   - Example: you declare a "garden" module with mod garden;
     
   - ### the compiler will look for the module's code: 
     
      * Inline (curly brackets replace the semicolon following mod garden)
      * In the file src/garden.rs
      * In ghe file src/garden/mod.rs
     
3. Declaring submodules
   - Submodules can be declared in files other tan the crate root.
   - Example: declare **mod vegetables;** in src/garden.rs.

   - The compiler will look for submodule's code within the dir named for the parent module : 
       * Inline, following mod vegetables (within curly brackets instead of the semicolon)
       * In the file src/garden/vegetables.rs
       * In the file src/garden/vegetables/mod.rs

4. Paths to code in modules
   - Once a module is part of uour crate, you can refer to code in that module
   - from anywhere else in that same crate using the path to the code.
  
5. The use keyword 
   - The use keyword creates shortcuts to items to reduce repetition of long paths.
   - intead of typing every time : **crate::garden::vegetables::Aspargus**
   - You can create a shortcut use : **crate::garden::vegetables::Aspargus**

 
6. Private VS Public
   - By default the code within a module is private from its parent modules.
   - To make a module public, declare it with pub mod instead of mod.
   - To make items within a public module public as well use pub before their declarations
  

7. Grouping in Modules
   - You can use modules to organize code for better readability, but also to separate responsibilities

8. 