# patator — Complete API Reference

Exhaustive per-item reference for **every** type, function, method, constant, and macro in the patator source (`lanjelot/patator` @ `964e87c`, v1.1). Language: **PY**. Generated from a full source read; test files are excluded. Each item shows its full signature and doc comment; composite types list their fields/members.

**270 documented items** across 3 source files.


---

## Modules / packages

- [`src`](#pkg-src) — 270 items

---


## `src` <a id='pkg-src'></a>

*270 documented items*


### `patator/src/patator/patator.py`


#### `Logger`  <sub>(class)</sub>

```python
class Logger:
```

**Fields:**
```python
def __init__(self, queue)
def send(self, action)
def quit(self)
def headers(self)
def result(self)
def save_response(self)
def save_hit(self)
def setLevel(self, level)
def warn(self, msg)
def info(self, msg)
def debug(self, msg)
```


#### `Logger.__init__`  <sub>(function)</sub>

```python
def Logger.__init__(self, queue):
```

#### `Logger.send`  <sub>(function)</sub>

```python
def Logger.send(self, action, *args):
```

#### `Logger.quit`  <sub>(function)</sub>

```python
def Logger.quit(self):
```

#### `Logger.headers`  <sub>(function)</sub>

```python
def Logger.headers(self):
```

#### `Logger.result`  <sub>(function)</sub>

```python
def Logger.result(self, *args):
```

#### `Logger.save_response`  <sub>(function)</sub>

```python
def Logger.save_response(self, *args):
```

#### `Logger.save_hit`  <sub>(function)</sub>

```python
def Logger.save_hit(self, *args):
```

#### `Logger.setLevel`  <sub>(function)</sub>

```python
def Logger.setLevel(self, level):
```

#### `Logger.warn`  <sub>(function)</sub>

```python
def Logger.warn(self, msg):
```

#### `Logger.info`  <sub>(function)</sub>

```python
def Logger.info(self, msg):
```

#### `Logger.debug`  <sub>(function)</sub>

```python
def Logger.debug(self, msg):
```

#### `TXTFormatter`  <sub>(class)</sub>

```python
class TXTFormatter(logging.Formatter):
```

**Fields:**
```python
def __init__(self, indicatorsfmt)
def format(self, record)
```


#### `TXTFormatter.__init__`  <sub>(function)</sub>

```python
def TXTFormatter.__init__(self, indicatorsfmt):
```

#### `TXTFormatter.format`  <sub>(function)</sub>

```python
def TXTFormatter.format(self, record):
```

#### `CSVFormatter`  <sub>(class)</sub>

```python
class CSVFormatter(logging.Formatter):
```

**Fields:**
```python
def __init__(self, indicatorsfmt)
def format(self, record)
```


#### `CSVFormatter.__init__`  <sub>(function)</sub>

```python
def CSVFormatter.__init__(self, indicatorsfmt):
```

#### `CSVFormatter.format`  <sub>(function)</sub>

```python
def CSVFormatter.format(self, record):
```

#### `XMLFormatter`  <sub>(class)</sub>

```python
class XMLFormatter(logging.Formatter):
```

**Fields:**
```python
def __init__(self, indicatorsfmt)
def format(self, record)
```


#### `XMLFormatter.__init__`  <sub>(function)</sub>

```python
def XMLFormatter.__init__(self, indicatorsfmt):
```

#### `XMLFormatter.format`  <sub>(function)</sub>

```python
def XMLFormatter.format(self, record):
```

#### `MsgFilter`  <sub>(class)</sub>

```python
class MsgFilter(logging.Filter):
```

**Fields:**
```python
def filter(self, record)
```


#### `MsgFilter.filter`  <sub>(function)</sub>

```python
def MsgFilter.filter(self, record):
```

#### `process_logs`  <sub>(function)</sub>

```python
def process_logs(queue, indicatorsfmt, argv, log_dir, runtime_file, csv_file, xml_file, hits_file):
```

#### `b`  <sub>(function)</sub>

```python
def b(x):
```

#### `B`  <sub>(function)</sub>

```python
def B(x):
```

#### `expand_path`  <sub>(function)</sub>

```python
def expand_path(s):
```

#### `strfutctime`  <sub>(function)</sub>

```python
def strfutctime():
```

#### `strflocaltime`  <sub>(function)</sub>

```python
def strflocaltime():
```

#### `which`  <sub>(function)</sub>

```python
def which(program):
```

#### `build_logdir`  <sub>(function)</sub>

```python
def build_logdir(opt_dir, opt_auto, assume_yes):
```

#### `create_dir`  <sub>(function)</sub>

```python
def create_dir(top_path, assume_yes):
```

#### `create_time_dir`  <sub>(function)</sub>

```python
def create_time_dir(top_path, desc):
```

#### `pprint_seconds`  <sub>(function)</sub>

```python
def pprint_seconds(seconds, fmt):
```

#### `repr23`  <sub>(function)</sub>

```python
def repr23(s):
```

#### `md5hex`  <sub>(function)</sub>

```python
def md5hex(plain):
```

#### `sha1hex`  <sub>(function)</sub>

```python
def sha1hex(plain):
```

#### `count_lines`  <sub>(function)</sub>

```python
def count_lines(filename):
```

#### `product`  <sub>(function)</sub>

```python
def product(xs, *rest):
```

#### `chain`  <sub>(class)</sub>

```python
class chain:
```

**Fields:**
```python
def __init__(self)
def __iter__(self)
```


#### `chain.__init__`  <sub>(function)</sub>

```python
def chain.__init__(self, *iterables):
```

#### `chain.__iter__`  <sub>(function)</sub>

```python
def chain.__iter__(self):
```

#### `FileIter`  <sub>(class)</sub>

```python
class FileIter:
```

**Fields:**
```python
def __init__(self, filename)
def __iter__(self)
```


#### `FileIter.__init__`  <sub>(function)</sub>

```python
def FileIter.__init__(self, filename):
```

#### `FileIter.__iter__`  <sub>(function)</sub>

```python
def FileIter.__iter__(self):
```

#### `padhex`  <sub>(function)</sub>

```python
def padhex(d):
```

#### `RangeIter`  <sub>(class)</sub>

```python
class RangeIter:
```

**Fields:**
```python
def __init__(self, typ, rng, random)
def __iter__(self)
def __len__(self)
```


#### `RangeIter.__init__`  <sub>(function)</sub>

```python
def RangeIter.__init__(self, typ, rng, random=None):
```

#### `RangeIter.__iter__`  <sub>(function)</sub>

```python
def RangeIter.__iter__(self):
```

#### `RangeIter.__len__`  <sub>(function)</sub>

```python
def RangeIter.__len__(self):
```

#### `ProgIter`  <sub>(class)</sub>

```python
class ProgIter:
```

**Fields:**
```python
def __init__(self, prog)
def __iter__(self)
```


#### `ProgIter.__init__`  <sub>(function)</sub>

```python
def ProgIter.__init__(self, prog):
```

#### `ProgIter.__iter__`  <sub>(function)</sub>

```python
def ProgIter.__iter__(self):
```

#### `Progress`  <sub>(class)</sub>

```python
class Progress:
```

**Fields:**
```python
def __init__(self)
```


#### `Progress.__init__`  <sub>(function)</sub>

```python
def Progress.__init__(self):
```

#### `TimeoutError`  <sub>(class)</sub>

```python
class TimeoutError(Exception):
```

#### `on_windows`  <sub>(function)</sub>

```python
def on_windows():
```

#### `ignore_ctrlc`  <sub>(function)</sub>

```python
def ignore_ctrlc():
```

#### `handle_alarm`  <sub>(function)</sub>

```python
def handle_alarm():
```

#### `raise_timeout`  <sub>(function)</sub>

```python
def raise_timeout(signum, frame):
```

#### `enable_alarm`  <sub>(function)</sub>

```python
def enable_alarm(timeout):
```

#### `disable_alarm`  <sub>(function)</sub>

```python
def disable_alarm():
```

#### `MyManager`  <sub>(class)</sub>

```python
class MyManager(SyncManager):
```

**Fields:**
```python
def _run_server(cls, registry, address, authkey, serializer, writer, initializer, initargs)
```


#### `MyManager._run_server`  <sub>(function)</sub>

```python
def MyManager._run_server(cls, registry, address, authkey, serializer, writer, initializer=None, initargs=()):
```

#### `ppstr`  <sub>(function)</sub>

```python
def ppstr(s):
```

#### `flatten`  <sub>(function)</sub>

```python
def flatten(l):
```

#### `parse_query`  <sub>(function)</sub>

> Same as urllib.parse.parse_qsl but without replacing '+' with ' '

```python
def parse_query(qs, keep_blank_values=False, encoding='utf-8', errors='replace'):
```

#### `Controller`  <sub>(class)</sub>

```python
class Controller:
```

**Fields:**
```python
builtin_actions = ...
available_encodings = ...
def expand_key(self, arg)
def find_file_keys(self, value)
def find_net_keys(self, value)
def find_combo_keys(self, value)
def find_module_keys(self, value)
def find_range_keys(self, value)
def find_prog_keys(self, value)
def usage_parser(self, name)
def parse_usage(self, argv)
def __init__(self, module, argv)
def update_actions(self, arg)
def lookup_actions(self, resp)
def should_free(self, payload)
def register_free(self, payload, opts)
def should_skip(self, prod)
def register_skip(self, prod, opts)
def fire(self)
def push_final(self, resp)
def show_final(self)
def start_threads(self)
def produce(self, task_queues, log_queue)
def consume(self, task_queue, report_queue, log_queue)
def monitor_progress(self)
def report_progress(self)
def monitor_interaction(self)
```


#### `Controller.expand_key`  <sub>(function)</sub>

```python
def Controller.expand_key(self, arg):
```

#### `Controller.find_file_keys`  <sub>(function)</sub>

```python
def Controller.find_file_keys(self, value):
```

#### `Controller.find_net_keys`  <sub>(function)</sub>

```python
def Controller.find_net_keys(self, value):
```

#### `Controller.find_combo_keys`  <sub>(function)</sub>

```python
def Controller.find_combo_keys(self, value):
```

#### `Controller.find_module_keys`  <sub>(function)</sub>

```python
def Controller.find_module_keys(self, value):
```

#### `Controller.find_range_keys`  <sub>(function)</sub>

```python
def Controller.find_range_keys(self, value):
```

#### `Controller.find_prog_keys`  <sub>(function)</sub>

```python
def Controller.find_prog_keys(self, value):
```

#### `Controller.usage_parser`  <sub>(function)</sub>

```python
def Controller.usage_parser(self, name):
```

#### `Controller.parse_usage`  <sub>(function)</sub>

```python
def Controller.parse_usage(self, argv):
```

#### `Controller.__init__`  <sub>(function)</sub>

```python
def Controller.__init__(self, module, argv):
```

#### `Controller.update_actions`  <sub>(function)</sub>

```python
def Controller.update_actions(self, arg):
```

#### `Controller.lookup_actions`  <sub>(function)</sub>

```python
def Controller.lookup_actions(self, resp):
```

#### `Controller.should_free`  <sub>(function)</sub>

```python
def Controller.should_free(self, payload):
```

#### `Controller.register_free`  <sub>(function)</sub>

```python
def Controller.register_free(self, payload, opts):
```

#### `Controller.should_skip`  <sub>(function)</sub>

```python
def Controller.should_skip(self, prod):
```

#### `Controller.register_skip`  <sub>(function)</sub>

```python
def Controller.register_skip(self, prod, opts):
```

#### `Controller.fire`  <sub>(function)</sub>

```python
def Controller.fire(self):
```

#### `Controller.push_final`  <sub>(function)</sub>

```python
def Controller.push_final(self, resp): pass
    def show_final(self): pass
    
    def start_threads(self):
```

#### `Controller.show_final`  <sub>(function)</sub>

```python
def Controller.show_final(self): pass
    
    def start_threads(self):
```

#### `Controller.start_threads`  <sub>(function)</sub>

```python
def Controller.start_threads(self):
```

#### `Controller.produce`  <sub>(function)</sub>

```python
def Controller.produce(self, task_queues, log_queue):
```

#### `Controller.consume`  <sub>(function)</sub>

```python
def Controller.consume(self, task_queue, report_queue, log_queue):
```

#### `Controller.monitor_progress`  <sub>(function)</sub>

```python
def Controller.monitor_progress(self):
```

#### `Controller.report_progress`  <sub>(function)</sub>

```python
def Controller.report_progress(self):
```

#### `Controller.monitor_interaction`  <sub>(function)</sub>

```python
def Controller.monitor_interaction(self):
```

#### `match_range`  <sub>(function)</sub>

```python
def match_range(size, val):
```

#### `Response_Base`  <sub>(class)</sub>

```python
class Response_Base:
```

**Fields:**
```python
available_conditions = ...
indicatorsfmt = ...
def __init__(self, code, mesg, timing, trace)
def indicators(self)
def __str__(self)
def match(self, key, val)
def match_code(self, val)
def match_size(self, val)
def match_time(self, val)
def match_mesg(self, val)
def match_fgrep(self, val)
def match_egrep(self, val)
def dump(self)
def str_target(self)
```


#### `Response_Base.__init__`  <sub>(function)</sub>

```python
def Response_Base.__init__(self, code, mesg, timing=0, trace=None):
```

#### `Response_Base.indicators`  <sub>(function)</sub>

```python
def Response_Base.indicators(self):
```

#### `Response_Base.__str__`  <sub>(function)</sub>

```python
def Response_Base.__str__(self):
```

#### `Response_Base.match`  <sub>(function)</sub>

```python
def Response_Base.match(self, key, val):
```

#### `Response_Base.match_code`  <sub>(function)</sub>

```python
def Response_Base.match_code(self, val):
```

#### `Response_Base.match_size`  <sub>(function)</sub>

```python
def Response_Base.match_size(self, val):
```

#### `Response_Base.match_time`  <sub>(function)</sub>

```python
def Response_Base.match_time(self, val):
```

#### `Response_Base.match_mesg`  <sub>(function)</sub>

```python
def Response_Base.match_mesg(self, val):
```

#### `Response_Base.match_fgrep`  <sub>(function)</sub>

```python
def Response_Base.match_fgrep(self, val):
```

#### `Response_Base.match_egrep`  <sub>(function)</sub>

```python
def Response_Base.match_egrep(self, val):
```

#### `Response_Base.dump`  <sub>(function)</sub>

```python
def Response_Base.dump(self):
```

#### `Response_Base.str_target`  <sub>(function)</sub>

```python
def Response_Base.str_target(self):
```

#### `Timing`  <sub>(class)</sub>

```python
class Timing:
```

**Fields:**
```python
def __enter__(self)
def __exit__(self, exc_type, exc_value, traceback)
```


#### `Timing.__enter__`  <sub>(function)</sub>

```python
def Timing.__enter__(self):
```

#### `Timing.__exit__`  <sub>(function)</sub>

```python
def Timing.__exit__(self, exc_type, exc_value, traceback):
```

#### `TCP_Connection`  <sub>(class)</sub>

```python
class TCP_Connection:
```

**Fields:**
```python
def __init__(self, fp, banner)
def close(self)
```


#### `TCP_Connection.__init__`  <sub>(function)</sub>

```python
def TCP_Connection.__init__(self, fp, banner=None):
```

#### `TCP_Connection.close`  <sub>(function)</sub>

```python
def TCP_Connection.close(self):
```

#### `TCP_Cache`  <sub>(class)</sub>

```python
class TCP_Cache:
```

**Fields:**
```python
available_actions = ...
available_options = ...
def __init__(self)
def __del__(self)
def bind(self, host, port)
def reset(self)
```


#### `TCP_Cache.__init__`  <sub>(function)</sub>

```python
def TCP_Cache.__init__(self):
```

#### `TCP_Cache.__del__`  <sub>(function)</sub>

```python
def TCP_Cache.__del__(self):
```

#### `TCP_Cache.bind`  <sub>(function)</sub>

```python
def TCP_Cache.bind(self, host, port, *args, **kwargs):
```

#### `TCP_Cache.reset`  <sub>(function)</sub>

```python
def TCP_Cache.reset(self, **kwargs):
```

#### `FTP_login`  <sub>(class)</sub>

> Brute-force FTP

```python
class FTP_login(TCP_Cache):
```

**Fields:**
```python
usage_hints = ...
available_options = ...
Response = ...
def connect(self, host, port, tls, timeout)
def execute(self, host, port, tls, user, password, timeout, persistent)
```


#### `FTP_login.connect`  <sub>(function)</sub>

```python
def FTP_login.connect(self, host, port, tls, timeout):
```

#### `FTP_login.execute`  <sub>(function)</sub>

```python
def FTP_login.execute(self, host, port='21', tls='0', user=None, password=None, timeout='10', persistent='1'):
```

#### `load_keyfile`  <sub>(function)</sub>

```python
def load_keyfile(keyfile):
```

#### `SSH_login`  <sub>(class)</sub>

> Brute-force SSH

```python
class SSH_login(TCP_Cache):
```

**Fields:**
```python
usage_hints = ...
available_options = ...
Response = ...
def connect(self, host, port, user)
def execute(self, host, port, user, password, auth_type, keyfile, persistent)
```


#### `SSH_login.connect`  <sub>(function)</sub>

```python
def SSH_login.connect(self, host, port, user):
```

#### `SSH_login.execute`  <sub>(function)</sub>

```python
def SSH_login.execute(self, host, port='22', user=None, password=None, auth_type='password', keyfile=None, persistent='1'):
```

#### `Telnet_login`  <sub>(class)</sub>

> Brute-force Telnet

```python
class Telnet_login(TCP_Cache):
```

**Fields:**
```python
usage_hints = ...
available_options = ...
Response = ...
def connect(self, host, port, timeout)
def execute(self, host, port, inputs, prompt_re, timeout, persistent)
```


#### `Telnet_login.connect`  <sub>(function)</sub>

```python
def Telnet_login.connect(self, host, port, timeout):
```

#### `Telnet_login.execute`  <sub>(function)</sub>

```python
def Telnet_login.execute(self, host, port='23', inputs=None, prompt_re='\\w+:', timeout='20', persistent='0'):
```

#### `SMTP_Base`  <sub>(class)</sub>

```python
class SMTP_Base(TCP_Cache):
```

**Fields:**
```python
available_options = ...
Response = ...
def connect(self, host, port, ssl, helo, starttls, timeout)
```


#### `SMTP_Base.connect`  <sub>(function)</sub>

```python
def SMTP_Base.connect(self, host, port, ssl, helo, starttls, timeout):
```

#### `SMTP_vrfy`  <sub>(class)</sub>

> Enumerate valid users using SMTP VRFY

```python
class SMTP_vrfy(SMTP_Base):
```

**Fields:**
```python
usage_hints = ...
def execute(self, host, port, ssl, helo, starttls, user, timeout, persistent)
```


#### `SMTP_vrfy.execute`  <sub>(function)</sub>

```python
def SMTP_vrfy.execute(self, host, port='', ssl='0', helo='', starttls='0', user=None, timeout='10', persistent='1'):
```

#### `SMTP_rcpt`  <sub>(class)</sub>

> Enumerate valid users using SMTP RCPT TO

```python
class SMTP_rcpt(SMTP_Base):
```

**Fields:**
```python
usage_hints = ...
available_options = ...
def execute(self, host, port, ssl, helo, starttls, mail_from, user, timeout, persistent)
```


#### `SMTP_rcpt.execute`  <sub>(function)</sub>

```python
def SMTP_rcpt.execute(self, host, port='', ssl='0', helo='', starttls='0', mail_from='test@example.org', user=None, timeout='10', persistent='1'):
```

#### `SMTP_login`  <sub>(class)</sub>

> Brute-force SMTP

```python
class SMTP_login(SMTP_Base):
```

**Fields:**
```python
usage_hints = ...
available_options = ...
def execute(self, host, port, ssl, helo, starttls, user, password, timeout, persistent)
```


#### `SMTP_login.execute`  <sub>(function)</sub>

```python
def SMTP_login.execute(self, host, port='', ssl='0', helo='', starttls='0', user=None, password=None, timeout='10', persistent='1'):
```

#### `Controller_Finger`  <sub>(class)</sub>

```python
class Controller_Finger(Controller):
```

**Fields:**
```python
user_list = ...
def push_final(self, resp)
def show_final(self)
```


#### `Controller_Finger.push_final`  <sub>(function)</sub>

```python
def Controller_Finger.push_final(self, resp):
```

#### `Controller_Finger.show_final`  <sub>(function)</sub>

```python
def Controller_Finger.show_final(self):
```

#### `Finger_lookup`  <sub>(class)</sub>

> Enumerate valid users using Finger

```python
class Finger_lookup:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, port, user, timeout)
```


#### `Finger_lookup.execute`  <sub>(function)</sub>

```python
def Finger_lookup.execute(self, host, port='79', user='', timeout='5'):
```

#### `DCOM_login`  <sub>(class)</sub>

> Brute-force DCOM

```python
class DCOM_login:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, port, uuid, ver, opnum, syntax, domain, user, password, lmhash, nthash, aeskey, tgt, tgs, authtype, authlevel)
```


#### `DCOM_login.execute`  <sub>(function)</sub>

```python
def DCOM_login.execute(self, host, port, uuid, ver, opnum='0', syntax='8a885d04-1ceb-11c9-9fe8-08002b104860:2.0', domain='', user='', password='', lmhash='', nthash='', aeskey=None, tgt=None, tgs=None, authtype=None, authlevel=None):
```

#### `LDAP_login`  <sub>(class)</sub>

> Brute-force LDAP

```python
class LDAP_login:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, port, binddn, bindpw, basedn, ssl)
```


#### `LDAP_login.execute`  <sub>(function)</sub>

```python
def LDAP_login.execute(self, host, port='389', binddn='', bindpw='', basedn='', ssl='0'):
```

#### `SMB_Connection`  <sub>(class)</sub>

```python
class SMB_Connection(TCP_Connection):
```

**Fields:**
```python
def close(self)
```


#### `SMB_Connection.close`  <sub>(function)</sub>

```python
def SMB_Connection.close(self):
```

#### `Response_SMB`  <sub>(class)</sub>

```python
class Response_SMB(Response_Base):
```

**Fields:**
```python
indicatorsfmt = ...
```


#### `split_ntlm`  <sub>(function)</sub>

```python
def split_ntlm(password_hash):
```

#### `SMB_login`  <sub>(class)</sub>

> Brute-force SMB

```python
class SMB_login(TCP_Cache):
```

**Fields:**
```python
usage_hints = ...
available_options = ...
Response = ...
def connect(self, host, port)
def execute(self, host, port, user, password, password_hash, domain, persistent)
```


#### `SMB_login.connect`  <sub>(function)</sub>

```python
def SMB_login.connect(self, host, port):
```

#### `SMB_login.execute`  <sub>(function)</sub>

```python
def SMB_login.execute(self, host, port='139', user=None, password='', password_hash=None, domain='', persistent='1'):
```

#### `DCE_Connection`  <sub>(class)</sub>

```python
class DCE_Connection(TCP_Connection):
```

**Fields:**
```python
def __init__(self, fp, rpct)
def close(self)
```


#### `DCE_Connection.__init__`  <sub>(function)</sub>

```python
def DCE_Connection.__init__(self, fp, rpct):
```

#### `DCE_Connection.close`  <sub>(function)</sub>

```python
def DCE_Connection.close(self):
```

#### `SMB_lookupsid`  <sub>(class)</sub>

> Brute-force SMB SID-lookup

```python
class SMB_lookupsid(TCP_Cache):
```

**Fields:**
```python
usage_hints = ...
available_options = ...
Response = ...
def connect(self, host, port, user, password, domain, password_hash, sid)
def execute(self, host, port, user, password, password_hash, domain, sid, rid, persistent)
```


#### `SMB_lookupsid.connect`  <sub>(function)</sub>

```python
def SMB_lookupsid.connect(self, host, port, user, password, domain, password_hash, sid):
```

#### `SMB_lookupsid.execute`  <sub>(function)</sub>

```python
def SMB_lookupsid.execute(self, host, port='445', user='', password='', password_hash=None, domain='', sid=None, rid=None, persistent='1'):
```

#### `POP_Connection`  <sub>(class)</sub>

```python
class POP_Connection(TCP_Connection):
```

**Fields:**
```python
def close(self)
```


#### `POP_Connection.close`  <sub>(function)</sub>

```python
def POP_Connection.close(self):
```

#### `POP_login`  <sub>(class)</sub>

> Brute-force POP3

```python
class POP_login(TCP_Cache):
```

**Fields:**
```python
usage_hints = ...
available_options = ...
Response = ...
def connect(self, host, port, ssl, timeout)
def execute(self, host, port, ssl, user, password, timeout, persistent)
```


#### `POP_login.connect`  <sub>(function)</sub>

```python
def POP_login.connect(self, host, port, ssl, timeout):
```

#### `POP_login.execute`  <sub>(function)</sub>

```python
def POP_login.execute(self, host, port='', ssl='0', user=None, password=None, timeout='10', persistent='1'):
```

#### `POP_passd`  <sub>(class)</sub>

> Brute-force poppassd (http://netwinsite.com/poppassd/)

```python
class POP_passd:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, port, user, password, timeout)
```


#### `POP_passd.execute`  <sub>(function)</sub>

```python
def POP_passd.execute(self, host, port='106', user=None, password=None, timeout='10'):
```

#### `IMAP_login`  <sub>(class)</sub>

> Brute-force IMAP4

```python
class IMAP_login:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, port, ssl, user, password)
```


#### `IMAP_login.execute`  <sub>(function)</sub>

```python
def IMAP_login.execute(self, host, port='', ssl='0', user=None, password=None):
```

#### `Rlogin_login`  <sub>(class)</sub>

> Brute-force rlogin

```python
class Rlogin_login(TCP_Cache):
```

**Fields:**
```python
usage_hints = ...
available_options = ...
Response = ...
def connect(self, host, port, timeout)
def execute(self, host, port, luser, user, password, prompt_re, timeout, persistent)
```


#### `Rlogin_login.connect`  <sub>(function)</sub>

```python
def Rlogin_login.connect(self, host, port, timeout):
```

#### `Rlogin_login.execute`  <sub>(function)</sub>

```python
def Rlogin_login.execute(self, host, port='513', luser='root', user='', password=None, prompt_re='\\w+:', timeout='10', persistent='0'):
```

#### `LineReceiver_Error`  <sub>(class)</sub>

```python
class LineReceiver_Error(Exception):
```

#### `LineReceiver`  <sub>(class)</sub>

```python
class LineReceiver:
```

**Fields:**
```python
def connect(self, host, port, timeout, ssl)
def close(self)
def sendcmd(self, cmd)
def getresp(self)
def parse(self, resp)
```


#### `LineReceiver.connect`  <sub>(function)</sub>

```python
def LineReceiver.connect(self, host, port, timeout, ssl=False):
```

#### `LineReceiver.close`  <sub>(function)</sub>

```python
def LineReceiver.close(self):
```

#### `LineReceiver.sendcmd`  <sub>(function)</sub>

```python
def LineReceiver.sendcmd(self, cmd):
```

#### `LineReceiver.getresp`  <sub>(function)</sub>

```python
def LineReceiver.getresp(self):
```

#### `LineReceiver.parse`  <sub>(function)</sub>

```python
def LineReceiver.parse(self, resp):
```

#### `VMauthd_login`  <sub>(class)</sub>

> Brute-force VMware Authentication Daemon

```python
class VMauthd_login(TCP_Cache):
```

**Fields:**
```python
usage_hints = ...
available_options = ...
Response = ...
def connect(self, host, port, ssl, timeout)
def execute(self, host, port, user, password, ssl, timeout, persistent)
```


#### `VMauthd_login.connect`  <sub>(function)</sub>

```python
def VMauthd_login.connect(self, host, port, ssl, timeout):
```

#### `VMauthd_login.execute`  <sub>(function)</sub>

```python
def VMauthd_login.execute(self, host, port='902', user=None, password=None, ssl='1', timeout='10', persistent='1'):
```

#### `MySQL_login`  <sub>(class)</sub>

> Brute-force MySQL

```python
class MySQL_login:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, port, user, password, timeout)
```


#### `MySQL_login.execute`  <sub>(function)</sub>

```python
def MySQL_login.execute(self, host, port='3306', user='anony', password='', timeout='10'):
```

#### `MySQL_query`  <sub>(class)</sub>

> Brute-force MySQL queries

```python
class MySQL_query(TCP_Cache):
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def connect(self, host, port, user, password)
def execute(self, host, port, user, password, query)
```


#### `MySQL_query.connect`  <sub>(function)</sub>

```python
def MySQL_query.connect(self, host, port, user, password):
```

#### `MySQL_query.execute`  <sub>(function)</sub>

```python
def MySQL_query.execute(self, host, port='3306', user='', password='', query='select @@version'):
```

#### `MSSQL_login`  <sub>(class)</sub>

> Brute-force MSSQL

```python
class MSSQL_login:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, port, user, password, windows_auth, domain, password_hash)
```


#### `MSSQL_login.execute`  <sub>(function)</sub>

```python
def MSSQL_login.execute(self, host, port='1433', user='', password='', windows_auth='0', domain='', password_hash=None): #, timeout='10'):
```

#### `Response_Oracle`  <sub>(class)</sub>

```python
class Response_Oracle(Response_Base):
```

**Fields:**
```python
indicatorsfmt = ...
```


#### `Oracle_login`  <sub>(class)</sub>

> Brute-force Oracle

```python
class Oracle_login:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, port, user, password, sid, service_name)
```


#### `Oracle_login.execute`  <sub>(function)</sub>

```python
def Oracle_login.execute(self, host, port='1521', user='', password='', sid='', service_name=''):
```

#### `Pgsql_login`  <sub>(class)</sub>

> Brute-force PostgreSQL

```python
class Pgsql_login:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, port, user, password, database, ssl, timeout)
```


#### `Pgsql_login.execute`  <sub>(function)</sub>

```python
def Pgsql_login.execute(self, host, port='5432', user=None, password=None, database='postgres', ssl='disable', timeout='10'):
```

#### `Response_HTTP`  <sub>(class)</sub>

```python
class Response_HTTP(Response_Base):
```

**Fields:**
```python
indicatorsfmt = ...
def __init__(self, code, response, timing, trace, content_length, target)
def indicators(self)
def __str__(self)
def match_clen(self, val)
def match_egrep(self, val)
def str_target(self)
available_conditions = ...
```


#### `Response_HTTP.__init__`  <sub>(function)</sub>

```python
def Response_HTTP.__init__(self, code, response, timing=0, trace=None, content_length=-1, target={}):
```

#### `Response_HTTP.indicators`  <sub>(function)</sub>

```python
def Response_HTTP.indicators(self):
```

#### `Response_HTTP.__str__`  <sub>(function)</sub>

```python
def Response_HTTP.__str__(self):
```

#### `Response_HTTP.match_clen`  <sub>(function)</sub>

```python
def Response_HTTP.match_clen(self, val):
```

#### `Response_HTTP.match_egrep`  <sub>(function)</sub>

```python
def Response_HTTP.match_egrep(self, val):
```

#### `Response_HTTP.str_target`  <sub>(function)</sub>

```python
def Response_HTTP.str_target(self):
```

#### `HTTPRequestParser`  <sub>(class)</sub>

```python
class HTTPRequestParser(BaseHTTPRequestHandler):
```

**Fields:**
```python
def __init__(self, fd)
def send_error(self, code, message)
```


#### `HTTPRequestParser.__init__`  <sub>(function)</sub>

```python
def HTTPRequestParser.__init__(self, fd):
```

#### `HTTPRequestParser.send_error`  <sub>(function)</sub>

```python
def HTTPRequestParser.send_error(self, code, message):
```

#### `Controller_HTTP`  <sub>(class)</sub>

```python
class Controller_HTTP(Controller):
```

**Fields:**
```python
def expand_key(self, arg)
```


#### `Controller_HTTP.expand_key`  <sub>(function)</sub>

```python
def Controller_HTTP.expand_key(self, arg):
```

#### `HTTP_fuzz`  <sub>(class)</sub>

> Brute-force HTTP

```python
class HTTP_fuzz(TCP_Cache):
```

**Fields:**
```python
usage_hints = ...
available_options = ...
Response = ...
def connect(self, host, port, scheme)
def perform_fp(fp, method, url, header, body)
def execute(self, url, host, port, scheme, path, params, query, fragment, body, header, method, auto_urlencode, pathasis, user_pass, auth_type, follow, max_follow, accept_cookie, proxy, proxy_type, resolve, ssl_cert, timeout_tcp, timeout, persistent, before_urls, before_header, before_egrep, after_urls, max_mem)
```


#### `HTTP_fuzz.connect`  <sub>(function)</sub>

```python
def HTTP_fuzz.connect(self, host, port, scheme):
```

#### `HTTP_fuzz.perform_fp`  <sub>(function)</sub>

```python
def HTTP_fuzz.perform_fp(fp, method, url, header='', body=''):
```

#### `HTTP_fuzz.execute`  <sub>(function)</sub>

```python
def HTTP_fuzz.execute(self, url=None, host=None, port='', scheme='http', path='/', params='', query='', fragment='', body='',
      header='', method='GET', auto_urlencode='1', pathasis='0', user_pass='', auth_type='basic',
      follow='0', max_follow='5', accept_cookie='0', proxy='', proxy_type='http', resolve='', ssl_cert='', timeout_tcp='10', timeout='20', persistent='1',
      before_urls='', before_header='', before_egrep='', after_urls='', max_mem='-1'):
```

#### `RDP_gateway`  <sub>(class)</sub>

> Brute-force RDP Gateway

```python
class RDP_gateway(HTTP_fuzz):
```

**Fields:**
```python
usage_hints = ...
def perform_fp(fp, method, url, header, body)
```


#### `RDP_gateway.perform_fp`  <sub>(function)</sub>

```python
def RDP_gateway.perform_fp(fp, method, url, header='', body=''):
```

#### `AJP_Connection`  <sub>(class)</sub>

```python
class AJP_Connection(TCP_Connection):
```

**Fields:**
```python
def close(self)
```


#### `AJP_Connection.close`  <sub>(function)</sub>

```python
def AJP_Connection.close(self):
```

#### `Response_AJP`  <sub>(class)</sub>

```python
class Response_AJP(Response_HTTP):
```

**Fields:**
```python
def __init__(self, code, response, timing, trace, content_length, target)
def __str__(self)
```


#### `Response_AJP.__init__`  <sub>(function)</sub>

```python
def Response_AJP.__init__(self, code, response, timing=0, trace=None, content_length=-1, target={}):
```

#### `Response_AJP.__str__`  <sub>(function)</sub>

```python
def Response_AJP.__str__(self):
```

#### `prepare_ajp_forward_request`  <sub>(function)</sub>

```python
def prepare_ajp_forward_request(target_host, req_uri, method):
```

#### `AJP_fuzz`  <sub>(class)</sub>

> Brute-force AJP

```python
class AJP_fuzz(TCP_Cache):
```

**Fields:**
```python
usage_hints = ...
available_options = ...
Response = ...
def connect(self, host, port)
def execute(self, url, host, port, path, params, query, header, user_pass, persistent)
```


#### `AJP_fuzz.connect`  <sub>(function)</sub>

```python
def AJP_fuzz.connect(self, host, port):
```

#### `AJP_fuzz.execute`  <sub>(function)</sub>

```python
def AJP_fuzz.execute(self, url=None, host=None, port='8009', path='/', params='', query='', header='', user_pass='', persistent='1'):
```

#### `RDP_login`  <sub>(class)</sub>

> Brute-force RDP (NLA)

```python
class RDP_login:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, port, user, password)
```


#### `RDP_login.execute`  <sub>(function)</sub>

```python
def RDP_login.execute(self, host, port='3389', user=None, password=None):
```

#### `VNC_Error`  <sub>(class)</sub>

```python
class VNC_Error(Exception):
```

#### `VNC`  <sub>(class)</sub>

```python
class VNC:
```

**Fields:**
```python
def connect(self, host, port, timeout)
def login(self, password)
def gen_key(self, key)
```


#### `VNC.connect`  <sub>(function)</sub>

```python
def VNC.connect(self, host, port, timeout):
```

#### `VNC.login`  <sub>(function)</sub>

```python
def VNC.login(self, password):
```

#### `VNC.gen_key`  <sub>(function)</sub>

```python
def VNC.gen_key(self, key):
```

#### `VNC_login`  <sub>(class)</sub>

> Brute-force VNC

```python
class VNC_login:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, port, password, timeout)
```


#### `VNC_login.execute`  <sub>(function)</sub>

```python
def VNC_login.execute(self, host, port=None, password=None, timeout='10'):
```

#### `dns_query`  <sub>(function)</sub>

```python
def dns_query(server, timeout, protocol, qname, qtype, qclass):
```

#### `generate_tld`  <sub>(function)</sub>

```python
def generate_tld():
```

#### `generate_srv`  <sub>(function)</sub>

```python
def generate_srv():
```

#### `HostInfo`  <sub>(class)</sub>

```python
class HostInfo:
```

**Fields:**
```python
def __init__(self)
def __str__(self)
```


#### `HostInfo.__init__`  <sub>(function)</sub>

```python
def HostInfo.__init__(self):
```

#### `HostInfo.__str__`  <sub>(function)</sub>

```python
def HostInfo.__str__(self):
```

#### `Controller_DNS`  <sub>(class)</sub>

```python
class Controller_DNS(Controller):
```

**Fields:**
```python
records = ...
hostmap = ...
def show_final(self)
def push_final(self, resp)
```


#### `Controller_DNS.show_final`  <sub>(function)</sub>

> Expected output:  
> Records -----  
> ftp.example.com.   IN A       10.0.1.1  
> www.example.com.   IN A       10.0.1.1  
> prod.example.com.   IN CNAME   www.example.com.  
> ipv6.example.com.   IN AAAA    dead:beef::  
> dev.example.com.   IN A       10.0.1.2  
> svn.example.com.   IN A       10.0.2.1  
> websrv1.example.com.   IN CNAME   prod.example.com.  
> blog.example.com.   IN CNAME   example.wordpress.com.

```python
def Controller_DNS.show_final(self):
```

#### `Controller_DNS.push_final`  <sub>(function)</sub>

```python
def Controller_DNS.push_final(self, resp):
```

#### `DNS_reverse`  <sub>(class)</sub>

> Reverse DNS lookup

```python
class DNS_reverse:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, server, timeout, protocol)
```


#### `DNS_reverse.execute`  <sub>(function)</sub>

```python
def DNS_reverse.execute(self, host, server='8.8.8.8', timeout='5', protocol='udp'):
```

#### `DNS_forward`  <sub>(class)</sub>

> Forward DNS lookup

```python
class DNS_forward:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
available_keys = ...
Response = ...
def execute(self, name, server, timeout, protocol, qtype, qclass)
```


#### `DNS_forward.execute`  <sub>(function)</sub>

```python
def DNS_forward.execute(self, name, server='8.8.8.8', timeout='5', protocol='udp', qtype='ANY', qclass='IN'):
```

#### `SNMP_login`  <sub>(class)</sub>

> Brute-force SNMP v1/2/3

```python
class SNMP_login:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, port, version, community, user, auth_key, auth_proto, priv_key, priv_proto, timeout, retries)
def async_cmd(self, host, port, security_model, timeout, retries)
```


#### `SNMP_login.execute`  <sub>(function)</sub>

```python
def SNMP_login.execute(self, host, port=None, version='2', community='public', user='op5user', auth_key='authPass', auth_proto='md5', priv_key='', priv_proto='des', timeout='1', retries='2'):
```

#### `SNMP_login.async_cmd`  <sub>(function)</sub>

```python
async def SNMP_login.async_cmd(self, host, port, security_model, timeout, retries):
```

#### `IKE_ENC`  <sub>(const)</sub>

```python
IKE_ENC = ...
```

#### `IKE_HASH`  <sub>(const)</sub>

```python
IKE_HASH = ...
```

#### `IKE_AUTH`  <sub>(const)</sub>

```python
IKE_AUTH = ...
```

#### `IKE_GROUP`  <sub>(const)</sub>

```python
IKE_GROUP = ...
```

#### `generate_transforms`  <sub>(function)</sub>

```python
def generate_transforms():
```

#### `Controller_IKE`  <sub>(class)</sub>

```python
class Controller_IKE(Controller):
```

**Fields:**
```python
results = ...
def show_final(self)
def push_final(self, resp)
```


#### `Controller_IKE.show_final`  <sub>(function)</sub>

> Expected output:  
> + 10.0.0.1:500 (Main Mode)  
> Encryption       Hash         Auth      Group  
> ---------- ----------   ---------- ----------  
> 3DES        MD5          PSK   modp1024  
> 3DES        MD5        XAUTH   modp1024  
> AES128       SHA1          PSK   modp1024  
> AES128       SHA1        XAUTH   modp1024  
>   
> + 10.0.0.1:500 (Aggressive Mode)  
> Encryption       Hash         Auth      Group  
> ---------- ----------   ---------- ----------  
> 3DES        MD5          PSK   modp1024  
> 3DES        MD5        XAUTH   modp1024  
> AES128       SHA1          PSK   modp1024  
> AES128       SHA1        XAUTH   modp1024

```python
def Controller_IKE.show_final(self):
```

#### `Controller_IKE.push_final`  <sub>(function)</sub>

```python
def Controller_IKE.push_final(self, resp):
```

#### `IKE_enum`  <sub>(class)</sub>

> Enumerate IKE transforms

```python
class IKE_enum:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
available_keys = ...
Response = ...
def __init__(self)
def execute(self, host, port, transform, aggressive, groupname, vid)
```


#### `IKE_enum.__init__`  <sub>(function)</sub>

```python
def IKE_enum.__init__(self):
```

#### `IKE_enum.execute`  <sub>(function)</sub>

```python
def IKE_enum.execute(self, host, port='500', transform='5,1,1,2', aggressive='0', groupname='foo', vid=''):
```

#### `Unzip_pass`  <sub>(class)</sub>

> Brute-force the password of encrypted ZIP files

```python
class Unzip_pass:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, zipfile, password)
```


#### `Unzip_pass.execute`  <sub>(function)</sub>

```python
def Unzip_pass.execute(self, zipfile, password):
```

#### `Keystore_pass`  <sub>(class)</sub>

> Brute-force the password of Java keystore files

```python
class Keystore_pass:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, keystore, password, storetype)
```


#### `Keystore_pass.execute`  <sub>(function)</sub>

```python
def Keystore_pass.execute(self, keystore, password, storetype='jks'):
```

#### `SQLCipher_pass`  <sub>(class)</sub>

> Brute-force the password of SQLCipher-encrypted databases

```python
class SQLCipher_pass:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, database, password)
```


#### `SQLCipher_pass.execute`  <sub>(function)</sub>

```python
def SQLCipher_pass.execute(self, database, password):
```

#### `Umbraco_crack`  <sub>(class)</sub>

> Crack Umbraco HMAC-SHA1 password hashes

```python
class Umbraco_crack:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, password, hashlist)
```


#### `Umbraco_crack.execute`  <sub>(function)</sub>

```python
def Umbraco_crack.execute(self, password, hashlist):
```

#### `TCP_fuzz`  <sub>(class)</sub>

> Fuzz TCP services

```python
class TCP_fuzz:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
Response = ...
def execute(self, host, port, data, timeout, ssl)
```


#### `TCP_fuzz.execute`  <sub>(function)</sub>

```python
def TCP_fuzz.execute(self, host, port, data='', timeout='2', ssl='0'):
```

#### `generate_tst`  <sub>(function)</sub>

```python
def generate_tst():
```

#### `Dummy_test`  <sub>(class)</sub>

> Testing module

```python
class Dummy_test:
```

**Fields:**
```python
usage_hints = ...
available_options = ...
available_actions = ...
available_keys = ...
Response = ...
def execute(self, data, data2, delay)
```


#### `Dummy_test.execute`  <sub>(function)</sub>

```python
def Dummy_test.execute(self, data, data2='', delay='1'):
```

#### `show_usage`  <sub>(function)</sub>

```python
def show_usage():
```

#### `cli`  <sub>(function)</sub>

```python
def cli():
```
