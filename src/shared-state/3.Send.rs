// T is Sync if and only if &T is Send

/*
  Send
 */
// A type T is Send
// if it is safe to move a T value
// to another thread.

/*
  Sync
 */
// A type T is Sync
// if it is safe to access a T value
// from multiple threads
// at the same time.
// T is Sync if and only if &T is Send