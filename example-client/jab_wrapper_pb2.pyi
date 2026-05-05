from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class ElementLocator(_message.Message):
    __slots__ = ("role", "name", "description", "states", "strict")
    ROLE_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    STATES_FIELD_NUMBER: _ClassVar[int]
    STRICT_FIELD_NUMBER: _ClassVar[int]
    role: str
    name: str
    description: str
    states: _containers.RepeatedScalarFieldContainer[str]
    strict: bool
    def __init__(self, role: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., states: _Optional[_Iterable[str]] = ..., strict: bool = ...) -> None: ...

class ElementInfo(_message.Message):
    __slots__ = ("context", "name", "role", "description", "states", "x", "y", "width", "height", "index_in_parent", "children_count")
    CONTEXT_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    ROLE_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    STATES_FIELD_NUMBER: _ClassVar[int]
    X_FIELD_NUMBER: _ClassVar[int]
    Y_FIELD_NUMBER: _ClassVar[int]
    WIDTH_FIELD_NUMBER: _ClassVar[int]
    HEIGHT_FIELD_NUMBER: _ClassVar[int]
    INDEX_IN_PARENT_FIELD_NUMBER: _ClassVar[int]
    CHILDREN_COUNT_FIELD_NUMBER: _ClassVar[int]
    context: int
    name: str
    role: str
    description: str
    states: str
    x: int
    y: int
    width: int
    height: int
    index_in_parent: int
    children_count: int
    def __init__(self, context: _Optional[int] = ..., name: _Optional[str] = ..., role: _Optional[str] = ..., description: _Optional[str] = ..., states: _Optional[str] = ..., x: _Optional[int] = ..., y: _Optional[int] = ..., width: _Optional[int] = ..., height: _Optional[int] = ..., index_in_parent: _Optional[int] = ..., children_count: _Optional[int] = ...) -> None: ...

class SelectWindowRequest(_message.Message):
    __slots__ = ("hwnd",)
    HWND_FIELD_NUMBER: _ClassVar[int]
    hwnd: int
    def __init__(self, hwnd: _Optional[int] = ...) -> None: ...

class SelectWindowResponse(_message.Message):
    __slots__ = ("success", "error")
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    ERROR_FIELD_NUMBER: _ClassVar[int]
    success: bool
    error: str
    def __init__(self, success: bool = ..., error: _Optional[str] = ...) -> None: ...

class FindElementsRequest(_message.Message):
    __slots__ = ("locator", "max_depth")
    LOCATOR_FIELD_NUMBER: _ClassVar[int]
    MAX_DEPTH_FIELD_NUMBER: _ClassVar[int]
    locator: ElementLocator
    max_depth: int
    def __init__(self, locator: _Optional[_Union[ElementLocator, _Mapping]] = ..., max_depth: _Optional[int] = ...) -> None: ...

class FindElementsResponse(_message.Message):
    __slots__ = ("elements", "error")
    ELEMENTS_FIELD_NUMBER: _ClassVar[int]
    ERROR_FIELD_NUMBER: _ClassVar[int]
    elements: _containers.RepeatedCompositeFieldContainer[ElementInfo]
    error: str
    def __init__(self, elements: _Optional[_Iterable[_Union[ElementInfo, _Mapping]]] = ..., error: _Optional[str] = ...) -> None: ...

class ClickElementRequest(_message.Message):
    __slots__ = ("context",)
    CONTEXT_FIELD_NUMBER: _ClassVar[int]
    context: int
    def __init__(self, context: _Optional[int] = ...) -> None: ...

class ClickElementResponse(_message.Message):
    __slots__ = ("success", "error")
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    ERROR_FIELD_NUMBER: _ClassVar[int]
    success: bool
    error: str
    def __init__(self, success: bool = ..., error: _Optional[str] = ...) -> None: ...

class TypeTextRequest(_message.Message):
    __slots__ = ("context", "text")
    CONTEXT_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    context: int
    text: str
    def __init__(self, context: _Optional[int] = ..., text: _Optional[str] = ...) -> None: ...

class TypeTextResponse(_message.Message):
    __slots__ = ("success", "error")
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    ERROR_FIELD_NUMBER: _ClassVar[int]
    success: bool
    error: str
    def __init__(self, success: bool = ..., error: _Optional[str] = ...) -> None: ...

class GetElementTextRequest(_message.Message):
    __slots__ = ("context",)
    CONTEXT_FIELD_NUMBER: _ClassVar[int]
    context: int
    def __init__(self, context: _Optional[int] = ...) -> None: ...

class GetElementTextResponse(_message.Message):
    __slots__ = ("text", "error")
    TEXT_FIELD_NUMBER: _ClassVar[int]
    ERROR_FIELD_NUMBER: _ClassVar[int]
    text: str
    error: str
    def __init__(self, text: _Optional[str] = ..., error: _Optional[str] = ...) -> None: ...

class GetVersionInfoRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class GetVersionInfoResponse(_message.Message):
    __slots__ = ("version",)
    VERSION_FIELD_NUMBER: _ClassVar[int]
    version: str
    def __init__(self, version: _Optional[str] = ...) -> None: ...

class ListWindowsRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class ListWindowsResponse(_message.Message):
    __slots__ = ("windows",)
    WINDOWS_FIELD_NUMBER: _ClassVar[int]
    windows: _containers.RepeatedCompositeFieldContainer[WindowInfo]
    def __init__(self, windows: _Optional[_Iterable[_Union[WindowInfo, _Mapping]]] = ...) -> None: ...

class WindowInfo(_message.Message):
    __slots__ = ("hwnd", "title")
    HWND_FIELD_NUMBER: _ClassVar[int]
    TITLE_FIELD_NUMBER: _ClassVar[int]
    hwnd: int
    title: str
    def __init__(self, hwnd: _Optional[int] = ..., title: _Optional[str] = ...) -> None: ...
