# Mutex Mistakes I Made

## Issues

- Took reference to MutexGuard
- Tried returning Mutex from function

## Lessons

- Lock gives ownership, not reference
- Never expose Mutex outside a struct
