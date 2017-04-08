# Milestones and Roadmap

## v0.2.0 - Basic Object Functionality 
Object: setattr, getattr, __call_ retrieved attr

**Status**: In progress

Get the __dict__ powered PyObject and related types functional enough to handle the
case in a rust test:

Example Python Code:
```python
# __setattr__ -> __getattr__ -> function -> __call__ case.

def bin_add(x, y):
    return x + y

obj = object()
setattr(obj, 'test_func', bin_add)

retrieved = getattr(obj, 'test_func')
result = retrieved(1, 2)
assert result == 3
```
