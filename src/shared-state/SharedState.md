
- Use the type system
to enforce synchronization of shared data 


- Arc<T>
  - share between threads
  - deallocate T when the last thread exits

- Mutex<T>
  - ensure mutual exclusion access to the T value

Send and Sync

- Send: if it's safe to move a T across a thread boundry
- Sync: if it's safe to move a &T across a thread boundry