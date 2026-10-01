# dirsearch — Complete API Reference

Exhaustive per-item reference for **every** type, function, method, constant, and macro in the dirsearch source (`maurosoria/dirsearch` @ `fbf5da9`, v0.4.3). Language: **PY**. Generated from a full source read; test files are excluded. Each item shows its full signature and doc comment; composite types list their fields/members.

**958 documented items** across 76 source files.


---

## Modules / packages

- [`(root)`](#pkg--root-) — 8 items
- [`db/categories`](#pkg-db-categories) — 8 items
- [`lib`](#pkg-lib) — 0 items
- [`lib/connection`](#pkg-lib-connection) — 175 items
- [`lib/controller`](#pkg-lib-controller) — 74 items
- [`lib/core`](#pkg-lib-core) — 323 items
- [`lib/parse`](#pkg-lib-parse) — 40 items
- [`lib/report`](#pkg-lib-report) — 126 items
- [`lib/utils`](#pkg-lib-utils) — 118 items
- [`lib/view`](#pkg-lib-view) — 38 items
- [`scripts`](#pkg-scripts) — 48 items

---


## `(root)` <a id='pkg--root-'></a>

*8 documented items*


### `dirsearch/dirsearch.py`


#### `main`  <sub>(function)</sub>

```python
def main():
```

### `dirsearch/setup.py`


#### `ROOT`  <sub>(const)</sub>

```python
ROOT = ...
```

#### `PACKAGE_DATA_EXCLUDED_NAMES`  <sub>(const)</sub>

```python
PACKAGE_DATA_EXCLUDED_NAMES = ...
```

#### `PACKAGE_DATA_EXCLUDED_SUFFIXES`  <sub>(const)</sub>

```python
PACKAGE_DATA_EXCLUDED_SUFFIXES = ...
```

#### `package_files`  <sub>(function)</sub>

```python
def package_files(directory: Path) -> list[str]:
```

#### `package_names`  <sub>(function)</sub>

```python
def package_names() -> list[str]:
```

#### `read_version`  <sub>(function)</sub>

```python
def read_version(path: Path) -> str:
```

#### `read_requirements`  <sub>(function)</sub>

```python
def read_requirements(path: Path) -> list[str]:
```


## `db/categories` <a id='pkg-db-categories'></a>

*8 documented items*


### `dirsearch/db/categories/generate_wpscan_wordlists.py`


#### `CATEGORIES_DIR`  <sub>(const)</sub>

```python
CATEGORIES_DIR = ...
```

#### `PLUGINS_FULL_PATH`  <sub>(const)</sub>

```python
PLUGINS_FULL_PATH = ...
```

#### `PLUGINS_VULN_PATH`  <sub>(const)</sub>

```python
PLUGINS_VULN_PATH = ...
```

#### `fetch_popular_plugins`  <sub>(function)</sub>

> Fetches a list of popular WordPress plugins.  
> Since WPScan API requires a token, we use a fallback method or a public list for demonstration.  
> Ideally, you would use: https://enterprise-data.wpscan.com/plugins.json.gz (Auth required)  
>   
> Here we mock it by fetching a known large list or scraping a popular list if possible.  
> For this script, we will use a static list combined with a fetch from a public wordlist repo.

```python
def fetch_popular_plugins():
```

#### `generate_wordlists`  <sub>(function)</sub>

```python
def generate_wordlists(plugins):
```

#### `main`  <sub>(function)</sub>

```python
def main():
```

### `dirsearch/db/categories/php/generate_wpscan_wordlists.py`


#### `download_file`  <sub>(function)</sub>

```python
def download_file(url, output_path):
```

#### `main`  <sub>(function)</sub>

```python
def main():
```


## `lib` <a id='pkg-lib'></a>

*0 documented items*



## `lib/connection` <a id='pkg-lib-connection'></a>

*175 documented items*


### `dirsearch/lib/connection/ip_overrides.py`


#### `IPOverrides`  <sub>(class)</sub>

> Requester-owned connection IPs configured by the --ip option.  
>   
> This mapping does not resolve DNS or collect a hostname's A/AAAA records.  
> Without an override, the HTTP transport resolves the hostname normally.  
> The controller configures overrides before workers start, and workers only  
> read the mapping, so the connection hot path needs no synchronization.

```python
class IPOverrides:
```

**Fields:**
```python
def __init__(self)
def _key(host, port)
def set_override(self, host, port, ip_address)
def get_override(self, host, port)
def connection_overrides(self)
```


#### `IPOverrides.__init__`  <sub>(function)</sub>

```python
def IPOverrides.__init__(self) -> None:
```

#### `IPOverrides._key`  <sub>(function)</sub>

```python
def IPOverrides._key(host: str, port: int) -> tuple[str, int]:
```

#### `IPOverrides.set_override`  <sub>(function)</sub>

```python
def IPOverrides.set_override(self, host: str, port: int, ip_address: str) -> None:
```

#### `IPOverrides.get_override`  <sub>(function)</sub>

> Return one forced connection IP, or None for normal DNS lookup.

```python
def IPOverrides.get_override(self, host: str, port: int) -> str | None:
```

#### `IPOverrides.connection_overrides`  <sub>(function)</sub>

> Return a stable snapshot suitable for configuring a transport.

```python
def IPOverrides.connection_overrides(self) -> list[tuple[str, int, str]]:
```

### `dirsearch/lib/connection/native.py`


#### `NativeScanEvent`  <sub>(class)</sub>

> One actionable Rust result, indexed into the original path chunk.

```python
class NativeScanEvent:
```

**Fields:**
```python
request_index: ...
path: ...
response: ...
error: ...
```


#### `NativeScanChunk`  <sub>(class)</sub>

> Actionable results from the completed range [start_index, end_index).

```python
class NativeScanChunk:
```

**Fields:**
```python
start_index: ...
end_index: ...
events: ...
```


#### `NativeHTTPBackend`  <sub>(class)</sub>

```python
class NativeHTTPBackend:
```

**Fields:**
```python
def __init__(self, proxy_override, session, auth_override, ip_overrides)
def _get_engine(self)
def session(self)
def rate(self)
def set_origin_authentication(self, auth_type, credential)
def _request_body()
def _load_client_identity()
def _load_random_user_agents()
def cancel(self)
def reset_cancel(self)
def close(self)
def scan(self, base_url, paths, query)
def scan_chunks(self, base_url, paths, callback, query)
def _make_scan_chunk(self, base_url, raw_paths, results)
def scan_unfiltered(self, base_url, path, query)
def _scan(self, base_url, paths, query)
def _convert_result(self, base_url, result)
def _proxy_urls()
def _normalize_proxy_urls(proxy_values)
def _using_proxy(self)
def _filter_options()
def _get_filter_config(self, apply_filters)
```


#### `NativeHTTPBackend.__init__`  <sub>(function)</sub>

```python
def NativeHTTPBackend.__init__(
        self,
        proxy_override: str | None = None,
        session: Any | None = None,
        auth_override: tuple[str, str] | None = None,
        ip_overrides: IPOverrides | None = None,
) -> None:
```

#### `NativeHTTPBackend._get_engine`  <sub>(function)</sub>

```python
def NativeHTTPBackend._get_engine(self):
```

#### `NativeHTTPBackend.session`  <sub>(function)</sub>

> Opaque Rust state shared with engines created for replay requests.

```python
def NativeHTTPBackend.session(self) -> Any:
```

#### `NativeHTTPBackend.rate`  <sub>(function)</sub>

```python
def NativeHTTPBackend.rate(self) -> int:
```

#### `NativeHTTPBackend.set_origin_authentication`  <sub>(function)</sub>

```python
def NativeHTTPBackend.set_origin_authentication(
        self, auth_type: str, credential: str
) -> None:
```

#### `NativeHTTPBackend._request_body`  <sub>(function)</sub>

```python
def NativeHTTPBackend._request_body() -> bytes:
```

#### `NativeHTTPBackend._load_client_identity`  <sub>(function)</sub>

```python
def NativeHTTPBackend._load_client_identity() -> tuple[bytes, bytes]:
```

#### `NativeHTTPBackend._load_random_user_agents`  <sub>(function)</sub>

```python
def NativeHTTPBackend._load_random_user_agents() -> list[str]:
```

#### `NativeHTTPBackend.cancel`  <sub>(function)</sub>

```python
def NativeHTTPBackend.cancel(self) -> None:
```

#### `NativeHTTPBackend.reset_cancel`  <sub>(function)</sub>

```python
def NativeHTTPBackend.reset_cancel(self) -> None:
```

#### `NativeHTTPBackend.close`  <sub>(function)</sub>

> Cancel active work and release the persistent Rust runtime and clients.

```python
def NativeHTTPBackend.close(self) -> None:
```

#### `NativeHTTPBackend.scan`  <sub>(function)</sub>

```python
def NativeHTTPBackend.scan(
        self,
        base_url: str,
        paths: Iterable[str],
        query: str = "",
) -> Iterator[tuple[str, NativeResponse | None, RequestException | None]]:
```

#### `NativeHTTPBackend.scan_chunks`  <sub>(function)</sub>

> Deliver ordered compact ranges while Rust is still scanning.

```python
def NativeHTTPBackend.scan_chunks(
        self,
        base_url: str,
        paths: list[str] | NativeWordlistChunk,
        callback: Callable[[NativeScanChunk], Any],
        query: str = "",
) -> int:
```

#### `NativeHTTPBackend._make_scan_chunk`  <sub>(function)</sub>

```python
def NativeHTTPBackend._make_scan_chunk(
        self,
        base_url: str,
        raw_paths: list[str] | NativeWordlistChunk,
        results: Iterable[Any],
        *,
        start_index: int,
        end_index: int,
) -> NativeScanChunk:
```

#### `NativeHTTPBackend.scan_unfiltered`  <sub>(function)</sub>

```python
def NativeHTTPBackend.scan_unfiltered(
        self,
        base_url: str,
        path: str,
        query: str = "",
) -> tuple[NativeResponse | None, RequestException | None]:
```

#### `NativeHTTPBackend._scan`  <sub>(function)</sub>

```python
def NativeHTTPBackend._scan(
        self,
        base_url: str,
        paths: Iterable[str] | NativeWordlistChunk,
        query: str,
        *,
        apply_filters: bool = True,
) -> tuple[list[str], list[Any]]:
```

#### `NativeHTTPBackend._convert_result`  <sub>(function)</sub>

```python
def NativeHTTPBackend._convert_result(
        self,
        base_url: str,
        result: Any,
) -> tuple[NativeResponse | None, RequestException | None]:
```

#### `NativeHTTPBackend._proxy_urls`  <sub>(function)</sub>

```python
def NativeHTTPBackend._proxy_urls() -> list[str]:
```

#### `NativeHTTPBackend._normalize_proxy_urls`  <sub>(function)</sub>

```python
def NativeHTTPBackend._normalize_proxy_urls(proxy_values: Iterable[str]) -> list[str]:
```

#### `NativeHTTPBackend._using_proxy`  <sub>(function)</sub>

```python
def NativeHTTPBackend._using_proxy(self) -> bool:
```

#### `NativeHTTPBackend._filter_options`  <sub>(function)</sub>

```python
def NativeHTTPBackend._filter_options() -> dict[str, Any]:
```

#### `NativeHTTPBackend._get_filter_config`  <sub>(function)</sub>

```python
def NativeHTTPBackend._get_filter_config(self, apply_filters: bool):
```

#### `NativeRequester`  <sub>(class)</sub>

> Minimal requester facade used by native scans and calibration.

```python
class NativeRequester:
```

**Fields:**
```python
def __init__(self)
def get_backend(self)
def rate(self)
def set_url(self, url)
def set_query(self, query)
def set_ip(self, host, port, ip_address)
def reset_auth(self)
def set_auth(self, auth_type, credential)
def _set_origin_authentication(self, auth_type, credential)
def request(self, path, proxy)
def close(self)
```


#### `NativeRequester.__init__`  <sub>(function)</sub>

```python
def NativeRequester.__init__(self) -> None:
```

#### `NativeRequester.get_backend`  <sub>(function)</sub>

```python
def NativeRequester.get_backend(self) -> NativeHTTPBackend:
```

#### `NativeRequester.rate`  <sub>(function)</sub>

```python
def NativeRequester.rate(self) -> int:
```

#### `NativeRequester.set_url`  <sub>(function)</sub>

```python
def NativeRequester.set_url(self, url: str) -> None:
```

#### `NativeRequester.set_query`  <sub>(function)</sub>

```python
def NativeRequester.set_query(self, query: str) -> None:
```

#### `NativeRequester.set_ip`  <sub>(function)</sub>

> Force a connection IP while preserving the target Host and SNI.

```python
def NativeRequester.set_ip(self, host: str, port: int, ip_address: str) -> None:
```

#### `NativeRequester.reset_auth`  <sub>(function)</sub>

```python
def NativeRequester.reset_auth(self) -> None:
```

#### `NativeRequester.set_auth`  <sub>(function)</sub>

```python
def NativeRequester.set_auth(self, auth_type: str, credential: str) -> None:
```

#### `NativeRequester._set_origin_authentication`  <sub>(function)</sub>

```python
def NativeRequester._set_origin_authentication(
        self, auth_type: str, credential: str
) -> None:
```

#### `NativeRequester.request`  <sub>(function)</sub>

```python
def NativeRequester.request(self, path: str, proxy: str | None = None) -> NativeResponse:
```

#### `NativeRequester.close`  <sub>(function)</sub>

```python
def NativeRequester.close(self) -> None:
```

### `dirsearch/lib/connection/proxy.py`


#### `PROXY_AUTHENTICATION_REQUIRED`  <sub>(const)</sub>

```python
PROXY_AUTHENTICATION_REQUIRED = ...
```

#### `_HTTP_STATUS_PATTERNS = ...`  <sub>(const)</sub>

```python
_HTTP_STATUS_PATTERNS = ...
```

#### `add_proxy_authentication`  <sub>(function)</sub>

> Add percent-encoded configured credentials unless the proxy has userinfo.

```python
def add_proxy_authentication(proxy: str, credential: str | None) -> str:
```

#### `proxy_error_status`  <sub>(function)</sub>

```python
def proxy_error_status(error: BaseException | str) -> int | None:
```

#### `is_proxy_connect_rejection`  <sub>(function)</sub>

```python
def is_proxy_connect_rejection(error: BaseException | str) -> bool:
```

#### `is_proxy_authentication_error`  <sub>(function)</sub>

```python
def is_proxy_authentication_error(error: BaseException | str) -> bool:
```

#### `_error_messages`  <sub>(function)</sub>

```python
def _error_messages(error: BaseException | str):
```

#### `format_proxy_error`  <sub>(function)</sub>

```python
def format_proxy_error(error: BaseException | str) -> str:
```

### `dirsearch/lib/connection/rate_limiter.py`


#### `RATE_WINDOW_SECONDS`  <sub>(const)</sub>

```python
RATE_WINDOW_SECONDS = ...
```

#### `RATE_WAIT_POLL_SECONDS`  <sub>(const)</sub>

```python
RATE_WAIT_POLL_SECONDS = ...
```

#### `RequestRateLimiter`  <sub>(class)</sub>

```python
class RequestRateLimiter:
```

**Fields:**
```python
def __init__(self, clock, sleep, async_sleep)
def wait(self, max_rate)
def wait_async(self, max_rate)
def rate(self)
def _reserve(self, max_rate)
def _discard_expired(self, now)
```


#### `RequestRateLimiter.__init__`  <sub>(function)</sub>

```python
def RequestRateLimiter.__init__(
        self,
        clock: Callable[[], float] | None = None,
        sleep: Callable[[float], None] | None = None,
        async_sleep: Callable[[float], Awaitable[None]] | None = None,
) -> None:
```

#### `RequestRateLimiter.wait`  <sub>(function)</sub>

```python
def RequestRateLimiter.wait(self, max_rate: int) -> None:
```

#### `RequestRateLimiter.wait_async`  <sub>(function)</sub>

```python
async def RequestRateLimiter.wait_async(self, max_rate: int) -> None:
```

#### `RequestRateLimiter.rate`  <sub>(function)</sub>

```python
def RequestRateLimiter.rate(self) -> int:
```

#### `RequestRateLimiter._reserve`  <sub>(function)</sub>

```python
def RequestRateLimiter._reserve(self, max_rate: int) -> float | None:
```

#### `RequestRateLimiter._discard_expired`  <sub>(function)</sub>

```python
def RequestRateLimiter._discard_expired(self, now: float) -> None:
```

### `dirsearch/lib/connection/requester.py`


#### `_join_request_target`  <sub>(function)</sub>

```python
def _join_request_target(base_url: str, quoted_path: str) -> str:
```

#### `_IPOverrideConnection`  <sub>(class)</sub>

> Change the connection IP without changing the URL hostname or TLS SNI.

```python
class _IPOverrideConnection:
```

**Fields:**
```python
def __init__(self)
def _new_conn(self)
```


#### `_IPOverrideConnection.__init__`  <sub>(function)</sub>

```python
def _IPOverrideConnection.__init__(self, *args, ip_overrides: IPOverrides | None = None, **kwargs):
```

#### `_IPOverrideConnection._new_conn`  <sub>(function)</sub>

```python
def _IPOverrideConnection._new_conn(self):
```

#### `_PathPreservingRequestMixin`  <sub>(class)</sub>

```python
class _PathPreservingRequestMixin:
```

**Fields:**
```python
def request(self, method, url, body, headers)
```


#### `_PathPreservingRequestMixin.request`  <sub>(function)</sub>

```python
def _PathPreservingRequestMixin.request(self, method, url, body=None, headers=None, *args, **kwargs):
```

#### `PathPreservingHTTPConnection`  <sub>(class)</sub>

```python
class PathPreservingHTTPConnection(
        _PathPreservingRequestMixin,
        _IPOverrideConnection,
        urllib3_connection.HTTPConnection,
):
```

#### `PathPreservingHTTPSConnection`  <sub>(class)</sub>

```python
class PathPreservingHTTPSConnection(
        _PathPreservingRequestMixin,
        _IPOverrideConnection,
        urllib3_connection.HTTPSConnection,
):
```

#### `PathPreservingSOCKSConnection`  <sub>(class)</sub>

```python
class PathPreservingSOCKSConnection(
        _PathPreservingRequestMixin,
        urllib3_socks.SOCKSConnection,
):
```

#### `PathPreservingSOCKSHTTPSConnection`  <sub>(class)</sub>

```python
class PathPreservingSOCKSHTTPSConnection(
        _PathPreservingRequestMixin,
        urllib3_socks.SOCKSHTTPSConnection,
):
```

#### `PathPreservingHTTPConnectionPool`  <sub>(class)</sub>

```python
class PathPreservingHTTPConnectionPool(urllib3_connectionpool.HTTPConnectionPool):
```

**Fields:**
```python
ConnectionCls = ...
```


#### `PathPreservingHTTPSConnectionPool`  <sub>(class)</sub>

```python
class PathPreservingHTTPSConnectionPool(urllib3_connectionpool.HTTPSConnectionPool):
```

**Fields:**
```python
ConnectionCls = ...
```


#### `PathPreservingSOCKSConnectionPool`  <sub>(class)</sub>

```python
class PathPreservingSOCKSConnectionPool(urllib3_socks.SOCKSHTTPConnectionPool):
```

**Fields:**
```python
ConnectionCls = ...
```


#### `PathPreservingSOCKSHTTPSConnectionPool`  <sub>(class)</sub>

```python
class PathPreservingSOCKSHTTPSConnectionPool(
        urllib3_socks.SOCKSHTTPSConnectionPool
):
```

**Fields:**
```python
ConnectionCls = ...
```


#### `PathPreservingPoolManager`  <sub>(class)</sub>

```python
class PathPreservingPoolManager(urllib3_poolmanager.PoolManager):
```

**Fields:**
```python
def __init__(self)
def _new_pool(self, scheme, host, port, request_context)
```


#### `PathPreservingPoolManager.__init__`  <sub>(function)</sub>

```python
def PathPreservingPoolManager.__init__(self, *args, ip_overrides: IPOverrides, **kwargs):
```

#### `PathPreservingPoolManager._new_pool`  <sub>(function)</sub>

```python
def PathPreservingPoolManager._new_pool(self, scheme, host, port, request_context=None):
```

#### `PathPreservingSocketOptionsAdapter`  <sub>(class)</sub>

```python
class PathPreservingSocketOptionsAdapter(SocketOptionsAdapter):
```

**Fields:**
```python
def __init__(self)
def init_poolmanager(self, connections, maxsize, block)
def request_url(self, request, proxies)
def proxy_manager_for(self, proxy)
def send(self, request)
```


#### `PathPreservingSocketOptionsAdapter.__init__`  <sub>(function)</sub>

```python
def PathPreservingSocketOptionsAdapter.__init__(self, **kwargs):
```

#### `PathPreservingSocketOptionsAdapter.init_poolmanager`  <sub>(function)</sub>

```python
def PathPreservingSocketOptionsAdapter.init_poolmanager(self, connections, maxsize, block=False):
```

#### `PathPreservingSocketOptionsAdapter.request_url`  <sub>(function)</sub>

```python
def PathPreservingSocketOptionsAdapter.request_url(self, request: requests.PreparedRequest, proxies: dict[str, str]) -> str:
```

#### `PathPreservingSocketOptionsAdapter.proxy_manager_for`  <sub>(function)</sub>

```python
def PathPreservingSocketOptionsAdapter.proxy_manager_for(self, proxy, **proxy_kwargs):
```

#### `PathPreservingSocketOptionsAdapter.send`  <sub>(function)</sub>

```python
def PathPreservingSocketOptionsAdapter.send(self, request, **kwargs):
```

#### `_is_requests_ssl_error`  <sub>(function)</sub>

> Check if the exception is a requests-wrapped SSL error.  
>   
> The requests library wraps ssl.SSLError inside requests.exceptions.SSLError,  
> so we need to inspect the exception chain to detect SSL errors that aren't  
> direct instances of ssl.SSLError.

```python
def _is_requests_ssl_error(exc: Exception) -> bool:
```

#### `_iter_exception_chain`  <sub>(function)</sub>

```python
def _iter_exception_chain(exc: BaseException) -> Generator[BaseException, None, None]:
```

#### `_is_dns_error`  <sub>(function)</sub>

> Return whether a request failure contains a DNS resolution error.  
>   
> Requests stores urllib3 failures in exception arguments and ``reason``  
> attributes instead of always using Python exception chaining. HTTPX keeps  
> the originating ``socket.gaierror`` as a cause on supported versions.  
> Inspect both wrapper styles without treating arbitrary connection errors  
> as DNS failures.

```python
def _is_dns_error(exc: Exception) -> bool:
```

#### `_find_ssl_error`  <sub>(function)</sub>

```python
def _find_ssl_error(exc: Exception) -> ssl.SSLError | None:
```

#### `_is_timeout_error`  <sub>(function)</sub>

```python
def _is_timeout_error(exc: Exception) -> bool:
```

#### `_is_response_read_error`  <sub>(function)</sub>

```python
def _is_response_read_error(exc: Exception) -> bool:
```

#### `_is_ssl_error`  <sub>(function)</sub>

```python
def _is_ssl_error(exc: Exception) -> bool:
```

#### `_format_ssl_error`  <sub>(function)</sub>

> Format SSL error with specific, actionable error messages.  
>   
> Provides detailed error messages based on the specific type of SSL error  
> instead of a generic 'Unexpected SSL error' message. This helps users  
> diagnose and fix SSL-related issues.

```python
def _format_ssl_error(exc: Exception, url: str = "") -> str:
```

#### `BaseRequester`  <sub>(class)</sub>

```python
class BaseRequester:
```

**Fields:**
```python
def __init__(self)
def _fetch_agents(self)
def set_url(self, url)
def set_query(self, query)
def set_ip(self, host, port, ip_address)
def request_path(self, path)
def set_header(self, key, value)
def _request_headers(self)
def reset_auth(self)
def wait_for_rate_limit(self)
def rate(self)
```


#### `BaseRequester.__init__`  <sub>(function)</sub>

```python
def BaseRequester.__init__(self) -> None:
```

#### `BaseRequester._fetch_agents`  <sub>(function)</sub>

```python
def BaseRequester._fetch_agents(self) -> None:
```

#### `BaseRequester.set_url`  <sub>(function)</sub>

```python
def BaseRequester.set_url(self, url: str) -> None:
```

#### `BaseRequester.set_query`  <sub>(function)</sub>

```python
def BaseRequester.set_query(self, query: str) -> None:
```

#### `BaseRequester.set_ip`  <sub>(function)</sub>

> Force the connection IP for a host while preserving its Host and SNI.

```python
def BaseRequester.set_ip(self, host: str, port: int, ip_address: str) -> None:
```

#### `BaseRequester.request_path`  <sub>(function)</sub>

```python
def BaseRequester.request_path(self, path: str) -> str:
```

#### `BaseRequester.set_header`  <sub>(function)</sub>

```python
def BaseRequester.set_header(self, key: str, value: str) -> None:
```

#### `BaseRequester._request_headers`  <sub>(function)</sub>

```python
def BaseRequester._request_headers(self) -> CaseInsensitiveDict:
```

#### `BaseRequester.reset_auth`  <sub>(function)</sub>

```python
def BaseRequester.reset_auth(self) -> None:
```

#### `BaseRequester.wait_for_rate_limit`  <sub>(function)</sub>

```python
def BaseRequester.wait_for_rate_limit(self) -> None:
```

#### `BaseRequester.rate`  <sub>(function)</sub>

```python
def BaseRequester.rate(self) -> int:
```

#### `HTTPBearerAuth`  <sub>(class)</sub>

```python
class HTTPBearerAuth(AuthBase):
```

**Fields:**
```python
def __init__(self, token)
def __call__(self, request)
```


#### `HTTPBearerAuth.__init__`  <sub>(function)</sub>

```python
def HTTPBearerAuth.__init__(self, token: str) -> None:
```

#### `HTTPBearerAuth.__call__`  <sub>(function)</sub>

```python
def HTTPBearerAuth.__call__(self, request: requests.PreparedRequest) -> requests.PreparedRequest:
```

#### `Requester`  <sub>(class)</sub>

```python
class Requester(BaseRequester):
```

**Fields:**
```python
def __init__(self)
def set_auth(self, type, credential)
def close(self)
def request(self, path, proxy)
```


#### `Requester.__init__`  <sub>(function)</sub>

```python
def Requester.__init__(self):
```

#### `Requester.set_auth`  <sub>(function)</sub>

```python
def Requester.set_auth(self, type: str, credential: str) -> None:
```

#### `Requester.close`  <sub>(function)</sub>

```python
def Requester.close(self) -> None:
```

#### `Requester.request`  <sub>(function)</sub>

```python
def Requester.request(self, path: str, proxy: str | None = None) -> Response:
```

#### `HTTPXBearerAuth`  <sub>(class)</sub>

```python
class HTTPXBearerAuth(httpx.Auth):
```

**Fields:**
```python
def __init__(self, token)
def auth_flow(self, request)
```


#### `HTTPXBearerAuth.__init__`  <sub>(function)</sub>

```python
def HTTPXBearerAuth.__init__(self, token: str) -> None:
```

#### `HTTPXBearerAuth.auth_flow`  <sub>(function)</sub>

```python
def HTTPXBearerAuth.auth_flow(self, request: httpx.Request) -> Generator[httpx.Request, None, None]:
```

#### `IPOverrideAsyncTransport`  <sub>(class)</sub>

```python
class IPOverrideAsyncTransport(httpx.AsyncBaseTransport):
```

**Fields:**
```python
def __init__(self, transport, ip_overrides)
def handle_async_request(self, request)
def aclose(self)
```


#### `IPOverrideAsyncTransport.__init__`  <sub>(function)</sub>

```python
def IPOverrideAsyncTransport.__init__(
        self,
        transport: httpx.AsyncBaseTransport,
        ip_overrides: IPOverrides,
) -> None:
```

#### `IPOverrideAsyncTransport.handle_async_request`  <sub>(function)</sub>

```python
async def IPOverrideAsyncTransport.handle_async_request(self, request: httpx.Request) -> httpx.Response:
```

#### `IPOverrideAsyncTransport.aclose`  <sub>(function)</sub>

```python
async def IPOverrideAsyncTransport.aclose(self) -> None:
```

#### `PathPreservingAsyncHTTPTransport`  <sub>(class)</sub>

> Pass a raw target into httpcore without leaking it into proxy subrequests.

```python
class PathPreservingAsyncHTTPTransport(httpx.AsyncHTTPTransport):
```

**Fields:**
```python
def handle_async_request(self, request)
```


#### `PathPreservingAsyncHTTPTransport.handle_async_request`  <sub>(function)</sub>

```python
async def PathPreservingAsyncHTTPTransport.handle_async_request(self, request: httpx.Request) -> httpx.Response:
```

#### `ProxyRoatingTransport`  <sub>(class)</sub>

```python
class ProxyRoatingTransport(httpx.AsyncBaseTransport):
```

**Fields:**
```python
def __init__(self, proxies)
def handle_async_request(self, request)
def aclose(self)
```


#### `ProxyRoatingTransport.__init__`  <sub>(function)</sub>

```python
def ProxyRoatingTransport.__init__(self, proxies: list[str], **kwargs: Any) -> None:
```

#### `ProxyRoatingTransport.handle_async_request`  <sub>(function)</sub>

```python
async def ProxyRoatingTransport.handle_async_request(self, request: httpx.Request) -> httpx.Response:
```

#### `ProxyRoatingTransport.aclose`  <sub>(function)</sub>

```python
async def ProxyRoatingTransport.aclose(self) -> None:
```

#### `AsyncRequester`  <sub>(class)</sub>

```python
class AsyncRequester(BaseRequester):
```

**Fields:**
```python
def __init__(self)
def _environment_proxy_mounts(self, transport_options)
def _uses_inherited_proxy(self, session, response)
def parse_proxy(self, proxy)
def set_auth(self, type, credential)
def close(self)
def replay_request(self, path, proxy)
def request(self, path, session, replay)
def wait_for_rate_limit(self)
```


#### `AsyncRequester.__init__`  <sub>(function)</sub>

```python
def AsyncRequester.__init__(self) -> None:
```

#### `AsyncRequester._environment_proxy_mounts`  <sub>(function)</sub>

```python
def AsyncRequester._environment_proxy_mounts(
        self, transport_options: dict[str, Any]
) -> dict[str, httpx.AsyncBaseTransport | None]:
```

#### `AsyncRequester._uses_inherited_proxy`  <sub>(function)</sub>

```python
def AsyncRequester._uses_inherited_proxy(
        self, session: httpx.AsyncClient, response: httpx.Response
) -> bool:
```

#### `AsyncRequester.parse_proxy`  <sub>(function)</sub>

```python
def AsyncRequester.parse_proxy(self, proxy: str) -> str | httpx.Proxy | None:
```

#### `AsyncRequester.set_auth`  <sub>(function)</sub>

```python
def AsyncRequester.set_auth(self, type: str, credential: str) -> None:
```

#### `AsyncRequester.close`  <sub>(function)</sub>

```python
async def AsyncRequester.close(self) -> None:
```

#### `AsyncRequester.replay_request`  <sub>(function)</sub>

```python
async def AsyncRequester.replay_request(self, path: str, proxy: str) -> AsyncResponse:
```

#### `AsyncRequester.request`  <sub>(function)</sub>

```python
async def AsyncRequester.request(
        self, path: str, session: httpx.AsyncClient | None = None, replay: bool = False
) -> AsyncResponse:
```

#### `AsyncRequester.wait_for_rate_limit`  <sub>(function)</sub>

```python
async def AsyncRequester.wait_for_rate_limit(self) -> None:
```

### `dirsearch/lib/connection/response.py`


#### `_decoded_content_length`  <sub>(function)</sub>

```python
def _decoded_content_length(headers) -> int | None:
```

#### `_declared_charset`  <sub>(function)</sub>

```python
def _declared_charset(headers) -> str | None:
```

#### `_is_known_charset`  <sub>(function)</sub>

```python
def _is_known_charset(charset: str | None) -> bool:
```

#### `_has_textual_media_type`  <sub>(function)</sub>

```python
def _has_textual_media_type(headers) -> bool:
```

#### `_should_decode_binary_text`  <sub>(function)</sub>

```python
def _should_decode_binary_text(headers, declared_charset: str | None) -> bool:
```

#### `_decode_response_body`  <sub>(function)</sub>

```python
def _decode_response_body(
        body: bytes,
        encoding: str | None,
        decode_binary_text: bool,
) -> str:
```

#### `ResponseHeaders`  <sub>(class)</sub>

> Read-only, case-insensitive response headers owned by dirsearch.

```python
class ResponseHeaders(Mapping[str, str]):
```

**Fields:**
```python
__slots__ = ...
def __init__(self, headers)
def __getitem__(self, name)
def __iter__(self)
def __len__(self)
def get_list(self, name)
def multi_items(self)
```


#### `ResponseHeaders.__init__`  <sub>(function)</sub>

```python
def ResponseHeaders.__init__(self, headers: Iterable[tuple[str, str]] = ()) -> None:
```

#### `ResponseHeaders.__getitem__`  <sub>(function)</sub>

```python
def ResponseHeaders.__getitem__(self, name: str) -> str:
```

#### `ResponseHeaders.__iter__`  <sub>(function)</sub>

```python
def ResponseHeaders.__iter__(self) -> Iterator[str]:
```

#### `ResponseHeaders.__len__`  <sub>(function)</sub>

```python
def ResponseHeaders.__len__(self) -> int:
```

#### `ResponseHeaders.get_list`  <sub>(function)</sub>

> Return every value for a header without combining duplicates.

```python
def ResponseHeaders.get_list(self, name: str) -> list[str]:
```

#### `ResponseHeaders.multi_items`  <sub>(function)</sub>

> Return header pairs in their original order, including duplicates.

```python
def ResponseHeaders.multi_items(self) -> list[tuple[str, str]]:
```

#### `_BodyCapture`  <sub>(class)</sub>

> Keep response bodies bounded while retaining a complete binary digest.

```python
class _BodyCapture:
```

**Fields:**
```python
def __init__(self, headers, capture_full_body)
def add(self, chunk)
def finish(self)
def body_digest(self)
```


#### `_BodyCapture.__init__`  <sub>(function)</sub>

```python
def _BodyCapture.__init__(self, headers, capture_full_body: bool) -> None:
```

#### `_BodyCapture.add`  <sub>(function)</sub>

```python
def _BodyCapture.add(self, chunk: bytes) -> bool:
```

#### `_BodyCapture.finish`  <sub>(function)</sub>

```python
def _BodyCapture.finish(self) -> None:
```

#### `_BodyCapture.body_digest`  <sub>(function)</sub>

```python
def _BodyCapture.body_digest(self) -> bytes | None:
```

#### `BaseResponse`  <sub>(class)</sub>

```python
class BaseResponse:
```

**Fields:**
```python
def __init__(self, url, status, headers, elapsed, history, final_url)
def type(self)
def length(self)
def size(self)
def body_complete(self)
def body_truncated(self)
def text(self)
def normalized_content(self)
def words(self)
def lines(self)
def __hash__(self)
def filter_fingerprint(self)
def _body_fingerprint(self)
def has_same_body(self, other)
def __eq__(self, other)
```


#### `BaseResponse.__init__`  <sub>(function)</sub>

```python
def BaseResponse.__init__(
        self,
        url: str,
        status: int,
        headers: Iterable[tuple[str, str]],
        elapsed: float = 0.0,
        history: Iterable[str] = (),
        final_url: str | None = None,
) -> None:
```

#### `BaseResponse.type`  <sub>(function)</sub>

```python
def BaseResponse.type(self) -> str:
```

#### `BaseResponse.length`  <sub>(function)</sub>

```python
def BaseResponse.length(self) -> int:
```

#### `BaseResponse.size`  <sub>(function)</sub>

```python
def BaseResponse.size(self) -> str:
```

#### `BaseResponse.body_complete`  <sub>(function)</sub>

> Return whether the captured body contains the complete response.

```python
def BaseResponse.body_complete(self) -> bool:
```

#### `BaseResponse.body_truncated`  <sub>(function)</sub>

> Return whether response capture stopped before the body was complete.

```python
def BaseResponse.body_truncated(self) -> bool:
```

#### `BaseResponse.text`  <sub>(function)</sub>

```python
def BaseResponse.text(self) -> str:
```

#### `BaseResponse.normalized_content`  <sub>(function)</sub>

> Return cached text with volatile response values normalized.

```python
def BaseResponse.normalized_content(self) -> str:
```

#### `BaseResponse.words`  <sub>(function)</sub>

```python
def BaseResponse.words(self) -> int:
```

#### `BaseResponse.lines`  <sub>(function)</sub>

```python
def BaseResponse.lines(self) -> int:
```

#### `BaseResponse.__hash__`  <sub>(function)</sub>

```python
def BaseResponse.__hash__(self) -> int:
```

#### `BaseResponse.filter_fingerprint`  <sub>(function)</sub>

> Return a path-agnostic fingerprint for --filter-threshold.

```python
def BaseResponse.filter_fingerprint(self) -> int:
```

#### `BaseResponse._body_fingerprint`  <sub>(function)</sub>

```python
def BaseResponse._body_fingerprint(self) -> bytes:
```

#### `BaseResponse.has_same_body`  <sub>(function)</sub>

> Return whether both responses contain the same complete body.

```python
def BaseResponse.has_same_body(self, other: BaseResponse) -> bool:
```

#### `BaseResponse.__eq__`  <sub>(function)</sub>

```python
def BaseResponse.__eq__(self, other: Any) -> bool:
```

#### `Response`  <sub>(class)</sub>

```python
class Response(BaseResponse):
```

**Fields:**
```python
def __init__(self, url, response, elapsed, capture_full_body)
```


#### `Response.__init__`  <sub>(function)</sub>

```python
def Response.__init__(
        self,
        url,
        response: requests.Response,
        elapsed: float = 0.0,
        capture_full_body: bool = False,
) -> None:
```

#### `AsyncResponse`  <sub>(class)</sub>

```python
class AsyncResponse(BaseResponse):
```

**Fields:**
```python
def create(cls, url, response, elapsed, capture_full_body)
```


#### `AsyncResponse.create`  <sub>(function)</sub>

```python
async def AsyncResponse.create(
        cls,
        url,
        response: httpx.Response,
        elapsed: float = 0.0,
        capture_full_body: bool = False,
) -> AsyncResponse:
```

#### `NativeResponse`  <sub>(class)</sub>

```python
class NativeResponse(BaseResponse):
```

**Fields:**
```python
def __init__(self, url, status, headers, body, elapsed, length, filtered, filter_reason, body_complete, history, final_url)
def length(self)
```


#### `NativeResponse.__init__`  <sub>(function)</sub>

```python
def NativeResponse.__init__(
        self,
        url: str,
        status: int,
        headers: list[tuple[str, str]],
        body: bytes | bytearray | list[int],
        elapsed: float = 0.0,
        length: int | None = None,
        filtered: bool = False,
        filter_reason: str | None = None,
        body_complete: bool | None = None,
        history: Iterable[str] = (),
        final_url: str | None = None,
) -> None:
```

#### `NativeResponse.length`  <sub>(function)</sub>

```python
def NativeResponse.length(self) -> int:
```


## `lib/controller` <a id='pkg-lib-controller'></a>

*74 documented items*


### `dirsearch/lib/controller/controller.py`


#### `ForceQuitHandler`  <sub>(class)</sub>

> Strategy for handling force quit on repeated Ctrl+C.  
>   
> Different platforms have different signal handling behaviors. This base  
> class defines the interface, with subclasses implementing platform-specific  
> logic.

```python
class ForceQuitHandler:
```

**Fields:**
```python
def check_force_quit(self)
def on_pause_start(self)
def on_resume(self)
```


#### `ForceQuitHandler.check_force_quit`  <sub>(function)</sub>

> Check if force quit should be triggered.  
>   
> Returns True if force quit was triggered (program will exit).

```python
def ForceQuitHandler.check_force_quit(self) -> bool:
```

#### `ForceQuitHandler.on_pause_start`  <sub>(function)</sub>

> Called when pause mode is entered.

```python
def ForceQuitHandler.on_pause_start(self) -> None:
```

#### `ForceQuitHandler.on_resume`  <sub>(function)</sub>

> Called when resuming from pause.

```python
def ForceQuitHandler.on_resume(self) -> None:
```

#### `StandardForceQuitHandler`  <sub>(class)</sub>

> Force quit handler for standard platforms.  
>   
> Immediately exits on any Ctrl+C during pause mode.

```python
class StandardForceQuitHandler(ForceQuitHandler):
```

**Fields:**
```python
def check_force_quit(self)
```


#### `StandardForceQuitHandler.check_force_quit`  <sub>(function)</sub>

```python
def StandardForceQuitHandler.check_force_quit(self) -> bool:
```

#### `PyInstallerLinuxForceQuitHandler`  <sub>(class)</sub>

> Force quit handler for PyInstaller Linux builds.  
>   
> PyInstaller on Linux has signal handling quirks that require multiple  
> rapid Ctrl+C presses to force quit. Uses SIGKILL for reliable termination.

```python
class PyInstallerLinuxForceQuitHandler(ForceQuitHandler):
```

**Fields:**
```python
def __init__(self)
def check_force_quit(self)
def on_pause_start(self)
def on_resume(self)
```


#### `PyInstallerLinuxForceQuitHandler.__init__`  <sub>(function)</sub>

```python
def PyInstallerLinuxForceQuitHandler.__init__(self) -> None:
```

#### `PyInstallerLinuxForceQuitHandler.check_force_quit`  <sub>(function)</sub>

```python
def PyInstallerLinuxForceQuitHandler.check_force_quit(self) -> bool:
```

#### `PyInstallerLinuxForceQuitHandler.on_pause_start`  <sub>(function)</sub>

```python
def PyInstallerLinuxForceQuitHandler.on_pause_start(self) -> None:
```

#### `PyInstallerLinuxForceQuitHandler.on_resume`  <sub>(function)</sub>

```python
def PyInstallerLinuxForceQuitHandler.on_resume(self) -> None:
```

#### `_create_force_quit_handler`  <sub>(function)</sub>

> Factory function to create the appropriate force quit handler.

```python
def _create_force_quit_handler() -> ForceQuitHandler:
```

#### `format_session_path`  <sub>(function)</sub>

```python
def format_session_path(path: str) -> str:
```

#### `Controller`  <sub>(class)</sub>

```python
class Controller:
```

**Fields:**
```python
def __init__(self)
def _close_reporter(self)
def _close_requester(self)
def _import(self, session_file)
def _format_output_history(self, output_history)
def _confirm_session_overwrite(self, session_file)
def _export(self, session_file)
def setup(self)
def run(self)
def _report_match_callback(self)
def start(self)
def get_time_limit(self, start_time)
def start_native_fuzzer(self, start_time)
def start_coroutines(self, start_time)
def process(self, start_time)
def set_target(self, url)
def crawl_target(self)
def add_crawled_paths(self, response)
def reset_consecutive_errors(self, response)
def reset_consecutive_errors_batch(self, _count)
def _prepare_response_stores(self)
def save_response(self, response)
def save_response_async(self, response)
def _save_response_to_store_async(self, store, artifact)
def _report_response_store_error(store, artifact, error)
def _close_response_stores(self)
def match_callback(self, response)
def update_progress_bar(self, response)
def update_progress_bar_batch(self, _count)
def raise_error(self, exception)
def append_error_log(self, exception)
def _force_exit(self)
def _reset_pause_state(self)
def handle_pause(self)
def add_directory(self, path)
def _is_excluded_subdir(path)
def recur(self, path)
def recur_for_redirect(self, path, redirect_path)
```


#### `Controller.__init__`  <sub>(function)</sub>

```python
def Controller.__init__(self) -> None:
```

#### `Controller._close_reporter`  <sub>(function)</sub>

```python
def Controller._close_reporter(self) -> None:
```

#### `Controller._close_requester`  <sub>(function)</sub>

```python
def Controller._close_requester(self) -> None:
```

#### `Controller._import`  <sub>(function)</sub>

```python
def Controller._import(self, session_file: str) -> None:
```

#### `Controller._format_output_history`  <sub>(function)</sub>

```python
def Controller._format_output_history(self, output_history: list[dict[str, Any]]) -> str:
```

#### `Controller._confirm_session_overwrite`  <sub>(function)</sub>

```python
def Controller._confirm_session_overwrite(self, session_file: str) -> None:
```

#### `Controller._export`  <sub>(function)</sub>

```python
def Controller._export(self, session_file: str) -> None:
```

#### `Controller.setup`  <sub>(function)</sub>

```python
def Controller.setup(self) -> None:
```

#### `Controller.run`  <sub>(function)</sub>

```python
def Controller.run(self) -> None:
```

#### `Controller._report_match_callback`  <sub>(function)</sub>

```python
def Controller._report_match_callback(self):
```

#### `Controller.start`  <sub>(function)</sub>

```python
def Controller.start(self) -> None:
```

#### `Controller.get_time_limit`  <sub>(function)</sub>

```python
def Controller.get_time_limit(
        self, start_time: float
) -> tuple[float | None, Exception | None]:
```

#### `Controller.start_native_fuzzer`  <sub>(function)</sub>

```python
def Controller.start_native_fuzzer(self, start_time: float) -> None:
```

#### `Controller.start_coroutines`  <sub>(function)</sub>

```python
async def Controller.start_coroutines(self, start_time: float) -> None:
```

#### `Controller.process`  <sub>(function)</sub>

```python
def Controller.process(self, start_time: float) -> None:
```

#### `Controller.set_target`  <sub>(function)</sub>

```python
def Controller.set_target(self, url: str) -> None:
```

#### `Controller.crawl_target`  <sub>(function)</sub>

```python
def Controller.crawl_target(self) -> None:
```

#### `Controller.add_crawled_paths`  <sub>(function)</sub>

```python
def Controller.add_crawled_paths(self, response: BaseResponse) -> None:
```

#### `Controller.reset_consecutive_errors`  <sub>(function)</sub>

```python
def Controller.reset_consecutive_errors(self, response: BaseResponse) -> None:
```

#### `Controller.reset_consecutive_errors_batch`  <sub>(function)</sub>

```python
def Controller.reset_consecutive_errors_batch(self, _count: int) -> None:
```

#### `Controller._prepare_response_stores`  <sub>(function)</sub>

```python
def Controller._prepare_response_stores(self) -> None:
```

#### `Controller.save_response`  <sub>(function)</sub>

```python
def Controller.save_response(self, response: BaseResponse) -> None:
```

#### `Controller.save_response_async`  <sub>(function)</sub>

```python
async def Controller.save_response_async(self, response: BaseResponse) -> None:
```

#### `Controller._save_response_to_store_async`  <sub>(function)</sub>

```python
async def Controller._save_response_to_store_async(
        self,
        store: BaseResponseStore,
        artifact: ResponseArtifact,
) -> None:
```

#### `Controller._report_response_store_error`  <sub>(function)</sub>

```python
def Controller._report_response_store_error(
        store: BaseResponseStore,
        artifact: ResponseArtifact,
        error: OSError | ValueError,
) -> None:
```

#### `Controller._close_response_stores`  <sub>(function)</sub>

```python
def Controller._close_response_stores(self) -> None:
```

#### `Controller.match_callback`  <sub>(function)</sub>

```python
def Controller.match_callback(
        self, response: BaseResponse
) -> Awaitable[BaseResponse] | None:
```

#### `Controller.update_progress_bar`  <sub>(function)</sub>

```python
def Controller.update_progress_bar(self, response: BaseResponse | None) -> None:
```

#### `Controller.update_progress_bar_batch`  <sub>(function)</sub>

```python
def Controller.update_progress_bar_batch(self, _count: int) -> None:
```

#### `Controller.raise_error`  <sub>(function)</sub>

```python
def Controller.raise_error(self, exception: RequestException) -> None:
```

#### `Controller.append_error_log`  <sub>(function)</sub>

```python
def Controller.append_error_log(self, exception: RequestException) -> None:
```

#### `Controller._force_exit`  <sub>(function)</sub>

> Force process termination, stopping asyncio loop if running.

```python
def Controller._force_exit(self) -> None:
```

#### `Controller._reset_pause_state`  <sub>(function)</sub>

```python
def Controller._reset_pause_state(self) -> None:
```

#### `Controller.handle_pause`  <sub>(function)</sub>

> Handle SIGINT (Ctrl+C) by pausing execution and showing options.

```python
def Controller.handle_pause(self) -> None:
```

#### `Controller.add_directory`  <sub>(function)</sub>

> Add directory to the recursion queue

```python
def Controller.add_directory(self, path: str) -> None:
```

#### `Controller._is_excluded_subdir`  <sub>(function)</sub>

```python
def Controller._is_excluded_subdir(path: str) -> bool:
```

#### `Controller.recur`  <sub>(function)</sub>

```python
def Controller.recur(self, path: str) -> list[str]:
```

#### `Controller.recur_for_redirect`  <sub>(function)</sub>

```python
def Controller.recur_for_redirect(self, path: str, redirect_path: str) -> list[str]:
```

### `dirsearch/lib/controller/session.py`


#### `SessionStore`  <sub>(class)</sub>

```python
class SessionStore:
```

**Fields:**
```python
SESSION_VERSION = ...
SESSION_BYTES_MARKER = ...
CHECKPOINT_FILE = ...
SESSION_OPTION_SET_KEYS = ...
SESSION_OPTION_TUPLE_KEYS = ...
FILES = ...
def __init__(self, options)
def list_sessions(self, base_path)
def load(self, session_path)
def save(self, controller, session_path, last_output)
def delete(self, session_path)
def apply_to_controller(self, controller, payload)
def restore_options(self, serialized)
def _serialize_controller_state(self, controller)
def _serialize_dictionary(self, controller)
def _serialize_options(self)
def _get_session_dir(self, session_path)
def _delete_session_files(self, session_dir, file_names)
def _read_json(self, path)
def _write_json(self, path, payload)
def _validate_payload(self, payload)
def _get_controller_history(self, controller)
def _load_output_history(self, session_dir)
def _deserialize_output_history(self, payload, start_time)
def _summarize_session_dir(self, session_dir)
def _summarize_session_file(self, session_file)
def _build_summary(self, session_path, meta_path, controller_state, options_state)
```


#### `SessionStore.__init__`  <sub>(function)</sub>

```python
def SessionStore.__init__(self, options: dict[str, Any]) -> None:
```

#### `SessionStore.list_sessions`  <sub>(function)</sub>

```python
def SessionStore.list_sessions(self, base_path: str) -> list[dict[str, Any]]:
```

#### `SessionStore.load`  <sub>(function)</sub>

```python
def SessionStore.load(self, session_path: str) -> dict[str, Any]:
```

#### `SessionStore.save`  <sub>(function)</sub>

```python
def SessionStore.save(self, controller: Any, session_path: str, last_output: str) -> None:
```

#### `SessionStore.delete`  <sub>(function)</sub>

> Delete session-owned files without removing unrelated entries.

```python
def SessionStore.delete(self, session_path: str) -> None:
```

#### `SessionStore.apply_to_controller`  <sub>(function)</sub>

```python
def SessionStore.apply_to_controller(self, controller: Any, payload: dict[str, Any]) -> None:
```

#### `SessionStore.restore_options`  <sub>(function)</sub>

```python
def SessionStore.restore_options(self, serialized: dict[str, Any]) -> dict[str, Any]:
```

#### `SessionStore._serialize_controller_state`  <sub>(function)</sub>

```python
def SessionStore._serialize_controller_state(self, controller: Any) -> dict[str, Any]:
```

#### `SessionStore._serialize_dictionary`  <sub>(function)</sub>

```python
def SessionStore._serialize_dictionary(self, controller: Any) -> dict[str, Any]:
```

#### `SessionStore._serialize_options`  <sub>(function)</sub>

```python
def SessionStore._serialize_options(self) -> dict[str, Any]:
```

#### `SessionStore._get_session_dir`  <sub>(function)</sub>

```python
def SessionStore._get_session_dir(self, session_path: str) -> str:
```

#### `SessionStore._delete_session_files`  <sub>(function)</sub>

```python
def SessionStore._delete_session_files(
        self,
        session_dir: str,
        file_names: Iterable[str],
) -> None:
```

#### `SessionStore._read_json`  <sub>(function)</sub>

```python
def SessionStore._read_json(self, path: str) -> dict[str, Any]:
```

#### `SessionStore._write_json`  <sub>(function)</sub>

```python
def SessionStore._write_json(self, path: str, payload: dict[str, Any]) -> None:
```

#### `SessionStore._validate_payload`  <sub>(function)</sub>

```python
def SessionStore._validate_payload(self, payload: dict[str, Any]) -> None:
```

#### `SessionStore._get_controller_history`  <sub>(function)</sub>

```python
def SessionStore._get_controller_history(self, controller: Any) -> list[dict[str, Any]] | None:
```

#### `SessionStore._load_output_history`  <sub>(function)</sub>

```python
def SessionStore._load_output_history(self, session_dir: str) -> list[dict[str, Any]]:
```

#### `SessionStore._deserialize_output_history`  <sub>(function)</sub>

```python
def SessionStore._deserialize_output_history(
        self,
        payload: dict[str, Any],
        start_time: Any,
) -> list[dict[str, Any]]:
```

#### `SessionStore._summarize_session_dir`  <sub>(function)</sub>

```python
def SessionStore._summarize_session_dir(self, session_dir: str) -> dict[str, Any] | None:
```

#### `SessionStore._summarize_session_file`  <sub>(function)</sub>

```python
def SessionStore._summarize_session_file(self, session_file: str) -> dict[str, Any] | None:
```

#### `SessionStore._build_summary`  <sub>(function)</sub>

```python
def SessionStore._build_summary(
        self,
        session_path: str,
        meta_path: str,
        controller_state: dict[str, Any],
        options_state: dict[str, Any],
) -> dict[str, Any]:
```


## `lib/core` <a id='pkg-lib-core'></a>

*323 documented items*


### `dirsearch/lib/core/api.py`


#### `FuzzerResult`  <sub>(class)</sub>

```python
class FuzzerResult:
```

**Fields:**
```python
url: ...
path: ...
status: ...
length: ...
content_type: ...
redirect: ...
elapsed: ...
headers: ...
body: ...
```


#### `WordlistState`  <sub>(class)</sub>

```python
class WordlistState:
```

**Fields:**
```python
items: ...
index: ...
```


#### `Wordlist`  <sub>(class)</sub>

```python
class Wordlist:
```

**Fields:**
```python
items: ...
def __init__(self, items)
def from_file(cls, path)
def from_template(cls, template)
def _dedupe(items)
def __iter__(self)
def __len__(self)
def state(self, index)
```


#### `Wordlist.__init__`  <sub>(function)</sub>

```python
def Wordlist.__init__(self, items: Iterable[str], *, max_entries: int | None = None) -> None:
```

#### `Wordlist.from_file`  <sub>(function)</sub>

```python
def Wordlist.from_file(cls, path: str) -> Wordlist:
```

#### `Wordlist.from_template`  <sub>(function)</sub>

```python
def Wordlist.from_template(
        cls,
        template: WordlistTemplate,
        *,
        extensions: Iterable[str] = (),
        placeholders: Mapping[str, Iterable[str] | str] | None = None,
        max_entries: int | None = None,
) -> Wordlist:
```

#### `Wordlist._dedupe`  <sub>(function)</sub>

```python
def Wordlist._dedupe(
        items: Iterable[str],
        *,
        max_entries: int | None = None,
) -> Iterator[str]:
```

#### `Wordlist.__iter__`  <sub>(function)</sub>

```python
def Wordlist.__iter__(self) -> Iterator[str]:
```

#### `Wordlist.__len__`  <sub>(function)</sub>

```python
def Wordlist.__len__(self) -> int:
```

#### `Wordlist.state`  <sub>(function)</sub>

```python
def Wordlist.state(self, index: int = 0) -> WordlistState:
```

#### `WordlistTemplate`  <sub>(class)</sub>

```python
class WordlistTemplate:
```

**Fields:**
```python
lines: ...
placeholders: ...
def __init__(self, lines, placeholders)
def from_file(cls, path, placeholders)
def from_builtin(cls, name, placeholders)
def _normalize_placeholders(placeholders)
def render(self)
```


#### `WordlistTemplate.__init__`  <sub>(function)</sub>

```python
def WordlistTemplate.__init__(
        self,
        lines: Iterable[str],
        placeholders: Mapping[str, Iterable[str] | str] | None = None,
) -> None:
```

#### `WordlistTemplate.from_file`  <sub>(function)</sub>

```python
def WordlistTemplate.from_file(
        cls,
        path: str,
        placeholders: Mapping[str, Iterable[str] | str] | None = None,
) -> WordlistTemplate:
```

#### `WordlistTemplate.from_builtin`  <sub>(function)</sub>

```python
def WordlistTemplate.from_builtin(
        cls,
        name: str,
        placeholders: Mapping[str, Iterable[str] | str] | None = None,
) -> WordlistTemplate:
```

#### `WordlistTemplate._normalize_placeholders`  <sub>(function)</sub>

```python
def WordlistTemplate._normalize_placeholders(
        placeholders: Mapping[str, Iterable[str] | str]
) -> dict[str, tuple[str, ...]]:
```

#### `WordlistTemplate.render`  <sub>(function)</sub>

```python
def WordlistTemplate.render(
        self,
        *,
        extensions: Iterable[str] = (),
        placeholders: Mapping[str, Iterable[str] | str] | None = None,
) -> Iterator[str]:
```

#### `FuzzerConfig`  <sub>(class)</sub>

```python
class FuzzerConfig:
```

**Fields:**
```python
url: ...
wordlist: ...
extensions: ...
headers: ...
http_method: ...
data: ...
timeout: ...
follow_redirects: ...
include_status_codes: ...
exclude_status_codes: ...
verify_tls: ...
user_agent: ...
result_predicate: ...
session_factory: ...
raise_on_error: ...
def from_raw_request(cls)
def __post_init__(self)
```


#### `FuzzerConfig.from_raw_request`  <sub>(function)</sub>

```python
def FuzzerConfig.from_raw_request(
        cls,
        *,
        wordlist: Wordlist | WordlistTemplate | Iterable[str],
        raw_request: str | bytes | None = None,
        raw_file: str | None = None,
        scheme: str = "http",
        extensions: Iterable[str] = (),
        timeout: float = 10.0,
        follow_redirects: bool = False,
        include_status_codes: Iterable[int] = frozenset(),
        exclude_status_codes: Iterable[int] = frozenset({404}),
        verify_tls: bool = False,
        user_agent: str | None = None,
        result_predicate: Callable[[FuzzerResult], bool] | None = None,
        session_factory: Callable[[], requests.Session] | None = None,
        raise_on_error: bool = False,
) -> FuzzerConfig:
```

#### `FuzzerConfig.__post_init__`  <sub>(function)</sub>

```python
def FuzzerConfig.__post_init__(self) -> None:
```

#### `DirsearchFuzzer`  <sub>(class)</sub>

```python
class DirsearchFuzzer:
```

**Fields:**
```python
def __init__(self, config)
def run(self)
def _wordlist(self)
def _request(self, session, headers, path)
def _is_match(self, result)
def _join_url(base_url, path)
```


#### `DirsearchFuzzer.__init__`  <sub>(function)</sub>

```python
def DirsearchFuzzer.__init__(
        self,
        config: FuzzerConfig,
        *,
        on_result: Callable[[FuzzerResult], Any] | None = None,
        on_not_found: Callable[[FuzzerResult], Any] | None = None,
        on_error: Callable[[Exception], Any] | None = None,
) -> None:
```

#### `DirsearchFuzzer.run`  <sub>(function)</sub>

```python
def DirsearchFuzzer.run(self) -> list[FuzzerResult]:
```

#### `DirsearchFuzzer._wordlist`  <sub>(function)</sub>

```python
def DirsearchFuzzer._wordlist(self) -> Wordlist:
```

#### `DirsearchFuzzer._request`  <sub>(function)</sub>

```python
def DirsearchFuzzer._request(
        self,
        session: requests.Session,
        headers: Mapping[str, str],
        path: str,
) -> FuzzerResult:
```

#### `DirsearchFuzzer._is_match`  <sub>(function)</sub>

```python
def DirsearchFuzzer._is_match(self, result: FuzzerResult) -> bool:
```

#### `DirsearchFuzzer._join_url`  <sub>(function)</sub>

```python
def DirsearchFuzzer._join_url(base_url: str, path: str) -> str:
```

### `dirsearch/lib/core/decorators.py`


#### `cached`  <sub>(function)</sub>

```python
def cached(timeout: int | float = 100) -> Callable[[F], F]:
```

#### `locked`  <sub>(function)</sub>

```python
def locked(func: F) -> F:
```

### `dirsearch/lib/core/dictionary.py`


#### `_NativeClaim`  <sub>(class)</sub>

```python
class _NativeClaim:
```

**Fields:**
```python
chunk: ...
start: ...
released: ...
```


#### `get_blacklists`  <sub>(function)</sub>

```python
def get_blacklists() -> dict[int, Dictionary]:
```

#### `Dictionary`  <sub>(class)</sub>

```python
class Dictionary:
```

**Fields:**
```python
def __init__(self)
def index(self)
def __next__(self)
def claim_next(self)
def claim_many(self, maximum)
def claim_native_many(self, maximum, base_path)
def release_native_claims(self, chunk, count)
def release_claim(self, path)
def release_claims(self, paths)
def requeue_claims(self)
def __contains__(self, item)
def __getstate__(self)
def __setstate__(self, state)
def __iter__(self)
def __len__(self)
def generate(self, files, is_blacklist)
def is_valid(self, path)
def add_extra(self, path)
def reset(self)
```


#### `Dictionary.__init__`  <sub>(function)</sub>

```python
def Dictionary.__init__(self, **kwargs: Any) -> None:
```

#### `Dictionary.index`  <sub>(function)</sub>

```python
def Dictionary.index(self) -> int:
```

#### `Dictionary.__next__`  <sub>(function)</sub>

```python
def Dictionary.__next__(self) -> str:
```

#### `Dictionary.claim_next`  <sub>(function)</sub>

> Return the next path and keep it recoverable until released.

```python
def Dictionary.claim_next(self) -> str:
```

#### `Dictionary.claim_many`  <sub>(function)</sub>

> Claim up to maximum paths atomically, preserving queue order.

```python
def Dictionary.claim_many(self, maximum: int) -> list[str]:
```

#### `Dictionary.claim_native_many`  <sub>(function)</sub>

> Claim a Rust-owned range without materializing its Python strings.

```python
def Dictionary.claim_native_many(
        self,
        maximum: int,
        base_path: str,
) -> list[str] | NativeWordlistChunk:
```

#### `Dictionary.release_native_claims`  <sub>(function)</sub>

> Release an ordered prefix from the active Rust-owned claim.

```python
def Dictionary.release_native_claims(
        self,
        chunk: NativeWordlistChunk,
        count: int,
) -> None:
```

#### `Dictionary.release_claim`  <sub>(function)</sub>

```python
def Dictionary.release_claim(self, path: str) -> None:
```

#### `Dictionary.release_claims`  <sub>(function)</sub>

> Release several completed claims in one bounded lock operation.

```python
def Dictionary.release_claims(self, paths: list[str]) -> None:
```

#### `Dictionary.requeue_claims`  <sub>(function)</sub>

> Make outstanding claims available again in their original order.

```python
def Dictionary.requeue_claims(self) -> None:
```

#### `Dictionary.__contains__`  <sub>(function)</sub>

```python
def Dictionary.__contains__(self, item: str) -> bool:
```

#### `Dictionary.__getstate__`  <sub>(function)</sub>

```python
def Dictionary.__getstate__(self) -> tuple[list[str], int, list[str], int]:
```

#### `Dictionary.__setstate__`  <sub>(function)</sub>

```python
def Dictionary.__setstate__(self, state: tuple[list[str], int, list[str], int]) -> None:
```

#### `Dictionary.__iter__`  <sub>(function)</sub>

```python
def Dictionary.__iter__(self) -> Iterator[str]:
```

#### `Dictionary.__len__`  <sub>(function)</sub>

```python
def Dictionary.__len__(self) -> int:
```

#### `Dictionary.generate`  <sub>(function)</sub>

> Dictionary.generate() behaviour  
>   
> Classic dirsearch wordlist:  
> 1. If %EXT% keyword is present, append one with each extension REPLACED.  
> 2. If the special word is no present, append line unmodified.  
>   
> Forced extensions wordlist (NEW):  
> This type of wordlist processing is a mix between classic processing  
> and DirBuster processing.  
> 1. If %EXT% keyword is present in the line, immediately process as "classic dirsearch" (1).  
> 2. If the line does not include the special word AND is NOT terminated by a slash,  
> append one with each extension APPENDED (line.ext) and ONLY ONE with a slash.  
> 3. If the line does not include the special word and IS ALREADY terminated by slash,  
> append line unmodified.

```python
def Dictionary.generate(
        self,
        files: list[str] = [],
        is_blacklist: bool = False,
) -> list[str] | NativeWordlistCorpus:
```

#### `Dictionary.is_valid`  <sub>(function)</sub>

```python
def Dictionary.is_valid(self, path: str) -> bool:
```

#### `Dictionary.add_extra`  <sub>(function)</sub>

> Queue a valid dynamically discovered path once.

```python
def Dictionary.add_extra(self, path: str) -> None:
```

#### `Dictionary.reset`  <sub>(function)</sub>

```python
def Dictionary.reset(self) -> None:
```

### `dirsearch/lib/core/exceptions.py`


#### `CannotConnectException`  <sub>(class)</sub>

```python
class CannotConnectException(Exception):
```

#### `FileExistsException`  <sub>(class)</sub>

```python
class FileExistsException(Exception):
```

#### `InvalidRawRequest`  <sub>(class)</sub>

```python
class InvalidRawRequest(Exception):
```

#### `InvalidURLException`  <sub>(class)</sub>

```python
class InvalidURLException(Exception):
```

#### `RequestException`  <sub>(class)</sub>

```python
class RequestException(Exception):
```

#### `SkipTargetInterrupt`  <sub>(class)</sub>

```python
class SkipTargetInterrupt(Exception):
```

#### `QuitInterrupt`  <sub>(class)</sub>

```python
class QuitInterrupt(Exception):
```

#### `UnpicklingError`  <sub>(class)</sub>

```python
class UnpicklingError(Exception):
```

#### `WordlistLimitError`  <sub>(class)</sub>

```python
class WordlistLimitError(Exception):
```

#### `WordlistBackendUnavailableError`  <sub>(class)</sub>

```python
class WordlistBackendUnavailableError(Exception):
```

### `dirsearch/lib/core/filters.py`


#### `SIZE_UNITS`  <sub>(const)</sub>

```python
SIZE_UNITS = ...
```

#### `SIZE_RE`  <sub>(const)</sub>

```python
SIZE_RE = ...
```

#### `parse_numeric_ranges`  <sub>(function)</sub>

```python
def parse_numeric_ranges(value: str | None) -> tuple[NumericRange, ...]:
```

#### `parse_time_filters`  <sub>(function)</sub>

```python
def parse_time_filters(value: str | None) -> tuple[TimeFilter, ...]:
```

#### `parse_size`  <sub>(function)</sub>

```python
def parse_size(value: str | int | None) -> int:
```

#### `parse_size_list`  <sub>(function)</sub>

```python
def parse_size_list(value: str | None) -> set[int]:
```

#### `validate_regex`  <sub>(function)</sub>

```python
def validate_regex(pattern: str | None, label: str) -> None:
```

#### `matches_numeric_ranges`  <sub>(function)</sub>

```python
def matches_numeric_ranges(value: int, ranges: tuple[NumericRange, ...]) -> bool:
```

#### `matches_time_filters`  <sub>(function)</sub>

```python
def matches_time_filters(elapsed: float, filters: tuple[TimeFilter, ...]) -> bool:
```

### `dirsearch/lib/core/fuzzer.py`


#### `AUTO_CALIBRATION_DUPLICATE_THRESHOLD`  <sub>(const)</sub>

```python
AUTO_CALIBRATION_DUPLICATE_THRESHOLD = ...
```

#### `AUTO_CALIBRATION_FORCED_THRESHOLD`  <sub>(const)</sub>

```python
AUTO_CALIBRATION_FORCED_THRESHOLD = ...
```

#### `AUTO_CALIBRATION_MIN_CONTENT_LENGTH`  <sub>(const)</sub>

```python
AUTO_CALIBRATION_MIN_CONTENT_LENGTH = ...
```

#### `response_headers_text`  <sub>(function)</sub>

```python
def response_headers_text(resp: BaseResponse) -> str:
```

#### `matches_header_text`  <sub>(function)</sub>

```python
def matches_header_text(resp: BaseResponse, patterns: list[str]) -> bool:
```

#### `matches_header_regex`  <sub>(function)</sub>

```python
def matches_header_regex(resp: BaseResponse, pattern: str) -> bool:
```

#### `BaseFuzzer`  <sub>(class)</sub>

```python
class BaseFuzzer:
```

**Fields:**
```python
def __init__(self, requester, dictionary)
def set_base_path(self, path)
def get_scanners_for(self, path)
def is_excluded(self, resp)
def matches_advanced_matchers(self, resp)
def matches_advanced_filters(self, resp)
def _combine_advanced_checks(checks, mode, default)
def is_auto_calibrated(self, resp)
def is_filter_threshold_reached(self, resp)
def should_record_auto_calibration(self, resp)
def has_advanced_matchers()
def response_fingerprint(resp)
def response_callbacks(self, path, response)
def process_response(self, path, response)
```


#### `BaseFuzzer.__init__`  <sub>(function)</sub>

```python
def BaseFuzzer.__init__(
        self,
        requester: BaseRequester,
        dictionary: Dictionary,
        *,
        match_callbacks: tuple[Callable[[BaseResponse], Any], ...],
        not_found_callbacks: tuple[Callable[[BaseResponse], Any], ...],
        error_callbacks: tuple[Callable[[RequestException], Any], ...],
) -> None:
```

#### `BaseFuzzer.set_base_path`  <sub>(function)</sub>

```python
def BaseFuzzer.set_base_path(self, path: str) -> None:
```

#### `BaseFuzzer.get_scanners_for`  <sub>(function)</sub>

```python
def BaseFuzzer.get_scanners_for(self, path: str) -> Generator[BaseScanner, None, None]:
```

#### `BaseFuzzer.is_excluded`  <sub>(function)</sub>

> Validate the response by different filters

```python
def BaseFuzzer.is_excluded(self, resp: BaseResponse) -> bool:
```

#### `BaseFuzzer.matches_advanced_matchers`  <sub>(function)</sub>

```python
def BaseFuzzer.matches_advanced_matchers(self, resp: BaseResponse) -> bool:
```

#### `BaseFuzzer.matches_advanced_filters`  <sub>(function)</sub>

```python
def BaseFuzzer.matches_advanced_filters(self, resp: BaseResponse) -> bool:
```

#### `BaseFuzzer._combine_advanced_checks`  <sub>(function)</sub>

```python
def BaseFuzzer._combine_advanced_checks(checks: list[bool], mode: str, default: bool) -> bool:
```

#### `BaseFuzzer.is_auto_calibrated`  <sub>(function)</sub>

```python
def BaseFuzzer.is_auto_calibrated(self, resp: BaseResponse) -> bool:
```

#### `BaseFuzzer.is_filter_threshold_reached`  <sub>(function)</sub>

```python
def BaseFuzzer.is_filter_threshold_reached(self, resp: BaseResponse) -> bool:
```

#### `BaseFuzzer.should_record_auto_calibration`  <sub>(function)</sub>

```python
def BaseFuzzer.should_record_auto_calibration(self, resp: BaseResponse) -> bool:
```

#### `BaseFuzzer.has_advanced_matchers`  <sub>(function)</sub>

```python
def BaseFuzzer.has_advanced_matchers() -> bool:
```

#### `BaseFuzzer.response_fingerprint`  <sub>(function)</sub>

```python
def BaseFuzzer.response_fingerprint(resp: BaseResponse) -> tuple:
```

#### `BaseFuzzer.response_callbacks`  <sub>(function)</sub>

```python
def BaseFuzzer.response_callbacks(
        self, path: str, response: BaseResponse
) -> tuple[Callable[[BaseResponse], Any], ...]:
```

#### `BaseFuzzer.process_response`  <sub>(function)</sub>

```python
def BaseFuzzer.process_response(self, path: str, response: BaseResponse) -> None:
```

#### `Fuzzer`  <sub>(class)</sub>

```python
class Fuzzer(BaseFuzzer):
```

**Fields:**
```python
def __init__(self, requester, dictionary)
def setup_scanners(self)
def setup_threads(self)
def start(self)
def is_finished(self)
def play(self)
def pause(self)
def quit(self)
def stop(self, timeout)
def _stop_with_exception(self, exception)
def scan(self, path)
def thread_proc(self)
```


#### `Fuzzer.__init__`  <sub>(function)</sub>

```python
def Fuzzer.__init__(
        self,
        requester: Requester,
        dictionary: Dictionary,
        *,
        match_callbacks: tuple[Callable[[BaseResponse], Any], ...],
        not_found_callbacks: tuple[Callable[[BaseResponse], Any], ...],
        error_callbacks: tuple[Callable[[RequestException], Any], ...],
) -> None:
```

#### `Fuzzer.setup_scanners`  <sub>(function)</sub>

```python
def Fuzzer.setup_scanners(self) -> None:
```

#### `Fuzzer.setup_threads`  <sub>(function)</sub>

```python
def Fuzzer.setup_threads(self) -> None:
```

#### `Fuzzer.start`  <sub>(function)</sub>

```python
def Fuzzer.start(self) -> None:
```

#### `Fuzzer.is_finished`  <sub>(function)</sub>

```python
def Fuzzer.is_finished(self) -> bool:
```

#### `Fuzzer.play`  <sub>(function)</sub>

```python
def Fuzzer.play(self) -> None:
```

#### `Fuzzer.pause`  <sub>(function)</sub>

> Pause all threads and wait for them to acknowledge.  
>   
> Returns True if all threads paused successfully, False if timeout occurred.

```python
def Fuzzer.pause(self) -> bool:
```

#### `Fuzzer.quit`  <sub>(function)</sub>

```python
def Fuzzer.quit(self) -> None:
```

#### `Fuzzer.stop`  <sub>(function)</sub>

```python
def Fuzzer.stop(self, timeout: float) -> bool:
```

#### `Fuzzer._stop_with_exception`  <sub>(function)</sub>

```python
def Fuzzer._stop_with_exception(self, exception: Exception) -> None:
```

#### `Fuzzer.scan`  <sub>(function)</sub>

```python
def Fuzzer.scan(self, path: str) -> None:
```

#### `Fuzzer.thread_proc`  <sub>(function)</sub>

```python
def Fuzzer.thread_proc(self) -> None:
```

#### `NativeFuzzer`  <sub>(class)</sub>

```python
class NativeFuzzer(Fuzzer):
```

**Fields:**
```python
def __init__(self, requester, dictionary)
def prepare_start(self)
def start(self)
def _process_native_chunk(self, paths, chunk)
def _process_filtered_range(self, paths, start, end)
def _process_filtered_paths(self, paths)
def _process_native_result(self, path, response, error)
def _release_paths(self, paths)
def _should_stop_processing(self)
def _next_chunk(self)
def is_finished(self)
def play(self)
def _reset_native_cancellation(self)
def pause(self)
def quit(self)
```


#### `NativeFuzzer.__init__`  <sub>(function)</sub>

```python
def NativeFuzzer.__init__(
        self,
        requester: NativeRequester,
        dictionary: Dictionary,
        *,
        match_callbacks: tuple[Callable[[BaseResponse], Any], ...],
        not_found_callbacks: tuple[Callable[[BaseResponse], Any], ...],
        error_callbacks: tuple[Callable[[RequestException], Any], ...],
        filtered_chunk_callbacks: tuple[Callable[[int], Any], ...] = (),
) -> None:
```

#### `NativeFuzzer.prepare_start`  <sub>(function)</sub>

```python
def NativeFuzzer.prepare_start(self) -> None:
```

#### `NativeFuzzer.start`  <sub>(function)</sub>

```python
def NativeFuzzer.start(self) -> None:
```

#### `NativeFuzzer._process_native_chunk`  <sub>(function)</sub>

> Expand Rust's compact event stream without rebuilding miss responses.

```python
def NativeFuzzer._process_native_chunk(
        self,
        paths: list[str] | NativeWordlistChunk,
        chunk: NativeScanChunk,
) -> None:
```

#### `NativeFuzzer._process_filtered_range`  <sub>(function)</sub>

```python
def NativeFuzzer._process_filtered_range(
        self,
        paths: list[str] | NativeWordlistChunk,
        start: int,
        end: int,
) -> None:
```

#### `NativeFuzzer._process_filtered_paths`  <sub>(function)</sub>

```python
def NativeFuzzer._process_filtered_paths(self, paths: list[str]) -> None:
```

#### `NativeFuzzer._process_native_result`  <sub>(function)</sub>

```python
def NativeFuzzer._process_native_result(
        self,
        path: str,
        response: BaseResponse | None,
        error: RequestException | None,
) -> None:
```

#### `NativeFuzzer._release_paths`  <sub>(function)</sub>

```python
def NativeFuzzer._release_paths(self, paths) -> None:
```

#### `NativeFuzzer._should_stop_processing`  <sub>(function)</sub>

```python
def NativeFuzzer._should_stop_processing(self) -> bool:
```

#### `NativeFuzzer._next_chunk`  <sub>(function)</sub>

```python
def NativeFuzzer._next_chunk(self) -> list[str] | NativeWordlistChunk:
```

#### `NativeFuzzer.is_finished`  <sub>(function)</sub>

```python
def NativeFuzzer.is_finished(self) -> bool:
```

#### `NativeFuzzer.play`  <sub>(function)</sub>

```python
def NativeFuzzer.play(self) -> None:
```

#### `NativeFuzzer._reset_native_cancellation`  <sub>(function)</sub>

```python
def NativeFuzzer._reset_native_cancellation(self) -> None:
```

#### `NativeFuzzer.pause`  <sub>(function)</sub>

```python
def NativeFuzzer.pause(self) -> bool:
```

#### `NativeFuzzer.quit`  <sub>(function)</sub>

```python
def NativeFuzzer.quit(self) -> None:
```

#### `AsyncFuzzer`  <sub>(class)</sub>

```python
class AsyncFuzzer(BaseFuzzer):
```

**Fields:**
```python
def __init__(self, requester, dictionary)
def setup_scanners(self)
def start(self)
def play(self)
def pause(self)
def quit(self)
def scan(self, path)
def run_callbacks(callbacks, value)
def task_proc(self)
```


#### `AsyncFuzzer.__init__`  <sub>(function)</sub>

```python
def AsyncFuzzer.__init__(
        self,
        requester: AsyncRequester,
        dictionary: Dictionary,
        *,
        match_callbacks: tuple[Callable[[BaseResponse], Any], ...],
        not_found_callbacks: tuple[Callable[[BaseResponse], Any], ...],
        error_callbacks: tuple[Callable[[RequestException], Any], ...],
) -> None:
```

#### `AsyncFuzzer.setup_scanners`  <sub>(function)</sub>

```python
async def AsyncFuzzer.setup_scanners(self) -> None:
```

#### `AsyncFuzzer.start`  <sub>(function)</sub>

```python
async def AsyncFuzzer.start(self) -> None:
```

#### `AsyncFuzzer.play`  <sub>(function)</sub>

```python
def AsyncFuzzer.play(self) -> None:
```

#### `AsyncFuzzer.pause`  <sub>(function)</sub>

```python
def AsyncFuzzer.pause(self) -> bool:
```

#### `AsyncFuzzer.quit`  <sub>(function)</sub>

```python
def AsyncFuzzer.quit(self) -> None:
```

#### `AsyncFuzzer.scan`  <sub>(function)</sub>

```python
async def AsyncFuzzer.scan(self, path: str) -> None:
```

#### `AsyncFuzzer.run_callbacks`  <sub>(function)</sub>

```python
async def AsyncFuzzer.run_callbacks(
        callbacks: tuple[Callable[[Any], Any], ...], value: Any
) -> None:
```

#### `AsyncFuzzer.task_proc`  <sub>(function)</sub>

```python
async def AsyncFuzzer.task_proc(self) -> None:
```

### `dirsearch/lib/core/logger.py`


#### `URL_USERINFO_PATTERN`  <sub>(const)</sub>

```python
URL_USERINFO_PATTERN = ...
```

#### `QUERY_VALUE_PATTERN`  <sub>(const)</sub>

```python
QUERY_VALUE_PATTERN = ...
```

#### `BARE_QUERY_COMPONENT_PATTERN`  <sub>(const)</sub>

```python
BARE_QUERY_COMPONENT_PATTERN = ...
```

#### `LOG_HANDLER_NAME`  <sub>(const)</sub>

```python
LOG_HANDLER_NAME = ...
```

#### `redact_log_text`  <sub>(function)</sub>

> Remove credentials and query values from rendered log output.

```python
def redact_log_text(text: str) -> str:
```

#### `RedactingFormatter`  <sub>(class)</sub>

```python
class RedactingFormatter(logging.Formatter):
```

**Fields:**
```python
def format(self, record)
```


#### `RedactingFormatter.format`  <sub>(function)</sub>

```python
def RedactingFormatter.format(self, record: logging.LogRecord) -> str:
```

#### `enable_logging`  <sub>(function)</sub>

```python
def enable_logging() -> None:
```

### `dirsearch/lib/core/native_builder.py`


#### `NATIVE_SOURCE_FILES`  <sub>(const)</sub>

```python
NATIVE_SOURCE_FILES = ...
```

#### `package_root`  <sub>(function)</sub>

```python
def package_root() -> Path:
```

#### `native_source_dir`  <sub>(function)</sub>

```python
def native_source_dir(root: Path | None = None) -> Path:
```

#### `read_os_release`  <sub>(function)</sub>

```python
def read_os_release(path: Path = Path("/etc/os-release")) -> dict[str, str]:
```

#### `install_hint`  <sub>(function)</sub>

```python
def install_hint(os_release: dict[str, str] | None = None) -> str:
```

#### `python_headers_available`  <sub>(function)</sub>

```python
def python_headers_available() -> bool:
```

#### `pip_available`  <sub>(function)</sub>

```python
def pip_available() -> bool:
```

#### `native_sources_available`  <sub>(function)</sub>

```python
def native_sources_available(source_dir: Path) -> bool:
```

#### `get_prerequisite_errors`  <sub>(function)</sub>

```python
def get_prerequisite_errors(
        source_dir: Path | None = None,
        version_info: Sequence[int] | None = None,
        which: Callable[[str], str | None] = shutil.which,
        has_pip: bool | None = None,
        has_python_headers: bool | None = None,
) -> list[str]:
```

#### `copy_native_sources`  <sub>(function)</sub>

```python
def copy_native_sources(source_dir: Path, build_root: Path) -> Path:
```

#### `run`  <sub>(function)</sub>

```python
def run(command: list[str]) -> None:
```

#### `build_native_engine`  <sub>(function)</sub>

```python
def build_native_engine(source_dir: Path, force_reinstall: bool = True) -> None:
```

#### `main`  <sub>(function)</sub>

```python
def main(argv: Sequence[str] | None = None) -> int:
```

### `dirsearch/lib/core/native_runtime.py`


#### `MIN_NATIVE_PYTHON`  <sub>(const)</sub>

```python
MIN_NATIVE_PYTHON = ...
```

#### `NATIVE_EXTENSION_VERSION`  <sub>(const)</sub>

```python
NATIVE_EXTENSION_VERSION = ...
```

#### `format_python_version`  <sub>(function)</sub>

```python
def format_python_version(version_info: Sequence[int] | None = None) -> str:
```

#### `get_native_python_version_error`  <sub>(function)</sub>

```python
def get_native_python_version_error(
        version_info: Sequence[int] | None = None,
) -> str | None:
```

#### `is_native_backend_available`  <sub>(function)</sub>

```python
def is_native_backend_available() -> bool:
```

#### `get_native_backend_install_error`  <sub>(function)</sub>

```python
def get_native_backend_install_error(
        version_info: Sequence[int] | None = None,
) -> str:
```

#### `get_native_extension_version_error`  <sub>(function)</sub>

```python
def get_native_extension_version_error(native_module: object) -> str | None:
```

#### `get_native_runtime_error`  <sub>(function)</sub>

```python
def get_native_runtime_error(
        request_backend: str = "python",
        wordlist_backend: str = "auto",
        version_info: Sequence[int] | None = None,
) -> str | None:
```

### `dirsearch/lib/core/options.py`


#### `validate_random_agent_headers`  <sub>(function)</sub>

```python
def validate_random_agent_headers(opt: Any) -> None:
```

#### `parse_options`  <sub>(function)</sub>

```python
def parse_options() -> dict[str, Any]:
```

#### `_parse_status_codes`  <sub>(function)</sub>

```python
def _parse_status_codes(str_: str) -> set[int]:
```

#### `_parse_advanced_ranges`  <sub>(function)</sub>

```python
def _parse_advanced_ranges(value: str | None, option_name: str) -> tuple[tuple[int, int], ...]:
```

#### `_parse_advanced_times`  <sub>(function)</sub>

```python
def _parse_advanced_times(value: str | None, option_name: str) -> tuple[tuple[str, float], ...]:
```

#### `_parse_size`  <sub>(function)</sub>

```python
def _parse_size(value: str | int | None, option_name: str) -> int:
```

#### `_parse_size_list`  <sub>(function)</sub>

```python
def _parse_size_list(value: str | None, option_name: str) -> set[int]:
```

#### `_validate_regex_option`  <sub>(function)</sub>

```python
def _validate_regex_option(pattern: str | None, option_name: str) -> None:
```

#### `validate_regex_options`  <sub>(function)</sub>

```python
def validate_regex_options(opt: Any) -> None:
```

#### `_validate_advanced_mode`  <sub>(function)</sub>

```python
def _validate_advanced_mode(value: str, option_name: str) -> None:
```

#### `validate_numeric_options`  <sub>(function)</sub>

```python
def validate_numeric_options(opt: Any) -> None:
```

#### `_is_cli_flag_present`  <sub>(function)</sub>

```python
def _is_cli_flag_present(*flags: str) -> bool:
```

#### `_access_file`  <sub>(function)</sub>

```python
def _access_file(path: str) -> File:
```

#### `_split_csv`  <sub>(function)</sub>

```python
def _split_csv(value: str | None) -> list[str]:
```

#### `_resolve_wordlist_categories`  <sub>(function)</sub>

```python
def _resolve_wordlist_categories(categories: list[str]) -> list[str]:
```

#### `_resolve_wordlists`  <sub>(function)</sub>

```python
def _resolve_wordlists(opt: Values) -> list[str]:
```

#### `merge_config`  <sub>(function)</sub>

```python
def merge_config(opt: Values) -> Values:
```

### `dirsearch/lib/core/request_backend.py`


#### `REQUEST_BACKENDS`  <sub>(const)</sub>

```python
REQUEST_BACKENDS = ...
```

#### `NATIVE_AUTHENTICATION_TYPES`  <sub>(const)</sub>

```python
NATIVE_AUTHENTICATION_TYPES = ...
```

#### `get_native_authentication_error`  <sub>(function)</sub>

```python
def get_native_authentication_error(auth_type: str | None) -> str | None:
```

#### `NATIVE_PROXY_SCHEMES`  <sub>(const)</sub>

```python
NATIVE_PROXY_SCHEMES = ...
```

#### `NATIVE_PROXY_SCHEME_ERROR`  <sub>(const)</sub>

```python
NATIVE_PROXY_SCHEME_ERROR = ...
```

#### `NATIVE_SOCKS4_AUTH_ERROR`  <sub>(const)</sub>

```python
NATIVE_SOCKS4_AUTH_ERROR = ...
```

#### `NATIVE_IP_PROXY_ERROR`  <sub>(const)</sub>

```python
NATIVE_IP_PROXY_ERROR = ...
```

#### `CLIENT_CERTIFICATE_PAIR_ERROR`  <sub>(const)</sub>

```python
CLIENT_CERTIFICATE_PAIR_ERROR = ...
```

#### `get_async_request_backend_error`  <sub>(function)</sub>

```python
def get_async_request_backend_error(opt: Values) -> str | None:
```

#### `get_native_request_backend_error`  <sub>(function)</sub>

```python
def get_native_request_backend_error(opt: Values) -> str | None:
```

### `dirsearch/lib/core/scanner.py`


#### `AUTO_CALIBRATION_EXTRA_SAMPLES`  <sub>(const)</sub>

```python
AUTO_CALIBRATION_EXTRA_SAMPLES = ...
```

#### `AMBIGUOUS_SIMILARITY_THRESHOLD`  <sub>(const)</sub>

```python
AMBIGUOUS_SIMILARITY_THRESHOLD = ...
```

#### `AMBIGUOUS_SIMILARITY_MAX_CONTENT_LENGTH`  <sub>(const)</sub>

```python
AMBIGUOUS_SIMILARITY_MAX_CONTENT_LENGTH = ...
```

#### `BaseScanner`  <sub>(class)</sub>

```python
class BaseScanner:
```

**Fields:**
```python
def __init__(self, requester, path, tested, context)
def check(self, path, response)
def classify(self, path, response)
def get_duplicate(self, response)
def is_wildcard(self, response)
def is_probable_wildcard(self, path, response)
def should_auto_calibrate(self)
def add_calibration_sample(self, response)
def generate_redirect_regex(first_loc, first_path, second_loc, second_path)
```


#### `BaseScanner.__init__`  <sub>(function)</sub>

```python
def BaseScanner.__init__(
        self,
        requester: BaseRequester,
        path: str = "",
        tested: dict[str, Any] = {},
        context: str = "all cases",
) -> None:
```

#### `BaseScanner.check`  <sub>(function)</sub>

> Perform analyzing to see if the response is wildcard or not

```python
def BaseScanner.check(self, path: str, response: BaseResponse) -> bool:
```

#### `BaseScanner.classify`  <sub>(function)</sub>

> Classify response against this wildcard profile.

```python
def BaseScanner.classify(self, path: str, response: BaseResponse) -> str:
```

#### `BaseScanner.get_duplicate`  <sub>(function)</sub>

```python
def BaseScanner.get_duplicate(self, response: BaseResponse) -> BaseScanner | None:
```

#### `BaseScanner.is_wildcard`  <sub>(function)</sub>

> Check if response is similar to wildcard response

```python
def BaseScanner.is_wildcard(self, response: BaseResponse) -> bool:
```

#### `BaseScanner.is_probable_wildcard`  <sub>(function)</sub>

> Conservative fallback for dynamic soft-404 templates.  
>   
> This only runs when the normal wildcard parser could not prove a match.  
> Size/line/word counts are deliberately weak signals here; the decision  
> requires high normalized body similarity and no strong redirect/content  
> divergence.

```python
def BaseScanner.is_probable_wildcard(self, path: str, response: BaseResponse) -> bool:
```

#### `BaseScanner.should_auto_calibrate`  <sub>(function)</sub>

```python
def BaseScanner.should_auto_calibrate(self) -> bool:
```

#### `BaseScanner.add_calibration_sample`  <sub>(function)</sub>

```python
def BaseScanner.add_calibration_sample(self, response: BaseResponse) -> None:
```

#### `BaseScanner.generate_redirect_regex`  <sub>(function)</sub>

> From 2 redirects of wildcard responses, generate a regexp that matches  
> every wildcard redirect.  
>   
> How it works:  
> 1. Replace path in 2 redirect URLs (if it gets reflected in) with a mark  
> (e.g. /path1 -> /foo/path1 and /path2 -> /foo/path2 will become /foo[mark] for both)  
> 2. Compare 2 redirects and generate a regex that matches both  
> (e.g. /foo[mark] and /foo[mark] will have the regex: ^/foo[mark]$)  
> 3. To check if a redirect is wildcard, replace path with the mark and check if it matches this regex  
> (e.g. /path3 -> /bar/path3, the redirect becomes /bar[mark], which doesn't match the regex ^/foo[mark]$)

```python
def BaseScanner.generate_redirect_regex(first_loc: str, first_path: str, second_loc: str, second_path: str) -> str:
```

#### `Scanner`  <sub>(class)</sub>

```python
class Scanner(BaseScanner):
```

**Fields:**
```python
def __init__(self, requester)
def setup(self)
def auto_calibrate(self, tested_paths)
```


#### `Scanner.__init__`  <sub>(function)</sub>

```python
def Scanner.__init__(
        self,
        requester: Requester,
        *,
        path: str = "",
        tested: dict[str, dict[str, Scanner]] = {},
        context: str = "all cases",
) -> None:
```

#### `Scanner.setup`  <sub>(function)</sub>

> Generate wildcard response information containers, this will be  
> used to compare with other path responses

```python
def Scanner.setup(self) -> None:
```

#### `Scanner.auto_calibrate`  <sub>(function)</sub>

```python
def Scanner.auto_calibrate(self, tested_paths: tuple[str, ...]) -> None:
```

#### `AsyncScanner`  <sub>(class)</sub>

```python
class AsyncScanner(BaseScanner):
```

**Fields:**
```python
def __init__(self, requester)
def create(cls, requester)
def setup(self)
def auto_calibrate(self, tested_paths)
```


#### `AsyncScanner.__init__`  <sub>(function)</sub>

```python
def AsyncScanner.__init__(
        self,
        requester: AsyncRequester,
        *,
        path: str = "",
        tested: dict[str, dict[str, AsyncScanner]] = {},
        context: str = "all cases",
) -> None:
```

#### `AsyncScanner.create`  <sub>(function)</sub>

```python
async def AsyncScanner.create(
        cls,
        requester: AsyncRequester,
        *,
        path: str = "",
        tested: dict[str, dict[str, AsyncScanner]] = {},
        context: str = "all cases",
) -> AsyncScanner:
```

#### `AsyncScanner.setup`  <sub>(function)</sub>

> Generate wildcard response information containers, this will be  
> used to compare with other path responses

```python
async def AsyncScanner.setup(self) -> None:
```

#### `AsyncScanner.auto_calibrate`  <sub>(function)</sub>

```python
async def AsyncScanner.auto_calibrate(self, tested_paths: tuple[str, ...]) -> None:
```

### `dirsearch/lib/core/settings.py`


#### `VERSION`  <sub>(const)</sub>

```python
VERSION = ...
```

#### `BANNER`  <sub>(const)</sub>

```python
BANNER = ...
```

#### `COMMAND`  <sub>(const)</sub>

```python
COMMAND = ...
```

#### `START_TIME`  <sub>(const)</sub>

```python
START_TIME = ...
```

#### `SCRIPT_PATH`  <sub>(const)</sub>

```python
SCRIPT_PATH = ...
```

#### `IS_WINDOWS`  <sub>(const)</sub>

```python
IS_WINDOWS = ...
```

#### `WORDLIST_CATEGORY_DIR`  <sub>(const)</sub>

```python
WORDLIST_CATEGORY_DIR = ...
```

#### `WORDLIST_CATEGORIES`  <sub>(const)</sub>

```python
WORDLIST_CATEGORIES = ...
```

#### `DEFAULT_ENCODING`  <sub>(const)</sub>

```python
DEFAULT_ENCODING = ...
```

#### `NEW_LINE`  <sub>(const)</sub>

```python
NEW_LINE = ...
```

#### `INVALID_CHARS_FOR_WINDOWS_FILENAME`  <sub>(const)</sub>

```python
INVALID_CHARS_FOR_WINDOWS_FILENAME = ...
```

#### `INVALID_FILENAME_CHAR_REPLACEMENT`  <sub>(const)</sub>

```python
INVALID_FILENAME_CHAR_REPLACEMENT = ...
```

#### `FILE_BASED_OUTPUT_FORMATS`  <sub>(const)</sub>

```python
FILE_BASED_OUTPUT_FORMATS = ...
```

#### `COMMON_EXTENSIONS`  <sub>(const)</sub>

```python
COMMON_EXTENSIONS = ...
```

#### `ARCHIVE_EXTENSIONS`  <sub>(const)</sub>

```python
ARCHIVE_EXTENSIONS = ...
```

#### `BACKUP_EXTENSIONS`  <sub>(const)</sub>

```python
BACKUP_EXTENSIONS = ...
```

#### `MEDIA_EXTENSIONS`  <sub>(const)</sub>

```python
MEDIA_EXTENSIONS = ...
```

#### `EXCLUDE_OVERWRITE_EXTENSIONS`  <sub>(const)</sub>

```python
EXCLUDE_OVERWRITE_EXTENSIONS = ...
```

#### `DB_ENGINES`  <sub>(const)</sub>

```python
DB_ENGINES = ...
```

#### `CRAWL_ATTRIBUTES`  <sub>(const)</sub>

```python
CRAWL_ATTRIBUTES = ...
```

#### `CRAWL_TAGS`  <sub>(const)</sub>

```python
CRAWL_TAGS = ...
```

#### `AUTHENTICATION_TYPES`  <sub>(const)</sub>

```python
AUTHENTICATION_TYPES = ...
```

#### `PROXY_SCHEMES`  <sub>(const)</sub>

```python
PROXY_SCHEMES = ...
```

#### `STANDARD_PORTS`  <sub>(const)</sub>

```python
STANDARD_PORTS = ...
```

#### `DEFAULT_TEST_PREFIXES`  <sub>(const)</sub>

```python
DEFAULT_TEST_PREFIXES = ...
```

#### `DEFAULT_TEST_SUFFIXES`  <sub>(const)</sub>

```python
DEFAULT_TEST_SUFFIXES = ...
```

#### `DEFAULT_TOR_PROXIES`  <sub>(const)</sub>

```python
DEFAULT_TOR_PROXIES = ...
```

#### `DEFAULT_HEADERS`  <sub>(const)</sub>

```python
DEFAULT_HEADERS = ...
```

#### `_get_default_session_dir`  <sub>(function)</sub>

```python
def _get_default_session_dir() -> str:
```

#### `DEFAULT_SESSION_DIR`  <sub>(const)</sub>

```python
DEFAULT_SESSION_DIR = ...
```

#### `DEFAULT_SESSION_FILE`  <sub>(const)</sub>

```python
DEFAULT_SESSION_FILE = ...
```

#### `REFLECTED_PATH_MARKER`  <sub>(const)</sub>

```python
REFLECTED_PATH_MARKER = ...
```

#### `WILDCARD_TEST_POINT_MARKER`  <sub>(const)</sub>

```python
WILDCARD_TEST_POINT_MARKER = ...
```

#### `EXTENSION_TAG`  <sub>(const)</sub>

```python
EXTENSION_TAG = ...
```

#### `EXTENSION_RECOGNITION_REGEX`  <sub>(const)</sub>

```python
EXTENSION_RECOGNITION_REGEX = ...
```

#### `QUERY_STRING_REGEX`  <sub>(const)</sub>

```python
QUERY_STRING_REGEX = ...
```

#### `READ_RESPONSE_ERROR_REGEX`  <sub>(const)</sub>

```python
READ_RESPONSE_ERROR_REGEX = ...
```

#### `URI_REGEX`  <sub>(const)</sub>

```python
URI_REGEX = ...
```

#### `ROBOTS_TXT_REGEX`  <sub>(const)</sub>

```python
ROBOTS_TXT_REGEX = ...
```

#### `UNKNOWN`  <sub>(const)</sub>

```python
UNKNOWN = ...
```

#### `TMP_PATH`  <sub>(const)</sub>

```python
TMP_PATH = ...
```

#### `DUMMY_DOMAIN`  <sub>(const)</sub>

```python
DUMMY_DOMAIN = ...
```

#### `DUMMY_URL`  <sub>(const)</sub>

```python
DUMMY_URL = ...
```

#### `DUMMY_WORD`  <sub>(const)</sub>

```python
DUMMY_WORD = ...
```

#### `DB_CONNECTION_TIMEOUT`  <sub>(const)</sub>

```python
DB_CONNECTION_TIMEOUT = ...
```

#### `SOCKET_TIMEOUT`  <sub>(const)</sub>

```python
SOCKET_TIMEOUT = ...
```

#### `RATE_UPDATE_DELAY`  <sub>(const)</sub>

```python
RATE_UPDATE_DELAY = ...
```

#### `MAX_REDIRECTS`  <sub>(const)</sub>

```python
MAX_REDIRECTS = ...
```

#### `ITER_CHUNK_SIZE`  <sub>(const)</sub>

```python
ITER_CHUNK_SIZE = ...
```

#### `MAX_RESPONSE_SIZE`  <sub>(const)</sub>

```python
MAX_RESPONSE_SIZE = ...
```

#### `MAX_RESPONSE_FILENAME_LENGTH`  <sub>(const)</sub>

```python
MAX_RESPONSE_FILENAME_LENGTH = ...
```

#### `TEST_PATH_LENGTH`  <sub>(const)</sub>

```python
TEST_PATH_LENGTH = ...
```

#### `MAX_CONSECUTIVE_REQUEST_ERRORS`  <sub>(const)</sub>

```python
MAX_CONSECUTIVE_REQUEST_ERRORS = ...
```

#### `SIGINT_WINDOW_SECONDS`  <sub>(const)</sub>

```python
SIGINT_WINDOW_SECONDS = ...
```

#### `SIGINT_FORCE_QUIT_THRESHOLD`  <sub>(const)</sub>

```python
SIGINT_FORCE_QUIT_THRESHOLD = ...
```

#### `NATIVE_PAUSE_TIMEOUT`  <sub>(const)</sub>

```python
NATIVE_PAUSE_TIMEOUT = ...
```

#### `NATIVE_WORKER_POLL_INTERVAL`  <sub>(const)</sub>

```python
NATIVE_WORKER_POLL_INTERVAL = ...
```

#### `NATIVE_WORKER_SHUTDOWN_TIMEOUT`  <sub>(const)</sub>

```python
NATIVE_WORKER_SHUTDOWN_TIMEOUT = ...
```

#### `THREADED_WORKER_SHUTDOWN_TIMEOUT`  <sub>(const)</sub>

```python
THREADED_WORKER_SHUTDOWN_TIMEOUT = ...
```

#### `URL_SAFE_CHARS`  <sub>(const)</sub>

```python
URL_SAFE_CHARS = ...
```

#### `TEXT_CHARS`  <sub>(const)</sub>

```python
TEXT_CHARS = ...
```

### `dirsearch/lib/core/structures.py`


#### `CaseInsensitiveDict`  <sub>(class)</sub>

```python
class CaseInsensitiveDict(dict):
```

**Fields:**
```python
def __init__(self)
def __setitem__(self, key, value)
def __getitem__(self, key)
def _convert_keys(self)
```


#### `CaseInsensitiveDict.__init__`  <sub>(function)</sub>

```python
def CaseInsensitiveDict.__init__(self, *args: Any, **kwargs: Any) -> None:
```

#### `CaseInsensitiveDict.__setitem__`  <sub>(function)</sub>

```python
def CaseInsensitiveDict.__setitem__(self, key: Any, value: Any) -> None:
```

#### `CaseInsensitiveDict.__getitem__`  <sub>(function)</sub>

```python
def CaseInsensitiveDict.__getitem__(self, key: Any) -> Any:
```

#### `CaseInsensitiveDict._convert_keys`  <sub>(function)</sub>

```python
def CaseInsensitiveDict._convert_keys(self) -> None:
```

#### `OrderedSet`  <sub>(class)</sub>

```python
class OrderedSet:
```

**Fields:**
```python
def __init__(self, items)
def __contains__(self, item)
def __eq__(self, other)
def __iter__(self)
def __len__(self)
def add(self, item)
def clear(self)
def discard(self, item)
def pop(self)
def remove(self, item)
def update(self, items)
```


#### `OrderedSet.__init__`  <sub>(function)</sub>

```python
def OrderedSet.__init__(self, items: list[Any] = []) -> None:
```

#### `OrderedSet.__contains__`  <sub>(function)</sub>

```python
def OrderedSet.__contains__(self, item: Any) -> bool:
```

#### `OrderedSet.__eq__`  <sub>(function)</sub>

```python
def OrderedSet.__eq__(self, other: Any) -> bool:
```

#### `OrderedSet.__iter__`  <sub>(function)</sub>

```python
def OrderedSet.__iter__(self) -> Iterator[Any]:
```

#### `OrderedSet.__len__`  <sub>(function)</sub>

```python
def OrderedSet.__len__(self) -> int:
```

#### `OrderedSet.add`  <sub>(function)</sub>

```python
def OrderedSet.add(self, item: Any) -> None:
```

#### `OrderedSet.clear`  <sub>(function)</sub>

```python
def OrderedSet.clear(self) -> None:
```

#### `OrderedSet.discard`  <sub>(function)</sub>

```python
def OrderedSet.discard(self, item: Any) -> None:
```

#### `OrderedSet.pop`  <sub>(function)</sub>

```python
def OrderedSet.pop(self) -> None:
```

#### `OrderedSet.remove`  <sub>(function)</sub>

```python
def OrderedSet.remove(self, item: Any) -> None:
```

#### `OrderedSet.update`  <sub>(function)</sub>

```python
def OrderedSet.update(self, items: list[Any]) -> None:
```

### `dirsearch/lib/core/wordlist_backend.py`


#### `WORDLIST_BACKENDS`  <sub>(const)</sub>

```python
WORDLIST_BACKENDS = ...
```

#### `NativeWordlistChunk`  <sub>(class)</sub>

> Python ownership token for a range that remains stored in Rust.

```python
class NativeWordlistChunk:
```

**Fields:**
```python
def __init__(self, native_chunk)
def __len__(self)
def path_at(self, index)
def to_list(self)
```


#### `NativeWordlistChunk.__init__`  <sub>(function)</sub>

```python
def NativeWordlistChunk.__init__(self, native_chunk: Any) -> None:
```

#### `NativeWordlistChunk.__len__`  <sub>(function)</sub>

```python
def NativeWordlistChunk.__len__(self) -> int:
```

#### `NativeWordlistChunk.path_at`  <sub>(function)</sub>

```python
def NativeWordlistChunk.path_at(self, index: int) -> str:
```

#### `NativeWordlistChunk.to_list`  <sub>(function)</sub>

```python
def NativeWordlistChunk.to_list(self) -> list[str]:
```

#### `NativeWordlistCorpus`  <sub>(class)</sub>

> Sequence facade that materializes Python strings only when requested.

```python
class NativeWordlistCorpus:
```

**Fields:**
```python
def __init__(self, native_wordlist)
def __len__(self)
def __getitem__(self, index)
def __iter__(self)
def __contains__(self, path)
def to_list(self)
def chunk(self, start, count, base_path)
```


#### `NativeWordlistCorpus.__init__`  <sub>(function)</sub>

```python
def NativeWordlistCorpus.__init__(self, native_wordlist: Any) -> None:
```

#### `NativeWordlistCorpus.__len__`  <sub>(function)</sub>

```python
def NativeWordlistCorpus.__len__(self) -> int:
```

#### `NativeWordlistCorpus.__getitem__`  <sub>(function)</sub>

```python
def NativeWordlistCorpus.__getitem__(self, index: int | slice) -> str | list[str]:
```

#### `NativeWordlistCorpus.__iter__`  <sub>(function)</sub>

```python
def NativeWordlistCorpus.__iter__(self) -> Iterator[str]:
```

#### `NativeWordlistCorpus.__contains__`  <sub>(function)</sub>

```python
def NativeWordlistCorpus.__contains__(self, path: object) -> bool:
```

#### `NativeWordlistCorpus.to_list`  <sub>(function)</sub>

```python
def NativeWordlistCorpus.to_list(self) -> list[str]:
```

#### `NativeWordlistCorpus.chunk`  <sub>(function)</sub>

```python
def NativeWordlistCorpus.chunk(self, start: int, count: int, base_path: str) -> NativeWordlistChunk:
```

#### `WordlistBackend`  <sub>(class)</sub>

```python
class WordlistBackend(Protocol):
```

**Fields:**
```python
name: ...
def generate(self, files, is_blacklist)
def is_valid(self, path)
```


#### `WordlistBackend.generate`  <sub>(function)</sub>

```python
def WordlistBackend.generate(
        self, files: list[str], is_blacklist: bool = False
) -> list[str] | NativeWordlistCorpus:
```

#### `WordlistBackend.is_valid`  <sub>(function)</sub>

```python
def WordlistBackend.is_valid(self, path: str) -> bool:
```

#### `is_valid_path`  <sub>(function)</sub>

```python
def is_valid_path(path: str) -> bool:
```

#### `PythonWordlistBackend`  <sub>(class)</sub>

```python
class PythonWordlistBackend:
```

**Fields:**
```python
name = ...
def generate(self, files, is_blacklist)
def is_valid(self, path)
def _add_wordlist_entry(self, wordlist, path)
```


#### `PythonWordlistBackend.generate`  <sub>(function)</sub>

```python
def PythonWordlistBackend.generate(
        self, files: list[str], is_blacklist: bool = False
) -> list[str] | NativeWordlistCorpus:
```

#### `PythonWordlistBackend.is_valid`  <sub>(function)</sub>

```python
def PythonWordlistBackend.is_valid(self, path: str) -> bool:
```

#### `PythonWordlistBackend._add_wordlist_entry`  <sub>(function)</sub>

```python
def PythonWordlistBackend._add_wordlist_entry(self, wordlist: OrderedSet, path: str) -> None:
```

#### `NativeWordlistBackend`  <sub>(class)</sub>

```python
class NativeWordlistBackend:
```

**Fields:**
```python
name = ...
def __init__(self)
def generate(self, files, is_blacklist)
def is_valid(self, path)
def _requires_python_template_expansion(self, files)
```


#### `NativeWordlistBackend.__init__`  <sub>(function)</sub>

```python
def NativeWordlistBackend.__init__(self) -> None:
```

#### `NativeWordlistBackend.generate`  <sub>(function)</sub>

```python
def NativeWordlistBackend.generate(
        self, files: list[str], is_blacklist: bool = False
) -> list[str] | NativeWordlistCorpus:
```

#### `NativeWordlistBackend.is_valid`  <sub>(function)</sub>

```python
def NativeWordlistBackend.is_valid(self, path: str) -> bool:
```

#### `NativeWordlistBackend._requires_python_template_expansion`  <sub>(function)</sub>

```python
def NativeWordlistBackend._requires_python_template_expansion(self, files: list[str]) -> bool:
```

#### `get_wordlist_backend`  <sub>(function)</sub>

```python
def get_wordlist_backend(name: str | None = None) -> WordlistBackend:
```

### `dirsearch/lib/core/wordlist_template.py`


#### `TOKEN_RE`  <sub>(const)</sub>

```python
TOKEN_RE = ...
```

#### `DATE_PLACEHOLDERS`  <sub>(const)</sub>

```python
DATE_PLACEHOLDERS = ...
```

#### `is_template_token`  <sub>(function)</sub>

```python
def is_template_token(token: str) -> bool:
```

#### `generate_backup_paths`  <sub>(function)</sub>

> Yield backup candidates for a discovered file path.

```python
def generate_backup_paths(path: str) -> Iterator[str]:
```

#### `_has_extension`  <sub>(function)</sub>

```python
def _has_extension(path: str, extensions: Iterable[str]) -> bool:
```

#### `normalize_placeholders`  <sub>(function)</sub>

```python
def normalize_placeholders(
        placeholders: Mapping[str, Iterable[str] | str] | None,
) -> dict[str, tuple[str, ...]]:
```

#### `expand_template_line`  <sub>(function)</sub>

```python
def expand_template_line(
        line: str,
        *,
        extensions: Iterable[str] = (),
        placeholders: Mapping[str, Iterable[str] | str] | None = None,
) -> Iterator[str]:
```

#### `_placeholder_values`  <sub>(function)</sub>

```python
def _placeholder_values(
        extensions: Iterable[str],
        placeholders: Mapping[str, Iterable[str] | str] | None,
) -> dict[str, tuple[str, ...]]:
```

#### `_resolve_token`  <sub>(function)</sub>

```python
def _resolve_token(token: str, values: Mapping[str, tuple[str, ...]]) -> tuple[str, ...] | None:
```

#### `_load_category`  <sub>(function)</sub>

```python
def _load_category(name: str) -> tuple[str, ...]:
```


## `lib/parse` <a id='pkg-lib-parse'></a>

*40 documented items*


### `dirsearch/lib/parse/cmdline.py`


#### `COMMON_HELP_OPTIONS`  <sub>(const)</sub>

```python
COMMON_HELP_OPTIONS = ...
```

#### `DirsearchOptionParser`  <sub>(class)</sub>

```python
class DirsearchOptionParser(OptionParser):
```

**Fields:**
```python
def _process_short_opts(self, rargs, values)
```


#### `DirsearchOptionParser._process_short_opts`  <sub>(function)</sub>

```python
def DirsearchOptionParser._process_short_opts(self, rargs, values) -> None:
```

#### `_iter_options`  <sub>(function)</sub>

```python
def _iter_options(parser: OptionParser):
```

#### `_is_common_help_option`  <sub>(function)</sub>

```python
def _is_common_help_option(option) -> bool:
```

#### `_build_common_help_parser`  <sub>(function)</sub>

```python
def _build_common_help_parser(parser: OptionParser) -> OptionParser:
```

#### `_show_common_help`  <sub>(function)</sub>

```python
def _show_common_help(option, option_string, value, parser: OptionParser) -> None:
```

#### `_show_all_help`  <sub>(function)</sub>

```python
def _show_all_help(option, option_string, value, parser: OptionParser) -> None:
```

#### `parse_arguments`  <sub>(function)</sub>

```python
def parse_arguments(arguments: list[str] | None = None) -> Values:
```

### `dirsearch/lib/parse/config.py`


#### `ConfigParser`  <sub>(class)</sub>

```python
class ConfigParser(configparser.ConfigParser):
```

**Fields:**
```python
def safe_get(self, section, option, default, allowed)
def safe_getfloat(self, section, option, default, allowed)
def safe_getboolean(self, section, option, default, allowed)
def safe_getint(self, section, option, default, allowed)
def safe_getlist(self, section, option, default, allowed)
```


#### `ConfigParser.safe_get`  <sub>(function)</sub>

```python
def ConfigParser.safe_get(
        self,
        section: str,
        option: str,
        default: str | None = None,
        allowed: tuple[str, ...] | None = None,
) -> str | None:
```

#### `ConfigParser.safe_getfloat`  <sub>(function)</sub>

```python
def ConfigParser.safe_getfloat(
        self,
        section: str,
        option: str,
        default: float = 0.0,
        allowed: tuple[float, ...] | None = None,
) -> float:
```

#### `ConfigParser.safe_getboolean`  <sub>(function)</sub>

```python
def ConfigParser.safe_getboolean(
        self,
        section: str,
        option: str,
        default: bool = False,
        allowed: tuple[bool, ...] | None = None,
) -> bool:
```

#### `ConfigParser.safe_getint`  <sub>(function)</sub>

```python
def ConfigParser.safe_getint(
        self,
        section: str,
        option: str,
        default: int = 0,
        allowed: tuple[int, ...] | None = None,
) -> int:
```

#### `ConfigParser.safe_getlist`  <sub>(function)</sub>

```python
def ConfigParser.safe_getlist(
        self,
        section: str,
        option: str,
        default: list[str] = [],
        allowed: tuple[str, ...] | None = None,
) -> list[str]:
```

### `dirsearch/lib/parse/headers.py`


#### `HeadersParser`  <sub>(class)</sub>

```python
class HeadersParser:
```

**Fields:**
```python
def __init__(self, headers)
def get(self, key)
def str_to_dict(headers)
def dict_to_str(headers)
def __iter__(self)
def __str__(self)
```


#### `HeadersParser.__init__`  <sub>(function)</sub>

```python
def HeadersParser.__init__(self, headers: str | dict[str, str]) -> None:
```

#### `HeadersParser.get`  <sub>(function)</sub>

```python
def HeadersParser.get(self, key: str) -> str:
```

#### `HeadersParser.str_to_dict`  <sub>(function)</sub>

```python
def HeadersParser.str_to_dict(headers: str) -> dict[str, str]:
```

#### `HeadersParser.dict_to_str`  <sub>(function)</sub>

```python
def HeadersParser.dict_to_str(headers: dict[str, str]) -> str:
```

#### `HeadersParser.__iter__`  <sub>(function)</sub>

```python
def HeadersParser.__iter__(self):
```

#### `HeadersParser.__str__`  <sub>(function)</sub>

```python
def HeadersParser.__str__(self) -> str:
```

### `dirsearch/lib/parse/nmap.py`


#### `parse_nmap`  <sub>(function)</sub>

```python
def parse_nmap(file: str) -> list[str]:
```

### `dirsearch/lib/parse/rawrequest.py`


#### `HEADER_ENCODING`  <sub>(const)</sub>

```python
HEADER_ENCODING = ...
```

#### `RawRequest`  <sub>(class)</sub>

```python
class RawRequest:
```

**Fields:**
```python
url: ...
method: ...
headers: ...
body: ...
```


#### `_raw_bytes`  <sub>(function)</sub>

```python
def _raw_bytes(raw_content: str | bytes) -> bytes:
```

#### `_split_head_body`  <sub>(function)</sub>

```python
def _split_head_body(raw_content: bytes) -> tuple[bytes, bytes | None]:
```

#### `_origin_form_target`  <sub>(function)</sub>

```python
def _origin_form_target(target: str, host: str, scheme: str | None) -> str:
```

#### `_absolute_form_target`  <sub>(function)</sub>

```python
def _absolute_form_target(target: str) -> str | None:
```

#### `_target_from_request_line`  <sub>(function)</sub>

```python
def _target_from_request_line(
        target: str,
        headers: HeadersParser,
        *,
        scheme: str | None = None,
) -> str:
```

#### `parse_raw_content`  <sub>(function)</sub>

```python
def parse_raw_content(
        raw_content: str | bytes,
        *,
        scheme: str | None = None,
) -> RawRequest:
```

#### `parse_raw`  <sub>(function)</sub>

```python
def parse_raw(
        raw_file: str,
        *,
        scheme: str | None = None,
) -> tuple[list[str], str, dict[str, str], bytes | None]:
```

### `dirsearch/lib/parse/url.py`


#### `_DEFAULT_PORTS = ...`  <sub>(const)</sub>

```python
_DEFAULT_PORTS = ...
```

#### `_origin`  <sub>(function)</sub>

```python
def _origin(value: str) -> tuple[str, str, int | None] | None:
```

#### `same_origin`  <sub>(function)</sub>

```python
def same_origin(first: str, second: str) -> bool:
```

#### `same_origin_path`  <sub>(function)</sub>

```python
def same_origin_path(base_url: str, location: str) -> str | None:
```

#### `clean_path`  <sub>(function)</sub>

```python
def clean_path(path: str, keep_queries: bool = False, keep_fragment: bool = False) -> str:
```

#### `parse_path`  <sub>(function)</sub>

```python
def parse_path(value: str) -> str:
```

#### `ensure_trailing_path_slash`  <sub>(function)</sub>

```python
def ensure_trailing_path_slash(url: str) -> str:
```

#### `append_query_string`  <sub>(function)</sub>

```python
def append_query_string(value: str, query: str) -> str:
```


## `lib/report` <a id='pkg-lib-report'></a>

*126 documented items*


### `dirsearch/lib/report/csv_report.py`


#### `CSVReport`  <sub>(class)</sub>

```python
class CSVReport(FileReportMixin, BaseReport):
```

**Fields:**
```python
__format__ = ...
__extension__ = ...
_newline = ...
def new(self)
def parse(self, file)
def save(self, file, result)
def write(self, file, rows)
```


#### `CSVReport.new`  <sub>(function)</sub>

```python
def CSVReport.new(self):
```

#### `CSVReport.parse`  <sub>(function)</sub>

```python
def CSVReport.parse(self, file):
```

#### `CSVReport.save`  <sub>(function)</sub>

```python
def CSVReport.save(self, file, result):
```

#### `CSVReport.write`  <sub>(function)</sub>

```python
def CSVReport.write(self, file, rows):
```

### `dirsearch/lib/report/directory_response_store.py`


#### `_truncate_filename`  <sub>(function)</sub>

```python
def _truncate_filename(value: str, max_bytes: int) -> str:
```

#### `response_filename`  <sub>(function)</sub>

> Build a portable filename without exposing the raw query string.

```python
def response_filename(
        url: str,
        status: int,
        max_length: int = MAX_RESPONSE_FILENAME_LENGTH,
) -> str:
```

#### `DirectoryResponseStore`  <sub>(class)</sub>

> Save one raw response body per exclusively created file.

```python
class DirectoryResponseStore(BaseResponseStore):
```

**Fields:**
```python
name = ...
def __init__(self, directory)
def save(self, artifact)
def _claim_file(self, base_name)
```


#### `DirectoryResponseStore.__init__`  <sub>(function)</sub>

```python
def DirectoryResponseStore.__init__(self, directory: str) -> None:
```

#### `DirectoryResponseStore.save`  <sub>(function)</sub>

```python
def DirectoryResponseStore.save(self, artifact: ResponseArtifact) -> str:
```

#### `DirectoryResponseStore._claim_file`  <sub>(function)</sub>

```python
def DirectoryResponseStore._claim_file(self, base_name: str) -> tuple[str, int]:
```

### `dirsearch/lib/report/factory.py`


#### `REPORT_PARSE_ERRORS`  <sub>(const)</sub>

```python
REPORT_PARSE_ERRORS = ...
```

#### `SQL_CONNECTION_ERRORS`  <sub>(const)</sub>

```python
SQL_CONNECTION_ERRORS = ...
```

#### `BaseReport`  <sub>(class)</sub>

```python
class BaseReport(ABC):
```

**Fields:**
```python
def __init__(self)
def initiate(self)
def save(self, result)
def flush(self)
```


#### `BaseReport.__init__`  <sub>(function)</sub>

```python
def BaseReport.__init__(self):
```

#### `BaseReport.initiate`  <sub>(function)</sub>

```python
def BaseReport.initiate(self):
```

#### `BaseReport.save`  <sub>(function)</sub>

```python
def BaseReport.save(self, result):
```

#### `BaseReport.flush`  <sub>(function)</sub>

```python
def BaseReport.flush(self):
```

#### `FileReportMixin`  <sub>(class)</sub>

```python
class FileReportMixin:
```

**Fields:**
```python
_newline = ...
def initiate(self, file)
def validate(self, file)
def parse(self, file)
def _atomic_writer(self, file)
def write(self, file, data)
def append(self, file, data)
def finish(self)
```


#### `FileReportMixin.initiate`  <sub>(function)</sub>

```python
def FileReportMixin.initiate(self, file):
```

#### `FileReportMixin.validate`  <sub>(function)</sub>

```python
def FileReportMixin.validate(self, file):
```

#### `FileReportMixin.parse`  <sub>(function)</sub>

```python
def FileReportMixin.parse(self, file):
```

#### `FileReportMixin._atomic_writer`  <sub>(function)</sub>

```python
def FileReportMixin._atomic_writer(self, file):
```

#### `FileReportMixin.write`  <sub>(function)</sub>

```python
def FileReportMixin.write(self, file, data):
```

#### `FileReportMixin.append`  <sub>(function)</sub>

```python
def FileReportMixin.append(self, file, data):
```

#### `FileReportMixin.finish`  <sub>(function)</sub>

```python
def FileReportMixin.finish(self):
```

#### `_StructuredReportState`  <sub>(class)</sub>

```python
class _StructuredReportState:
```

**Fields:**
```python
journal_base_hash: ...
journal_entries: ...
applied_entries: ...
```


#### `StructuredFileReportMixin`  <sub>(class)</sub>

> Persist entries in a journal and compact them into one final snapshot.

```python
class StructuredFileReportMixin(FileReportMixin):
```

**Fields:**
```python
_journal_version = ...
def __init__(self)
def journal_path(file)
def _state_for_journal(self, data)
def _new_state(self)
def _state_hash(self, data)
def _apply_entry(self, data, entry)
def _journal_record(self, record)
def _write_journal_header(self, file, state)
def _append_journal_record(self, file, state, record)
def _read_journal(self, file, data)
def initiate(self, file)
def save_entry(self, file, entry)
def _compact(self, file, state)
def flush(self)
finish = ...
```


#### `StructuredFileReportMixin.__init__`  <sub>(function)</sub>

```python
def StructuredFileReportMixin.__init__(self):
```

#### `StructuredFileReportMixin.journal_path`  <sub>(function)</sub>

```python
def StructuredFileReportMixin.journal_path(file):
```

#### `StructuredFileReportMixin._state_for_journal`  <sub>(function)</sub>

```python
def StructuredFileReportMixin._state_for_journal(self, data):
```

#### `StructuredFileReportMixin._new_state`  <sub>(function)</sub>

```python
def StructuredFileReportMixin._new_state(self):
```

#### `StructuredFileReportMixin._state_hash`  <sub>(function)</sub>

```python
def StructuredFileReportMixin._state_hash(self, data):
```

#### `StructuredFileReportMixin._apply_entry`  <sub>(function)</sub>

```python
def StructuredFileReportMixin._apply_entry(self, data, entry):
```

#### `StructuredFileReportMixin._journal_record`  <sub>(function)</sub>

```python
def StructuredFileReportMixin._journal_record(self, record):
```

#### `StructuredFileReportMixin._write_journal_header`  <sub>(function)</sub>

```python
def StructuredFileReportMixin._write_journal_header(self, file, state):
```

#### `StructuredFileReportMixin._append_journal_record`  <sub>(function)</sub>

```python
def StructuredFileReportMixin._append_journal_record(self, file, state, record):
```

#### `StructuredFileReportMixin._read_journal`  <sub>(function)</sub>

```python
def StructuredFileReportMixin._read_journal(self, file, data):
```

#### `StructuredFileReportMixin.initiate`  <sub>(function)</sub>

```python
def StructuredFileReportMixin.initiate(self, file):
```

#### `StructuredFileReportMixin.save_entry`  <sub>(function)</sub>

```python
def StructuredFileReportMixin.save_entry(self, file, entry):
```

#### `StructuredFileReportMixin._compact`  <sub>(function)</sub>

```python
def StructuredFileReportMixin._compact(self, file, state):
```

#### `StructuredFileReportMixin.flush`  <sub>(function)</sub>

```python
def StructuredFileReportMixin.flush(self):
```

#### `SQLReportMixin`  <sub>(class)</sub>

```python
class SQLReportMixin:
```

**Fields:**
```python
_conn = ...
_conn_database = ...
def get_connection(self, database)
def _commit(self, conn)
def _after_save(self, conn)
def _close_connection(self)
def get_create_table_query(self, table)
def get_insert_table_query(self, table, values)
def initiate(self, database, table)
def save(self, database, table, result)
def flush(self)
def finish(self)
```


#### `SQLReportMixin.get_connection`  <sub>(function)</sub>

```python
def SQLReportMixin.get_connection(self, database):
```

#### `SQLReportMixin._commit`  <sub>(function)</sub>

```python
def SQLReportMixin._commit(self, conn):
```

#### `SQLReportMixin._after_save`  <sub>(function)</sub>

```python
def SQLReportMixin._after_save(self, conn):
```

#### `SQLReportMixin._close_connection`  <sub>(function)</sub>

```python
def SQLReportMixin._close_connection(self):
```

#### `SQLReportMixin.get_create_table_query`  <sub>(function)</sub>

```python
def SQLReportMixin.get_create_table_query(self, table):
```

#### `SQLReportMixin.get_insert_table_query`  <sub>(function)</sub>

```python
def SQLReportMixin.get_insert_table_query(self, table, values):
```

#### `SQLReportMixin.initiate`  <sub>(function)</sub>

```python
def SQLReportMixin.initiate(self, database, table):
```

#### `SQLReportMixin.save`  <sub>(function)</sub>

```python
def SQLReportMixin.save(self, database, table, result):
```

#### `SQLReportMixin.flush`  <sub>(function)</sub>

```python
def SQLReportMixin.flush(self):
```

#### `SQLReportMixin.finish`  <sub>(function)</sub>

```python
def SQLReportMixin.finish(self):
```

### `dirsearch/lib/report/html_report.py`


#### `HTMLReport`  <sub>(class)</sub>

```python
class HTMLReport(StructuredFileReportMixin, BaseReport):
```

**Fields:**
```python
__format__ = ...
__extension__ = ...
def new(self)
def _new_state(self)
def parse(self, file)
def _apply_entry(results, entry)
def save(self, file, result)
def write(self, file, results)
def generate(self, results)
```


#### `HTMLReport.new`  <sub>(function)</sub>

```python
def HTMLReport.new(self):
```

#### `HTMLReport._new_state`  <sub>(function)</sub>

```python
def HTMLReport._new_state(self):
```

#### `HTMLReport.parse`  <sub>(function)</sub>

```python
def HTMLReport.parse(self, file):
```

#### `HTMLReport._apply_entry`  <sub>(function)</sub>

```python
def HTMLReport._apply_entry(results, entry):
```

#### `HTMLReport.save`  <sub>(function)</sub>

```python
def HTMLReport.save(self, file, result):
```

#### `HTMLReport.write`  <sub>(function)</sub>

```python
def HTMLReport.write(self, file, results):
```

#### `HTMLReport.generate`  <sub>(function)</sub>

```python
def HTMLReport.generate(self, results):
```

### `dirsearch/lib/report/json_report.py`


#### `JSONReport`  <sub>(class)</sub>

```python
class JSONReport(StructuredFileReportMixin, BaseReport):
```

**Fields:**
```python
__format__ = ...
__extension__ = ...
def new(self)
def parse(self, file)
def _apply_entry(data, entry)
def save(self, file, result)
def write(self, file, data)
```


#### `JSONReport.new`  <sub>(function)</sub>

```python
def JSONReport.new(self):
```

#### `JSONReport.parse`  <sub>(function)</sub>

```python
def JSONReport.parse(self, file):
```

#### `JSONReport._apply_entry`  <sub>(function)</sub>

```python
def JSONReport._apply_entry(data, entry):
```

#### `JSONReport.save`  <sub>(function)</sub>

```python
def JSONReport.save(self, file, result):
```

#### `JSONReport.write`  <sub>(function)</sub>

```python
def JSONReport.write(self, file, data):
```

### `dirsearch/lib/report/jsonl_response_store.py`


#### `JSONL_RESPONSE_SCHEMA`  <sub>(const)</sub>

```python
JSONL_RESPONSE_SCHEMA = ...
```

#### `JSONL_BASE64_CHUNK_SIZE`  <sub>(const)</sub>

```python
JSONL_BASE64_CHUNK_SIZE = ...
```

#### `JsonlResponseStore`  <sub>(class)</sub>

> Append complete JSONL records under a lock owned by this output file.

```python
class JsonlResponseStore(BaseResponseStore):
```

**Fields:**
```python
name = ...
def __init__(self, file_path)
def _validate_existing_file(self)
def _existing_file_needs_separator(self)
def save(self, artifact)
def _write_all(self, data)
def close(self)
```


#### `JsonlResponseStore.__init__`  <sub>(function)</sub>

```python
def JsonlResponseStore.__init__(self, file_path: str) -> None:
```

#### `JsonlResponseStore._validate_existing_file`  <sub>(function)</sub>

```python
def JsonlResponseStore._validate_existing_file(self) -> None:
```

#### `JsonlResponseStore._existing_file_needs_separator`  <sub>(function)</sub>

```python
def JsonlResponseStore._existing_file_needs_separator(self) -> bool:
```

#### `JsonlResponseStore.save`  <sub>(function)</sub>

```python
def JsonlResponseStore.save(self, artifact: ResponseArtifact) -> str:
```

#### `JsonlResponseStore._write_all`  <sub>(function)</sub>

```python
def JsonlResponseStore._write_all(self, data: bytes) -> None:
```

#### `JsonlResponseStore.close`  <sub>(function)</sub>

```python
def JsonlResponseStore.close(self) -> None:
```

### `dirsearch/lib/report/manager.py`


#### `ReportManager`  <sub>(class)</sub>

```python
class ReportManager:
```

**Fields:**
```python
def __init__(self, formats)
def _load_report(self, handler)
def prepare(self, target)
def save(self, result)
def save_async(self, result)
def flush(self)
def finish(self)
def format(self, string, target, handler)
```


#### `ReportManager.__init__`  <sub>(function)</sub>

```python
def ReportManager.__init__(self, formats):
```

#### `ReportManager._load_report`  <sub>(function)</sub>

```python
def ReportManager._load_report(self, handler):
```

#### `ReportManager.prepare`  <sub>(function)</sub>

```python
def ReportManager.prepare(self, target):
```

#### `ReportManager.save`  <sub>(function)</sub>

```python
def ReportManager.save(self, result):
```

#### `ReportManager.save_async`  <sub>(function)</sub>

```python
async def ReportManager.save_async(self, result):
```

#### `ReportManager.flush`  <sub>(function)</sub>

```python
def ReportManager.flush(self):
```

#### `ReportManager.finish`  <sub>(function)</sub>

```python
def ReportManager.finish(self):
```

#### `ReportManager.format`  <sub>(function)</sub>

```python
def ReportManager.format(self, string, target, handler):
```

### `dirsearch/lib/report/markdown_report.py`


#### `MarkdownReport`  <sub>(class)</sub>

```python
class MarkdownReport(FileReportMixin, BaseReport):
```

**Fields:**
```python
__format__ = ...
__extension__ = ...
def new(self)
def save(self, file, result)
```


#### `MarkdownReport.new`  <sub>(function)</sub>

```python
def MarkdownReport.new(self):
```

#### `MarkdownReport.save`  <sub>(function)</sub>

```python
def MarkdownReport.save(self, file, result):
```

### `dirsearch/lib/report/mysql_report.py`


#### `MySQLReport`  <sub>(class)</sub>

```python
class MySQLReport(SQLReportMixin, BaseReport):
```

**Fields:**
```python
__format__ = ...
__extension__ = ...
_reuse = ...
def is_valid(self, url)
def connect(self, url)
```


#### `MySQLReport.is_valid`  <sub>(function)</sub>

```python
def MySQLReport.is_valid(self, url):
```

#### `MySQLReport.connect`  <sub>(function)</sub>

```python
def MySQLReport.connect(self, url):
```

### `dirsearch/lib/report/plain_text_report.py`


#### `PlainTextReport`  <sub>(class)</sub>

```python
class PlainTextReport(FileReportMixin, BaseReport):
```

**Fields:**
```python
__format__ = ...
__extension__ = ...
def new(self)
def save(self, file, result)
```


#### `PlainTextReport.new`  <sub>(function)</sub>

```python
def PlainTextReport.new(self):
```

#### `PlainTextReport.save`  <sub>(function)</sub>

```python
def PlainTextReport.save(self, file, result):
```

### `dirsearch/lib/report/postgresql_report.py`


#### `PostgreSQLReport`  <sub>(class)</sub>

```python
class PostgreSQLReport(SQLReportMixin, BaseReport):
```

**Fields:**
```python
__format__ = ...
__extension__ = ...
_reuse = ...
def is_valid(self, url)
def connect(self, url)
```


#### `PostgreSQLReport.is_valid`  <sub>(function)</sub>

```python
def PostgreSQLReport.is_valid(self, url):
```

#### `PostgreSQLReport.connect`  <sub>(function)</sub>

```python
def PostgreSQLReport.connect(self, url):
```

### `dirsearch/lib/report/response_store.py`


#### `ResponseArtifact`  <sub>(class)</sub>

> Backend-neutral response data passed to response stores.

```python
class ResponseArtifact:
```

**Fields:**
```python
timestamp: ...
url: ...
status: ...
headers: ...
content_length: ...
content_type: ...
redirect: ...
elapsed: ...
body: ...
body_complete: ...
body_truncated: ...
def from_response(cls, response)
```


#### `ResponseArtifact.from_response`  <sub>(function)</sub>

```python
def ResponseArtifact.from_response(cls, response: BaseResponse) -> ResponseArtifact:
```

#### `BaseResponseStore`  <sub>(class)</sub>

> Shared lifecycle and async adapter for response artifact stores.

```python
class BaseResponseStore(ABC):
```

**Fields:**
```python
name = ...
def __init__(self, destination)
def closed(self)
def ensure_open(self)
def save(self, artifact)
def save_async(self, artifact)
def close(self)
def __enter__(self)
def __exit__(self)
```


#### `BaseResponseStore.__init__`  <sub>(function)</sub>

```python
def BaseResponseStore.__init__(self, destination: str) -> None:
```

#### `BaseResponseStore.closed`  <sub>(function)</sub>

```python
def BaseResponseStore.closed(self) -> bool:
```

#### `BaseResponseStore.ensure_open`  <sub>(function)</sub>

```python
def BaseResponseStore.ensure_open(self) -> None:
```

#### `BaseResponseStore.save`  <sub>(function)</sub>

```python
def BaseResponseStore.save(self, artifact: ResponseArtifact) -> str:
```

#### `BaseResponseStore.save_async`  <sub>(function)</sub>

> Offload synchronous stores; native async stores may override this.

```python
async def BaseResponseStore.save_async(self, artifact: ResponseArtifact) -> str:
```

#### `BaseResponseStore.close`  <sub>(function)</sub>

```python
def BaseResponseStore.close(self) -> None:
```

#### `BaseResponseStore.__enter__`  <sub>(function)</sub>

```python
def BaseResponseStore.__enter__(self) -> BaseResponseStore:
```

#### `BaseResponseStore.__exit__`  <sub>(function)</sub>

```python
def BaseResponseStore.__exit__(self, *_args) -> None:
```

#### `create_response_stores`  <sub>(function)</sub>

> Build configured stores while keeping controller orchestration generic.

```python
def create_response_stores(
        directory: str | None,
        jsonl_file: str | None,
) -> tuple[BaseResponseStore, ...]:
```

### `dirsearch/lib/report/simple_report.py`


#### `SimpleReport`  <sub>(class)</sub>

```python
class SimpleReport(FileReportMixin, BaseReport):
```

**Fields:**
```python
__format__ = ...
__extension__ = ...
def new(self)
def save(self, file, result)
```


#### `SimpleReport.new`  <sub>(function)</sub>

```python
def SimpleReport.new(self):
```

#### `SimpleReport.save`  <sub>(function)</sub>

```python
def SimpleReport.save(self, file, result):
```

### `dirsearch/lib/report/sqlite_report.py`


#### `SQLiteReport`  <sub>(class)</sub>

```python
class SQLiteReport(SQLReportMixin, BaseReport):
```

**Fields:**
```python
__format__ = ...
__extension__ = ...
_reuse = ...
def __init__(self, commit_batch_size)
def _commit(self, conn)
def _after_save(self, conn)
def get_create_table_query(self, table)
def get_insert_table_query(self, table, values)
def connect(self, file)
```


#### `SQLiteReport.__init__`  <sub>(function)</sub>

```python
def SQLiteReport.__init__(self, commit_batch_size=None):
```

#### `SQLiteReport._commit`  <sub>(function)</sub>

```python
def SQLiteReport._commit(self, conn):
```

#### `SQLiteReport._after_save`  <sub>(function)</sub>

```python
def SQLiteReport._after_save(self, conn):
```

#### `SQLiteReport.get_create_table_query`  <sub>(function)</sub>

```python
def SQLiteReport.get_create_table_query(self, table):
```

#### `SQLiteReport.get_insert_table_query`  <sub>(function)</sub>

```python
def SQLiteReport.get_insert_table_query(self, table, values):
```

#### `SQLiteReport.connect`  <sub>(function)</sub>

```python
def SQLiteReport.connect(self, file):
```

### `dirsearch/lib/report/xml_report.py`


#### `XMLReport`  <sub>(class)</sub>

```python
class XMLReport(StructuredFileReportMixin, BaseReport):
```

**Fields:**
```python
__format__ = ...
__extension__ = ...
def new(self)
def parse(self, file)
def _state_for_journal(self, root)
def _apply_entry(root, entry)
def save(self, file, result)
def write(self, file, root)
```


#### `XMLReport.new`  <sub>(function)</sub>

```python
def XMLReport.new(self):
```

#### `XMLReport.parse`  <sub>(function)</sub>

```python
def XMLReport.parse(self, file):
```

#### `XMLReport._state_for_journal`  <sub>(function)</sub>

```python
def XMLReport._state_for_journal(self, root):
```

#### `XMLReport._apply_entry`  <sub>(function)</sub>

```python
def XMLReport._apply_entry(root, entry):
```

#### `XMLReport.save`  <sub>(function)</sub>

```python
def XMLReport.save(self, file, result):
```

#### `XMLReport.write`  <sub>(function)</sub>

```python
def XMLReport.write(self, file, root):
```


## `lib/utils` <a id='pkg-lib-utils'></a>

*118 documented items*


### `dirsearch/lib/utils/cli.py`


#### `print_diagnostic`  <sub>(function)</sub>

```python
def print_diagnostic(message: object) -> None:
```

#### `fail`  <sub>(function)</sub>

```python
def fail(*messages: object) -> NoReturn:
```

### `dirsearch/lib/utils/command.py`


#### `REDACTED_VALUE`  <sub>(const)</sub>

```python
REDACTED_VALUE = ...
```

#### `SENSITIVE_OPTIONS`  <sub>(const)</sub>

```python
SENSITIVE_OPTIONS = ...
```

#### `SENSITIVE_SHORT_OPTIONS`  <sub>(const)</sub>

```python
SENSITIVE_SHORT_OPTIONS = ...
```

#### `TARGET_OPTIONS`  <sub>(const)</sub>

```python
TARGET_OPTIONS = ...
```

#### `SHORT_FLAG_OPTIONS`  <sub>(const)</sub>

```python
SHORT_FLAG_OPTIONS = ...
```

#### `_matches_option`  <sub>(function)</sub>

```python
def _matches_option(option: str, candidates: frozenset[str]) -> bool:
```

#### `_redact_value`  <sub>(function)</sub>

```python
def _redact_value(option: str, value: str) -> str:
```

#### `_find_short_value_option`  <sub>(function)</sub>

```python
def _find_short_value_option(argument: str) -> tuple[str, int] | None:
```

#### `redact_command`  <sub>(function)</sub>

> Format command metadata without persisting credential-bearing values.

```python
def redact_command(arguments: Sequence[str]) -> str:
```

### `dirsearch/lib/utils/common.py`


#### `get_config_file`  <sub>(function)</sub>

```python
def get_config_file():
```

#### `safequote`  <sub>(function)</sub>

```python
def safequote(string_: str) -> str:
```

#### `_strip_and_uniquify_callback`  <sub>(function)</sub>

```python
def _strip_and_uniquify_callback(array, item):
```

#### `strip_and_uniquify`  <sub>(function)</sub>

```python
def strip_and_uniquify(array, type_=list):
```

#### `lstrip_once`  <sub>(function)</sub>

```python
def lstrip_once(string, pattern):
```

#### `rstrip_once`  <sub>(function)</sub>

```python
def rstrip_once(string, pattern):
```

#### `get_valid_filename`  <sub>(function)</sub>

```python
def get_valid_filename(string):
```

#### `get_readable_size`  <sub>(function)</sub>

```python
def get_readable_size(num):
```

#### `get_response_length`  <sub>(function)</sub>

```python
def get_response_length(headers, body_length):
```

#### `is_binary`  <sub>(function)</sub>

```python
def is_binary(bytes) -> bool:
```

#### `is_ipv6`  <sub>(function)</sub>

```python
def is_ipv6(ip):
```

#### `iprange`  <sub>(function)</sub>

```python
def iprange(subnet):
```

#### `merge_path`  <sub>(function)</sub>

```python
def merge_path(url, path):
```

#### `read_stdin`  <sub>(function)</sub>

```python
def read_stdin():
```

#### `replace_path`  <sub>(function)</sub>

```python
def replace_path(string, path, replace_with):
```

### `dirsearch/lib/utils/crawl.py`


#### `_ESCAPED_SLASH_REGEX = ...`  <sub>(const)</sub>

```python
_ESCAPED_SLASH_REGEX = ...
```

#### `_MEDIA_SUFFIXES = ...`  <sub>(const)</sub>

```python
_MEDIA_SUFFIXES = ...
```

#### `_TEXT_URL_CHARS = ...`  <sub>(const)</sub>

```python
_TEXT_URL_CHARS = ...
```

#### `_TEXT_URL_QUOTES = ...`  <sub>(const)</sub>

```python
_TEXT_URL_QUOTES = ...
```

#### `_ASCII_WHITESPACE = ...`  <sub>(const)</sub>

```python
_ASCII_WHITESPACE = ...
```

#### `_filter`  <sub>(function)</sub>

```python
def _filter(paths):
```

#### `_trim_unquoted_url`  <sub>(function)</sub>

```python
def _trim_unquoted_url(path):
```

#### `_extract_scoped_paths`  <sub>(function)</sub>

```python
def _extract_scoped_paths(scope, content):
```

#### `_srcset_urls`  <sub>(function)</sub>

> Yield URL tokens from an HTML srcset value.

```python
def _srcset_urls(value):
```

#### `_browser_url_value`  <sub>(function)</sub>

> Normalize reverse solidus in an HTTP URL path like a browser.

```python
def _browser_url_value(value):
```

#### `_document_base_url`  <sub>(function)</sub>

```python
def _document_base_url(url, soup):
```

#### `_same_origin_crawl_path`  <sub>(function)</sub>

```python
def _same_origin_crawl_path(scope, base_url, value):
```

#### `Crawler`  <sub>(class)</sub>

```python
class Crawler:
```

**Fields:**
```python
def crawl(cls, response)
def text_crawl(url, scope, content)
def html_crawl(url, scope, content)
def robots_crawl(url, scope, content)
```


#### `Crawler.crawl`  <sub>(function)</sub>

```python
def Crawler.crawl(cls, response):
```

#### `Crawler.text_crawl`  <sub>(function)</sub>

```python
def Crawler.text_crawl(url, scope, content):
```

#### `Crawler.html_crawl`  <sub>(function)</sub>

```python
def Crawler.html_crawl(url, scope, content):
```

#### `Crawler.robots_crawl`  <sub>(function)</sub>

```python
def Crawler.robots_crawl(url, scope, content):
```

### `dirsearch/lib/utils/diff.py`


#### `_DYNAMIC_TOKEN_REGEXES = ...`  <sub>(const)</sub>

```python
_DYNAMIC_TOKEN_REGEXES = ...
```

#### `_HTML_ATTRIBUTE_VALUE_REGEX = ...`  <sub>(const)</sub>

```python
_HTML_ATTRIBUTE_VALUE_REGEX = ...
```

#### `normalize_dynamic_content`  <sub>(function)</sub>

> Return content with common volatile values replaced by stable markers.

```python
def normalize_dynamic_content(content: str) -> str:
```

#### `content_similarity`  <sub>(function)</sub>

> Return similarity after removing common dynamic values.

```python
def content_similarity(content1: str, content2: str) -> float:
```

#### `normalized_content_similarity`  <sub>(function)</sub>

> Return similarity between content that is already normalized.

```python
def normalized_content_similarity(content1: str, content2: str) -> float:
```

#### `DynamicContentParser`  <sub>(class)</sub>

```python
class DynamicContentParser:
```

**Fields:**
```python
def __init__(self, content1, content2)
def static_patterns(self)
def is_ambiguous(self)
def add_sample(self, content)
def compare_to(self, content, normalized_content)
def similarity_to(self, content, normalized_content)
def _recalculate(self)
def get_static_patterns(patterns)
```


#### `DynamicContentParser.__init__`  <sub>(function)</sub>

```python
def DynamicContentParser.__init__(self, content1, content2):
```

#### `DynamicContentParser.static_patterns`  <sub>(function)</sub>

```python
def DynamicContentParser.static_patterns(self):
```

#### `DynamicContentParser.is_ambiguous`  <sub>(function)</sub>

```python
def DynamicContentParser.is_ambiguous(self):
```

#### `DynamicContentParser.add_sample`  <sub>(function)</sub>

```python
def DynamicContentParser.add_sample(self, content):
```

#### `DynamicContentParser.compare_to`  <sub>(function)</sub>

> DynamicContentParser.compare_to() workflow  
>   
> 1. Check if the wildcard response is static or not, if yes, compare two responses.  
> 2. If it's not static, get static patterns (split by space) and check if the response  
> has all of them.  
> 3. In some cases, checking static patterns isn't reliable enough, so we check the similarity  
> ratio of the two responses.

```python
def DynamicContentParser.compare_to(self, content, normalized_content=None):
```

#### `DynamicContentParser.similarity_to`  <sub>(function)</sub>

```python
def DynamicContentParser.similarity_to(self, content, normalized_content=None):
```

#### `DynamicContentParser._recalculate`  <sub>(function)</sub>

```python
def DynamicContentParser._recalculate(self):
```

#### `DynamicContentParser.get_static_patterns`  <sub>(function)</sub>

```python
def DynamicContentParser.get_static_patterns(patterns):
```

#### `generate_matching_regex`  <sub>(function)</sub>

```python
def generate_matching_regex(string1: str, string2: str) -> str:
```

### `dirsearch/lib/utils/file.py`


#### `File`  <sub>(class)</sub>

```python
class File:
```

**Fields:**
```python
def __init__(self)
def path(self)
def path(self, value)
def is_valid(self)
def exists(self)
def can_read(self)
def can_write(self)
def read(self)
def get_lines(self)
def __enter__(self)
def __exit__(self, type, value, tb)
```


#### `File.__init__`  <sub>(function)</sub>

```python
def File.__init__(self, *path_components):
```

#### `File.path`  <sub>(function)</sub>

```python
def File.path(self):
```

#### `File.path`  <sub>(function)</sub>

```python
def File.path(self, value):
```

#### `File.is_valid`  <sub>(function)</sub>

```python
def File.is_valid(self):
```

#### `File.exists`  <sub>(function)</sub>

```python
def File.exists(self):
```

#### `File.can_read`  <sub>(function)</sub>

```python
def File.can_read(self):
```

#### `File.can_write`  <sub>(function)</sub>

```python
def File.can_write(self):
```

#### `File.read`  <sub>(function)</sub>

```python
def File.read(self):
```

#### `File.get_lines`  <sub>(function)</sub>

```python
def File.get_lines(self):
```

#### `File.__enter__`  <sub>(function)</sub>

```python
def File.__enter__(self):
```

#### `File.__exit__`  <sub>(function)</sub>

```python
def File.__exit__(self, type, value, tb):
```

#### `FileUtils`  <sub>(class)</sub>

```python
class FileUtils:
```

**Fields:**
```python
def build_path()
def format_datetime_for_path(value)
def get_abs_path(file_name)
def exists(file_name)
def is_empty(file_name)
def can_read(file_name)
def can_write(cls, path)
def read(file_name)
def read_bytes(file_name)
def get_files(cls, directory)
def get_lines(file_name)
def is_dir(path)
def is_file(path)
def is_link(path)
def parent(path, depth)
def create_dir(cls, directory)
def create_private_dir(directory)
def create_writable_dir(cls, directory)
def open_binary_append(cls, file_name)
def append_private_text(cls, file_name, data, encoding, newline)
def open_exclusive(file_name)
def atomic_write_private_text(file_name, encoding, newline)
def _open_exclusive_windows(file_name)
def remove(file_name)
def write_lines(file_name, lines, overwrite)
```


#### `FileUtils.build_path`  <sub>(function)</sub>

```python
def FileUtils.build_path(*path_components: str) -> str:
```

#### `FileUtils.format_datetime_for_path`  <sub>(function)</sub>

```python
def FileUtils.format_datetime_for_path(value: str) -> str:
```

#### `FileUtils.get_abs_path`  <sub>(function)</sub>

```python
def FileUtils.get_abs_path(file_name):
```

#### `FileUtils.exists`  <sub>(function)</sub>

```python
def FileUtils.exists(file_name):
```

#### `FileUtils.is_empty`  <sub>(function)</sub>

```python
def FileUtils.is_empty(file_name):
```

#### `FileUtils.can_read`  <sub>(function)</sub>

```python
def FileUtils.can_read(file_name):
```

#### `FileUtils.can_write`  <sub>(function)</sub>

```python
def FileUtils.can_write(cls, path):
```

#### `FileUtils.read`  <sub>(function)</sub>

```python
def FileUtils.read(file_name):
```

#### `FileUtils.read_bytes`  <sub>(function)</sub>

```python
def FileUtils.read_bytes(file_name):
```

#### `FileUtils.get_files`  <sub>(function)</sub>

```python
def FileUtils.get_files(cls, directory):
```

#### `FileUtils.get_lines`  <sub>(function)</sub>

```python
def FileUtils.get_lines(file_name: str) -> list[str]:
```

#### `FileUtils.is_dir`  <sub>(function)</sub>

```python
def FileUtils.is_dir(path):
```

#### `FileUtils.is_file`  <sub>(function)</sub>

```python
def FileUtils.is_file(path):
```

#### `FileUtils.is_link`  <sub>(function)</sub>

```python
def FileUtils.is_link(path):
```

#### `FileUtils.parent`  <sub>(function)</sub>

```python
def FileUtils.parent(path, depth=1):
```

#### `FileUtils.create_dir`  <sub>(function)</sub>

```python
def FileUtils.create_dir(cls, directory):
```

#### `FileUtils.create_private_dir`  <sub>(function)</sub>

> Create a private leaf directory without following a leaf link.

```python
def FileUtils.create_private_dir(directory: str) -> None:
```

#### `FileUtils.create_writable_dir`  <sub>(function)</sub>

> Create a directory and prove that files can be created inside it.

```python
def FileUtils.create_writable_dir(cls, directory: str) -> None:
```

#### `FileUtils.open_binary_append`  <sub>(function)</sub>

> Open a binary file descriptor for append without following links.

```python
def FileUtils.open_binary_append(cls, file_name: str) -> int:
```

#### `FileUtils.append_private_text`  <sub>(function)</sub>

> Append text and roll back a failed write to the previous file size.

```python
def FileUtils.append_private_text(
        cls,
        file_name: str,
        data: str,
        encoding: str | None = "utf-8",
        newline: str | None = None,
) -> None:
```

#### `FileUtils.open_exclusive`  <sub>(function)</sub>

> Create a private binary file without replacing or following a path.

```python
def FileUtils.open_exclusive(file_name: str) -> int:
```

#### `FileUtils.atomic_write_private_text`  <sub>(function)</sub>

> Write text through a private same-directory replacement file.

```python
def FileUtils.atomic_write_private_text(
        file_name: str,
        encoding: str | None = "utf-8",
        newline: str | None = None,
):
```

#### `FileUtils._open_exclusive_windows`  <sub>(function)</sub>

> Create a Windows file without following an existing reparse point.

```python
def FileUtils._open_exclusive_windows(file_name: str) -> int:
```

#### `FileUtils.remove`  <sub>(function)</sub>

```python
def FileUtils.remove(file_name: str) -> None:
```

#### `FileUtils.write_lines`  <sub>(function)</sub>

```python
def FileUtils.write_lines(file_name, lines, overwrite=False):
```

### `dirsearch/lib/utils/mimetype.py`


#### `MimeTypeUtils`  <sub>(class)</sub>

```python
class MimeTypeUtils:
```

**Fields:**
```python
def to_text(content)
def is_json(content)
def is_xml(content)
def is_query_string(content)
```


#### `MimeTypeUtils.to_text`  <sub>(function)</sub>

```python
def MimeTypeUtils.to_text(content):
```

#### `MimeTypeUtils.is_json`  <sub>(function)</sub>

```python
def MimeTypeUtils.is_json(content):
```

#### `MimeTypeUtils.is_xml`  <sub>(function)</sub>

```python
def MimeTypeUtils.is_xml(content):
```

#### `MimeTypeUtils.is_query_string`  <sub>(function)</sub>

```python
def MimeTypeUtils.is_query_string(content):
```

#### `guess_mimetype`  <sub>(function)</sub>

```python
def guess_mimetype(content) -> str:
```

### `dirsearch/lib/utils/random.py`


#### `StealthWordGenerator`  <sub>(class)</sub>

```python
class StealthWordGenerator:
```

**Fields:**
```python
separators = ...
common_directories = ...
word_bank = ...
_onsets = ...
_nuclei = ...
_codas = ...
_suffixes = ...
def __init__(self, seed, rng)
def generate(self, omit)
def _candidate_from_bank(self, word_count, separator)
def _pseudo_word(self, min_length, max_length)
def _is_valid(self, candidate, omitted)
def _normalize_omit(omit)
def _shannon_entropy(value)
```


#### `StealthWordGenerator.__init__`  <sub>(function)</sub>

```python
def StealthWordGenerator.__init__(
        self,
        seed: int | str | bytes | bytearray | None = None,
        rng: random.Random | None = None,
) -> None:
```

#### `StealthWordGenerator.generate`  <sub>(function)</sub>

```python
def StealthWordGenerator.generate(self, omit: str | Iterable[str] | None = None) -> str:
```

#### `StealthWordGenerator._candidate_from_bank`  <sub>(function)</sub>

```python
def StealthWordGenerator._candidate_from_bank(self, word_count: int, separator: str) -> str:
```

#### `StealthWordGenerator._pseudo_word`  <sub>(function)</sub>

```python
def StealthWordGenerator._pseudo_word(self, min_length: int, max_length: int) -> str:
```

#### `StealthWordGenerator._is_valid`  <sub>(function)</sub>

```python
def StealthWordGenerator._is_valid(self, candidate: str, omitted: set[str]) -> bool:
```

#### `StealthWordGenerator._normalize_omit`  <sub>(function)</sub>

```python
def StealthWordGenerator._normalize_omit(omit: str | Iterable[str] | None) -> set[str]:
```

#### `StealthWordGenerator._shannon_entropy`  <sub>(function)</sub>

```python
def StealthWordGenerator._shannon_entropy(value: str) -> float:
```

#### `rand_stealth_word`  <sub>(function)</sub>

```python
def rand_stealth_word(omit: str | Iterable[str] | None = None) -> str:
```

#### `rand_string`  <sub>(function)</sub>

```python
def rand_string(n, omit=None):
```

### `dirsearch/lib/utils/safe_xml.py`


#### `FORBIDDEN_XML_MARKUP`  <sub>(const)</sub>

```python
FORBIDDEN_XML_MARKUP = ...
```

#### `UnsafeXML`  <sub>(class)</sub>

```python
class UnsafeXML(ValueError):
```

#### `reject_unsafe_xml_markup`  <sub>(function)</sub>

```python
def reject_unsafe_xml_markup(content: bytes | str) -> bytes | str:
```

#### `fromstring`  <sub>(function)</sub>

```python
def fromstring(content: bytes | str):
```

#### `parse_file`  <sub>(function)</sub>

```python
def parse_file(path: str | Path):
```

### `dirsearch/lib/utils/schemedet.py`


#### `detect_scheme`  <sub>(function)</sub>

```python
def detect_scheme(host, port, connect_host=None):
```


## `lib/view` <a id='pkg-lib-view'></a>

*38 documented items*


### `dirsearch/lib/view/colors.py`


#### `BACK_COLORS`  <sub>(const)</sub>

```python
BACK_COLORS = ...
```

#### `FORE_COLORS`  <sub>(const)</sub>

```python
FORE_COLORS = ...
```

#### `STYLES`  <sub>(const)</sub>

```python
STYLES = ...
```

#### `disable_color`  <sub>(function)</sub>

```python
def disable_color():
```

#### `set_color`  <sub>(function)</sub>

```python
def set_color(msg, fore="none", back="none", style="normal"):
```

#### `clean_color`  <sub>(function)</sub>

```python
def clean_color(msg):
```

### `dirsearch/lib/view/terminal.py`


#### `MAX_DISPLAY_TEXT_LENGTH`  <sub>(const)</sub>

```python
MAX_DISPLAY_TEXT_LENGTH = ...
```

#### `TERMINAL_HISTORY_MEMORY_LIMIT`  <sub>(const)</sub>

```python
TERMINAL_HISTORY_MEMORY_LIMIT = ...
```

#### `safe_display_text`  <sub>(function)</sub>

```python
def safe_display_text(value, max_length=MAX_DISPLAY_TEXT_LENGTH):
```

#### `CLI`  <sub>(class)</sub>

```python
class CLI:
```

**Fields:**
```python
def __init__(self)
def buffer(self)
def close(self)
def erase()
def in_line(self, string)
def new_line(self, string, do_save)
def status_report(self, response, full_url)
def last_path(self, index, length, current_job, all_jobs, rate, errors)
def new_directories(self, directories)
def error(self, reason)
def warning(self, message, do_save)
def header(self, message)
def print_header(self, headers)
def config(self, wordlist_size)
def target(self, target)
def log_file(self, file)
```


#### `CLI.__init__`  <sub>(function)</sub>

```python
def CLI.__init__(self):
```

#### `CLI.buffer`  <sub>(function)</sub>

```python
def CLI.buffer(self):
```

#### `CLI.close`  <sub>(function)</sub>

```python
def CLI.close(self):
```

#### `CLI.erase`  <sub>(function)</sub>

```python
def CLI.erase():
```

#### `CLI.in_line`  <sub>(function)</sub>

```python
def CLI.in_line(self, string):
```

#### `CLI.new_line`  <sub>(function)</sub>

```python
def CLI.new_line(self, string="", do_save=True):
```

#### `CLI.status_report`  <sub>(function)</sub>

```python
def CLI.status_report(self, response, full_url):
```

#### `CLI.last_path`  <sub>(function)</sub>

```python
def CLI.last_path(self, index, length, current_job, all_jobs, rate, errors):
```

#### `CLI.new_directories`  <sub>(function)</sub>

```python
def CLI.new_directories(self, directories):
```

#### `CLI.error`  <sub>(function)</sub>

```python
def CLI.error(self, reason):
```

#### `CLI.warning`  <sub>(function)</sub>

```python
def CLI.warning(self, message, do_save=True):
```

#### `CLI.header`  <sub>(function)</sub>

```python
def CLI.header(self, message):
```

#### `CLI.print_header`  <sub>(function)</sub>

```python
def CLI.print_header(self, headers):
```

#### `CLI.config`  <sub>(function)</sub>

```python
def CLI.config(self, wordlist_size):
```

#### `CLI.target`  <sub>(function)</sub>

```python
def CLI.target(self, target):
```

#### `CLI.log_file`  <sub>(function)</sub>

```python
def CLI.log_file(self, file):
```

#### `QuietCLI`  <sub>(class)</sub>

```python
class QuietCLI(CLI):
```

**Fields:**
```python
def status_report(self, response, full_url)
def last_path()
def new_directories()
def warning()
def header()
def config()
def target()
def log_file()
```


#### `QuietCLI.status_report`  <sub>(function)</sub>

```python
def QuietCLI.status_report(self, response, full_url):
```

#### `QuietCLI.last_path`  <sub>(function)</sub>

```python
def QuietCLI.last_path(*args):
```

#### `QuietCLI.new_directories`  <sub>(function)</sub>

```python
def QuietCLI.new_directories(*args):
```

#### `QuietCLI.warning`  <sub>(function)</sub>

```python
def QuietCLI.warning(*args, **kwargs):
```

#### `QuietCLI.header`  <sub>(function)</sub>

```python
def QuietCLI.header(*args):
```

#### `QuietCLI.config`  <sub>(function)</sub>

```python
def QuietCLI.config(*args):
```

#### `QuietCLI.target`  <sub>(function)</sub>

```python
def QuietCLI.target(*args):
```

#### `QuietCLI.log_file`  <sub>(function)</sub>

```python
def QuietCLI.log_file(*args):
```

#### `EmptyCLI`  <sub>(class)</sub>

```python
class EmptyCLI(QuietCLI):
```

**Fields:**
```python
def status_report()
def error()
```


#### `EmptyCLI.status_report`  <sub>(function)</sub>

```python
def EmptyCLI.status_report(*args):
```

#### `EmptyCLI.error`  <sub>(function)</sub>

```python
def EmptyCLI.error(*args):
```


## `scripts` <a id='pkg-scripts'></a>

*48 documented items*


### `dirsearch/scripts/build_native.py`


#### `PROJECT_ROOT`  <sub>(const)</sub>

```python
PROJECT_ROOT = ...
```

#### `run`  <sub>(function)</sub>

```python
def run(command: list[str], cwd: Path | None = None) -> None:
```

#### `clean_wheel_dir`  <sub>(function)</sub>

```python
def clean_wheel_dir(wheel_dir: Path) -> None:
```

#### `resolve_python`  <sub>(function)</sub>

```python
def resolve_python(value: str) -> Path:
```

#### `read_native_version`  <sub>(function)</sub>

```python
def read_native_version(manifest: Path) -> str:
```

#### `install_native_wheel`  <sub>(function)</sub>

```python
def install_native_wheel(python: Path, wheel: Path, expected_version: str) -> None:
```

#### `build_native`  <sub>(function)</sub>

```python
def build_native(args: argparse.Namespace) -> Path:
```

#### `main`  <sub>(function)</sub>

```python
def main() -> None:
```

### `dirsearch/scripts/build_portable.py`


#### `PBS_REPOSITORY`  <sub>(const)</sub>

```python
PBS_REPOSITORY = ...
```

#### `PBS_API`  <sub>(const)</sub>

```python
PBS_API = ...
```

#### `TARGETS`  <sub>(const)</sub>

```python
TARGETS = ...
```

#### `run`  <sub>(function)</sub>

```python
def run(command: list[str], cwd: Path | None = None) -> None:
```

#### `github_request`  <sub>(function)</sub>

```python
def github_request(url: str) -> urllib.request.Request:
```

#### `release_metadata`  <sub>(function)</sub>

```python
def release_metadata(tag: str) -> dict:
```

#### `select_python_asset`  <sub>(function)</sub>

```python
def select_python_asset(release: dict, python_version: str, target: str) -> dict:
```

#### `download`  <sub>(function)</sub>

```python
def download(url: str, destination: Path) -> None:
```

#### `extract_python`  <sub>(function)</sub>

```python
def extract_python(archive: Path, destination: Path) -> Path:
```

#### `copy_app`  <sub>(function)</sub>

```python
def copy_app(project_root: Path, destination: Path) -> None:
```

#### `chmod_executable`  <sub>(function)</sub>

```python
def chmod_executable(path: Path) -> None:
```

#### `write_launchers`  <sub>(function)</sub>

```python
def write_launchers(stage: Path, target: str) -> None:
```

#### `install_dependencies`  <sub>(function)</sub>

```python
def install_dependencies(python: Path, app: Path, stack: str) -> None:
```

#### `find_python`  <sub>(function)</sub>

```python
def find_python(stage: Path, target: str) -> Path:
```

#### `archive_stage`  <sub>(function)</sub>

```python
def archive_stage(stage: Path, output: Path, archive_type: str) -> None:
```

#### `build`  <sub>(function)</sub>

```python
def build(args: argparse.Namespace) -> None:
```

#### `main`  <sub>(function)</sub>

```python
def main() -> None:
```

### `dirsearch/scripts/configure_stack.py`


#### `STACKS`  <sub>(const)</sub>

```python
STACKS = ...
```

#### `replace_setting`  <sub>(function)</sub>

```python
def replace_setting(text: str, key: str, value: str) -> str:
```

#### `configure`  <sub>(function)</sub>

```python
def configure(config_path: Path, stack: str) -> None:
```

#### `main`  <sub>(function)</sub>

```python
def main() -> None:
```

### `dirsearch/scripts/split_wordlists.py`


#### `CATEGORY_ORDER`  <sub>(const)</sub>

```python
CATEGORY_ORDER = ...
```

#### `CONF_EXTENSIONS`  <sub>(const)</sub>

```python
CONF_EXTENSIONS = ...
```

#### `BACKUP_SUFFIXES`  <sub>(const)</sub>

```python
BACKUP_SUFFIXES = ...
```

#### `DB_EXTENSIONS`  <sub>(const)</sub>

```python
DB_EXTENSIONS = ...
```

#### `KEY_EXTENSIONS`  <sub>(const)</sub>

```python
KEY_EXTENSIONS = ...
```

#### `KEY_FILENAMES`  <sub>(const)</sub>

```python
KEY_FILENAMES = ...
```

#### `WEB_EXTENSIONS`  <sub>(const)</sub>

```python
WEB_EXTENSIONS = ...
```

#### `VCS_REGEX`  <sub>(const)</sub>

```python
VCS_REGEX = ...
```

#### `is_extension_tag`  <sub>(function)</sub>

```python
def is_extension_tag(line: str) -> bool:
```

#### `is_conf`  <sub>(function)</sub>

```python
def is_conf(line: str) -> bool:
```

#### `is_vcs`  <sub>(function)</sub>

```python
def is_vcs(line: str) -> bool:
```

#### `is_backup`  <sub>(function)</sub>

```python
def is_backup(line: str) -> bool:
```

#### `is_db`  <sub>(function)</sub>

```python
def is_db(line: str) -> bool:
```

#### `is_log`  <sub>(function)</sub>

```python
def is_log(line: str) -> bool:
```

#### `is_key`  <sub>(function)</sub>

```python
def is_key(line: str) -> bool:
```

#### `is_web`  <sub>(function)</sub>

```python
def is_web(line: str) -> bool:
```

#### `CATEGORY_RULES`  <sub>(const)</sub>

```python
CATEGORY_RULES = ...
```

#### `split_wordlists`  <sub>(function)</sub>

```python
def split_wordlists(source_path: str, dest_dir: str) -> dict[str, list[str]]:
```

#### `main`  <sub>(function)</sub>

```python
def main() -> None:
```
