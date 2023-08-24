from vmath_internal import PyVec2

class Vec2:
    def __init__(self, x, y):
        self._internal = PyVec2(x, y)

    def to_tuple(self):
        return self._internal.to_tuple()

    def __repr__(self):
        return self._internal.as_str()

    def __str__(self):
        return self._internal.as_str()
