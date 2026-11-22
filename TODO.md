# TODO

- [ ] GUI
  - [x] App themes
    - [x] Light
    - [x] Dark
  - [x] Make new drills
    - [x] Show drill conditions
    - [ ] Maybe a hint about the concept or documentation (this decreases their
      grade)
    - [x] Submit and update FSRS parameters
  - [x] Save drills
  - [x] Show retainability percent
  - [x] Show due date of when to review
- [x] Generating drills
  - [x] Java
  - [x] Rust
  - [x] SQL
- [x] Parser + Abstract Syntax Tree
  - [x] Error free for the pre-generated answer
  - [x] Still parses after non-fatal errors
- [ ] Checking
  - [x] Set grade based on fatal and non-fatal parsing errors
  - [ ] Check if conditions have been fulfilled (basically the same as the
    bottom one)
  - [ ] Check the AST of the submitted code and generated code and grade it
- [x] FSRS
  - [x] Check retainability after a few days
  - [x] Set due dates to review
