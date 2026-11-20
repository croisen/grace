# TODO

- Make a rust drill in rust/src/api/rust/ based on the java drill on
  rust/src/api/java/mod.rs
- Keep track of what drill one is for via the DrillLanguage enum in
  rust/src/api/mod.rs
- Make the app's main screen in flutter have cards that act like a dropdown
  menu to be able to see the current retainability of the concepts chosen
  for the drill, along with the due date
  - This is done by checking the DrillMemory via the retrievability and
  due_date_str methods in rust/src/api/mod.rs
- Find a way to save DrillCollection in rust/src/api/mod.rs to a database via
  flutter's sqlite3 ffi package or sqflite
- Save the current theme selected via shared_preferences
