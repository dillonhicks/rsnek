# Milestones and Roadmap

## v0.2.0 - Basic Object Functionality 
Object: setattr, getattr, __call_ retrieved attr

**Status**: Complete 2017.04.08 #rRPYd5f4a09

Get the __dict__ powered PyObject and related types functional enough to handle the
case in a rust test:

Example Python Code:
```python
# __setattr__ -> __getattr__ -> function -> __call__ case.
from builtins import object, setattr, getattr

def bin_add(x, y):
    return x + y

obj = object()
setattr(obj, 'test_func', bin_add)

retrieved = getattr(obj, 'test_func')
result = retrieved(1, 2)
assert result == 3
```


## v0.3.0 - Minimal Viable Interpreter: Calculator

Should be able to take a single python file containing simple arithmetic and logical expression 
and execute successfully.


Example Python Code:
```python
x = 1
y = 2.0
z = x + y
print(z)
```

Will require:

- code object implementation
- minimal ast
- parser
- modules
- interpreter loop


## v0.4.0 - Threadsafe Refcounts

libfringe allows for generators and self managed stacks that can be the basis of uthreads for 
optimizing single threaded cases.

- Should try to abstract the single vs multithreaded concurrency primitives
  - Determine if the crossbeam crate has anything that can help

## v0.5.0 - Classobj

Properly implement class objects and switch the hardcoded types to use them for builtin types


## vFuture

- scopes
- Implement outstanding core types
  - Numbers
  - Collections
  - Etc
- stdlib implementation targets
- 5..100% language parity targets
- frames
- tracebacks
- Switch macros.rs to use procedural macros based on builtin enum variants

