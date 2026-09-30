# wfuzz — Complete API Reference

Exhaustive per-item reference for **every** type, function, method, constant, and macro in the wfuzz source (`xmendez/wfuzz` @ `2263cd0`, v3.1.0). Language: **PY**. Generated from a full source read; test files are excluded. Each item shows its full signature and doc comment; composite types list their fields/members.

**1200 documented items** across 113 source files.


---

## Modules / packages

- [`(root)`](#pkg--root-) — 0 items
- [`docs`](#pkg-docs) — 0 items
- [`src`](#pkg-src) — 0 items
- [`src/wfuzz`](#pkg-src-wfuzz) — 1200 items

---


## `(root)` <a id='pkg--root-'></a>

*0 documented items*



## `docs` <a id='pkg-docs'></a>

*0 documented items*



## `src` <a id='pkg-src'></a>

*0 documented items*



## `src/wfuzz` <a id='pkg-src-wfuzz'></a>

*1200 documented items*


### `wfuzz/src/wfuzz/__init__.py`


#### `warning_on_one_line`  <sub>(function)</sub>

```python
def warning_on_one_line(message, category, filename, lineno, file=None, line=None):
```

### `wfuzz/src/wfuzz/api.py`


#### `fuzz`  <sub>(function)</sub>

```python
def fuzz(**kwargs):
```

#### `get_payloads`  <sub>(function)</sub>

```python
def get_payloads(iterator):
```

#### `get_payload`  <sub>(function)</sub>

```python
def get_payload(iterator):
```

#### `encode`  <sub>(function)</sub>

```python
def encode(name, value):
```

#### `decode`  <sub>(function)</sub>

```python
def decode(name, value):
```

#### `payload`  <sub>(function)</sub>

```python
def payload(**kwargs):
```

#### `get_session`  <sub>(function)</sub>

```python
def get_session(cline):
```

### `wfuzz/src/wfuzz/core.py`


#### `Fuzzer`  <sub>(class)</sub>

```python
class Fuzzer(object):
```

**Fields:**
```python
def __init__(self, options)
def __iter__(self)
def __next__(self)
def stats(self)
def cancel_job(self)
def pause_job(self)
def resume_job(self)
```


#### `Fuzzer.__init__`  <sub>(function)</sub>

```python
def Fuzzer.__init__(self, options):
```

#### `Fuzzer.__iter__`  <sub>(function)</sub>

```python
def Fuzzer.__iter__(self):
```

#### `Fuzzer.__next__`  <sub>(function)</sub>

```python
def Fuzzer.__next__(self):
```

#### `Fuzzer.stats`  <sub>(function)</sub>

```python
def Fuzzer.stats(self):
```

#### `Fuzzer.cancel_job`  <sub>(function)</sub>

```python
def Fuzzer.cancel_job(self):
```

#### `Fuzzer.pause_job`  <sub>(function)</sub>

```python
def Fuzzer.pause_job(self):
```

#### `Fuzzer.resume_job`  <sub>(function)</sub>

```python
def Fuzzer.resume_job(self):
```

### `wfuzz/src/wfuzz/dictionaries.py`


#### `BaseIterator`  <sub>(class)</sub>

```python
class BaseIterator:
```

**Fields:**
```python
def count(self)
def width(self)
def payloads(self)
def cleanup(self)
```


#### `BaseIterator.count`  <sub>(function)</sub>

```python
def BaseIterator.count(self):
```

#### `BaseIterator.width`  <sub>(function)</sub>

```python
def BaseIterator.width(self):
```

#### `BaseIterator.payloads`  <sub>(function)</sub>

```python
def BaseIterator.payloads(self):
```

#### `BaseIterator.cleanup`  <sub>(function)</sub>

```python
def BaseIterator.cleanup(self):
```

#### `BaseDictionary`  <sub>(class)</sub>

```python
class BaseDictionary:
```

**Fields:**
```python
def count(self)
def next_word(self)
def __next__(self)
def __iter__(self)
def close(self)
```


#### `BaseDictionary.count`  <sub>(function)</sub>

```python
def BaseDictionary.count(self):
```

#### `BaseDictionary.next_word`  <sub>(function)</sub>

```python
def BaseDictionary.next_word(self):
```

#### `BaseDictionary.__next__`  <sub>(function)</sub>

```python
def BaseDictionary.__next__(self):
```

#### `BaseDictionary.__iter__`  <sub>(function)</sub>

```python
def BaseDictionary.__iter__(self):
```

#### `BaseDictionary.close`  <sub>(function)</sub>

```python
def BaseDictionary.close(self):
```

#### `EncodeIt`  <sub>(class)</sub>

```python
class EncodeIt(BaseDictionary):
```

**Fields:**
```python
def __init__(self, parent, encoders_list)
def count(self)
def concatenate(self, encoder_name, payload_word)
def encode(self, encoder_name, payload_word)
def next_word(self)
def _gen(self)
def __next__(self)
```


#### `EncodeIt.__init__`  <sub>(function)</sub>

```python
def EncodeIt.__init__(self, parent, encoders_list):
```

#### `EncodeIt.count`  <sub>(function)</sub>

```python
def EncodeIt.count(self):
```

#### `EncodeIt.concatenate`  <sub>(function)</sub>

```python
def EncodeIt.concatenate(self, encoder_name, payload_word):
```

#### `EncodeIt.encode`  <sub>(function)</sub>

```python
def EncodeIt.encode(self, encoder_name, payload_word):
```

#### `EncodeIt.next_word`  <sub>(function)</sub>

```python
def EncodeIt.next_word(self):
```

#### `EncodeIt._gen`  <sub>(function)</sub>

```python
def EncodeIt._gen(self):
```

#### `EncodeIt.__next__`  <sub>(function)</sub>

```python
def EncodeIt.__next__(self):
```

#### `TupleIt`  <sub>(class)</sub>

```python
class TupleIt(BaseDictionary, BaseIterator):
```

**Fields:**
```python
def __init__(self, parent)
def count(self)
def width(self)
def payloads(self)
def next_word(self)
```


#### `TupleIt.__init__`  <sub>(function)</sub>

```python
def TupleIt.__init__(self, parent):
```

#### `TupleIt.count`  <sub>(function)</sub>

```python
def TupleIt.count(self):
```

#### `TupleIt.width`  <sub>(function)</sub>

```python
def TupleIt.width(self):
```

#### `TupleIt.payloads`  <sub>(function)</sub>

```python
def TupleIt.payloads(self):
```

#### `TupleIt.next_word`  <sub>(function)</sub>

```python
def TupleIt.next_word(self):
```

#### `WrapperIt`  <sub>(class)</sub>

```python
class WrapperIt(BaseDictionary):
```

**Fields:**
```python
def __init__(self, iterator)
def count(self)
def get_type(self)
def next_word(self)
```


#### `WrapperIt.__init__`  <sub>(function)</sub>

```python
def WrapperIt.__init__(self, iterator):
```

#### `WrapperIt.count`  <sub>(function)</sub>

```python
def WrapperIt.count(self):
```

#### `WrapperIt.get_type`  <sub>(function)</sub>

```python
def WrapperIt.get_type(self):
```

#### `WrapperIt.next_word`  <sub>(function)</sub>

```python
def WrapperIt.next_word(self):
```

#### `SliceIt`  <sub>(class)</sub>

```python
class SliceIt(BaseDictionary):
```

**Fields:**
```python
def __init__(self, payload, slicestr)
def count(self)
def get_type(self)
def _get_filtered_value(self, item)
def next_word(self)
```


#### `SliceIt.__init__`  <sub>(function)</sub>

```python
def SliceIt.__init__(self, payload, slicestr):
```

#### `SliceIt.count`  <sub>(function)</sub>

```python
def SliceIt.count(self):
```

#### `SliceIt.get_type`  <sub>(function)</sub>

```python
def SliceIt.get_type(self):
```

#### `SliceIt._get_filtered_value`  <sub>(function)</sub>

```python
def SliceIt._get_filtered_value(self, item):
```

#### `SliceIt.next_word`  <sub>(function)</sub>

```python
def SliceIt.next_word(self):
```

#### `AllVarDictio`  <sub>(class)</sub>

```python
class AllVarDictio(BaseDictionary, BaseIterator):
```

**Fields:**
```python
def __init__(self, iterator, allvar_len)
def count(self)
def width(self)
def payloads(self)
def next_word(self)
```


#### `AllVarDictio.__init__`  <sub>(function)</sub>

```python
def AllVarDictio.__init__(self, iterator, allvar_len):
```

#### `AllVarDictio.count`  <sub>(function)</sub>

```python
def AllVarDictio.count(self):
```

#### `AllVarDictio.width`  <sub>(function)</sub>

```python
def AllVarDictio.width(self):
```

#### `AllVarDictio.payloads`  <sub>(function)</sub>

```python
def AllVarDictio.payloads(self):
```

#### `AllVarDictio.next_word`  <sub>(function)</sub>

```python
def AllVarDictio.next_word(self):
```

### `wfuzz/src/wfuzz/exception.py`


#### `FuzzException`  <sub>(class)</sub>

```python
class FuzzException(Exception):
```

#### `FuzzExceptBadOptions`  <sub>(class)</sub>

```python
class FuzzExceptBadOptions(FuzzException):
```

#### `FuzzExceptNoPluginError`  <sub>(class)</sub>

```python
class FuzzExceptNoPluginError(FuzzException):
```

#### `FuzzExceptPluginLoadError`  <sub>(class)</sub>

```python
class FuzzExceptPluginLoadError(FuzzException):
```

#### `FuzzExceptIncorrectFilter`  <sub>(class)</sub>

```python
class FuzzExceptIncorrectFilter(FuzzException):
```

#### `FuzzExceptBadAPI`  <sub>(class)</sub>

```python
class FuzzExceptBadAPI(FuzzException):
```

#### `FuzzExceptInternalError`  <sub>(class)</sub>

```python
class FuzzExceptInternalError(FuzzException):
```

#### `FuzzExceptBadFile`  <sub>(class)</sub>

```python
class FuzzExceptBadFile(FuzzException):
```

#### `FuzzExceptBadInstall`  <sub>(class)</sub>

```python
class FuzzExceptBadInstall(FuzzException):
```

#### `FuzzExceptBadRecipe`  <sub>(class)</sub>

```python
class FuzzExceptBadRecipe(FuzzException):
```

#### `FuzzExceptMissingAPIKey`  <sub>(class)</sub>

```python
class FuzzExceptMissingAPIKey(FuzzException):
```

#### `FuzzExceptPluginBadParams`  <sub>(class)</sub>

```python
class FuzzExceptPluginBadParams(FuzzException):
```

#### `FuzzExceptResourceParseError`  <sub>(class)</sub>

```python
class FuzzExceptResourceParseError(FuzzException):
```

#### `FuzzExceptPluginError`  <sub>(class)</sub>

```python
class FuzzExceptPluginError(FuzzException):
```

#### `FuzzExceptNetError`  <sub>(class)</sub>

```python
class FuzzExceptNetError(FuzzException):
```

### `wfuzz/src/wfuzz/externals/moduleman/loader.py`


#### `IModuleLoader`  <sub>(class)</sub>

```python
class IModuleLoader:
```

**Fields:**
```python
def __init__(self)
def set_params(self)
def load(self, registrant)
```


#### `IModuleLoader.__init__`  <sub>(function)</sub>

```python
def IModuleLoader.__init__(self, **params):
```

#### `IModuleLoader.set_params`  <sub>(function)</sub>

```python
def IModuleLoader.set_params(self, **params):
```

#### `IModuleLoader.load`  <sub>(function)</sub>

```python
def IModuleLoader.load(self, registrant):
```

#### `FileLoader`  <sub>(class)</sub>

```python
class FileLoader(IModuleLoader):
```

**Fields:**
```python
def __init__(self)
def set_params(self)
def load(self, registrant)
def _build_id(self, filename, objname)
def _load_py_from_file(self, filename)
```


#### `FileLoader.__init__`  <sub>(function)</sub>

```python
def FileLoader.__init__(self, **params):
```

#### `FileLoader.set_params`  <sub>(function)</sub>

```python
def FileLoader.set_params(self, **params):
```

#### `FileLoader.load`  <sub>(function)</sub>

```python
def FileLoader.load(self, registrant):
```

#### `FileLoader._build_id`  <sub>(function)</sub>

```python
def FileLoader._build_id(self, filename, objname):
```

#### `FileLoader._load_py_from_file`  <sub>(function)</sub>

> Opens "filename", inspects it and calls the registrant

```python
def FileLoader._load_py_from_file(self, filename):
```

#### `DirLoader`  <sub>(class)</sub>

```python
class DirLoader(FileLoader):
```

**Fields:**
```python
def __init__(self)
def set_params(self)
def load(self, registrant)
def _build_id(self, filename, objname)
def __load_all(self, dir_name)
def __walk_dir_tree(self, dirname)
```


#### `DirLoader.__init__`  <sub>(function)</sub>

```python
def DirLoader.__init__(self, **params):
```

#### `DirLoader.set_params`  <sub>(function)</sub>

```python
def DirLoader.set_params(self, **params):
```

#### `DirLoader.load`  <sub>(function)</sub>

```python
def DirLoader.load(self, registrant):
```

#### `DirLoader._build_id`  <sub>(function)</sub>

```python
def DirLoader._build_id(self, filename, objname):
```

#### `DirLoader.__load_all`  <sub>(function)</sub>

> loads all plugins and creates a loaded list of scripts from directory plugins like:  
> [ ( category,[script1, script2,...] ), (category2,[script1, (subcategory,[script1,script2]),...]) ]

```python
def DirLoader.__load_all(self, dir_name):
```

#### `DirLoader.__walk_dir_tree`  <sub>(function)</sub>

```python
def DirLoader.__walk_dir_tree(self, dirname):
```

### `wfuzz/src/wfuzz/externals/moduleman/modulefilter.py`


#### `PYPARSING`  <sub>(const)</sub>

```python
PYPARSING = ...
```

#### `IFilter`  <sub>(class)</sub>

```python
class IFilter:
```

**Fields:**
```python
def is_visible(self, plugin, filter_string)
```


#### `IFilter.is_visible`  <sub>(function)</sub>

```python
def IFilter.is_visible(self, plugin, filter_string):
```

#### `Filter`  <sub>(class)</sub>

```python
class Filter(IFilter):
```

**Fields:**
```python
def __init__(self)
def __compute_neg_formula(self, tokens)
def __compute_element(self, tokens)
def __myreduce(self, elements)
def __compute_formula(self, tokens)
def simple_filter(self, plugin, filter_string)
def simple_filter_banned_keywords(self, filter_string)
def is_visible(self, plugin, filter_string)
```


#### `Filter.__init__`  <sub>(function)</sub>

```python
def Filter.__init__(self):
```

#### `Filter.__compute_neg_formula`  <sub>(function)</sub>

```python
def Filter.__compute_neg_formula(self, tokens):
```

#### `Filter.__compute_element`  <sub>(function)</sub>

```python
def Filter.__compute_element(self, tokens):
```

#### `Filter.__myreduce`  <sub>(function)</sub>

```python
def Filter.__myreduce(self, elements):
```

#### `Filter.__compute_formula`  <sub>(function)</sub>

```python
def Filter.__compute_formula(self, tokens):
```

#### `Filter.simple_filter`  <sub>(function)</sub>

```python
def Filter.simple_filter(self, plugin, filter_string):
```

#### `Filter.simple_filter_banned_keywords`  <sub>(function)</sub>

```python
def Filter.simple_filter_banned_keywords(self, filter_string):
```

#### `Filter.is_visible`  <sub>(function)</sub>

```python
def Filter.is_visible(self, plugin, filter_string):
```

### `wfuzz/src/wfuzz/externals/moduleman/plugin.py`


#### `moduleman_plugin`  <sub>(function)</sub>

```python
def moduleman_plugin(*args):
```

### `wfuzz/src/wfuzz/externals/moduleman/registrant.py`


#### `IRegistrant`  <sub>(class)</sub>

```python
class IRegistrant:
```

**Fields:**
```python
def __init__(self, loader, plg_filter)
def register(self, identifier, module)
def start_loading(self)
def load(self)
def end_loading(self)
def modify_instance(self, module)
```


#### `IRegistrant.__init__`  <sub>(function)</sub>

```python
def IRegistrant.__init__(self, loader, plg_filter):
```

#### `IRegistrant.register`  <sub>(function)</sub>

```python
def IRegistrant.register(self, identifier, module):
```

#### `IRegistrant.start_loading`  <sub>(function)</sub>

```python
def IRegistrant.start_loading(self):
```

#### `IRegistrant.load`  <sub>(function)</sub>

```python
def IRegistrant.load(self):
```

#### `IRegistrant.end_loading`  <sub>(function)</sub>

```python
def IRegistrant.end_loading(self):
```

#### `IRegistrant.modify_instance`  <sub>(function)</sub>

```python
def IRegistrant.modify_instance(self, module):
```

#### `KnowledgeBase`  <sub>(class)</sub>

```python
class KnowledgeBase(MutableMapping):
```

**Fields:**
```python
def __init__(self)
def __getitem__(self, key)
def __setitem__(self, key, value)
def __delitem__(self, key)
def __len__(self)
def __str__(self)
def __iter__(self)
```


#### `KnowledgeBase.__init__`  <sub>(function)</sub>

```python
def KnowledgeBase.__init__(self, *args, **kwargs):
```

#### `KnowledgeBase.__getitem__`  <sub>(function)</sub>

```python
def KnowledgeBase.__getitem__(self, key):
```

#### `KnowledgeBase.__setitem__`  <sub>(function)</sub>

```python
def KnowledgeBase.__setitem__(self, key, value):
```

#### `KnowledgeBase.__delitem__`  <sub>(function)</sub>

```python
def KnowledgeBase.__delitem__(self, key):
```

#### `KnowledgeBase.__len__`  <sub>(function)</sub>

```python
def KnowledgeBase.__len__(self):
```

#### `KnowledgeBase.__str__`  <sub>(function)</sub>

```python
def KnowledgeBase.__str__(self):
```

#### `KnowledgeBase.__iter__`  <sub>(function)</sub>

```python
def KnowledgeBase.__iter__(self):
```

#### `BRegistrant`  <sub>(class)</sub>

```python
class BRegistrant(IRegistrant):
```

**Fields:**
```python
def __init__(self, loader, plg_filter)
def register(self, identifier, module)
def load(self)
def start_loading(self)
def end_loading(self)
def modify_instance(self, module)
def plugin_state(self, identifier, state)
def __get_plugins(self, category, sorting)
def get_plugin(self, identifier)
def get_plugins(self, category, sorting)
def get_plugins_ext(self, category, sorting)
def get_plugins_names(self, category, sorting)
def get_plugins_ids(self, category, sorting)
```


#### `BRegistrant.__init__`  <sub>(function)</sub>

```python
def BRegistrant.__init__(self, loader, plg_filter=Filter()):
```

#### `BRegistrant.register`  <sub>(function)</sub>

```python
def BRegistrant.register(self, identifier, module):
```

#### `BRegistrant.load`  <sub>(function)</sub>

```python
def BRegistrant.load(self):
```

#### `BRegistrant.start_loading`  <sub>(function)</sub>

```python
def BRegistrant.start_loading(self):
```

#### `BRegistrant.end_loading`  <sub>(function)</sub>

```python
def BRegistrant.end_loading(self):
```

#### `BRegistrant.modify_instance`  <sub>(function)</sub>

```python
def BRegistrant.modify_instance(self, module):
```

#### `BRegistrant.plugin_state`  <sub>(function)</sub>

```python
def BRegistrant.plugin_state(self, identifier, state):
```

#### `BRegistrant.__get_plugins`  <sub>(function)</sub>

```python
def BRegistrant.__get_plugins(self, category, sorting):
```

#### `BRegistrant.get_plugin`  <sub>(function)</sub>

```python
def BRegistrant.get_plugin(self, identifier):
```

#### `BRegistrant.get_plugins`  <sub>(function)</sub>

```python
def BRegistrant.get_plugins(self, category="$all$", sorting="true"):
```

#### `BRegistrant.get_plugins_ext`  <sub>(function)</sub>

```python
def BRegistrant.get_plugins_ext(self, category="$all$", sorting="true"):
```

#### `BRegistrant.get_plugins_names`  <sub>(function)</sub>

```python
def BRegistrant.get_plugins_names(self, category="$all$", sorting="true"):
```

#### `BRegistrant.get_plugins_ids`  <sub>(function)</sub>

```python
def BRegistrant.get_plugins_ids(self, category="$all$", sorting="true"):
```

#### `MulRegistrant`  <sub>(class)</sub>

```python
class MulRegistrant(BRegistrant):
```

**Fields:**
```python
def load(self)
```


#### `MulRegistrant.load`  <sub>(function)</sub>

```python
def MulRegistrant.load(self):
```

### `wfuzz/src/wfuzz/externals/reqresp/Request.py`


#### `PYCURL_PATH_AS_IS`  <sub>(const)</sub>

```python
PYCURL_PATH_AS_IS = ...
```

#### `Request`  <sub>(class)</sub>

```python
class Request:
```

**Fields:**
```python
def __init__(self)
def method(self)
def method(self, value)
def setFinalUrl(self, fu)
def __str__(self)
def getHost(self)
def getXML(self, obj)
def __getattr__(self, name)
def setUrl(self, urltmp)
def getProxy(self)
def setProxy(self, prox, ptype)
def setFollowLocation(self, value)
def setConnTimeout(self, time)
def getConnTimeout(self)
def setTotalTimeout(self, time)
def getTotalTimeout(self)
def setAuth(self, method, string)
def getAuth(self)
def existsGETVar(self, key)
def existPOSTVar(self, key)
def setVariablePOST(self, key, value)
def setVariableGET(self, key, value)
def getGETVars(self)
def getPOSTVars(self)
def setPostData(self, pd, boundary)
def addHeader(self, key, value)
def delHeader(self, key)
def __getitem__(self, key)
def getHeaders(self)
def head(self)
def createPath(self, newpath)
def to_pycurl_object(c, req)
def response_from_conn_object(self, conn, header, body)
def perform(self)
def getAll(self)
def header_callback(self, data)
def body_callback(self, data)
def Substitute(self, src, dst)
def parseRequest(self, rawRequest, prot)
```


#### `Request.__init__`  <sub>(function)</sub>

```python
def Request.__init__(self):
```

#### `Request.method`  <sub>(function)</sub>

```python
def Request.method(self):
```

#### `Request.method`  <sub>(function)</sub>

```python
def Request.method(self, value):
```

#### `Request.setFinalUrl`  <sub>(function)</sub>

```python
def Request.setFinalUrl(self, fu):
```

#### `Request.__str__`  <sub>(function)</sub>

```python
def Request.__str__(self):
```

#### `Request.getHost`  <sub>(function)</sub>

```python
def Request.getHost(self):
```

#### `Request.getXML`  <sub>(function)</sub>

```python
def Request.getXML(self, obj):
```

#### `Request.__getattr__`  <sub>(function)</sub>

```python
def Request.__getattr__(self, name):
```

#### `Request.setUrl`  <sub>(function)</sub>

```python
def Request.setUrl(self, urltmp):
```

#### `Request.getProxy`  <sub>(function)</sub>

```python
def Request.getProxy(self):
```

#### `Request.setProxy`  <sub>(function)</sub>

```python
def Request.setProxy(self, prox, ptype):
```

#### `Request.setFollowLocation`  <sub>(function)</sub>

```python
def Request.setFollowLocation(self, value):
```

#### `Request.setConnTimeout`  <sub>(function)</sub>

```python
def Request.setConnTimeout(self, time):
```

#### `Request.getConnTimeout`  <sub>(function)</sub>

```python
def Request.getConnTimeout(self):
```

#### `Request.setTotalTimeout`  <sub>(function)</sub>

```python
def Request.setTotalTimeout(self, time):
```

#### `Request.getTotalTimeout`  <sub>(function)</sub>

```python
def Request.getTotalTimeout(self):
```

#### `Request.setAuth`  <sub>(function)</sub>

```python
def Request.setAuth(self, method, string):
```

#### `Request.getAuth`  <sub>(function)</sub>

```python
def Request.getAuth(self):
```

#### `Request.existsGETVar`  <sub>(function)</sub>

```python
def Request.existsGETVar(self, key):
```

#### `Request.existPOSTVar`  <sub>(function)</sub>

```python
def Request.existPOSTVar(self, key):
```

#### `Request.setVariablePOST`  <sub>(function)</sub>

```python
def Request.setVariablePOST(self, key, value):
```

#### `Request.setVariableGET`  <sub>(function)</sub>

```python
def Request.setVariableGET(self, key, value):
```

#### `Request.getGETVars`  <sub>(function)</sub>

```python
def Request.getGETVars(self):
```

#### `Request.getPOSTVars`  <sub>(function)</sub>

```python
def Request.getPOSTVars(self):
```

#### `Request.setPostData`  <sub>(function)</sub>

```python
def Request.setPostData(self, pd, boundary=None):
```

#### `Request.addHeader`  <sub>(function)</sub>

```python
def Request.addHeader(self, key, value):
```

#### `Request.delHeader`  <sub>(function)</sub>

```python
def Request.delHeader(self, key):
```

#### `Request.__getitem__`  <sub>(function)</sub>

```python
def Request.__getitem__(self, key):
```

#### `Request.getHeaders`  <sub>(function)</sub>

```python
def Request.getHeaders(self):
```

#### `Request.head`  <sub>(function)</sub>

```python
def Request.head(self):
```

#### `Request.createPath`  <sub>(function)</sub>

> Creates new url from a location header || Hecho para el followLocation=true

```python
def Request.createPath(self, newpath):
```

#### `Request.to_pycurl_object`  <sub>(function)</sub>

```python
def Request.to_pycurl_object(c, req):
```

#### `Request.response_from_conn_object`  <sub>(function)</sub>

```python
def Request.response_from_conn_object(self, conn, header, body):
```

#### `Request.perform`  <sub>(function)</sub>

```python
def Request.perform(self):
```

#### `Request.getAll`  <sub>(function)</sub>

```python
def Request.getAll(self):
```

#### `Request.header_callback`  <sub>(function)</sub>

```python
def Request.header_callback(self, data):
```

#### `Request.body_callback`  <sub>(function)</sub>

```python
def Request.body_callback(self, data):
```

#### `Request.Substitute`  <sub>(function)</sub>

```python
def Request.Substitute(self, src, dst):
```

#### `Request.parseRequest`  <sub>(function)</sub>

> Aun esta en fase BETA y por probar

```python
def Request.parseRequest(self, rawRequest, prot="http"):
```

### `wfuzz/src/wfuzz/externals/reqresp/Response.py`


#### `get_encoding_from_headers`  <sub>(function)</sub>

> Returns encodings from given HTTP Header Dict.  
>   
> :param headers: dictionary to extract encoding from.  
> :rtype: str

```python
def get_encoding_from_headers(headers):
```

#### `get_encodings_from_content`  <sub>(function)</sub>

> Returns encodings from given content string.  
>   
> :param content: bytestring to extract encodings from.

```python
def get_encodings_from_content(content):
```

#### `Response`  <sub>(class)</sub>

```python
class Response:
```

**Fields:**
```python
def __init__(self, protocol, code, message)
def addHeader(self, key, value)
def delHeader(self, key)
def addContent(self, text)
def __getitem__(self, key)
def getCookie(self)
def has_header(self, key)
def getLocation(self)
def header_equal(self, header, value)
def getHeaders(self)
def getContent(self)
def getTextHeaders(self)
def getAll(self)
def Substitute(self, src, dst)
def getAll_wpost(self)
def parseResponse(self, rawheader, rawbody, type)
```


#### `Response.__init__`  <sub>(function)</sub>

```python
def Response.__init__(self, protocol="", code="", message=""):
```

#### `Response.addHeader`  <sub>(function)</sub>

```python
def Response.addHeader(self, key, value):
```

#### `Response.delHeader`  <sub>(function)</sub>

```python
def Response.delHeader(self, key):
```

#### `Response.addContent`  <sub>(function)</sub>

```python
def Response.addContent(self, text):
```

#### `Response.__getitem__`  <sub>(function)</sub>

```python
def Response.__getitem__(self, key):
```

#### `Response.getCookie`  <sub>(function)</sub>

```python
def Response.getCookie(self):
```

#### `Response.has_header`  <sub>(function)</sub>

```python
def Response.has_header(self, key):
```

#### `Response.getLocation`  <sub>(function)</sub>

```python
def Response.getLocation(self):
```

#### `Response.header_equal`  <sub>(function)</sub>

```python
def Response.header_equal(self, header, value):
```

#### `Response.getHeaders`  <sub>(function)</sub>

```python
def Response.getHeaders(self):
```

#### `Response.getContent`  <sub>(function)</sub>

```python
def Response.getContent(self):
```

#### `Response.getTextHeaders`  <sub>(function)</sub>

```python
def Response.getTextHeaders(self):
```

#### `Response.getAll`  <sub>(function)</sub>

```python
def Response.getAll(self):
```

#### `Response.Substitute`  <sub>(function)</sub>

```python
def Response.Substitute(self, src, dst):
```

#### `Response.getAll_wpost`  <sub>(function)</sub>

```python
def Response.getAll_wpost(self):
```

#### `Response.parseResponse`  <sub>(function)</sub>

```python
def Response.parseResponse(self, rawheader, rawbody=None, type="curl"):
```

### `wfuzz/src/wfuzz/externals/reqresp/TextParser.py`


#### `TextParser`  <sub>(class)</sub>

```python
class TextParser(object):
```

**Fields:**
```python
def __init__(self)
def __del__(self)
def __str__(self)
def __iter__(self)
def __next__(self)
def setSource(self, t)
def seekinit(self)
def readUntil(self, pattern, caseSens)
def search(self, pattern, caseSens, debug)
def __getitem__(self, key)
def skip(self, lines)
def readLine(self)
```


#### `TextParser.__init__`  <sub>(function)</sub>

```python
def TextParser.__init__(self):
```

#### `TextParser.__del__`  <sub>(function)</sub>

```python
def TextParser.__del__(self):
```

#### `TextParser.__str__`  <sub>(function)</sub>

```python
def TextParser.__str__(self):
```

#### `TextParser.__iter__`  <sub>(function)</sub>

```python
def TextParser.__iter__(self):
```

#### `TextParser.__next__`  <sub>(function)</sub>

```python
def TextParser.__next__(self):
```

#### `TextParser.setSource`  <sub>(function)</sub>

> Se especifica el tipo de entrada. Puede ser fichero o entrada estandard  
>   
> Ejemplos: setSource("file","/tmp/file")  
> setSource("stdin")

```python
def TextParser.setSource(self, t, *args):
```

#### `TextParser.seekinit`  <sub>(function)</sub>

```python
def TextParser.seekinit(self):
```

#### `TextParser.readUntil`  <sub>(function)</sub>

> Lee lineas hasta que el patron (pattern) conincide en alguna linea

```python
def TextParser.readUntil(self, pattern, caseSens=True):
```

#### `TextParser.search`  <sub>(function)</sub>

> Intenta hacer Matching entre el pattern pasado por parametro y la ultima linea leida

```python
def TextParser.search(self, pattern, caseSens=True, debug=0):
```

#### `TextParser.__getitem__`  <sub>(function)</sub>

> Para acceder a cada uno de los patrones que coinciden, esta preparado paragrupos de patrones, no para solo un patron

```python
def TextParser.__getitem__(self, key):
```

#### `TextParser.skip`  <sub>(function)</sub>

> Salta las lines que se indiquen en el parametro

```python
def TextParser.skip(self, lines):
```

#### `TextParser.readLine`  <sub>(function)</sub>

> Lee la siguiente linea eliminando retornos de carro

```python
def TextParser.readLine(self):
```

### `wfuzz/src/wfuzz/externals/reqresp/Variables.py`


#### `Variable`  <sub>(class)</sub>

```python
class Variable:
```

**Fields:**
```python
def __init__(self, name, value, extraInfo)
def restore(self)
def change(self, newval)
def update(self, val)
def append(self, val)
def __str__(self)
```


#### `Variable.__init__`  <sub>(function)</sub>

```python
def Variable.__init__(self, name, value="", extraInfo=""):
```

#### `Variable.restore`  <sub>(function)</sub>

```python
def Variable.restore(self):
```

#### `Variable.change`  <sub>(function)</sub>

```python
def Variable.change(self, newval):
```

#### `Variable.update`  <sub>(function)</sub>

```python
def Variable.update(self, val):
```

#### `Variable.append`  <sub>(function)</sub>

```python
def Variable.append(self, val):
```

#### `Variable.__str__`  <sub>(function)</sub>

```python
def Variable.__str__(self):
```

#### `VariablesSet`  <sub>(class)</sub>

```python
class VariablesSet:
```

**Fields:**
```python
def __init__(self)
def names(self)
def existsVar(self, name)
def addVariable(self, name, value, extraInfo)
def getVariable(self, name)
def urlEncoded(self)
def json_encoded(self)
def parse_json_encoded(self, cad)
def parseUrlEncoded(self, cad)
def multipartEncoded(self)
def parseMultipart(self, cad, boundary)
```


#### `VariablesSet.__init__`  <sub>(function)</sub>

```python
def VariablesSet.__init__(self):
```

#### `VariablesSet.names`  <sub>(function)</sub>

```python
def VariablesSet.names(self):
```

#### `VariablesSet.existsVar`  <sub>(function)</sub>

```python
def VariablesSet.existsVar(self, name):
```

#### `VariablesSet.addVariable`  <sub>(function)</sub>

```python
def VariablesSet.addVariable(self, name, value="", extraInfo=""):
```

#### `VariablesSet.getVariable`  <sub>(function)</sub>

```python
def VariablesSet.getVariable(self, name):
```

#### `VariablesSet.urlEncoded`  <sub>(function)</sub>

```python
def VariablesSet.urlEncoded(self):
```

#### `VariablesSet.json_encoded`  <sub>(function)</sub>

```python
def VariablesSet.json_encoded(self):
```

#### `VariablesSet.parse_json_encoded`  <sub>(function)</sub>

```python
def VariablesSet.parse_json_encoded(self, cad):
```

#### `VariablesSet.parseUrlEncoded`  <sub>(function)</sub>

```python
def VariablesSet.parseUrlEncoded(self, cad):
```

#### `VariablesSet.multipartEncoded`  <sub>(function)</sub>

```python
def VariablesSet.multipartEncoded(self):
```

#### `VariablesSet.parseMultipart`  <sub>(function)</sub>

```python
def VariablesSet.parseMultipart(self, cad, boundary):
```

### `wfuzz/src/wfuzz/externals/reqresp/cache.py`


#### `HttpCache`  <sub>(class)</sub>

```python
class HttpCache:
```

**Fields:**
```python
def __init__(self)
def update_cache(self, req, category)
def msg_in_cache(self, req, category)
```


#### `HttpCache.__init__`  <sub>(function)</sub>

```python
def HttpCache.__init__(self):
```

#### `HttpCache.update_cache`  <sub>(function)</sub>

```python
def HttpCache.update_cache(self, req, category="default"):
```

#### `HttpCache.msg_in_cache`  <sub>(function)</sub>

```python
def HttpCache.msg_in_cache(self, req, category="default"):
```

### `wfuzz/src/wfuzz/externals/reqresp/exceptions.py`


#### `ReqRespException`  <sub>(class)</sub>

```python
class ReqRespException(Exception):
```

**Fields:**
```python
def __init__(self, etype, msg)
```


#### `ReqRespException.__init__`  <sub>(function)</sub>

```python
def ReqRespException.__init__(self, etype, msg):
```

### `wfuzz/src/wfuzz/externals/settings/settings.py`


#### `SettingsBase`  <sub>(class)</sub>

> Contains application settings. uses a ConfigParser

```python
class SettingsBase:
```

**Fields:**
```python
def __init__(self, save)
def get_config_file(self)
def set_defaults(self)
def has_option(self, section, setting)
def set(self, section, setting, value)
def get(self, section, setting)
def get_section(self, section)
def get_options(self, section)
def get_sections(self)
def get_all(self)
def set_all(self, sett)
def save(self)
def _path_to_program_dir(self)
```


#### `SettingsBase.__init__`  <sub>(function)</sub>

```python
def SettingsBase.__init__(self, save=False):
```

#### `SettingsBase.get_config_file`  <sub>(function)</sub>

> Returns the name of the file where the config is saved.

```python
def SettingsBase.get_config_file(self):
```

#### `SettingsBase.set_defaults`  <sub>(function)</sub>

> Returns a dictionary with the default settings in the form of  
> {                 Section: [                     ("setting_x", '5'),  
> ...  
> ("setting_y", '5'),  
> ],  
> ...  
> }

```python
def SettingsBase.set_defaults(self):
```

#### `SettingsBase.has_option`  <sub>(function)</sub>

```python
def SettingsBase.has_option(self, section, setting):
```

#### `SettingsBase.set`  <sub>(function)</sub>

```python
def SettingsBase.set(self, section, setting, value):
```

#### `SettingsBase.get`  <sub>(function)</sub>

```python
def SettingsBase.get(self, section, setting):
```

#### `SettingsBase.get_section`  <sub>(function)</sub>

```python
def SettingsBase.get_section(self, section):
```

#### `SettingsBase.get_options`  <sub>(function)</sub>

```python
def SettingsBase.get_options(self, section):
```

#### `SettingsBase.get_sections`  <sub>(function)</sub>

```python
def SettingsBase.get_sections(self):
```

#### `SettingsBase.get_all`  <sub>(function)</sub>

```python
def SettingsBase.get_all(self):
```

#### `SettingsBase.set_all`  <sub>(function)</sub>

```python
def SettingsBase.set_all(self, sett):
```

#### `SettingsBase.save`  <sub>(function)</sub>

```python
def SettingsBase.save(self):
```

#### `SettingsBase._path_to_program_dir`  <sub>(function)</sub>

> Returns path to program directory

```python
def SettingsBase._path_to_program_dir(self):
```

### `wfuzz/src/wfuzz/facade.py`


#### `ERROR_CODE`  <sub>(const)</sub>

```python
ERROR_CODE = ...
```

#### `BASELINE_CODE`  <sub>(const)</sub>

```python
BASELINE_CODE = ...
```

#### `Settings`  <sub>(class)</sub>

```python
class Settings(SettingsBase):
```

**Fields:**
```python
def get_config_file(self)
def set_defaults(self)
```


#### `Settings.get_config_file`  <sub>(function)</sub>

```python
def Settings.get_config_file(self):
```

#### `Settings.set_defaults`  <sub>(function)</sub>

```python
def Settings.set_defaults(self):
```

#### `MyRegistrant`  <sub>(class)</sub>

```python
class MyRegistrant(MulRegistrant):
```

**Fields:**
```python
def get_plugin(self, identifier)
```


#### `MyRegistrant.get_plugin`  <sub>(function)</sub>

```python
def MyRegistrant.get_plugin(self, identifier):
```

#### `Facade`  <sub>(class)</sub>

```python
class Facade(metaclass=Singleton):
```

**Fields:**
```python
def __init__(self)
def _load(self, cat)
def proxy(self, which)
def get_registrants(self)
def __getattr__(self, name)
```


#### `Facade.__init__`  <sub>(function)</sub>

```python
def Facade.__init__(self):
```

#### `Facade._load`  <sub>(function)</sub>

```python
def Facade._load(self, cat):
```

#### `Facade.proxy`  <sub>(function)</sub>

```python
def Facade.proxy(self, which):
```

#### `Facade.get_registrants`  <sub>(function)</sub>

```python
def Facade.get_registrants(self):
```

#### `Facade.__getattr__`  <sub>(function)</sub>

```python
def Facade.__getattr__(self, name):
```

### `wfuzz/src/wfuzz/factories/dictfactory.py`


#### `DictionaryFactory`  <sub>(class)</sub>

```python
class DictionaryFactory(ObjectFactory):
```

**Fields:**
```python
def __init__(self)
```


#### `DictionaryFactory.__init__`  <sub>(function)</sub>

```python
def DictionaryFactory.__init__(self):
```

#### `BaseDictioBuilder`  <sub>(class)</sub>

```python
class BaseDictioBuilder:
```

**Fields:**
```python
def validate(options, selected_dic)
def get_dictio(options, selected_dic)
```


#### `BaseDictioBuilder.validate`  <sub>(function)</sub>

```python
def BaseDictioBuilder.validate(options, selected_dic):
```

#### `BaseDictioBuilder.get_dictio`  <sub>(function)</sub>

```python
def BaseDictioBuilder.get_dictio(options, selected_dic):
```

#### `DictioFromIterableBuilder`  <sub>(class)</sub>

```python
class DictioFromIterableBuilder(BaseDictioBuilder):
```

**Fields:**
```python
def __call__(self, options)
```


#### `DictioFromIterableBuilder.__call__`  <sub>(function)</sub>

```python
def DictioFromIterableBuilder.__call__(self, options):
```

#### `DictioFromPayloadBuilder`  <sub>(class)</sub>

```python
class DictioFromPayloadBuilder(BaseDictioBuilder):
```

**Fields:**
```python
def __call__(self, options)
```


#### `DictioFromPayloadBuilder.__call__`  <sub>(function)</sub>

```python
def DictioFromPayloadBuilder.__call__(self, options):
```

#### `DictioFromAllVarBuilder`  <sub>(class)</sub>

```python
class DictioFromAllVarBuilder(BaseDictioBuilder):
```

**Fields:**
```python
def from_all_fuzz_request_gen(options, dictio_list)
def __call__(self, options)
```


#### `DictioFromAllVarBuilder.from_all_fuzz_request_gen`  <sub>(function)</sub>

```python
def DictioFromAllVarBuilder.from_all_fuzz_request_gen(options, dictio_list):
```

#### `DictioFromAllVarBuilder.__call__`  <sub>(function)</sub>

```python
def DictioFromAllVarBuilder.__call__(self, options):
```

#### `DictioFromOptions`  <sub>(class)</sub>

```python
class DictioFromOptions(BaseDictioBuilder):
```

**Fields:**
```python
def __call__(self, options)
```


#### `DictioFromOptions.__call__`  <sub>(function)</sub>

```python
def DictioFromOptions.__call__(self, options):
```

### `wfuzz/src/wfuzz/factories/fuzzfactory.py`


#### `FuzzRequestFactory`  <sub>(class)</sub>

```python
class FuzzRequestFactory(ObjectFactory):
```

**Fields:**
```python
def __init__(self)
```


#### `FuzzRequestFactory.__init__`  <sub>(function)</sub>

```python
def FuzzRequestFactory.__init__(self):
```

#### `RequestBuilder`  <sub>(class)</sub>

```python
class RequestBuilder:
```

**Fields:**
```python
def __call__(self, options)
```


#### `RequestBuilder.__call__`  <sub>(function)</sub>

```python
def RequestBuilder.__call__(self, options):
```

#### `SeedBuilder`  <sub>(class)</sub>

```python
class SeedBuilder:
```

**Fields:**
```python
def __call__(self, options)
```


#### `SeedBuilder.__call__`  <sub>(function)</sub>

```python
def SeedBuilder.__call__(self, options):
```

### `wfuzz/src/wfuzz/factories/fuzzresfactory.py`


#### `FuzzResultFactory`  <sub>(class)</sub>

```python
class FuzzResultFactory(ObjectFactory):
```

**Fields:**
```python
def __init__(self)
```


#### `FuzzResultFactory.__init__`  <sub>(function)</sub>

```python
def FuzzResultFactory.__init__(self):
```

#### `FuzzResultDictioBuilder`  <sub>(class)</sub>

```python
class FuzzResultDictioBuilder:
```

**Fields:**
```python
def __call__(self, options, dictio_item)
```


#### `FuzzResultDictioBuilder.__call__`  <sub>(function)</sub>

```python
def FuzzResultDictioBuilder.__call__(self, options, dictio_item):
```

#### `SeedResultBuilder`  <sub>(class)</sub>

```python
class SeedResultBuilder:
```

**Fields:**
```python
def __call__(self, options)
```


#### `SeedResultBuilder.__call__`  <sub>(function)</sub>

```python
def SeedResultBuilder.__call__(self, options):
```

#### `BaselineResultBuilder`  <sub>(class)</sub>

```python
class BaselineResultBuilder:
```

**Fields:**
```python
def __call__(self, options)
```


#### `BaselineResultBuilder.__call__`  <sub>(function)</sub>

```python
def BaselineResultBuilder.__call__(self, options):
```

#### `FuzzResultAllVarBuilder`  <sub>(class)</sub>

```python
class FuzzResultAllVarBuilder:
```

**Fields:**
```python
def __call__(self, options, var_name, payload)
```


#### `FuzzResultAllVarBuilder.__call__`  <sub>(function)</sub>

```python
def FuzzResultAllVarBuilder.__call__(self, options, var_name, payload):
```

#### `FuzzResultDictSeedBuilder`  <sub>(class)</sub>

```python
class FuzzResultDictSeedBuilder:
```

**Fields:**
```python
def __call__(self, options, dictio)
```


#### `FuzzResultDictSeedBuilder.__call__`  <sub>(function)</sub>

```python
def FuzzResultDictSeedBuilder.__call__(self, options, dictio):
```

#### `SeedRecursiveBuilder`  <sub>(class)</sub>

```python
class SeedRecursiveBuilder:
```

**Fields:**
```python
def __call__(self, seed)
```


#### `SeedRecursiveBuilder.__call__`  <sub>(function)</sub>

```python
def SeedRecursiveBuilder.__call__(self, seed):
```

#### `FuzzResRecursiveBuilder`  <sub>(class)</sub>

```python
class FuzzResRecursiveBuilder:
```

**Fields:**
```python
def __call__(self, seed, url)
```


#### `FuzzResRecursiveBuilder.__call__`  <sub>(function)</sub>

```python
def FuzzResRecursiveBuilder.__call__(self, seed, url):
```

### `wfuzz/src/wfuzz/factories/payman.py`


#### `PayManFactory`  <sub>(class)</sub>

```python
class PayManFactory(ObjectFactory):
```

**Fields:**
```python
def __init__(self)
```


#### `PayManFactory.__init__`  <sub>(function)</sub>

```python
def PayManFactory.__init__(self):
```

#### `FuzzReqPayloadManBuilder`  <sub>(class)</sub>

```python
class FuzzReqPayloadManBuilder:
```

**Fields:**
```python
def __call__(self, freq)
```


#### `FuzzReqPayloadManBuilder.__call__`  <sub>(function)</sub>

```python
def FuzzReqPayloadManBuilder.__call__(self, freq):
```

#### `OnePayloadManBuilder`  <sub>(class)</sub>

```python
class OnePayloadManBuilder:
```

**Fields:**
```python
def __call__(self, content)
```


#### `OnePayloadManBuilder.__call__`  <sub>(function)</sub>

```python
def OnePayloadManBuilder.__call__(self, content):
```

#### `BaselinePayloadManBuilder`  <sub>(class)</sub>

```python
class BaselinePayloadManBuilder:
```

**Fields:**
```python
def __call__(self, freq)
```


#### `BaselinePayloadManBuilder.__call__`  <sub>(function)</sub>

```python
def BaselinePayloadManBuilder.__call__(self, freq):
```

### `wfuzz/src/wfuzz/factories/plugin_factory.py`


#### `PluginFactory`  <sub>(class)</sub>

```python
class PluginFactory(ObjectFactory):
```

**Fields:**
```python
def __init__(self)
```


#### `PluginFactory.__init__`  <sub>(function)</sub>

```python
def PluginFactory.__init__(self):
```

#### `PluginRecursiveBuilder`  <sub>(class)</sub>

```python
class PluginRecursiveBuilder:
```

**Fields:**
```python
def __call__(self, name, seed, url)
```


#### `PluginRecursiveBuilder.__call__`  <sub>(function)</sub>

```python
def PluginRecursiveBuilder.__call__(self, name, seed, url):
```

#### `PluginErrorBuilder`  <sub>(class)</sub>

```python
class PluginErrorBuilder:
```

**Fields:**
```python
def __call__(self, name, exception)
```


#### `PluginErrorBuilder.__call__`  <sub>(function)</sub>

```python
def PluginErrorBuilder.__call__(self, name, exception):
```

#### `PluginFindingBuilder`  <sub>(class)</sub>

```python
class PluginFindingBuilder:
```

**Fields:**
```python
def __call__(self, name, itype, message, data, severity)
```


#### `PluginFindingBuilder.__call__`  <sub>(function)</sub>

```python
def PluginFindingBuilder.__call__(self, name, itype, message, data, severity):
```

#### `PluginFindingSummaryBuilder`  <sub>(class)</sub>

```python
class PluginFindingSummaryBuilder:
```

**Fields:**
```python
def __call__(self, message)
```


#### `PluginFindingSummaryBuilder.__call__`  <sub>(function)</sub>

```python
def PluginFindingSummaryBuilder.__call__(self, message):
```

### `wfuzz/src/wfuzz/factories/reqresp_factory.py`


#### `PYCURL_PATH_AS_IS`  <sub>(const)</sub>

```python
PYCURL_PATH_AS_IS = ...
```

#### `ReqRespRequestFactory`  <sub>(class)</sub>

```python
class ReqRespRequestFactory(HttpRequestFactory):
```

**Fields:**
```python
def to_http_object(options, req, pycurl_c)
def from_http_object(options, req, pycurl_c, header, body)
```


#### `ReqRespRequestFactory.to_http_object`  <sub>(function)</sub>

```python
def ReqRespRequestFactory.to_http_object(options, req, pycurl_c):
```

#### `ReqRespRequestFactory.from_http_object`  <sub>(function)</sub>

```python
def ReqRespRequestFactory.from_http_object(options, req, pycurl_c, header, body):
```

### `wfuzz/src/wfuzz/filters/ppfilter.py`


#### `PYPARSING`  <sub>(const)</sub>

```python
PYPARSING = ...
```

#### `FuzzResFilter`  <sub>(class)</sub>

```python
class FuzzResFilter:
```

**Fields:**
```python
FUZZ_MARKER_REGEX = ...
def __init__(self, filter_string)
def set_baseline(self, res)
def _compute_res_symbol(self, tokens)
def _compute_fuzz_symbol(self, tokens)
def __compute_res_value(self, tokens)
def _get_payload_value(self, p_index)
def _get_field_value(self, fuzz_val, field)
def __compute_bbb_symbol(self, tokens)
def _get_operator_value(self, location, fuzz_val, match_dict)
def __compute_xxx_value(self, tokens)
def __compute_expr(self, tokens)
def __myreduce(self, elements)
def __compute_not_operator(self, tokens)
def __compute_formula(self, tokens)
def is_active(self)
def is_visible(self, res, filter_string)
def get_fuzz_words(self)
```


#### `FuzzResFilter.__init__`  <sub>(function)</sub>

```python
def FuzzResFilter.__init__(self, filter_string=None):
```

#### `FuzzResFilter.set_baseline`  <sub>(function)</sub>

```python
def FuzzResFilter.set_baseline(self, res):
```

#### `FuzzResFilter._compute_res_symbol`  <sub>(function)</sub>

```python
def FuzzResFilter._compute_res_symbol(self, tokens):
```

#### `FuzzResFilter._compute_fuzz_symbol`  <sub>(function)</sub>

```python
def FuzzResFilter._compute_fuzz_symbol(self, tokens):
```

#### `FuzzResFilter.__compute_res_value`  <sub>(function)</sub>

```python
def FuzzResFilter.__compute_res_value(self, tokens):
```

#### `FuzzResFilter._get_payload_value`  <sub>(function)</sub>

```python
def FuzzResFilter._get_payload_value(self, p_index):
```

#### `FuzzResFilter._get_field_value`  <sub>(function)</sub>

```python
def FuzzResFilter._get_field_value(self, fuzz_val, field):
```

#### `FuzzResFilter.__compute_bbb_symbol`  <sub>(function)</sub>

```python
def FuzzResFilter.__compute_bbb_symbol(self, tokens):
```

#### `FuzzResFilter._get_operator_value`  <sub>(function)</sub>

```python
def FuzzResFilter._get_operator_value(self, location, fuzz_val, match_dict):
```

#### `FuzzResFilter.__compute_xxx_value`  <sub>(function)</sub>

```python
def FuzzResFilter.__compute_xxx_value(self, tokens):
```

#### `FuzzResFilter.__compute_expr`  <sub>(function)</sub>

```python
def FuzzResFilter.__compute_expr(self, tokens):
```

#### `FuzzResFilter.__myreduce`  <sub>(function)</sub>

```python
def FuzzResFilter.__myreduce(self, elements):
```

#### `FuzzResFilter.__compute_not_operator`  <sub>(function)</sub>

```python
def FuzzResFilter.__compute_not_operator(self, tokens):
```

#### `FuzzResFilter.__compute_formula`  <sub>(function)</sub>

```python
def FuzzResFilter.__compute_formula(self, tokens):
```

#### `FuzzResFilter.is_active`  <sub>(function)</sub>

```python
def FuzzResFilter.is_active(self):
```

#### `FuzzResFilter.is_visible`  <sub>(function)</sub>

```python
def FuzzResFilter.is_visible(self, res, filter_string=None):
```

#### `FuzzResFilter.get_fuzz_words`  <sub>(function)</sub>

```python
def FuzzResFilter.get_fuzz_words(self):
```

#### `FuzzResFilterSlice`  <sub>(class)</sub>

```python
class FuzzResFilterSlice(FuzzResFilter):
```

**Fields:**
```python
def _compute_fuzz_symbol(self, tokens)
```


#### `FuzzResFilterSlice._compute_fuzz_symbol`  <sub>(function)</sub>

```python
def FuzzResFilterSlice._compute_fuzz_symbol(self, tokens):
```

### `wfuzz/src/wfuzz/filters/simplefilter.py`


#### `FuzzResSimpleFilter`  <sub>(class)</sub>

```python
class FuzzResSimpleFilter:
```

**Fields:**
```python
def __init__(self, ffilter)
def is_active(self)
def set_baseline(self, res)
def is_visible(self, res)
def from_options(filter_options)
```


#### `FuzzResSimpleFilter.__init__`  <sub>(function)</sub>

```python
def FuzzResSimpleFilter.__init__(self, ffilter=None):
```

#### `FuzzResSimpleFilter.is_active`  <sub>(function)</sub>

```python
def FuzzResSimpleFilter.is_active(self):
```

#### `FuzzResSimpleFilter.set_baseline`  <sub>(function)</sub>

```python
def FuzzResSimpleFilter.set_baseline(self, res):
```

#### `FuzzResSimpleFilter.is_visible`  <sub>(function)</sub>

```python
def FuzzResSimpleFilter.is_visible(self, res):
```

#### `FuzzResSimpleFilter.from_options`  <sub>(function)</sub>

```python
def FuzzResSimpleFilter.from_options(filter_options):
```

### `wfuzz/src/wfuzz/fuzzobjects.py`


#### `FuzzWordType`  <sub>(class)</sub>

```python
class FuzzWordType(Enum):
```

#### `FuzzType`  <sub>(class)</sub>

```python
class FuzzType(Enum):
```

#### `FuzzItem`  <sub>(class)</sub>

```python
class FuzzItem(object):
```

**Fields:**
```python
newid = ...
def __init__(self, item_type)
def __str__(self)
def __lt__(self, other)
def __le__(self, other)
def __gt__(self, other)
def __ge__(self, other)
def __eq__(self, other)
def __ne__(self, other)
```


#### `FuzzItem.__init__`  <sub>(function)</sub>

```python
def FuzzItem.__init__(self, item_type):
```

#### `FuzzItem.__str__`  <sub>(function)</sub>

```python
def FuzzItem.__str__(self):
```

#### `FuzzItem.__lt__`  <sub>(function)</sub>

```python
def FuzzItem.__lt__(self, other):
```

#### `FuzzItem.__le__`  <sub>(function)</sub>

```python
def FuzzItem.__le__(self, other):
```

#### `FuzzItem.__gt__`  <sub>(function)</sub>

```python
def FuzzItem.__gt__(self, other):
```

#### `FuzzItem.__ge__`  <sub>(function)</sub>

```python
def FuzzItem.__ge__(self, other):
```

#### `FuzzItem.__eq__`  <sub>(function)</sub>

```python
def FuzzItem.__eq__(self, other):
```

#### `FuzzItem.__ne__`  <sub>(function)</sub>

```python
def FuzzItem.__ne__(self, other):
```

#### `FuzzStats`  <sub>(class)</sub>

```python
class FuzzStats:
```

**Fields:**
```python
def __init__(self)
def from_options(options)
def get_stats(self)
def mark_start(self)
def mark_end(self)
def cancelled(self)
def cancelled(self, v)
def __str__(self)
def update(self, fuzzstats2)
```


#### `FuzzStats.__init__`  <sub>(function)</sub>

```python
def FuzzStats.__init__(self):
```

#### `FuzzStats.from_options`  <sub>(function)</sub>

```python
def FuzzStats.from_options(options):
```

#### `FuzzStats.get_stats`  <sub>(function)</sub>

```python
def FuzzStats.get_stats(self):
```

#### `FuzzStats.mark_start`  <sub>(function)</sub>

```python
def FuzzStats.mark_start(self):
```

#### `FuzzStats.mark_end`  <sub>(function)</sub>

```python
def FuzzStats.mark_end(self):
```

#### `FuzzStats.cancelled`  <sub>(function)</sub>

```python
def FuzzStats.cancelled(self):
```

#### `FuzzStats.cancelled`  <sub>(function)</sub>

```python
def FuzzStats.cancelled(self, v):
```

#### `FuzzStats.__str__`  <sub>(function)</sub>

```python
def FuzzStats.__str__(self):
```

#### `FuzzStats.update`  <sub>(function)</sub>

```python
def FuzzStats.update(self, fuzzstats2):
```

#### `FuzzPayload`  <sub>(class)</sub>

```python
class FuzzPayload:
```

**Fields:**
```python
def __init__(self)
def value(self)
def description(self)
def __str__(self)
```


#### `FuzzPayload.__init__`  <sub>(function)</sub>

```python
def FuzzPayload.__init__(self):
```

#### `FuzzPayload.value`  <sub>(function)</sub>

```python
def FuzzPayload.value(self):
```

#### `FuzzPayload.description`  <sub>(function)</sub>

```python
def FuzzPayload.description(self):
```

#### `FuzzPayload.__str__`  <sub>(function)</sub>

```python
def FuzzPayload.__str__(self):
```

#### `FPayloadManager`  <sub>(class)</sub>

```python
class FPayloadManager:
```

**Fields:**
```python
def __init__(self)
def add(self, payload_dict, fuzzword, is_baseline)
def update_from_dictio(self, dictio_item)
def get_fuzz_words(self)
def get_payload(self, index)
def get_payload_type(self, index)
def get_payload_content(self, index)
def get_payloads(self)
def description(self)
def __str__(self)
```


#### `FPayloadManager.__init__`  <sub>(function)</sub>

```python
def FPayloadManager.__init__(self):
```

#### `FPayloadManager.add`  <sub>(function)</sub>

```python
def FPayloadManager.add(self, payload_dict, fuzzword=None, is_baseline=False):
```

#### `FPayloadManager.update_from_dictio`  <sub>(function)</sub>

```python
def FPayloadManager.update_from_dictio(self, dictio_item):
```

#### `FPayloadManager.get_fuzz_words`  <sub>(function)</sub>

```python
def FPayloadManager.get_fuzz_words(self):
```

#### `FPayloadManager.get_payload`  <sub>(function)</sub>

```python
def FPayloadManager.get_payload(self, index):
```

#### `FPayloadManager.get_payload_type`  <sub>(function)</sub>

```python
def FPayloadManager.get_payload_type(self, index):
```

#### `FPayloadManager.get_payload_content`  <sub>(function)</sub>

```python
def FPayloadManager.get_payload_content(self, index):
```

#### `FPayloadManager.get_payloads`  <sub>(function)</sub>

```python
def FPayloadManager.get_payloads(self):
```

#### `FPayloadManager.description`  <sub>(function)</sub>

```python
def FPayloadManager.description(self):
```

#### `FPayloadManager.__str__`  <sub>(function)</sub>

```python
def FPayloadManager.__str__(self):
```

#### `FuzzError`  <sub>(class)</sub>

```python
class FuzzError(FuzzItem):
```

**Fields:**
```python
def __init__(self, exception)
```


#### `FuzzError.__init__`  <sub>(function)</sub>

```python
def FuzzError.__init__(self, exception):
```

#### `FuzzResult`  <sub>(class)</sub>

```python
class FuzzResult(FuzzItem):
```

**Fields:**
```python
newid = ...
FUZZRESULT_SHARED_FILTER = ...
def __init__(self, history, exception, track_id)
def plugins(self)
def update(self, exception)
def __str__(self)
def description(self)
def eval(self, expr)
def _field(self, separator)
def content(self)
def url(self)
def code(self)
def timer(self)
def update_from_options(self, options)
```


#### `FuzzResult.__init__`  <sub>(function)</sub>

```python
def FuzzResult.__init__(self, history=None, exception=None, track_id=True):
```

#### `FuzzResult.plugins`  <sub>(function)</sub>

```python
def FuzzResult.plugins(self):
```

#### `FuzzResult.update`  <sub>(function)</sub>

```python
def FuzzResult.update(self, exception=None):
```

#### `FuzzResult.__str__`  <sub>(function)</sub>

```python
def FuzzResult.__str__(self):
```

#### `FuzzResult.description`  <sub>(function)</sub>

```python
def FuzzResult.description(self):
```

#### `FuzzResult.eval`  <sub>(function)</sub>

```python
def FuzzResult.eval(self, expr):
```

#### `FuzzResult._field`  <sub>(function)</sub>

```python
def FuzzResult._field(self, separator=", "):
```

#### `FuzzResult.content`  <sub>(function)</sub>

```python
def FuzzResult.content(self):
```

#### `FuzzResult.url`  <sub>(function)</sub>

```python
def FuzzResult.url(self):
```

#### `FuzzResult.code`  <sub>(function)</sub>

```python
def FuzzResult.code(self):
```

#### `FuzzResult.timer`  <sub>(function)</sub>

```python
def FuzzResult.timer(self):
```

#### `FuzzResult.update_from_options`  <sub>(function)</sub>

```python
def FuzzResult.update_from_options(self, options):
```

#### `FuzzPlugin`  <sub>(class)</sub>

```python
class FuzzPlugin(FuzzItem):
```

**Fields:**
```python
OUTPUT_SOURCE = ...
SUMMARY_ITYPE = ...
MIN_VERBOSE = ...
def __init__(self)
def is_visible(self, verbose)
```


#### `FuzzPlugin.__init__`  <sub>(function)</sub>

```python
def FuzzPlugin.__init__(self):
```

#### `FuzzPlugin.is_visible`  <sub>(function)</sub>

```python
def FuzzPlugin.is_visible(self, verbose):
```

### `wfuzz/src/wfuzz/fuzzqueues.py`


#### `AllVarQ`  <sub>(class)</sub>

```python
class AllVarQ(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options)
def get_name(self)
def cancel(self)
def items_to_process(self)
def process(self, item)
```


#### `AllVarQ.__init__`  <sub>(function)</sub>

```python
def AllVarQ.__init__(self, options):
```

#### `AllVarQ.get_name`  <sub>(function)</sub>

```python
def AllVarQ.get_name(self):
```

#### `AllVarQ.cancel`  <sub>(function)</sub>

```python
def AllVarQ.cancel(self):
```

#### `AllVarQ.items_to_process`  <sub>(function)</sub>

```python
def AllVarQ.items_to_process(self):
```

#### `AllVarQ.process`  <sub>(function)</sub>

```python
def AllVarQ.process(self, item):
```

#### `SeedQ`  <sub>(class)</sub>

```python
class SeedQ(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options)
def get_name(self)
def cancel(self)
def items_to_process(self)
def send_baseline(self)
def restart(self, seed)
def process(self, item)
def get_fuzz_res(self, dictio_item)
def send_dictionary(self)
```


#### `SeedQ.__init__`  <sub>(function)</sub>

```python
def SeedQ.__init__(self, options):
```

#### `SeedQ.get_name`  <sub>(function)</sub>

```python
def SeedQ.get_name(self):
```

#### `SeedQ.cancel`  <sub>(function)</sub>

```python
def SeedQ.cancel(self):
```

#### `SeedQ.items_to_process`  <sub>(function)</sub>

```python
def SeedQ.items_to_process(self):
```

#### `SeedQ.send_baseline`  <sub>(function)</sub>

```python
def SeedQ.send_baseline(self):
```

#### `SeedQ.restart`  <sub>(function)</sub>

```python
def SeedQ.restart(self, seed):
```

#### `SeedQ.process`  <sub>(function)</sub>

```python
def SeedQ.process(self, item):
```

#### `SeedQ.get_fuzz_res`  <sub>(function)</sub>

```python
def SeedQ.get_fuzz_res(self, dictio_item):
```

#### `SeedQ.send_dictionary`  <sub>(function)</sub>

```python
def SeedQ.send_dictionary(self):
```

#### `SaveQ`  <sub>(class)</sub>

```python
class SaveQ(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options)
def get_name(self)
def _cleanup(self)
def process(self, item)
```


#### `SaveQ.__init__`  <sub>(function)</sub>

```python
def SaveQ.__init__(self, options):
```

#### `SaveQ.get_name`  <sub>(function)</sub>

```python
def SaveQ.get_name(self):
```

#### `SaveQ._cleanup`  <sub>(function)</sub>

```python
def SaveQ._cleanup(self):
```

#### `SaveQ.process`  <sub>(function)</sub>

```python
def SaveQ.process(self, item):
```

#### `ConsolePrinterQ`  <sub>(class)</sub>

```python
class ConsolePrinterQ(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options)
def mystart(self)
def get_name(self)
def _cleanup(self)
def process(self, item)
```


#### `ConsolePrinterQ.__init__`  <sub>(function)</sub>

```python
def ConsolePrinterQ.__init__(self, options):
```

#### `ConsolePrinterQ.mystart`  <sub>(function)</sub>

```python
def ConsolePrinterQ.mystart(self):
```

#### `ConsolePrinterQ.get_name`  <sub>(function)</sub>

```python
def ConsolePrinterQ.get_name(self):
```

#### `ConsolePrinterQ._cleanup`  <sub>(function)</sub>

```python
def ConsolePrinterQ._cleanup(self):
```

#### `ConsolePrinterQ.process`  <sub>(function)</sub>

```python
def ConsolePrinterQ.process(self, item):
```

#### `CLIPrinterQ`  <sub>(class)</sub>

```python
class CLIPrinterQ(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options)
def mystart(self)
def process_discarded(self)
def get_name(self)
def _cleanup(self)
def process(self, item)
```


#### `CLIPrinterQ.__init__`  <sub>(function)</sub>

```python
def CLIPrinterQ.__init__(self, options):
```

#### `CLIPrinterQ.mystart`  <sub>(function)</sub>

```python
def CLIPrinterQ.mystart(self):
```

#### `CLIPrinterQ.process_discarded`  <sub>(function)</sub>

```python
def CLIPrinterQ.process_discarded(self):
```

#### `CLIPrinterQ.get_name`  <sub>(function)</sub>

```python
def CLIPrinterQ.get_name(self):
```

#### `CLIPrinterQ._cleanup`  <sub>(function)</sub>

```python
def CLIPrinterQ._cleanup(self):
```

#### `CLIPrinterQ.process`  <sub>(function)</sub>

```python
def CLIPrinterQ.process(self, item):
```

#### `PrinterQ`  <sub>(class)</sub>

```python
class PrinterQ(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options)
def get_name(self)
def _cleanup(self)
def process(self, item)
```


#### `PrinterQ.__init__`  <sub>(function)</sub>

```python
def PrinterQ.__init__(self, options):
```

#### `PrinterQ.get_name`  <sub>(function)</sub>

```python
def PrinterQ.get_name(self):
```

#### `PrinterQ._cleanup`  <sub>(function)</sub>

```python
def PrinterQ._cleanup(self):
```

#### `PrinterQ.process`  <sub>(function)</sub>

```python
def PrinterQ.process(self, item):
```

#### `RoutingQ`  <sub>(class)</sub>

```python
class RoutingQ(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options, routes)
def get_name(self)
def items_to_process(self)
def process(self, item)
```


#### `RoutingQ.__init__`  <sub>(function)</sub>

```python
def RoutingQ.__init__(self, options, routes):
```

#### `RoutingQ.get_name`  <sub>(function)</sub>

```python
def RoutingQ.get_name(self):
```

#### `RoutingQ.items_to_process`  <sub>(function)</sub>

```python
def RoutingQ.items_to_process(self):
```

#### `RoutingQ.process`  <sub>(function)</sub>

```python
def RoutingQ.process(self, item):
```

#### `FilterQ`  <sub>(class)</sub>

```python
class FilterQ(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options, ffilter)
def get_name(self)
def process(self, item)
```


#### `FilterQ.__init__`  <sub>(function)</sub>

```python
def FilterQ.__init__(self, options, ffilter):
```

#### `FilterQ.get_name`  <sub>(function)</sub>

```python
def FilterQ.get_name(self):
```

#### `FilterQ.process`  <sub>(function)</sub>

```python
def FilterQ.process(self, item):
```

#### `SliceQ`  <sub>(class)</sub>

```python
class SliceQ(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options, prefilter)
def get_name(self)
def process(self, item)
```


#### `SliceQ.__init__`  <sub>(function)</sub>

```python
def SliceQ.__init__(self, options, prefilter):
```

#### `SliceQ.get_name`  <sub>(function)</sub>

```python
def SliceQ.get_name(self):
```

#### `SliceQ.process`  <sub>(function)</sub>

```python
def SliceQ.process(self, item):
```

#### `JobQ`  <sub>(class)</sub>

```python
class JobQ(FuzzRRQueue):
```

**Fields:**
```python
def __init__(self, options)
def get_name(self)
def process(self, item)
```


#### `JobQ.__init__`  <sub>(function)</sub>

```python
def JobQ.__init__(self, options):
```

#### `JobQ.get_name`  <sub>(function)</sub>

```python
def JobQ.get_name(self):
```

#### `JobQ.process`  <sub>(function)</sub>

```python
def JobQ.process(self, item):
```

#### `JobMan`  <sub>(class)</sub>

```python
class JobMan(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options, selected_plugins)
def get_name(self)
def process(self, res)
def process_results(self, res, plugins_res_queue)
```


#### `JobMan.__init__`  <sub>(function)</sub>

```python
def JobMan.__init__(self, options, selected_plugins):
```

#### `JobMan.get_name`  <sub>(function)</sub>

```python
def JobMan.get_name(self):
```

#### `JobMan.process`  <sub>(function)</sub>

```python
def JobMan.process(self, res):
```

#### `JobMan.process_results`  <sub>(function)</sub>

```python
def JobMan.process_results(self, res, plugins_res_queue):
```

#### `RecursiveQ`  <sub>(class)</sub>

```python
class RecursiveQ(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options)
def get_name(self)
def process(self, fuzz_res)
```


#### `RecursiveQ.__init__`  <sub>(function)</sub>

```python
def RecursiveQ.__init__(self, options):
```

#### `RecursiveQ.get_name`  <sub>(function)</sub>

```python
def RecursiveQ.get_name(self):
```

#### `RecursiveQ.process`  <sub>(function)</sub>

```python
def RecursiveQ.process(self, fuzz_res):
```

#### `PassPayloadQ`  <sub>(class)</sub>

```python
class PassPayloadQ(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options)
def get_name(self)
def process(self, item)
```


#### `PassPayloadQ.__init__`  <sub>(function)</sub>

```python
def PassPayloadQ.__init__(self, options):
```

#### `PassPayloadQ.get_name`  <sub>(function)</sub>

```python
def PassPayloadQ.get_name(self):
```

#### `PassPayloadQ.process`  <sub>(function)</sub>

```python
def PassPayloadQ.process(self, item):
```

#### `DryRunQ`  <sub>(class)</sub>

```python
class DryRunQ(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options)
def get_name(self)
def process(self, item)
```


#### `DryRunQ.__init__`  <sub>(function)</sub>

```python
def DryRunQ.__init__(self, options):
```

#### `DryRunQ.get_name`  <sub>(function)</sub>

```python
def DryRunQ.get_name(self):
```

#### `DryRunQ.process`  <sub>(function)</sub>

```python
def DryRunQ.process(self, item):
```

#### `HttpQueue`  <sub>(class)</sub>

```python
class HttpQueue(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options)
def cancel(self)
def mystart(self)
def get_name(self)
def _cleanup(self)
def items_to_process(self)
def process(self, obj)
def __read_http_results(self)
```


#### `HttpQueue.__init__`  <sub>(function)</sub>

```python
def HttpQueue.__init__(self, options):
```

#### `HttpQueue.cancel`  <sub>(function)</sub>

```python
def HttpQueue.cancel(self):
```

#### `HttpQueue.mystart`  <sub>(function)</sub>

```python
def HttpQueue.mystart(self):
```

#### `HttpQueue.get_name`  <sub>(function)</sub>

```python
def HttpQueue.get_name(self):
```

#### `HttpQueue._cleanup`  <sub>(function)</sub>

```python
def HttpQueue._cleanup(self):
```

#### `HttpQueue.items_to_process`  <sub>(function)</sub>

```python
def HttpQueue.items_to_process(self):
```

#### `HttpQueue.process`  <sub>(function)</sub>

```python
def HttpQueue.process(self, obj):
```

#### `HttpQueue.__read_http_results`  <sub>(function)</sub>

```python
def HttpQueue.__read_http_results(self):
```

#### `HttpReceiver`  <sub>(class)</sub>

```python
class HttpReceiver(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options)
def get_name(self)
def process(self, res)
```


#### `HttpReceiver.__init__`  <sub>(function)</sub>

```python
def HttpReceiver.__init__(self, options):
```

#### `HttpReceiver.get_name`  <sub>(function)</sub>

```python
def HttpReceiver.get_name(self):
```

#### `HttpReceiver.process`  <sub>(function)</sub>

```python
def HttpReceiver.process(self, res):
```

### `wfuzz/src/wfuzz/fuzzrequest.py`


#### `headers`  <sub>(class)</sub>

```python
class headers(object):
```

**Fields:**
```python
def __init__(self, req)
def response(self)
def request(self)
def request(self, values_dict)
def all(self)
```


#### `headers.__init__`  <sub>(function)</sub>

```python
def headers.__init__(self, req):
```

#### `headers.response`  <sub>(function)</sub>

```python
def headers.response(self):
```

#### `headers.request`  <sub>(function)</sub>

```python
def headers.request(self):
```

#### `headers.request`  <sub>(function)</sub>

```python
def headers.request(self, values_dict):
```

#### `headers.all`  <sub>(function)</sub>

```python
def headers.all(self):
```

#### `cookies`  <sub>(class)</sub>

```python
class cookies(object):
```

**Fields:**
```python
def __init__(self, req)
def response(self)
def request(self)
def request(self, values)
def all(self)
```


#### `cookies.__init__`  <sub>(function)</sub>

```python
def cookies.__init__(self, req):
```

#### `cookies.response`  <sub>(function)</sub>

```python
def cookies.response(self):
```

#### `cookies.request`  <sub>(function)</sub>

```python
def cookies.request(self):
```

#### `cookies.request`  <sub>(function)</sub>

```python
def cookies.request(self, values):
```

#### `cookies.all`  <sub>(function)</sub>

```python
def cookies.all(self):
```

#### `params`  <sub>(class)</sub>

```python
class params(object):
```

**Fields:**
```python
def __init__(self, req)
def get(self)
def get(self, values)
def post(self)
def post(self, pp)
def raw_post(self)
def all(self)
def all(self, values)
```


#### `params.__init__`  <sub>(function)</sub>

```python
def params.__init__(self, req):
```

#### `params.get`  <sub>(function)</sub>

```python
def params.get(self):
```

#### `params.get`  <sub>(function)</sub>

```python
def params.get(self, values):
```

#### `params.post`  <sub>(function)</sub>

```python
def params.post(self):
```

#### `params.post`  <sub>(function)</sub>

```python
def params.post(self, pp):
```

#### `params.raw_post`  <sub>(function)</sub>

```python
def params.raw_post(self):
```

#### `params.all`  <sub>(function)</sub>

```python
def params.all(self):
```

#### `params.all`  <sub>(function)</sub>

```python
def params.all(self, values):
```

#### `FuzzRequest`  <sub>(class)</sub>

```python
class FuzzRequest(FuzzRequestUrlMixing, FuzzRequestSoupMixing):
```

**Fields:**
```python
def __init__(self)
def __str__(self)
def raw_request(self)
def raw_request(self, rawReq, scheme)
def raw_content(self)
def headers(self)
def params(self)
def cookies(self)
def method(self)
def method(self, method)
def scheme(self)
def scheme(self, s)
def host(self)
def path(self)
def redirect_url(self)
def url(self)
def url(self, u)
def content(self)
def code(self)
def code(self, c)
def auth(self)
def auth(self, creds_dict)
def follow(self)
def follow(self, f)
def reqtime(self)
def reqtime(self, t)
def wf_allvars_set(self)
def wf_allvars_set(self, varset)
def wf_allvars(self)
def wf_allvars(self, bl)
def wf_proxy(self)
def wf_proxy(self, proxy_tuple)
def update_from_raw_http(self, raw, scheme, raw_response, raw_content)
def to_cache_key(self)
def update_from_options(self, options)
```


#### `FuzzRequest.__init__`  <sub>(function)</sub>

```python
def FuzzRequest.__init__(self):
```

#### `FuzzRequest.__str__`  <sub>(function)</sub>

```python
def FuzzRequest.__str__(self):
```

#### `FuzzRequest.raw_request`  <sub>(function)</sub>

```python
def FuzzRequest.raw_request(self):
```

#### `FuzzRequest.raw_request`  <sub>(function)</sub>

```python
def FuzzRequest.raw_request(self, rawReq, scheme):
```

#### `FuzzRequest.raw_content`  <sub>(function)</sub>

```python
def FuzzRequest.raw_content(self):
```

#### `FuzzRequest.headers`  <sub>(function)</sub>

```python
def FuzzRequest.headers(self):
```

#### `FuzzRequest.params`  <sub>(function)</sub>

```python
def FuzzRequest.params(self):
```

#### `FuzzRequest.cookies`  <sub>(function)</sub>

```python
def FuzzRequest.cookies(self):
```

#### `FuzzRequest.method`  <sub>(function)</sub>

```python
def FuzzRequest.method(self):
```

#### `FuzzRequest.method`  <sub>(function)</sub>

```python
def FuzzRequest.method(self, method):
```

#### `FuzzRequest.scheme`  <sub>(function)</sub>

```python
def FuzzRequest.scheme(self):
```

#### `FuzzRequest.scheme`  <sub>(function)</sub>

```python
def FuzzRequest.scheme(self, s):
```

#### `FuzzRequest.host`  <sub>(function)</sub>

```python
def FuzzRequest.host(self):
```

#### `FuzzRequest.path`  <sub>(function)</sub>

```python
def FuzzRequest.path(self):
```

#### `FuzzRequest.redirect_url`  <sub>(function)</sub>

```python
def FuzzRequest.redirect_url(self):
```

#### `FuzzRequest.url`  <sub>(function)</sub>

```python
def FuzzRequest.url(self):
```

#### `FuzzRequest.url`  <sub>(function)</sub>

```python
def FuzzRequest.url(self, u):
```

#### `FuzzRequest.content`  <sub>(function)</sub>

```python
def FuzzRequest.content(self):
```

#### `FuzzRequest.code`  <sub>(function)</sub>

```python
def FuzzRequest.code(self):
```

#### `FuzzRequest.code`  <sub>(function)</sub>

```python
def FuzzRequest.code(self, c):
```

#### `FuzzRequest.auth`  <sub>(function)</sub>

```python
def FuzzRequest.auth(self):
```

#### `FuzzRequest.auth`  <sub>(function)</sub>

```python
def FuzzRequest.auth(self, creds_dict):
```

#### `FuzzRequest.follow`  <sub>(function)</sub>

```python
def FuzzRequest.follow(self):
```

#### `FuzzRequest.follow`  <sub>(function)</sub>

```python
def FuzzRequest.follow(self, f):
```

#### `FuzzRequest.reqtime`  <sub>(function)</sub>

```python
def FuzzRequest.reqtime(self):
```

#### `FuzzRequest.reqtime`  <sub>(function)</sub>

```python
def FuzzRequest.reqtime(self, t):
```

#### `FuzzRequest.wf_allvars_set`  <sub>(function)</sub>

```python
def FuzzRequest.wf_allvars_set(self):
```

#### `FuzzRequest.wf_allvars_set`  <sub>(function)</sub>

```python
def FuzzRequest.wf_allvars_set(self, varset):
```

#### `FuzzRequest.wf_allvars`  <sub>(function)</sub>

```python
def FuzzRequest.wf_allvars(self):
```

#### `FuzzRequest.wf_allvars`  <sub>(function)</sub>

```python
def FuzzRequest.wf_allvars(self, bl):
```

#### `FuzzRequest.wf_proxy`  <sub>(function)</sub>

```python
def FuzzRequest.wf_proxy(self):
```

#### `FuzzRequest.wf_proxy`  <sub>(function)</sub>

```python
def FuzzRequest.wf_proxy(self, proxy_tuple):
```

#### `FuzzRequest.update_from_raw_http`  <sub>(function)</sub>

```python
def FuzzRequest.update_from_raw_http(self, raw, scheme, raw_response=None, raw_content=None):
```

#### `FuzzRequest.to_cache_key`  <sub>(function)</sub>

```python
def FuzzRequest.to_cache_key(self):
```

#### `FuzzRequest.update_from_options`  <sub>(function)</sub>

```python
def FuzzRequest.update_from_options(self, options):
```

### `wfuzz/src/wfuzz/helpers/file_func.py`


#### `get_filter_help_file`  <sub>(function)</sub>

```python
def get_filter_help_file():
```

#### `create_dir`  <sub>(function)</sub>

```python
def create_dir(dir_path):
```

#### `get_home`  <sub>(function)</sub>

```python
def get_home(check=False, directory=None):
```

#### `get_config_dir`  <sub>(function)</sub>

```python
def get_config_dir(check=False):
```

#### `get_path`  <sub>(function)</sub>

```python
def get_path(directory=None):
```

#### `find_file_in_paths`  <sub>(function)</sub>

```python
def find_file_in_paths(name, path):
```

#### `FileDetOpener`  <sub>(class)</sub>

```python
class FileDetOpener:
```

**Fields:**
```python
typical_encodings = ...
def __init__(self, file_path, encoding)
def close(self)
def reset(self)
def __iter__(self)
def __next__(self)
def detect_encoding(self)
next = ...
```


#### `FileDetOpener.__init__`  <sub>(function)</sub>

```python
def FileDetOpener.__init__(self, file_path, encoding=None):
```

#### `FileDetOpener.close`  <sub>(function)</sub>

```python
def FileDetOpener.close(self):
```

#### `FileDetOpener.reset`  <sub>(function)</sub>

```python
def FileDetOpener.reset(self):
```

#### `FileDetOpener.__iter__`  <sub>(function)</sub>

```python
def FileDetOpener.__iter__(self):
```

#### `FileDetOpener.__next__`  <sub>(function)</sub>

```python
def FileDetOpener.__next__(self):
```

#### `FileDetOpener.detect_encoding`  <sub>(function)</sub>

```python
def FileDetOpener.detect_encoding(self):
```

#### `open_file_detect_encoding`  <sub>(function)</sub>

```python
def open_file_detect_encoding(file_path):
```

### `wfuzz/src/wfuzz/helpers/obj_dic.py`


#### `CaseInsensitiveDict`  <sub>(class)</sub>

```python
class CaseInsensitiveDict(MutableMapping):
```

**Fields:**
```python
def __init__(self)
def __contains__(self, k)
def __delitem__(self, k)
def __getitem__(self, k)
def get(self, k, default)
def __setitem__(self, k, v)
def __iter__(self)
def __len__(self)
```


#### `CaseInsensitiveDict.__init__`  <sub>(function)</sub>

```python
def CaseInsensitiveDict.__init__(self, *args, **kwargs):
```

#### `CaseInsensitiveDict.__contains__`  <sub>(function)</sub>

```python
def CaseInsensitiveDict.__contains__(self, k):
```

#### `CaseInsensitiveDict.__delitem__`  <sub>(function)</sub>

```python
def CaseInsensitiveDict.__delitem__(self, k):
```

#### `CaseInsensitiveDict.__getitem__`  <sub>(function)</sub>

```python
def CaseInsensitiveDict.__getitem__(self, k):
```

#### `CaseInsensitiveDict.get`  <sub>(function)</sub>

```python
def CaseInsensitiveDict.get(self, k, default=None):
```

#### `CaseInsensitiveDict.__setitem__`  <sub>(function)</sub>

```python
def CaseInsensitiveDict.__setitem__(self, k, v):
```

#### `CaseInsensitiveDict.__iter__`  <sub>(function)</sub>

```python
def CaseInsensitiveDict.__iter__(self):
```

#### `CaseInsensitiveDict.__len__`  <sub>(function)</sub>

```python
def CaseInsensitiveDict.__len__(self):
```

#### `DotDict`  <sub>(class)</sub>

```python
class DotDict(CaseInsensitiveDict):
```

**Fields:**
```python
def __getattr__(obj, name)
def __add__(self, other)
def __radd__(self, other)
def __getitem__(self, key)
def __str__(self)
```


#### `DotDict.__getattr__`  <sub>(function)</sub>

```python
def DotDict.__getattr__(obj, name):
```

#### `DotDict.__add__`  <sub>(function)</sub>

```python
def DotDict.__add__(self, other):
```

#### `DotDict.__radd__`  <sub>(function)</sub>

```python
def DotDict.__radd__(self, other):
```

#### `DotDict.__getitem__`  <sub>(function)</sub>

```python
def DotDict.__getitem__(self, key):
```

#### `DotDict.__str__`  <sub>(function)</sub>

```python
def DotDict.__str__(self):
```

### `wfuzz/src/wfuzz/helpers/obj_dyn.py`


#### `_check_allowed_field`  <sub>(function)</sub>

```python
def _check_allowed_field(attr):
```

#### `_get_alias`  <sub>(function)</sub>

```python
def _get_alias(attr):
```

#### `rsetattr`  <sub>(function)</sub>

```python
def rsetattr(obj, attr, new_val, operation):
```

#### `rgetattr`  <sub>(function)</sub>

```python
def rgetattr(obj, attr, *args):
```

### `wfuzz/src/wfuzz/helpers/obj_factory.py`


#### `Singleton`  <sub>(class)</sub>

> Singleton metaclass. Use by defining the metaclass of a class Singleton,  
> e.g.: class ThereCanBeOnlyOne:  
> __metaclass__ = Singleton

```python
class Singleton(type):
```

**Fields:**
```python
def __call__(class_)
def deleteInstance(class_)
def hasInstance(class_)
```


#### `Singleton.__call__`  <sub>(function)</sub>

```python
def Singleton.__call__(class_, *args, **kwargs):
```

#### `Singleton.deleteInstance`  <sub>(function)</sub>

> Delete the (only) instance. This method is mainly for unittests so  
> they can start with a clean slate.

```python
def Singleton.deleteInstance(class_):
```

#### `Singleton.hasInstance`  <sub>(function)</sub>

> Has the (only) instance been created already?

```python
def Singleton.hasInstance(class_):
```

#### `ObjectFactory`  <sub>(class)</sub>

```python
class ObjectFactory:
```

**Fields:**
```python
def __init__(self, builders)
def create(self, key)
```


#### `ObjectFactory.__init__`  <sub>(function)</sub>

```python
def ObjectFactory.__init__(self, builders):
```

#### `ObjectFactory.create`  <sub>(function)</sub>

```python
def ObjectFactory.create(self, key, *args, **kwargs):
```

#### `HttpRequestFactory`  <sub>(class)</sub>

```python
class HttpRequestFactory(abc.ABC):
```

**Fields:**
```python
def to_http_object(options, to_http, from_req)
def from_http_object(options, from_http, raw_header, raw_body)
```


#### `HttpRequestFactory.to_http_object`  <sub>(function)</sub>

```python
def HttpRequestFactory.to_http_object(options, to_http, from_req):
```

#### `HttpRequestFactory.from_http_object`  <sub>(function)</sub>

```python
def HttpRequestFactory.from_http_object(options, from_http, raw_header, raw_body):
```

#### `SeedBuilderHelper`  <sub>(class)</sub>

```python
class SeedBuilderHelper:
```

**Fields:**
```python
FUZZ_MARKERS_REGEX = ...
REQ_ATTR = ...
def _get_markers(text)
def get_marker_dict(freq)
def _remove_markers(freq, markers, mark_name)
def remove_baseline_markers(freq, markers)
def remove_nonfuzz_markers(freq, markers)
def replace_markers(freq, fpm)
```


#### `SeedBuilderHelper._get_markers`  <sub>(function)</sub>

```python
def SeedBuilderHelper._get_markers(text):
```

#### `SeedBuilderHelper.get_marker_dict`  <sub>(function)</sub>

```python
def SeedBuilderHelper.get_marker_dict(freq):
```

#### `SeedBuilderHelper._remove_markers`  <sub>(function)</sub>

```python
def SeedBuilderHelper._remove_markers(freq, markers, mark_name):
```

#### `SeedBuilderHelper.remove_baseline_markers`  <sub>(function)</sub>

```python
def SeedBuilderHelper.remove_baseline_markers(freq, markers):
```

#### `SeedBuilderHelper.remove_nonfuzz_markers`  <sub>(function)</sub>

```python
def SeedBuilderHelper.remove_nonfuzz_markers(freq, markers):
```

#### `SeedBuilderHelper.replace_markers`  <sub>(function)</sub>

```python
def SeedBuilderHelper.replace_markers(freq, fpm):
```

### `wfuzz/src/wfuzz/helpers/str_func.py`


#### `json_minify`  <sub>(function)</sub>

> Created on 20/01/2011  
> v0.2 (C) Gerald Storer  
> MIT License  
> Based on JSON.minify.js:  
> https://github.com/getify/JSON.minify  
> Contributers:  
> - Pradyun S. Gedam (conditions and variable names changed)

```python
def json_minify(string, strip_space=True):
```

#### `python2_3_convert_from_unicode`  <sub>(function)</sub>

```python
def python2_3_convert_from_unicode(text):
```

#### `python2_3_convert_to_unicode`  <sub>(function)</sub>

```python
def python2_3_convert_to_unicode(text):
```

#### `convert_to_unicode`  <sub>(function)</sub>

```python
def convert_to_unicode(text):
```

#### `value_in_any_list_item`  <sub>(function)</sub>

```python
def value_in_any_list_item(value, list_obj):
```

### `wfuzz/src/wfuzz/helpers/utils.py`


#### `MyCounter`  <sub>(class)</sub>

```python
class MyCounter:
```

**Fields:**
```python
def __init__(self, count)
def inc(self)
def dec(self)
def _operation(self, dec)
def __call__(self)
```


#### `MyCounter.__init__`  <sub>(function)</sub>

```python
def MyCounter.__init__(self, count=0):
```

#### `MyCounter.inc`  <sub>(function)</sub>

```python
def MyCounter.inc(self):
```

#### `MyCounter.dec`  <sub>(function)</sub>

```python
def MyCounter.dec(self):
```

#### `MyCounter._operation`  <sub>(function)</sub>

```python
def MyCounter._operation(self, dec):
```

#### `MyCounter.__call__`  <sub>(function)</sub>

```python
def MyCounter.__call__(self):
```

#### `diff`  <sub>(function)</sub>

```python
def diff(param1, param2):
```

### `wfuzz/src/wfuzz/mixins.py`


#### `FuzzRequestSoupMixing`  <sub>(class)</sub>

```python
class FuzzRequestSoupMixing(object):
```

**Fields:**
```python
def get_soup(self)
```


#### `FuzzRequestSoupMixing.get_soup`  <sub>(function)</sub>

```python
def FuzzRequestSoupMixing.get_soup(self):
```

#### `FuzzRequestUrlMixing`  <sub>(class)</sub>

```python
class FuzzRequestUrlMixing(object):
```

**Fields:**
```python
def urlparse(self)
def urlp(self)
def pstrip(self)
def is_path(self)
def recursive_url(self)
```


#### `FuzzRequestUrlMixing.urlparse`  <sub>(function)</sub>

```python
def FuzzRequestUrlMixing.urlparse(self):
```

#### `FuzzRequestUrlMixing.urlp`  <sub>(function)</sub>

```python
def FuzzRequestUrlMixing.urlp(self):
```

#### `FuzzRequestUrlMixing.pstrip`  <sub>(function)</sub>

```python
def FuzzRequestUrlMixing.pstrip(self):
```

#### `FuzzRequestUrlMixing.is_path`  <sub>(function)</sub>

```python
def FuzzRequestUrlMixing.is_path(self):
```

#### `FuzzRequestUrlMixing.recursive_url`  <sub>(function)</sub>

```python
def FuzzRequestUrlMixing.recursive_url(self):
```

### `wfuzz/src/wfuzz/myhttp.py`


#### `UNRECOVERABLE_PYCURL_EXCEPTIONS`  <sub>(const)</sub>

```python
UNRECOVERABLE_PYCURL_EXCEPTIONS = ...
```

#### `HttpPool`  <sub>(class)</sub>

```python
class HttpPool:
```

**Fields:**
```python
newid = ...
def __init__(self, options)
def _initialize(self)
def job_stats(self)
def iter_results(self, poolid)
def _new_pool(self)
def _prepare_curl_h(self, curl_h, fuzzres, poolid)
def enqueue(self, fuzzres, poolid)
def _stop_to_pools(self)
def cleanup(self)
def register(self)
def deregister(self)
def _get_next_proxy(self, proxy_list)
def _set_extra_options(self, c, fuzzres, poolid)
def _process_curl_handle(self, curl_h)
def _process_curl_should_retry(self, res, errno, poolid)
def _process_curl_handle_error(self, res, errno, errmsg, poolid)
def _read_multi_stack(self)
```


#### `HttpPool.__init__`  <sub>(function)</sub>

```python
def HttpPool.__init__(self, options):
```

#### `HttpPool._initialize`  <sub>(function)</sub>

```python
def HttpPool._initialize(self):
```

#### `HttpPool.job_stats`  <sub>(function)</sub>

```python
def HttpPool.job_stats(self):
```

#### `HttpPool.iter_results`  <sub>(function)</sub>

```python
def HttpPool.iter_results(self, poolid):
```

#### `HttpPool._new_pool`  <sub>(function)</sub>

```python
def HttpPool._new_pool(self):
```

#### `HttpPool._prepare_curl_h`  <sub>(function)</sub>

```python
def HttpPool._prepare_curl_h(self, curl_h, fuzzres, poolid):
```

#### `HttpPool.enqueue`  <sub>(function)</sub>

```python
def HttpPool.enqueue(self, fuzzres, poolid):
```

#### `HttpPool._stop_to_pools`  <sub>(function)</sub>

```python
def HttpPool._stop_to_pools(self):
```

#### `HttpPool.cleanup`  <sub>(function)</sub>

```python
def HttpPool.cleanup(self):
```

#### `HttpPool.register`  <sub>(function)</sub>

```python
def HttpPool.register(self):
```

#### `HttpPool.deregister`  <sub>(function)</sub>

```python
def HttpPool.deregister(self):
```

#### `HttpPool._get_next_proxy`  <sub>(function)</sub>

```python
def HttpPool._get_next_proxy(self, proxy_list):
```

#### `HttpPool._set_extra_options`  <sub>(function)</sub>

```python
def HttpPool._set_extra_options(self, c, fuzzres, poolid):
```

#### `HttpPool._process_curl_handle`  <sub>(function)</sub>

```python
def HttpPool._process_curl_handle(self, curl_h):
```

#### `HttpPool._process_curl_should_retry`  <sub>(function)</sub>

```python
def HttpPool._process_curl_should_retry(self, res, errno, poolid):
```

#### `HttpPool._process_curl_handle_error`  <sub>(function)</sub>

```python
def HttpPool._process_curl_handle_error(self, res, errno, errmsg, poolid):
```

#### `HttpPool._read_multi_stack`  <sub>(function)</sub>

```python
def HttpPool._read_multi_stack(self):
```

### `wfuzz/src/wfuzz/myqueues.py`


#### `MyPriorityQueue`  <sub>(class)</sub>

```python
class MyPriorityQueue(PriorityQueue):
```

**Fields:**
```python
def __init__(self, limit)
def _put_priority(self, prio, item, wait)
def put(self, item, wait)
def put_first(self, item, wait)
def put_last(self, item, wait)
def get(self)
```


#### `MyPriorityQueue.__init__`  <sub>(function)</sub>

```python
def MyPriorityQueue.__init__(self, limit=0):
```

#### `MyPriorityQueue._put_priority`  <sub>(function)</sub>

```python
def MyPriorityQueue._put_priority(self, prio, item, wait):
```

#### `MyPriorityQueue.put`  <sub>(function)</sub>

```python
def MyPriorityQueue.put(self, item, wait=True):
```

#### `MyPriorityQueue.put_first`  <sub>(function)</sub>

```python
def MyPriorityQueue.put_first(self, item, wait=True):
```

#### `MyPriorityQueue.put_last`  <sub>(function)</sub>

```python
def MyPriorityQueue.put_last(self, item, wait=True):
```

#### `MyPriorityQueue.get`  <sub>(function)</sub>

```python
def MyPriorityQueue.get(self):
```

#### `FuzzQueue`  <sub>(class)</sub>

```python
class FuzzQueue(MyPriorityQueue, Thread):
```

**Fields:**
```python
def __init__(self, options, queue_out, limit)
def next_queue(self, q)
def process(self, item)
def get_name(self)
def process_discarded(self)
def items_to_process(self)
def cancel(self)
def mystart(self)
def set_syncq(self, q)
def qstart(self)
def send_first(self, item)
def send_last(self, item)
def qout_join(self)
def send(self, item)
def discard(self, item)
def join(self)
def tjoin(self)
def _cleanup(self)
def _throw(self, e)
def get_stats(self)
def run(self)
```


#### `FuzzQueue.__init__`  <sub>(function)</sub>

```python
def FuzzQueue.__init__(self, options, queue_out=None, limit=0):
```

#### `FuzzQueue.next_queue`  <sub>(function)</sub>

```python
def FuzzQueue.next_queue(self, q):
```

#### `FuzzQueue.process`  <sub>(function)</sub>

```python
def FuzzQueue.process(self, item):
```

#### `FuzzQueue.get_name`  <sub>(function)</sub>

```python
def FuzzQueue.get_name(self):
```

#### `FuzzQueue.process_discarded`  <sub>(function)</sub>

```python
def FuzzQueue.process_discarded(self):
```

#### `FuzzQueue.items_to_process`  <sub>(function)</sub>

```python
def FuzzQueue.items_to_process(self):
```

#### `FuzzQueue.cancel`  <sub>(function)</sub>

```python
def FuzzQueue.cancel(self):
```

#### `FuzzQueue.mystart`  <sub>(function)</sub>

```python
def FuzzQueue.mystart(self):
```

#### `FuzzQueue.set_syncq`  <sub>(function)</sub>

```python
def FuzzQueue.set_syncq(self, q):
```

#### `FuzzQueue.qstart`  <sub>(function)</sub>

```python
def FuzzQueue.qstart(self):
```

#### `FuzzQueue.send_first`  <sub>(function)</sub>

```python
def FuzzQueue.send_first(self, item):
```

#### `FuzzQueue.send_last`  <sub>(function)</sub>

```python
def FuzzQueue.send_last(self, item):
```

#### `FuzzQueue.qout_join`  <sub>(function)</sub>

```python
def FuzzQueue.qout_join(self):
```

#### `FuzzQueue.send`  <sub>(function)</sub>

```python
def FuzzQueue.send(self, item):
```

#### `FuzzQueue.discard`  <sub>(function)</sub>

```python
def FuzzQueue.discard(self, item):
```

#### `FuzzQueue.join`  <sub>(function)</sub>

```python
def FuzzQueue.join(self):
```

#### `FuzzQueue.tjoin`  <sub>(function)</sub>

```python
def FuzzQueue.tjoin(self):
```

#### `FuzzQueue._cleanup`  <sub>(function)</sub>

```python
def FuzzQueue._cleanup(self):
```

#### `FuzzQueue._throw`  <sub>(function)</sub>

```python
def FuzzQueue._throw(self, e):
```

#### `FuzzQueue.get_stats`  <sub>(function)</sub>

```python
def FuzzQueue.get_stats(self):
```

#### `FuzzQueue.run`  <sub>(function)</sub>

```python
def FuzzQueue.run(self):
```

#### `LastFuzzQueue`  <sub>(class)</sub>

```python
class LastFuzzQueue(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options, queue_out, limit)
def get_name(self)
def process(self)
def _cleanup(self)
def _throw(self, e)
def run(self)
```


#### `LastFuzzQueue.__init__`  <sub>(function)</sub>

```python
def LastFuzzQueue.__init__(self, options, queue_out=None, limit=0):
```

#### `LastFuzzQueue.get_name`  <sub>(function)</sub>

```python
def LastFuzzQueue.get_name(self):
```

#### `LastFuzzQueue.process`  <sub>(function)</sub>

```python
def LastFuzzQueue.process(self):
```

#### `LastFuzzQueue._cleanup`  <sub>(function)</sub>

```python
def LastFuzzQueue._cleanup(self):
```

#### `LastFuzzQueue._throw`  <sub>(function)</sub>

```python
def LastFuzzQueue._throw(self, e):
```

#### `LastFuzzQueue.run`  <sub>(function)</sub>

```python
def LastFuzzQueue.run(self):
```

#### `FuzzListQueue`  <sub>(class)</sub>

```python
class FuzzListQueue(FuzzQueue):
```

**Fields:**
```python
def __init__(self, options, queues_out, limit)
def set_syncq(self, q)
def qstart(self)
def send_first(self, item)
def send_last(self, item)
def send(self, item)
def qout_join(self)
def join(self)
def next_queue(self, nextq)
def get_stats(self)
```


#### `FuzzListQueue.__init__`  <sub>(function)</sub>

```python
def FuzzListQueue.__init__(self, options, queues_out, limit=0):
```

#### `FuzzListQueue.set_syncq`  <sub>(function)</sub>

```python
def FuzzListQueue.set_syncq(self, q):
```

#### `FuzzListQueue.qstart`  <sub>(function)</sub>

```python
def FuzzListQueue.qstart(self):
```

#### `FuzzListQueue.send_first`  <sub>(function)</sub>

```python
def FuzzListQueue.send_first(self, item):
```

#### `FuzzListQueue.send_last`  <sub>(function)</sub>

```python
def FuzzListQueue.send_last(self, item):
```

#### `FuzzListQueue.send`  <sub>(function)</sub>

```python
def FuzzListQueue.send(self, item):
```

#### `FuzzListQueue.qout_join`  <sub>(function)</sub>

```python
def FuzzListQueue.qout_join(self):
```

#### `FuzzListQueue.join`  <sub>(function)</sub>

```python
def FuzzListQueue.join(self):
```

#### `FuzzListQueue.next_queue`  <sub>(function)</sub>

```python
def FuzzListQueue.next_queue(self, nextq):
```

#### `FuzzListQueue.get_stats`  <sub>(function)</sub>

```python
def FuzzListQueue.get_stats(self):
```

#### `FuzzRRQueue`  <sub>(class)</sub>

```python
class FuzzRRQueue(FuzzListQueue):
```

**Fields:**
```python
def __init__(self, options, queues_out, limit)
def send(self, item)
def _get_next_route(self)
```


#### `FuzzRRQueue.__init__`  <sub>(function)</sub>

```python
def FuzzRRQueue.__init__(self, options, queues_out, limit=0):
```

#### `FuzzRRQueue.send`  <sub>(function)</sub>

```python
def FuzzRRQueue.send(self, item):
```

#### `FuzzRRQueue._get_next_route`  <sub>(function)</sub>

```python
def FuzzRRQueue._get_next_route(self):
```

#### `QueueManager`  <sub>(class)</sub>

```python
class QueueManager:
```

**Fields:**
```python
def __init__(self, options)
def add(self, name, q)
def bind(self, lastq)
def __getitem__(self, key)
def join(self, remove)
def start(self)
def cleanup(self)
def cancel(self)
def get_stats(self)
```


#### `QueueManager.__init__`  <sub>(function)</sub>

```python
def QueueManager.__init__(self, options):
```

#### `QueueManager.add`  <sub>(function)</sub>

```python
def QueueManager.add(self, name, q):
```

#### `QueueManager.bind`  <sub>(function)</sub>

```python
def QueueManager.bind(self, lastq):
```

#### `QueueManager.__getitem__`  <sub>(function)</sub>

```python
def QueueManager.__getitem__(self, key):
```

#### `QueueManager.join`  <sub>(function)</sub>

```python
def QueueManager.join(self, remove=False):
```

#### `QueueManager.start`  <sub>(function)</sub>

```python
def QueueManager.start(self):
```

#### `QueueManager.cleanup`  <sub>(function)</sub>

```python
def QueueManager.cleanup(self):
```

#### `QueueManager.cancel`  <sub>(function)</sub>

```python
def QueueManager.cancel(self):
```

#### `QueueManager.get_stats`  <sub>(function)</sub>

```python
def QueueManager.get_stats(self):
```

### `wfuzz/src/wfuzz/options.py`


#### `FuzzSession`  <sub>(class)</sub>

```python
class FuzzSession(UserDict):
```

**Fields:**
```python
def __init__(self)
def _defaults(self)
def update(self, options)
def validate(self)
def export_to_file(self, filename)
def import_from_file(self, filename)
def import_json(self, data)
def export_json(self)
def payload(self)
def fuzz(self)
def get_payloads(self, iterator)
def get_payload(self, iterator)
def __enter__(self)
def __exit__(self)
def get_fuzz_words(self)
def compile_dictio(self)
def compile_seeds(self)
def compile(self)
def close(self)
```


#### `FuzzSession.__init__`  <sub>(function)</sub>

```python
def FuzzSession.__init__(self, **kwargs):
```

#### `FuzzSession._defaults`  <sub>(function)</sub>

```python
def FuzzSession._defaults(self):
```

#### `FuzzSession.update`  <sub>(function)</sub>

```python
def FuzzSession.update(self, options):
```

#### `FuzzSession.validate`  <sub>(function)</sub>

```python
def FuzzSession.validate(self):
```

#### `FuzzSession.export_to_file`  <sub>(function)</sub>

```python
def FuzzSession.export_to_file(self, filename):
```

#### `FuzzSession.import_from_file`  <sub>(function)</sub>

```python
def FuzzSession.import_from_file(self, filename):
```

#### `FuzzSession.import_json`  <sub>(function)</sub>

```python
def FuzzSession.import_json(self, data):
```

#### `FuzzSession.export_json`  <sub>(function)</sub>

```python
def FuzzSession.export_json(self):
```

#### `FuzzSession.payload`  <sub>(function)</sub>

```python
def FuzzSession.payload(self, **kwargs):
```

#### `FuzzSession.fuzz`  <sub>(function)</sub>

```python
def FuzzSession.fuzz(self, **kwargs):
```

#### `FuzzSession.get_payloads`  <sub>(function)</sub>

```python
def FuzzSession.get_payloads(self, iterator):
```

#### `FuzzSession.get_payload`  <sub>(function)</sub>

```python
def FuzzSession.get_payload(self, iterator):
```

#### `FuzzSession.__enter__`  <sub>(function)</sub>

```python
def FuzzSession.__enter__(self):
```

#### `FuzzSession.__exit__`  <sub>(function)</sub>

```python
def FuzzSession.__exit__(self, *args):
```

#### `FuzzSession.get_fuzz_words`  <sub>(function)</sub>

```python
def FuzzSession.get_fuzz_words(self):
```

#### `FuzzSession.compile_dictio`  <sub>(function)</sub>

```python
def FuzzSession.compile_dictio(self):
```

#### `FuzzSession.compile_seeds`  <sub>(function)</sub>

```python
def FuzzSession.compile_seeds(self):
```

#### `FuzzSession.compile`  <sub>(function)</sub>

```python
def FuzzSession.compile(self):
```

#### `FuzzSession.close`  <sub>(function)</sub>

```python
def FuzzSession.close(self):
```

### `wfuzz/src/wfuzz/plugin_api/base.py`


#### `BasePlugin`  <sub>(class)</sub>

```python
class BasePlugin:
```

**Fields:**
```python
def __init__(self)
def run(self, fuzzresult, control_queue, results_queue)
def process(self, fuzzresult)
def validate(self)
def add_result(self, itype, issue, data, severity)
def queue_url(self, url)
def _bool(self, value)
```


#### `BasePlugin.__init__`  <sub>(function)</sub>

```python
def BasePlugin.__init__(self):
```

#### `BasePlugin.run`  <sub>(function)</sub>

```python
def BasePlugin.run(self, fuzzresult, control_queue, results_queue):
```

#### `BasePlugin.process`  <sub>(function)</sub>

> This is were the plugin processing is done. Any wfuzz plugin must implement this method, do its job with the fuzzresult received and:  
> - queue_url: if it is a discovery plugin enqueing more HTTP request that at some point will generate more results  
> - add_result: Add information about the obtained results after the processing with an accurate description  
>   
> A kbase (get_kbase, has_kbase, add_kbase) is shared between all plugins. this can be used to store and retrieve relevant "collaborative" information.

```python
def BasePlugin.process(self, fuzzresult):
```

#### `BasePlugin.validate`  <sub>(function)</sub>

```python
def BasePlugin.validate(self):
```

#### `BasePlugin.add_result`  <sub>(function)</sub>

```python
def BasePlugin.add_result(self, itype, issue, data, severity=FuzzPlugin.INFO):
```

#### `BasePlugin.queue_url`  <sub>(function)</sub>

```python
def BasePlugin.queue_url(self, url):
```

#### `BasePlugin._bool`  <sub>(function)</sub>

```python
def BasePlugin._bool(self, value):
```

#### `BasePrinter`  <sub>(class)</sub>

```python
class BasePrinter:
```

**Fields:**
```python
def __init__(self, output)
def header(self)
def footer(self)
def result(self)
```


#### `BasePrinter.__init__`  <sub>(function)</sub>

```python
def BasePrinter.__init__(self, output):
```

#### `BasePrinter.header`  <sub>(function)</sub>

```python
def BasePrinter.header(self):
```

#### `BasePrinter.footer`  <sub>(function)</sub>

```python
def BasePrinter.footer(self):
```

#### `BasePrinter.result`  <sub>(function)</sub>

```python
def BasePrinter.result(self):
```

#### `BasePayload`  <sub>(class)</sub>

```python
class BasePayload(object):
```

**Fields:**
```python
def __init__(self, params)
def get_type(self)
def get_next(self)
def __next__(self)
def count(self)
def __iter__(self)
def close(self)
def find_file(self, name)
```


#### `BasePayload.__init__`  <sub>(function)</sub>

```python
def BasePayload.__init__(self, params):
```

#### `BasePayload.get_type`  <sub>(function)</sub>

```python
def BasePayload.get_type(self):
```

#### `BasePayload.get_next`  <sub>(function)</sub>

```python
def BasePayload.get_next(self):
```

#### `BasePayload.__next__`  <sub>(function)</sub>

```python
def BasePayload.__next__(self):
```

#### `BasePayload.count`  <sub>(function)</sub>

```python
def BasePayload.count(self):
```

#### `BasePayload.__iter__`  <sub>(function)</sub>

```python
def BasePayload.__iter__(self):
```

#### `BasePayload.close`  <sub>(function)</sub>

```python
def BasePayload.close(self):
```

#### `BasePayload.find_file`  <sub>(function)</sub>

```python
def BasePayload.find_file(self, name):
```

### `wfuzz/src/wfuzz/plugin_api/mixins.py`


#### `DiscoveryPluginMixin`  <sub>(class)</sub>

```python
class DiscoveryPluginMixin:
```

**Fields:**
```python
def queue_url(self, url)
```


#### `DiscoveryPluginMixin.queue_url`  <sub>(function)</sub>

```python
def DiscoveryPluginMixin.queue_url(self, url):
```

### `wfuzz/src/wfuzz/plugin_api/payloadtools.py`


#### `IMPORTED_SHODAN`  <sub>(const)</sub>

```python
IMPORTED_SHODAN = ...
```

#### `BingIter`  <sub>(class)</sub>

```python
class BingIter(object):
```

**Fields:**
```python
def __init__(self, dork, offset, limit, key)
def _do_search(self, offset, limit)
def __iter__(self)
def __next__(self)
```


#### `BingIter.__init__`  <sub>(function)</sub>

```python
def BingIter.__init__(self, dork, offset=0, limit=0, key=None):
```

#### `BingIter._do_search`  <sub>(function)</sub>

```python
def BingIter._do_search(self, offset=0, limit=50):
```

#### `BingIter.__iter__`  <sub>(function)</sub>

```python
def BingIter.__iter__(self):
```

#### `BingIter.__next__`  <sub>(function)</sub>

```python
def BingIter.__next__(self):
```

#### `ShodanIter`  <sub>(class)</sub>

```python
class ShodanIter:
```

**Fields:**
```python
SHODAN_RES_PER_PAGE = ...
MAX_ENQUEUED_RES = ...
NUM_OF_WORKERS = ...
SLOW_START = ...
def __init__(self, dork, page, limit)
def _do_search(self)
def __iter__(self)
def _start(self)
def _stop(self)
def __next__(self)
```


#### `ShodanIter.__init__`  <sub>(function)</sub>

```python
def ShodanIter.__init__(self, dork, page, limit):
```

#### `ShodanIter._do_search`  <sub>(function)</sub>

```python
def ShodanIter._do_search(self):
```

#### `ShodanIter.__iter__`  <sub>(function)</sub>

```python
def ShodanIter.__iter__(self):
```

#### `ShodanIter._start`  <sub>(function)</sub>

```python
def ShodanIter._start(self):
```

#### `ShodanIter._stop`  <sub>(function)</sub>

```python
def ShodanIter._stop(self):
```

#### `ShodanIter.__next__`  <sub>(function)</sub>

```python
def ShodanIter.__next__(self):
```

### `wfuzz/src/wfuzz/plugin_api/urlutils.py`


#### `FuzzRequestParse`  <sub>(class)</sub>

```python
class FuzzRequestParse(ParseResult):
```

**Fields:**
```python
def ffname(self)
def fext(self)
def fname(self)
def isbllist(self)
def hasquery(self)
def cache_key(self, base_urlp)
```


#### `FuzzRequestParse.ffname`  <sub>(function)</sub>

> Returns script plus extension from an URL. ie. http://www.localhost.com/kk/index.html?id=3  
> will return index.html

```python
def FuzzRequestParse.ffname(self):
```

#### `FuzzRequestParse.fext`  <sub>(function)</sub>

> Returns script extension from an URL. ie. http://www.localhost.com/kk/index.html?id=3  
> will return .html

```python
def FuzzRequestParse.fext(self):
```

#### `FuzzRequestParse.fname`  <sub>(function)</sub>

> Returns script name from an URL. ie. http://www.localhost.com/kk/index.html?id=3  
> will return index

```python
def FuzzRequestParse.fname(self):
```

#### `FuzzRequestParse.isbllist`  <sub>(function)</sub>

```python
def FuzzRequestParse.isbllist(self):
```

#### `FuzzRequestParse.hasquery`  <sub>(function)</sub>

```python
def FuzzRequestParse.hasquery(self):
```

#### `FuzzRequestParse.cache_key`  <sub>(function)</sub>

```python
def FuzzRequestParse.cache_key(self, base_urlp=None):
```

#### `parse_url`  <sub>(function)</sub>

```python
def parse_url(url):
```

#### `check_content_type`  <sub>(function)</sub>

```python
def check_content_type(fuzzresult, which):
```

### `wfuzz/src/wfuzz/plugins/encoders/encoders.py`


#### `none`  <sub>(class)</sub>

```python
class none:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
def decode(self, string)
```


#### `none.encode`  <sub>(function)</sub>

```python
def none.encode(self, string):
```

#### `none.decode`  <sub>(function)</sub>

```python
def none.decode(self, string):
```

#### `urlencode`  <sub>(class)</sub>

```python
class urlencode:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
def decode(self, string)
```


#### `urlencode.encode`  <sub>(function)</sub>

```python
def urlencode.encode(self, string):
```

#### `urlencode.decode`  <sub>(function)</sub>

```python
def urlencode.decode(self, string):
```

#### `double_urlencode`  <sub>(class)</sub>

```python
class double_urlencode:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
def decode(self, string)
```


#### `double_urlencode.encode`  <sub>(function)</sub>

```python
def double_urlencode.encode(self, string):
```

#### `double_urlencode.decode`  <sub>(function)</sub>

```python
def double_urlencode.decode(self, string):
```

#### `base64`  <sub>(class)</sub>

```python
class base64:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
def decode(self, string)
```


#### `base64.encode`  <sub>(function)</sub>

```python
def base64.encode(self, string):
```

#### `base64.decode`  <sub>(function)</sub>

```python
def base64.decode(self, string):
```

#### `uri_triple_hex`  <sub>(class)</sub>

```python
class uri_triple_hex:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `uri_triple_hex.encode`  <sub>(function)</sub>

```python
def uri_triple_hex.encode(self, string):
```

#### `uri_double_hex`  <sub>(class)</sub>

```python
class uri_double_hex:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `uri_double_hex.encode`  <sub>(function)</sub>

```python
def uri_double_hex.encode(self, string):
```

#### `uri_hex`  <sub>(class)</sub>

```python
class uri_hex:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `uri_hex.encode`  <sub>(function)</sub>

```python
def uri_hex.encode(self, string):
```

#### `random_upper`  <sub>(class)</sub>

```python
class random_upper:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `random_upper.encode`  <sub>(function)</sub>

```python
def random_upper.encode(self, string):
```

#### `second_nibble_hex`  <sub>(class)</sub>

```python
class second_nibble_hex:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `second_nibble_hex.encode`  <sub>(function)</sub>

```python
def second_nibble_hex.encode(self, string):
```

#### `first_nibble_hex`  <sub>(class)</sub>

```python
class first_nibble_hex:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `first_nibble_hex.encode`  <sub>(function)</sub>

```python
def first_nibble_hex.encode(self, string):
```

#### `doble_nibble_hex`  <sub>(class)</sub>

```python
class doble_nibble_hex:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `doble_nibble_hex.encode`  <sub>(function)</sub>

```python
def doble_nibble_hex.encode(self, string):
```

#### `sha1`  <sub>(class)</sub>

```python
class sha1:
```

**Fields:**
```python
name = ...
summary = ...
author = ...
version = ...
category = ...
priority = ...
def encode(self, string)
```


#### `sha1.encode`  <sub>(function)</sub>

```python
def sha1.encode(self, string):
```

#### `sha256`  <sub>(class)</sub>

```python
class sha256:
```

**Fields:**
```python
name = ...
summary = ...
author = ...
version = ...
category = ...
priority = ...
def encode(self, string)
```


#### `sha256.encode`  <sub>(function)</sub>

```python
def sha256.encode(self, string):
```

#### `sha512`  <sub>(class)</sub>

```python
class sha512:
```

**Fields:**
```python
name = ...
summary = ...
author = ...
version = ...
category = ...
priority = ...
def encode(self, string)
```


#### `sha512.encode`  <sub>(function)</sub>

```python
def sha512.encode(self, string):
```

#### `md5`  <sub>(class)</sub>

```python
class md5:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `md5.encode`  <sub>(function)</sub>

```python
def md5.encode(self, string):
```

#### `hexlify`  <sub>(class)</sub>

```python
class hexlify:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
def decode(self, string)
```


#### `hexlify.encode`  <sub>(function)</sub>

```python
def hexlify.encode(self, string):
```

#### `hexlify.decode`  <sub>(function)</sub>

```python
def hexlify.decode(self, string):
```

#### `html_escape`  <sub>(class)</sub>

```python
class html_escape:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `html_escape.encode`  <sub>(function)</sub>

```python
def html_escape.encode(self, string):
```

#### `html_decimal`  <sub>(class)</sub>

```python
class html_decimal:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `html_decimal.encode`  <sub>(function)</sub>

```python
def html_decimal.encode(self, string):
```

#### `html_hexadecimal`  <sub>(class)</sub>

```python
class html_hexadecimal:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `html_hexadecimal.encode`  <sub>(function)</sub>

```python
def html_hexadecimal.encode(self, string):
```

#### `utf8_binary`  <sub>(class)</sub>

```python
class utf8_binary:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `utf8_binary.encode`  <sub>(function)</sub>

```python
def utf8_binary.encode(self, string):
```

#### `utf8`  <sub>(class)</sub>

```python
class utf8:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `utf8.encode`  <sub>(function)</sub>

```python
def utf8.encode(self, string):
```

#### `uri_unicode`  <sub>(class)</sub>

```python
class uri_unicode:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
```


#### `uri_unicode.encode`  <sub>(function)</sub>

```python
def uri_unicode.encode(self, string):
```

#### `mysql_char`  <sub>(class)</sub>

```python
class mysql_char:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
def decode(self, string)
```


#### `mysql_char.encode`  <sub>(function)</sub>

```python
def mysql_char.encode(self, string):
```

#### `mysql_char.decode`  <sub>(function)</sub>

```python
def mysql_char.decode(self, string):
```

#### `mssql_char`  <sub>(class)</sub>

```python
class mssql_char:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
def decode(self, string)
```


#### `mssql_char.encode`  <sub>(function)</sub>

```python
def mssql_char.encode(self, string):
```

#### `mssql_char.decode`  <sub>(function)</sub>

```python
def mssql_char.decode(self, string):
```

#### `oracle_char`  <sub>(class)</sub>

```python
class oracle_char:
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def encode(self, string)
def decode(self, string)
```


#### `oracle_char.encode`  <sub>(function)</sub>

```python
def oracle_char.encode(self, string):
```

#### `oracle_char.decode`  <sub>(function)</sub>

```python
def oracle_char.decode(self, string):
```

### `wfuzz/src/wfuzz/plugins/iterators/iterations.py`


#### `zip`  <sub>(class)</sub>

```python
class zip(BaseIterator):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def __init__(self)
def count(self)
def width(self)
def payloads(self)
def __next__(self)
def __iter__(self)
```


#### `zip.__init__`  <sub>(function)</sub>

```python
def zip.__init__(self, *i):
```

#### `zip.count`  <sub>(function)</sub>

```python
def zip.count(self):
```

#### `zip.width`  <sub>(function)</sub>

```python
def zip.width(self):
```

#### `zip.payloads`  <sub>(function)</sub>

```python
def zip.payloads(self):
```

#### `zip.__next__`  <sub>(function)</sub>

```python
def zip.__next__(self):
```

#### `zip.__iter__`  <sub>(function)</sub>

```python
def zip.__iter__(self):
```

#### `product`  <sub>(class)</sub>

```python
class product(BaseIterator):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def __init__(self)
def count(self)
def width(self)
def payloads(self)
def __next__(self)
def __iter__(self)
```


#### `product.__init__`  <sub>(function)</sub>

```python
def product.__init__(self, *i):
```

#### `product.count`  <sub>(function)</sub>

```python
def product.count(self):
```

#### `product.width`  <sub>(function)</sub>

```python
def product.width(self):
```

#### `product.payloads`  <sub>(function)</sub>

```python
def product.payloads(self):
```

#### `product.__next__`  <sub>(function)</sub>

```python
def product.__next__(self):
```

#### `product.__iter__`  <sub>(function)</sub>

```python
def product.__iter__(self):
```

#### `chain`  <sub>(class)</sub>

```python
class chain(BaseIterator):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def __init__(self)
def count(self)
def width(self)
def payloads(self)
def __next__(self)
def __iter__(self)
```


#### `chain.__init__`  <sub>(function)</sub>

```python
def chain.__init__(self, *i):
```

#### `chain.count`  <sub>(function)</sub>

```python
def chain.count(self):
```

#### `chain.width`  <sub>(function)</sub>

```python
def chain.width(self):
```

#### `chain.payloads`  <sub>(function)</sub>

```python
def chain.payloads(self):
```

#### `chain.__next__`  <sub>(function)</sub>

```python
def chain.__next__(self):
```

#### `chain.__iter__`  <sub>(function)</sub>

```python
def chain.__iter__(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/autorize.py`


#### `autorize`  <sub>(class)</sub>

```python
class autorize(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def count(self)
def get_next(self)
def get_type(self)
def _gen_wfuzz(self, output_fn)
```


#### `autorize.__init__`  <sub>(function)</sub>

```python
def autorize.__init__(self, params):
```

#### `autorize.count`  <sub>(function)</sub>

```python
def autorize.count(self):
```

#### `autorize.get_next`  <sub>(function)</sub>

```python
def autorize.get_next(self):
```

#### `autorize.get_type`  <sub>(function)</sub>

```python
def autorize.get_type(self):
```

#### `autorize._gen_wfuzz`  <sub>(function)</sub>

```python
def autorize._gen_wfuzz(self, output_fn):
```

### `wfuzz/src/wfuzz/plugins/payloads/bing.py`


#### `bing`  <sub>(class)</sub>

```python
class bing(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def count(self)
def get_next(self)
def get_type(self)
```


#### `bing.__init__`  <sub>(function)</sub>

```python
def bing.__init__(self, params):
```

#### `bing.count`  <sub>(function)</sub>

```python
def bing.count(self):
```

#### `bing.get_next`  <sub>(function)</sub>

```python
def bing.get_next(self):
```

#### `bing.get_type`  <sub>(function)</sub>

```python
def bing.get_type(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/buffer_overflow.py`


#### `buffer_overflow`  <sub>(class)</sub>

```python
class buffer_overflow(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def count(self)
def get_next(self)
def get_type(self)
```


#### `buffer_overflow.__init__`  <sub>(function)</sub>

```python
def buffer_overflow.__init__(self, params):
```

#### `buffer_overflow.count`  <sub>(function)</sub>

```python
def buffer_overflow.count(self):
```

#### `buffer_overflow.get_next`  <sub>(function)</sub>

```python
def buffer_overflow.get_next(self):
```

#### `buffer_overflow.get_type`  <sub>(function)</sub>

```python
def buffer_overflow.get_type(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/burpitem.py`


#### `burpitem`  <sub>(class)</sub>

```python
class burpitem(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def count(self)
def get_next(self)
def get_type(self)
def _gen_burpitem(self, output_fn)
```


#### `burpitem.__init__`  <sub>(function)</sub>

```python
def burpitem.__init__(self, params):
```

#### `burpitem.count`  <sub>(function)</sub>

```python
def burpitem.count(self):
```

#### `burpitem.get_next`  <sub>(function)</sub>

```python
def burpitem.get_next(self):
```

#### `burpitem.get_type`  <sub>(function)</sub>

```python
def burpitem.get_type(self):
```

#### `burpitem._gen_burpitem`  <sub>(function)</sub>

```python
def burpitem._gen_burpitem(self, output_fn):
```

### `wfuzz/src/wfuzz/plugins/payloads/burplog.py`


#### `CRLF`  <sub>(const)</sub>

```python
CRLF = ...
```

#### `DELIMITER`  <sub>(const)</sub>

```python
DELIMITER = ...
```

#### `CRLF_DELIMITER`  <sub>(const)</sub>

```python
CRLF_DELIMITER = ...
```

#### `HEADER`  <sub>(const)</sub>

```python
HEADER = ...
```

#### `burplog`  <sub>(class)</sub>

```python
class burplog(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def count(self)
def get_type(self)
def get_next(self)
def parse_burp_log(self, burp_log)
```


#### `burplog.__init__`  <sub>(function)</sub>

```python
def burplog.__init__(self, params):
```

#### `burplog.count`  <sub>(function)</sub>

```python
def burplog.count(self):
```

#### `burplog.get_type`  <sub>(function)</sub>

```python
def burplog.get_type(self):
```

#### `burplog.get_next`  <sub>(function)</sub>

```python
def burplog.get_next(self):
```

#### `burplog.parse_burp_log`  <sub>(function)</sub>

```python
def burplog.parse_burp_log(self, burp_log):
```

### `wfuzz/src/wfuzz/plugins/payloads/burpstate.py`


#### `TAG`  <sub>(const)</sub>

```python
TAG = ...
```

#### `burpstate`  <sub>(class)</sub>

```python
class burpstate(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def __iter__(self)
def count(self)
def get_type(self)
def get_next(self)
def milliseconds_to_date(self, milliseconds)
def burp_binary_field(self, field, i)
def strip_cdata(self, data)
def burp_to_xml(self, filename)
```


#### `burpstate.__init__`  <sub>(function)</sub>

```python
def burpstate.__init__(self, params):
```

#### `burpstate.__iter__`  <sub>(function)</sub>

```python
def burpstate.__iter__(self):
```

#### `burpstate.count`  <sub>(function)</sub>

```python
def burpstate.count(self):
```

#### `burpstate.get_type`  <sub>(function)</sub>

```python
def burpstate.get_type(self):
```

#### `burpstate.get_next`  <sub>(function)</sub>

```python
def burpstate.get_next(self):
```

#### `burpstate.milliseconds_to_date`  <sub>(function)</sub>

> Convert milliseconds since Epoch (from Java) to Python date structure:  
> See: http://java.sun.com/j2se/1.4.2/docs/api/java/util/Date.html  
>   
> There is no direct way to convert milliseconds since Epoch to Python object  
> So we convert the milliseconds to seconds first as a POSIX timestamp which  
> can be used to get a valid date, and then use the parsed values from that  
> object along with converting mili -> micro seconds in a new date object.

```python
def burpstate.milliseconds_to_date(self, milliseconds):
```

#### `burpstate.burp_binary_field`  <sub>(function)</sub>

> Strip Burp Suite's binary format characters types from our data.  
> The first character after the leading tag describes the type of the data.

```python
def burpstate.burp_binary_field(self, field, i):
```

#### `burpstate.strip_cdata`  <sub>(function)</sub>

```python
def burpstate.strip_cdata(self, data):
```

#### `burpstate.burp_to_xml`  <sub>(function)</sub>

> Unzip Burp's file, remove non-printable characters, CDATA any HTML,  
> include a valid XML header and trailer, and return a valid XML string.

```python
def burpstate.burp_to_xml(self, filename):
```

### `wfuzz/src/wfuzz/plugins/payloads/dirwalk.py`


#### `dirwalk`  <sub>(class)</sub>

```python
class dirwalk(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def _my_gen(self, directory)
def get_next(self)
def get_type(self)
def count(self)
```


#### `dirwalk.__init__`  <sub>(function)</sub>

```python
def dirwalk.__init__(self, params):
```

#### `dirwalk._my_gen`  <sub>(function)</sub>

```python
def dirwalk._my_gen(self, directory):
```

#### `dirwalk.get_next`  <sub>(function)</sub>

```python
def dirwalk.get_next(self):
```

#### `dirwalk.get_type`  <sub>(function)</sub>

```python
def dirwalk.get_type(self):
```

#### `dirwalk.count`  <sub>(function)</sub>

```python
def dirwalk.count(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/file.py`


#### `file`  <sub>(class)</sub>

```python
class file(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def get_type(self)
def get_next(self)
def count(self)
```


#### `file.__init__`  <sub>(function)</sub>

```python
def file.__init__(self, params):
```

#### `file.get_type`  <sub>(function)</sub>

```python
def file.get_type(self):
```

#### `file.get_next`  <sub>(function)</sub>

```python
def file.get_next(self):
```

#### `file.count`  <sub>(function)</sub>

```python
def file.count(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/guitab.py`


#### `guitab`  <sub>(class)</sub>

```python
class guitab(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def count(self)
def get_type(self)
def get_next(self)
```


#### `guitab.__init__`  <sub>(function)</sub>

```python
def guitab.__init__(self, params):
```

#### `guitab.count`  <sub>(function)</sub>

```python
def guitab.count(self):
```

#### `guitab.get_type`  <sub>(function)</sub>

```python
def guitab.get_type(self):
```

#### `guitab.get_next`  <sub>(function)</sub>

```python
def guitab.get_next(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/hexrand.py`


#### `hexrand`  <sub>(class)</sub>

```python
class hexrand(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def __iter__(self)
def count(self)
def get_type(self)
def get_next(self)
```


#### `hexrand.__init__`  <sub>(function)</sub>

```python
def hexrand.__init__(self, params):
```

#### `hexrand.__iter__`  <sub>(function)</sub>

```python
def hexrand.__iter__(self):
```

#### `hexrand.count`  <sub>(function)</sub>

```python
def hexrand.count(self):
```

#### `hexrand.get_type`  <sub>(function)</sub>

```python
def hexrand.get_type(self):
```

#### `hexrand.get_next`  <sub>(function)</sub>

```python
def hexrand.get_next(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/hexrange.py`


#### `hexrange`  <sub>(class)</sub>

```python
class hexrange(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def count(self)
def get_type(self)
def get_next(self)
```


#### `hexrange.__init__`  <sub>(function)</sub>

```python
def hexrange.__init__(self, params):
```

#### `hexrange.count`  <sub>(function)</sub>

```python
def hexrange.count(self):
```

#### `hexrange.get_type`  <sub>(function)</sub>

```python
def hexrange.get_type(self):
```

#### `hexrange.get_next`  <sub>(function)</sub>

```python
def hexrange.get_next(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/ipnet.py`


#### `ipnet`  <sub>(class)</sub>

```python
class ipnet(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def get_type(self)
def get_next(self)
def count(self)
```


#### `ipnet.__init__`  <sub>(function)</sub>

```python
def ipnet.__init__(self, params):
```

#### `ipnet.get_type`  <sub>(function)</sub>

```python
def ipnet.get_type(self):
```

#### `ipnet.get_next`  <sub>(function)</sub>

```python
def ipnet.get_next(self):
```

#### `ipnet.count`  <sub>(function)</sub>

```python
def ipnet.count(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/iprange.py`


#### `iprange`  <sub>(class)</sub>

```python
class iprange(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def get_type(self)
def get_next(self)
def count(self)
```


#### `iprange.__init__`  <sub>(function)</sub>

```python
def iprange.__init__(self, params):
```

#### `iprange.get_type`  <sub>(function)</sub>

```python
def iprange.get_type(self):
```

#### `iprange.get_next`  <sub>(function)</sub>

```python
def iprange.get_next(self):
```

#### `iprange.count`  <sub>(function)</sub>

```python
def iprange.count(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/list.py`


#### `list`  <sub>(class)</sub>

```python
class list(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def count(self)
def get_type(self)
def get_next(self)
```


#### `list.__init__`  <sub>(function)</sub>

```python
def list.__init__(self, params):
```

#### `list.count`  <sub>(function)</sub>

```python
def list.count(self):
```

#### `list.get_type`  <sub>(function)</sub>

```python
def list.get_type(self):
```

#### `list.get_next`  <sub>(function)</sub>

```python
def list.get_next(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/names.py`


#### `names`  <sub>(class)</sub>

```python
class names(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def count(self)
def get_type(self)
def get_next(self)
```


#### `names.__init__`  <sub>(function)</sub>

```python
def names.__init__(self, params):
```

#### `names.count`  <sub>(function)</sub>

```python
def names.count(self):
```

#### `names.get_type`  <sub>(function)</sub>

```python
def names.get_type(self):
```

#### `names.get_next`  <sub>(function)</sub>

```python
def names.get_next(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/permutation.py`


#### `permutation`  <sub>(class)</sub>

```python
class permutation(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def count(self)
def get_type(self)
def get_next(self)
def xcombinations(self, items, n)
```


#### `permutation.__init__`  <sub>(function)</sub>

```python
def permutation.__init__(self, params):
```

#### `permutation.count`  <sub>(function)</sub>

```python
def permutation.count(self):
```

#### `permutation.get_type`  <sub>(function)</sub>

```python
def permutation.get_type(self):
```

#### `permutation.get_next`  <sub>(function)</sub>

```python
def permutation.get_next(self):
```

#### `permutation.xcombinations`  <sub>(function)</sub>

```python
def permutation.xcombinations(self, items, n):
```

### `wfuzz/src/wfuzz/plugins/payloads/range.py`


#### `range`  <sub>(class)</sub>

```python
class range(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def get_type(self)
def get_next(self)
def count(self)
def __iter__(self)
```


#### `range.__init__`  <sub>(function)</sub>

```python
def range.__init__(self, params):
```

#### `range.get_type`  <sub>(function)</sub>

```python
def range.get_type(self):
```

#### `range.get_next`  <sub>(function)</sub>

```python
def range.get_next(self):
```

#### `range.count`  <sub>(function)</sub>

```python
def range.count(self):
```

#### `range.__iter__`  <sub>(function)</sub>

```python
def range.__iter__(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/shodanp.py`


#### `shodanp`  <sub>(class)</sub>

```python
class shodanp(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def count(self)
def close(self)
def get_type(self)
def get_next(self)
```


#### `shodanp.__init__`  <sub>(function)</sub>

```python
def shodanp.__init__(self, params):
```

#### `shodanp.count`  <sub>(function)</sub>

```python
def shodanp.count(self):
```

#### `shodanp.close`  <sub>(function)</sub>

```python
def shodanp.close(self):
```

#### `shodanp.get_type`  <sub>(function)</sub>

```python
def shodanp.get_type(self):
```

#### `shodanp.get_next`  <sub>(function)</sub>

```python
def shodanp.get_next(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/stdin.py`


#### `stdin`  <sub>(class)</sub>

```python
class stdin(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def count(self)
def get_type(self)
def get_next(self)
```


#### `stdin.__init__`  <sub>(function)</sub>

```python
def stdin.__init__(self, params):
```

#### `stdin.count`  <sub>(function)</sub>

```python
def stdin.count(self):
```

#### `stdin.get_type`  <sub>(function)</sub>

```python
def stdin.get_type(self):
```

#### `stdin.get_next`  <sub>(function)</sub>

```python
def stdin.get_next(self):
```

### `wfuzz/src/wfuzz/plugins/payloads/wfuzzp.py`


#### `wfuzzp`  <sub>(class)</sub>

```python
class wfuzzp(BasePayload):
```

**Fields:**
```python
name = ...
author = ...
version = ...
description = ...
summary = ...
category = ...
priority = ...
parameters = ...
default_parameter = ...
def __init__(self, params)
def count(self)
def get_next(self)
def get_type(self)
def _gen_wfuzz(self, output_fn)
```


#### `wfuzzp.__init__`  <sub>(function)</sub>

```python
def wfuzzp.__init__(self, params):
```

#### `wfuzzp.count`  <sub>(function)</sub>

```python
def wfuzzp.count(self):
```

#### `wfuzzp.get_next`  <sub>(function)</sub>

```python
def wfuzzp.get_next(self):
```

#### `wfuzzp.get_type`  <sub>(function)</sub>

```python
def wfuzzp.get_type(self):
```

#### `wfuzzp._gen_wfuzz`  <sub>(function)</sub>

```python
def wfuzzp._gen_wfuzz(self, output_fn):
```

### `wfuzz/src/wfuzz/plugins/printers/printers.py`


#### `magictree`  <sub>(class)</sub>

```python
class magictree(BasePrinter):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def __init__(self, output)
def __create_xml_element(self, parent, caption, text)
def header(self, summary)
def result(self, fuzz_result)
def footer(self, summary)
```


#### `magictree.__init__`  <sub>(function)</sub>

```python
def magictree.__init__(self, output):
```

#### `magictree.__create_xml_element`  <sub>(function)</sub>

```python
def magictree.__create_xml_element(self, parent, caption, text):
```

#### `magictree.header`  <sub>(function)</sub>

```python
def magictree.header(self, summary):
```

#### `magictree.result`  <sub>(function)</sub>

```python
def magictree.result(self, fuzz_result):
```

#### `magictree.footer`  <sub>(function)</sub>

```python
def magictree.footer(self, summary):
```

#### `html`  <sub>(class)</sub>

```python
class html(BasePrinter):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def __init__(self, output)
def header(self, summary)
def result(self, fuzz_result)
def footer(self, summary)
```


#### `html.__init__`  <sub>(function)</sub>

```python
def html.__init__(self, output):
```

#### `html.header`  <sub>(function)</sub>

```python
def html.header(self, summary):
```

#### `html.result`  <sub>(function)</sub>

```python
def html.result(self, fuzz_result):
```

#### `html.footer`  <sub>(function)</sub>

```python
def html.footer(self, summary):
```

#### `json`  <sub>(class)</sub>

```python
class json(BasePrinter):
```

**Fields:**
```python
name = ...
summary = ...
author = ...
version = ...
category = ...
priority = ...
def __init__(self, output)
def header(self, res)
def result(self, res)
def footer(self, summary)
```


#### `json.__init__`  <sub>(function)</sub>

```python
def json.__init__(self, output):
```

#### `json.header`  <sub>(function)</sub>

```python
def json.header(self, res):
```

#### `json.result`  <sub>(function)</sub>

```python
def json.result(self, res):
```

#### `json.footer`  <sub>(function)</sub>

```python
def json.footer(self, summary):
```

#### `raw`  <sub>(class)</sub>

```python
class raw(BasePrinter):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def __init__(self, output)
def header(self, summary)
def _print_verbose(self, res)
def _print(self, res)
def result(self, res)
def footer(self, summary)
```


#### `raw.__init__`  <sub>(function)</sub>

```python
def raw.__init__(self, output):
```

#### `raw.header`  <sub>(function)</sub>

```python
def raw.header(self, summary):
```

#### `raw._print_verbose`  <sub>(function)</sub>

```python
def raw._print_verbose(self, res):
```

#### `raw._print`  <sub>(function)</sub>

```python
def raw._print(self, res):
```

#### `raw.result`  <sub>(function)</sub>

```python
def raw.result(self, res):
```

#### `raw.footer`  <sub>(function)</sub>

```python
def raw.footer(self, summary):
```

#### `field`  <sub>(class)</sub>

```python
class field(BasePrinter):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
category = ...
priority = ...
def __init__(self, output)
def header(self, summary)
def result(self, res)
def footer(self, summary)
```


#### `field.__init__`  <sub>(function)</sub>

```python
def field.__init__(self, output):
```

#### `field.header`  <sub>(function)</sub>

```python
def field.header(self, summary):
```

#### `field.result`  <sub>(function)</sub>

```python
def field.result(self, res):
```

#### `field.footer`  <sub>(function)</sub>

```python
def field.footer(self, summary):
```

#### `csv`  <sub>(class)</sub>

```python
class csv(BasePrinter):
```

**Fields:**
```python
name = ...
author = ...
summary = ...
version = ...
category = ...
priority = ...
def write(self, e)
def __init__(self, output)
def header(self, summary)
def result(self, res)
def noresult(self, res)
def footer(self, summary)
def _print_csv(self, values)
```


#### `csv.write`  <sub>(function)</sub>

```python
def csv.write(self, e):
```

#### `csv.__init__`  <sub>(function)</sub>

```python
def csv.__init__(self, output):
```

#### `csv.header`  <sub>(function)</sub>

```python
def csv.header(self, summary):
```

#### `csv.result`  <sub>(function)</sub>

```python
def csv.result(self, res):
```

#### `csv.noresult`  <sub>(function)</sub>

```python
def csv.noresult(self, res):
```

#### `csv.footer`  <sub>(function)</sub>

```python
def csv.footer(self, summary):
```

#### `csv._print_csv`  <sub>(function)</sub>

```python
def csv._print_csv(self, values):
```

### `wfuzz/src/wfuzz/plugins/scripts/backups.py`


#### `backups`  <sub>(class)</sub>

```python
class backups(BasePlugin):
```

**Fields:**
```python
name = ...
summary = ...
description = ...
author = ...
version = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def process(self, fuzzresult)
```


#### `backups.__init__`  <sub>(function)</sub>

```python
def backups.__init__(self):
```

#### `backups.validate`  <sub>(function)</sub>

```python
def backups.validate(self, fuzzresult):
```

#### `backups.process`  <sub>(function)</sub>

```python
def backups.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/plugins/scripts/cookies.py`


#### `KBASE_NEW_COOKIE`  <sub>(const)</sub>

```python
KBASE_NEW_COOKIE = ...
```

#### `cookies`  <sub>(class)</sub>

```python
class cookies(BasePlugin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def process(self, fuzzresult)
```


#### `cookies.__init__`  <sub>(function)</sub>

```python
def cookies.__init__(self):
```

#### `cookies.validate`  <sub>(function)</sub>

```python
def cookies.validate(self, fuzzresult):
```

#### `cookies.process`  <sub>(function)</sub>

```python
def cookies.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/plugins/scripts/cvs_extractor.py`


#### `cvs_extractor`  <sub>(class)</sub>

```python
class cvs_extractor(BasePlugin, DiscoveryPluginMixin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def process(self, fuzzresult)
```


#### `cvs_extractor.__init__`  <sub>(function)</sub>

```python
def cvs_extractor.__init__(self):
```

#### `cvs_extractor.validate`  <sub>(function)</sub>

```python
def cvs_extractor.validate(self, fuzzresult):
```

#### `cvs_extractor.process`  <sub>(function)</sub>

```python
def cvs_extractor.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/plugins/scripts/errors.py`


#### `errors`  <sub>(class)</sub>

```python
class errors(BasePlugin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def process(self, fuzzresult)
```


#### `errors.__init__`  <sub>(function)</sub>

```python
def errors.__init__(self):
```

#### `errors.validate`  <sub>(function)</sub>

```python
def errors.validate(self, fuzzresult):
```

#### `errors.process`  <sub>(function)</sub>

```python
def errors.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/plugins/scripts/grep.py`


#### `grep`  <sub>(class)</sub>

```python
class grep(BasePlugin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def process(self, fuzzresult)
```


#### `grep.__init__`  <sub>(function)</sub>

```python
def grep.__init__(self):
```

#### `grep.validate`  <sub>(function)</sub>

```python
def grep.validate(self, fuzzresult):
```

#### `grep.process`  <sub>(function)</sub>

```python
def grep.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/plugins/scripts/headers.py`


#### `KBASE_KEY`  <sub>(const)</sub>

```python
KBASE_KEY = ...
```

#### `KBASE_KEY_RESP_UNCOMMON`  <sub>(const)</sub>

```python
KBASE_KEY_RESP_UNCOMMON = ...
```

#### `KBASE_KEY_REQ_UNCOMMON`  <sub>(const)</sub>

```python
KBASE_KEY_REQ_UNCOMMON = ...
```

#### `SERVER_HEADERS`  <sub>(const)</sub>

```python
SERVER_HEADERS = ...
```

#### `COMMON_RESPONSE_HEADERS_REGEX_LIST`  <sub>(const)</sub>

```python
COMMON_RESPONSE_HEADERS_REGEX_LIST = ...
```

#### `COMMON_RESPONSE_HEADERS_REGEX`  <sub>(const)</sub>

```python
COMMON_RESPONSE_HEADERS_REGEX = ...
```

#### `COMMON_REQ_HEADERS_REGEX_LIST`  <sub>(const)</sub>

```python
COMMON_REQ_HEADERS_REGEX_LIST = ...
```

#### `COMMON_REQ_HEADERS_REGEX`  <sub>(const)</sub>

```python
COMMON_REQ_HEADERS_REGEX = ...
```

#### `headers`  <sub>(class)</sub>

```python
class headers(BasePlugin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def check_request_header(self, fuzzresult, header, value)
def check_response_header(self, fuzzresult, header, value)
def check_server_header(self, fuzzresult, header, value)
def process(self, fuzzresult)
```


#### `headers.__init__`  <sub>(function)</sub>

```python
def headers.__init__(self):
```

#### `headers.validate`  <sub>(function)</sub>

```python
def headers.validate(self, fuzzresult):
```

#### `headers.check_request_header`  <sub>(function)</sub>

```python
def headers.check_request_header(self, fuzzresult, header, value):
```

#### `headers.check_response_header`  <sub>(function)</sub>

```python
def headers.check_response_header(self, fuzzresult, header, value):
```

#### `headers.check_server_header`  <sub>(function)</sub>

```python
def headers.check_server_header(self, fuzzresult, header, value):
```

#### `headers.process`  <sub>(function)</sub>

```python
def headers.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/plugins/scripts/links.py`


#### `KBASE_PARAM_PATH`  <sub>(const)</sub>

```python
KBASE_PARAM_PATH = ...
```

#### `KBASE_PARAM_ENQUEUE`  <sub>(const)</sub>

```python
KBASE_PARAM_ENQUEUE = ...
```

#### `KBASE_PARAM_DOMAIN_REGEX`  <sub>(const)</sub>

```python
KBASE_PARAM_DOMAIN_REGEX = ...
```

#### `KBASE_PARAM_REGEX`  <sub>(const)</sub>

```python
KBASE_PARAM_REGEX = ...
```

#### `KBASE_NEW_DOMAIN`  <sub>(const)</sub>

```python
KBASE_NEW_DOMAIN = ...
```

#### `links`  <sub>(class)</sub>

```python
class links(BasePlugin, DiscoveryPluginMixin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def process(self, fuzzresult)
def process_link(self, fuzzresult, link_url)
def enqueue_link(self, fuzzresult, link_url, parsed_link)
def from_domain(self, fuzzresult, parsed_link)
```


#### `links.__init__`  <sub>(function)</sub>

```python
def links.__init__(self):
```

#### `links.validate`  <sub>(function)</sub>

```python
def links.validate(self, fuzzresult):
```

#### `links.process`  <sub>(function)</sub>

```python
def links.process(self, fuzzresult):
```

#### `links.process_link`  <sub>(function)</sub>

```python
def links.process_link(self, fuzzresult, link_url):
```

#### `links.enqueue_link`  <sub>(function)</sub>

```python
def links.enqueue_link(self, fuzzresult, link_url, parsed_link):
```

#### `links.from_domain`  <sub>(function)</sub>

```python
def links.from_domain(self, fuzzresult, parsed_link):
```

### `wfuzz/src/wfuzz/plugins/scripts/listing.py`


#### `listing`  <sub>(class)</sub>

```python
class listing(BasePlugin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def process(self, fuzzresult)
```


#### `listing.__init__`  <sub>(function)</sub>

```python
def listing.__init__(self):
```

#### `listing.validate`  <sub>(function)</sub>

```python
def listing.validate(self, fuzzresult):
```

#### `listing.process`  <sub>(function)</sub>

```python
def listing.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/plugins/scripts/npm_deps.py`


#### `npm_deps`  <sub>(class)</sub>

```python
class npm_deps(BasePlugin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
REGEX_PATT = ...
REGEX_DEP = ...
REGEX_DEV_DEP = ...
def __init__(self)
def validate(self, fuzzresult)
def process(self, fuzzresult)
```


#### `npm_deps.__init__`  <sub>(function)</sub>

```python
def npm_deps.__init__(self):
```

#### `npm_deps.validate`  <sub>(function)</sub>

```python
def npm_deps.validate(self, fuzzresult):
```

#### `npm_deps.process`  <sub>(function)</sub>

```python
def npm_deps.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/plugins/scripts/robots.py`


#### `robots`  <sub>(class)</sub>

```python
class robots(BasePlugin, DiscoveryPluginMixin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def process(self, fuzzresult)
```


#### `robots.__init__`  <sub>(function)</sub>

```python
def robots.__init__(self):
```

#### `robots.validate`  <sub>(function)</sub>

```python
def robots.validate(self, fuzzresult):
```

#### `robots.process`  <sub>(function)</sub>

```python
def robots.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/plugins/scripts/screenshot.py`


#### `screenshot`  <sub>(class)</sub>

```python
class screenshot(BasePlugin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def process(self, fuzzresult)
```


#### `screenshot.__init__`  <sub>(function)</sub>

```python
def screenshot.__init__(self):
```

#### `screenshot.validate`  <sub>(function)</sub>

```python
def screenshot.validate(self, fuzzresult):
```

#### `screenshot.process`  <sub>(function)</sub>

```python
def screenshot.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/plugins/scripts/sitemap.py`


#### `sitemap`  <sub>(class)</sub>

```python
class sitemap(BasePlugin, DiscoveryPluginMixin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def process(self, fuzzresult)
```


#### `sitemap.__init__`  <sub>(function)</sub>

```python
def sitemap.__init__(self):
```

#### `sitemap.validate`  <sub>(function)</sub>

```python
def sitemap.validate(self, fuzzresult):
```

#### `sitemap.process`  <sub>(function)</sub>

```python
def sitemap.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/plugins/scripts/svn_extractor.py`


#### `svn_extractor`  <sub>(class)</sub>

```python
class svn_extractor(BasePlugin, DiscoveryPluginMixin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def readsvn(self, content)
def process(self, fuzzresult)
```


#### `svn_extractor.__init__`  <sub>(function)</sub>

```python
def svn_extractor.__init__(self):
```

#### `svn_extractor.validate`  <sub>(function)</sub>

```python
def svn_extractor.validate(self, fuzzresult):
```

#### `svn_extractor.readsvn`  <sub>(function)</sub>

> Function shamesly copied (and adapted) from https://github.com/anantshri/svn-extractor/  
> Credit (C) Anant Shrivastava http://anantshri.info

```python
def svn_extractor.readsvn(self, content):
```

#### `svn_extractor.process`  <sub>(function)</sub>

```python
def svn_extractor.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/plugins/scripts/title.py`


#### `title`  <sub>(class)</sub>

```python
class title(BasePlugin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def process(self, fuzzresult)
```


#### `title.__init__`  <sub>(function)</sub>

```python
def title.__init__(self):
```

#### `title.validate`  <sub>(function)</sub>

```python
def title.validate(self, fuzzresult):
```

#### `title.process`  <sub>(function)</sub>

```python
def title.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/plugins/scripts/wcdb.py`


#### `wcdb_extractor`  <sub>(class)</sub>

```python
class wcdb_extractor(BasePlugin, DiscoveryPluginMixin):
```

**Fields:**
```python
name = ...
author = ...
version = ...
summary = ...
description = ...
category = ...
priority = ...
parameters = ...
def __init__(self)
def validate(self, fuzzresult)
def readwc(self, content)
def process(self, fuzzresult)
```


#### `wcdb_extractor.__init__`  <sub>(function)</sub>

```python
def wcdb_extractor.__init__(self):
```

#### `wcdb_extractor.validate`  <sub>(function)</sub>

```python
def wcdb_extractor.validate(self, fuzzresult):
```

#### `wcdb_extractor.readwc`  <sub>(function)</sub>

> Function shamesly copied (and adapted) from https://github.com/anantshri/svn-extractor/  
> Credit (C) Anant Shrivastava http://anantshri.info

```python
def wcdb_extractor.readwc(self, content):
```

#### `wcdb_extractor.process`  <sub>(function)</sub>

```python
def wcdb_extractor.process(self, fuzzresult):
```

### `wfuzz/src/wfuzz/ui/console/clparser.py`


#### `REPEATABLE_OPTS`  <sub>(const)</sub>

```python
REPEATABLE_OPTS = ...
```

#### `CLParser`  <sub>(class)</sub>

```python
class CLParser:
```

**Fields:**
```python
def __init__(self, argv, short_opts, long_opts, help_banner, brief_usage, verbose_usage, usage)
def show_brief_usage(self)
def show_verbose_usage(self)
def show_usage(self)
def show_plugins_help(self, registrant, cols, category)
def show_plugins_names(self, registrant)
def show_plugin_ext_help(self, registrant, category)
def parse_cl(self)
def _parse_help_opt(self, optsd)
def _check_options(self, optsd)
def _parse_filters(self, optsd, filter_params)
def _parse_payload(self, optsd, options)
def _parse_seed(self, url, optsd, options)
def _parse_conn_options(self, optsd, conn_options)
def _parse_options(self, optsd, options)
def _parse_scripts(self, optsd, options)
```


#### `CLParser.__init__`  <sub>(function)</sub>

```python
def CLParser.__init__(
        self,
        argv,
        short_opts=short_opts,
        long_opts=long_opts,
        help_banner=help_banner,
        brief_usage=brief_usage,
        verbose_usage=verbose_usage,
        usage=usage,
):
```

#### `CLParser.show_brief_usage`  <sub>(function)</sub>

```python
def CLParser.show_brief_usage(self):
```

#### `CLParser.show_verbose_usage`  <sub>(function)</sub>

```python
def CLParser.show_verbose_usage(self):
```

#### `CLParser.show_usage`  <sub>(function)</sub>

```python
def CLParser.show_usage(self):
```

#### `CLParser.show_plugins_help`  <sub>(function)</sub>

```python
def CLParser.show_plugins_help(self, registrant, cols=3, category="$all$"):
```

#### `CLParser.show_plugins_names`  <sub>(function)</sub>

```python
def CLParser.show_plugins_names(self, registrant):
```

#### `CLParser.show_plugin_ext_help`  <sub>(function)</sub>

```python
def CLParser.show_plugin_ext_help(self, registrant, category="$all$"):
```

#### `CLParser.parse_cl`  <sub>(function)</sub>

```python
def CLParser.parse_cl(self):
```

#### `CLParser._parse_help_opt`  <sub>(function)</sub>

```python
def CLParser._parse_help_opt(self, optsd):
```

#### `CLParser._check_options`  <sub>(function)</sub>

```python
def CLParser._check_options(self, optsd):
```

#### `CLParser._parse_filters`  <sub>(function)</sub>

> filter_params = dict(  
> hs = None,  
> hc = [],  
> hw = [],  
> hl = [],  
> hh = [],  
> ss = None,  
> sc = [],  
> sw = [],  
> sl = [],  
> sh = [],  
> filter = "",  
> prefilter = "",  
> ),

```python
def CLParser._parse_filters(self, optsd, filter_params):
```

#### `CLParser._parse_payload`  <sub>(function)</sub>

> options = dict(  
> payloads = [],  
> iterator = None,  
> )

```python
def CLParser._parse_payload(self, optsd, options):
```

#### `CLParser._parse_seed`  <sub>(function)</sub>

```python
def CLParser._parse_seed(self, url, optsd, options):
```

#### `CLParser._parse_conn_options`  <sub>(function)</sub>

```python
def CLParser._parse_conn_options(self, optsd, conn_options):
```

#### `CLParser._parse_options`  <sub>(function)</sub>

```python
def CLParser._parse_options(self, optsd, options):
```

#### `CLParser._parse_scripts`  <sub>(function)</sub>

> options = dict(  
> script = "",  
> script_args = {},  
> )

```python
def CLParser._parse_scripts(self, optsd, options):
```

### `wfuzz/src/wfuzz/ui/console/common.py`


#### `Term`  <sub>(class)</sub>

```python
class Term:
```

**Fields:**
```python
reset = ...
bright = ...
dim = ...
underscore = ...
blink = ...
reverse = ...
hidden = ...
delete = ...
oneup = ...
fgBlack = ...
fgRed = ...
fgGreen = ...
fgYellow = ...
fgBlue = ...
fgMagenta = ...
fgCyan = ...
fgWhite = ...
bgBlack = ...
bgRed = ...
bgGreen = ...
bgYellow = ...
bgBlue = ...
bgMagenta = ...
bgCyan = ...
bgWhite = ...
noColour = ...
def get_colour(self, code)
def delete_line(self)
def set_colour(self, colour)
def write(self, string, colour)
def go_up(self, lines)
def erase_lines(self, lines)
```


#### `Term.get_colour`  <sub>(function)</sub>

```python
def Term.get_colour(self, code):
```

#### `Term.delete_line`  <sub>(function)</sub>

```python
def Term.delete_line(self):
```

#### `Term.set_colour`  <sub>(function)</sub>

```python
def Term.set_colour(self, colour):
```

#### `Term.write`  <sub>(function)</sub>

```python
def Term.write(self, string, colour):
```

#### `Term.go_up`  <sub>(function)</sub>

```python
def Term.go_up(self, lines):
```

#### `Term.erase_lines`  <sub>(function)</sub>

```python
def Term.erase_lines(self, lines):
```

### `wfuzz/src/wfuzz/ui/console/getch.py`


#### `_Getch`  <sub>(class)</sub>

> Gets a single character from standard input.  Does not echo to the screen.

```python
class _Getch:
```

**Fields:**
```python
def __init__(self)
def __call__(self)
```


#### `_Getch.__init__`  <sub>(function)</sub>

```python
def _Getch.__init__(self):
```

#### `_Getch.__call__`  <sub>(function)</sub>

```python
def _Getch.__call__(self):
```

#### `_GetchUnix`  <sub>(class)</sub>

```python
class _GetchUnix:
```

**Fields:**
```python
def __init__(self)
def __call__(self)
```


#### `_GetchUnix.__init__`  <sub>(function)</sub>

```python
def _GetchUnix.__init__(self):
```

#### `_GetchUnix.__call__`  <sub>(function)</sub>

```python
def _GetchUnix.__call__(self):
```

#### `_GetchWindows`  <sub>(class)</sub>

```python
class _GetchWindows:
```

**Fields:**
```python
def __init__(self)
def __call__(self)
```


#### `_GetchWindows.__init__`  <sub>(function)</sub>

```python
def _GetchWindows.__init__(self):
```

#### `_GetchWindows.__call__`  <sub>(function)</sub>

```python
def _GetchWindows.__call__(self):
```

#### `_GetchMacCarbon`  <sub>(class)</sub>

> A function which returns the current ASCII key that is down;  
> if no ASCII key is down, the null string is returned.  The  
> page http://www.mactech.com/macintosh-c/chap02-1.html was  
> very helpful in figuring out how to do this.

```python
class _GetchMacCarbon:
```

**Fields:**
```python
def __init__(self)
def __call__(self)
```


#### `_GetchMacCarbon.__init__`  <sub>(function)</sub>

```python
def _GetchMacCarbon.__init__(self):
```

#### `_GetchMacCarbon.__call__`  <sub>(function)</sub>

```python
def _GetchMacCarbon.__call__(self):
```

### `wfuzz/src/wfuzz/ui/console/mvc.py`


#### `SimpleEventDispatcher`  <sub>(class)</sub>

```python
class SimpleEventDispatcher:
```

**Fields:**
```python
def __init__(self)
def create_event(self, msg)
def subscribe(self, func, msg, dynamic)
def notify(self, msg)
```


#### `SimpleEventDispatcher.__init__`  <sub>(function)</sub>

```python
def SimpleEventDispatcher.__init__(self):
```

#### `SimpleEventDispatcher.create_event`  <sub>(function)</sub>

```python
def SimpleEventDispatcher.create_event(self, msg):
```

#### `SimpleEventDispatcher.subscribe`  <sub>(function)</sub>

```python
def SimpleEventDispatcher.subscribe(self, func, msg, dynamic=False):
```

#### `SimpleEventDispatcher.notify`  <sub>(function)</sub>

```python
def SimpleEventDispatcher.notify(self, msg, **event):
```

#### `KeyPress`  <sub>(class)</sub>

```python
class KeyPress(threading.Thread):
```

**Fields:**
```python
def __init__(self)
def cancel_job(self)
def run(self)
```


#### `KeyPress.__init__`  <sub>(function)</sub>

```python
def KeyPress.__init__(self):
```

#### `KeyPress.cancel_job`  <sub>(function)</sub>

```python
def KeyPress.cancel_job(self):
```

#### `KeyPress.run`  <sub>(function)</sub>

```python
def KeyPress.run(self):
```

#### `Controller`  <sub>(class)</sub>

```python
class Controller:
```

**Fields:**
```python
def __init__(self, fuzzer, view)
def on_exit(self)
def on_help(self)
def on_pause(self)
def on_stats(self)
def show_debug_stats(self)
def show_stats(self)
```


#### `Controller.__init__`  <sub>(function)</sub>

```python
def Controller.__init__(self, fuzzer, view):
```

#### `Controller.on_exit`  <sub>(function)</sub>

```python
def Controller.on_exit(self, **event):
```

#### `Controller.on_help`  <sub>(function)</sub>

```python
def Controller.on_help(self, **event):
```

#### `Controller.on_pause`  <sub>(function)</sub>

```python
def Controller.on_pause(self, **event):
```

#### `Controller.on_stats`  <sub>(function)</sub>

```python
def Controller.on_stats(self, **event):
```

#### `Controller.show_debug_stats`  <sub>(function)</sub>

```python
def Controller.show_debug_stats(self):
```

#### `Controller.show_stats`  <sub>(function)</sub>

```python
def Controller.show_stats(self):
```

#### `View`  <sub>(class)</sub>

```python
class View:
```

**Fields:**
```python
widths = ...
verbose_widths = ...
def __init__(self, session_options)
def _print_verbose(self, res, print_nres)
def _print_header(self, rows, maxWidths)
def _print_line(self, rows, maxWidths)
def _print(self, res, print_nres)
def header(self, summary)
def result(self, res)
def footer(self, summary)
```


#### `View.__init__`  <sub>(function)</sub>

```python
def View.__init__(self, session_options):
```

#### `View._print_verbose`  <sub>(function)</sub>

```python
def View._print_verbose(self, res, print_nres=True):
```

#### `View._print_header`  <sub>(function)</sub>

```python
def View._print_header(self, rows, maxWidths):
```

#### `View._print_line`  <sub>(function)</sub>

```python
def View._print_line(self, rows, maxWidths):
```

#### `View._print`  <sub>(function)</sub>

```python
def View._print(self, res, print_nres=True):
```

#### `View.header`  <sub>(function)</sub>

```python
def View.header(self, summary):
```

#### `View.result`  <sub>(function)</sub>

```python
def View.result(self, res):
```

#### `View.footer`  <sub>(function)</sub>

```python
def View.footer(self, summary):
```

### `wfuzz/src/wfuzz/ui/console/output.py`


#### `indent`  <sub>(function)</sub>

> @author http://code.activestate.com/recipes/267662-table-indentation/  
>   
> Indents a table by column.  
> - rows: A sequence of sequences of items, one sequence per row.  
> - hasHeader: True if the first row consists of the columns' names.  
> - headerChar: Character to be used for the row separator line  
> (if hasHeader==True or separateRows==True).  
> - delim: The column delimiter.  
> - justify: Determines how are data justified in their column.  
> Valid values are 'left','right' and 'center'.  
> - separateRows: True if rows are to be separated by a line  
> of 'headerChar's.  
> - prefix: A string prepended to each printed row.  
> - postfix: A string appended to each printed row.  
> - wrapfunc: A function f(text) for wrapping text; each element in  
> the table is first wrapped by this function.

```python
def indent(
        rows,
        hasHeader=False,
        headerChar="-",
        delim=" | ",
        justify="left",
        separateRows=False,
        prefix="",
        postfix="",
        wrapfunc=lambda x: x,
):
```

#### `wrap_always`  <sub>(function)</sub>

> A simple word-wrap function that wraps text on exactly width characters.  
> It doesn't split the text in words.

```python
def wrap_always(text, width):
```

#### `wrap_always_list`  <sub>(function)</sub>

```python
def wrap_always_list(alltext, width):
```

#### `table_print`  <sub>(function)</sub>

```python
def table_print(rows, width=80):
```

#### `getTerminalSize`  <sub>(function)</sub>

```python
def getTerminalSize():
```

#### `_getTerminalSize_windows`  <sub>(function)</sub>

```python
def _getTerminalSize_windows():
```

#### `_getTerminalSize_tput`  <sub>(function)</sub>

```python
def _getTerminalSize_tput():
```

#### `_getTerminalSize_linux`  <sub>(function)</sub>

```python
def _getTerminalSize_linux():
```

### `wfuzz/src/wfuzz/ui/gui/controller.py`


#### `WfuzzInterpreter`  <sub>(class)</sub>

```python
class WfuzzInterpreter:
```

**Fields:**
```python
def __init__(self, model)
def onecmd(self, cmd)
def do_wfilter(self, cmd)
def do_wfuzz(self, cmd)
def do_delete(self, cmd)
def do_tab(self, cmd)
```


#### `WfuzzInterpreter.__init__`  <sub>(function)</sub>

```python
def WfuzzInterpreter.__init__(self, model):
```

#### `WfuzzInterpreter.onecmd`  <sub>(function)</sub>

```python
def WfuzzInterpreter.onecmd(self, cmd):
```

#### `WfuzzInterpreter.do_wfilter`  <sub>(function)</sub>

```python
def WfuzzInterpreter.do_wfilter(self, cmd):
```

#### `WfuzzInterpreter.do_wfuzz`  <sub>(function)</sub>

```python
def WfuzzInterpreter.do_wfuzz(self, cmd):
```

#### `WfuzzInterpreter.do_delete`  <sub>(function)</sub>

```python
def WfuzzInterpreter.do_delete(self, cmd):
```

#### `WfuzzInterpreter.do_tab`  <sub>(function)</sub>

```python
def WfuzzInterpreter.do_tab(self, cmd):
```

#### `GUIController`  <sub>(class)</sub>

```python
class GUIController:
```

**Fields:**
```python
def __init__(self, view)
def start_gui(self)
def on_exit(self, msg)
```


#### `GUIController.__init__`  <sub>(function)</sub>

```python
def GUIController.__init__(self, view):
```

#### `GUIController.start_gui`  <sub>(function)</sub>

```python
def GUIController.start_gui(self):
```

#### `GUIController.on_exit`  <sub>(function)</sub>

```python
def GUIController.on_exit(self, msg):
```

### `wfuzz/src/wfuzz/ui/gui/guicontrols.py`


#### `RedirectText`  <sub>(class)</sub>

```python
class RedirectText(object):
```

**Fields:**
```python
def __init__(self, aWxTextCtrl)
def write(self, string)
```


#### `RedirectText.__init__`  <sub>(function)</sub>

```python
def RedirectText.__init__(self, aWxTextCtrl):
```

#### `RedirectText.write`  <sub>(function)</sub>

```python
def RedirectText.write(self, string):
```

#### `ConsolePanel`  <sub>(class)</sub>

```python
class ConsolePanel(wx.Panel):
```

**Fields:**
```python
def __init__(self, parent, interpreter)
def __bind_events(self, e)
def get_last_line(self)
```


#### `ConsolePanel.__init__`  <sub>(function)</sub>

```python
def ConsolePanel.__init__(self, parent, interpreter):
```

#### `ConsolePanel.__bind_events`  <sub>(function)</sub>

```python
def ConsolePanel.__bind_events(self, e):
```

#### `ConsolePanel.get_last_line`  <sub>(function)</sub>

```python
def ConsolePanel.get_last_line(self):
```

#### `ListPanel`  <sub>(class)</sub>

```python
class ListPanel(wx.Panel):
```

**Fields:**
```python
def __init__(self, parent, log, model, interpreter)
def OnItemSelected(self, event)
def MakePaneContent(self, pane)
```


#### `ListPanel.__init__`  <sub>(function)</sub>

```python
def ListPanel.__init__(self, parent, log, model, interpreter):
```

#### `ListPanel.OnItemSelected`  <sub>(function)</sub>

```python
def ListPanel.OnItemSelected(self, event):
```

#### `ListPanel.MakePaneContent`  <sub>(function)</sub>

```python
def ListPanel.MakePaneContent(self, pane):
```

#### `HttpRawPanel`  <sub>(class)</sub>

```python
class HttpRawPanel(wx.Panel):
```

**Fields:**
```python
def __init__(self, parent, frame)
def CreateHTMLCtrl(self)
```


#### `HttpRawPanel.__init__`  <sub>(function)</sub>

```python
def HttpRawPanel.__init__(self, parent, frame):
```

#### `HttpRawPanel.CreateHTMLCtrl`  <sub>(function)</sub>

```python
def HttpRawPanel.CreateHTMLCtrl(self):
```

#### `MainNotebookPanel`  <sub>(class)</sub>

```python
class MainNotebookPanel(wx.Panel):
```

**Fields:**
```python
def __init__(self, parent, frame, interpreter)
def create_web_view(self)
def on_selected_row(self, row)
```


#### `MainNotebookPanel.__init__`  <sub>(function)</sub>

```python
def MainNotebookPanel.__init__(self, parent, frame, interpreter):
```

#### `MainNotebookPanel.create_web_view`  <sub>(function)</sub>

```python
def MainNotebookPanel.create_web_view(self):
```

#### `MainNotebookPanel.on_selected_row`  <sub>(function)</sub>

```python
def MainNotebookPanel.on_selected_row(self, row):
```

#### `WfuzzFrame`  <sub>(class)</sub>

```python
class WfuzzFrame(wx.Frame):
```

**Fields:**
```python
def __init__(self, parent, id, title, pos, size, style)
def start_gui(self, controller)
def OnClose(self, event)
def OnExit(self, event)
def OnAbout(self, event)
def CreateNotebook(self)
def OnAddTab(self, name, model, interp)
```


#### `WfuzzFrame.__init__`  <sub>(function)</sub>

```python
def WfuzzFrame.__init__(
        self,
        parent,
        id=-1,
        title="Wfuzz",
        pos=wx.DefaultPosition,
        size=wx.DefaultSize,
        style=wx.DEFAULT_FRAME_STYLE | wx.SUNKEN_BORDER | wx.CLIP_CHILDREN,
):
```

#### `WfuzzFrame.start_gui`  <sub>(function)</sub>

```python
def WfuzzFrame.start_gui(self, controller):
```

#### `WfuzzFrame.OnClose`  <sub>(function)</sub>

```python
def WfuzzFrame.OnClose(self, event):
```

#### `WfuzzFrame.OnExit`  <sub>(function)</sub>

```python
def WfuzzFrame.OnExit(self, event):
```

#### `WfuzzFrame.OnAbout`  <sub>(function)</sub>

```python
def WfuzzFrame.OnAbout(self, event):
```

#### `WfuzzFrame.CreateNotebook`  <sub>(function)</sub>

```python
def WfuzzFrame.CreateNotebook(self):
```

#### `WfuzzFrame.OnAddTab`  <sub>(function)</sub>

```python
def WfuzzFrame.OnAddTab(self, name, model, interp):
```

### `wfuzz/src/wfuzz/ui/gui/model.py`


#### `GUIModel`  <sub>(class)</sub>

```python
class GUIModel(dv.PyDataViewIndexListModel):
```

**Fields:**
```python
def __init__(self, data)
def GetColumnType(self, col)
def GetValueByRow(self, row, col)
def GetColumnCount(self)
def GetCount(self)
def GetAttrByRow(self, row, col, attr)
def Compare(self, item1, item2, col, ascending)
def DeleteRows(self, rows)
def AddRow(self, value)
def Clear(self)
def DeleteRows_by_filter(self, filter_string)
```


#### `GUIModel.__init__`  <sub>(function)</sub>

```python
def GUIModel.__init__(self, data=None):
```

#### `GUIModel.GetColumnType`  <sub>(function)</sub>

```python
def GUIModel.GetColumnType(self, col):
```

#### `GUIModel.GetValueByRow`  <sub>(function)</sub>

```python
def GUIModel.GetValueByRow(self, row, col):
```

#### `GUIModel.GetColumnCount`  <sub>(function)</sub>

```python
def GUIModel.GetColumnCount(self):
```

#### `GUIModel.GetCount`  <sub>(function)</sub>

```python
def GUIModel.GetCount(self):
```

#### `GUIModel.GetAttrByRow`  <sub>(function)</sub>

```python
def GUIModel.GetAttrByRow(self, row, col, attr):
```

#### `GUIModel.Compare`  <sub>(function)</sub>

```python
def GUIModel.Compare(self, item1, item2, col, ascending):
```

#### `GUIModel.DeleteRows`  <sub>(function)</sub>

```python
def GUIModel.DeleteRows(self, rows):
```

#### `GUIModel.AddRow`  <sub>(function)</sub>

```python
def GUIModel.AddRow(self, value):
```

#### `GUIModel.Clear`  <sub>(function)</sub>

```python
def GUIModel.Clear(self):
```

#### `GUIModel.DeleteRows_by_filter`  <sub>(function)</sub>

```python
def GUIModel.DeleteRows_by_filter(self, filter_string):
```

### `wfuzz/src/wfuzz/wfuzz.py`


#### `PROFILING`  <sub>(const)</sub>

```python
PROFILING = ...
```

#### `print_profiling`  <sub>(function)</sub>

```python
def print_profiling(profiling_list, profiling_header):
```

#### `main`  <sub>(function)</sub>

```python
def main():
```

#### `main_filter`  <sub>(function)</sub>

```python
def main_filter():
```

#### `main_encoder`  <sub>(function)</sub>

```python
def main_encoder():
```

#### `main_gui`  <sub>(function)</sub>

```python
def main_gui():
```
