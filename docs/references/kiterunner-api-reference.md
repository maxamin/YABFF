# kiterunner — Complete API Reference

Exhaustive per-item reference for **every** type, function, method, constant, and macro in the kiterunner source (`assetnote/kiterunner` @ `7d5824c`, v1.0.2). Language: **GO**. Generated from a full source read; test files are excluded. Each item shows its full signature and doc comment; composite types list their fields/members.

**864 documented items** across 72 source files.


---

## Modules / packages

- [`(root)`](#pkg--root-) — 0 items
- [`cmd/kiterunner`](#pkg-cmd-kiterunner) — 16 items
- [`cmd/testServer`](#pkg-cmd-testserver) — 13 items
- [`internal/art`](#pkg-internal-art) — 3 items
- [`internal/kitebuilder`](#pkg-internal-kitebuilder) — 19 items
- [`internal/scan`](#pkg-internal-scan) — 49 items
- [`internal/wordlist`](#pkg-internal-wordlist) — 27 items
- [`pkg/context`](#pkg-pkg-context) — 4 items
- [`pkg/convert`](#pkg-pkg-convert) — 4 items
- [`pkg/errors`](#pkg-pkg-errors) — 5 items
- [`pkg/http`](#pkg-pkg-http) — 93 items
- [`pkg/kitebuilder`](#pkg-pkg-kitebuilder) — 22 items
- [`pkg/kiterunner`](#pkg-pkg-kiterunner) — 102 items
- [`pkg/log`](#pkg-pkg-log) — 4 items
- [`pkg/proute`](#pkg-pkg-proute) — 503 items

---


## `(root)` <a id='pkg--root-'></a>

*0 documented items*



## `cmd/kiterunner` <a id='pkg-cmd-kiterunner'></a>

*16 documented items*


### `kiterunner/cmd/kiterunner/cmd/brute.go`


#### `init`  <sub>(func)</sub>

```go
func init()
```

### `kiterunner/cmd/kiterunner/cmd/kitebuilder.go`


#### `init`  <sub>(func)</sub>

```go
func init()
```

### `kiterunner/cmd/kiterunner/cmd/kitebuilderCompile.go`


#### `init`  <sub>(func)</sub>

```go
func init()
```

### `kiterunner/cmd/kiterunner/cmd/kitebuilderConvert.go`


#### `init`  <sub>(func)</sub>

```go
func init()
```

### `kiterunner/cmd/kiterunner/cmd/kitebuilderParse.go`


#### `init`  <sub>(func)</sub>

```go
func init()
```

### `kiterunner/cmd/kiterunner/cmd/kitebuilderReplay.go`


#### `init`  <sub>(func)</sub>

```go
func init()
```

### `kiterunner/cmd/kiterunner/cmd/root.go`


#### `Execute`  <sub>(func)</sub>

> Execute adds all child commands to the root command and sets flags appropriately.  
> This is called by main.main(). It only needs to happen once to the rootCmd.

```go
func Execute()
```

#### `init`  <sub>(func)</sub>

```go
func init()
```

#### `initLogging`  <sub>(func)</sub>

```go
func initLogging()
```

#### `initConfig`  <sub>(func)</sub>

> initConfig reads in config file and ENV variables if set.

```go
func initConfig()
```

### `kiterunner/cmd/kiterunner/cmd/scan.go`


#### `init`  <sub>(func)</sub>

```go
func init()
```

### `kiterunner/cmd/kiterunner/cmd/version.go`


#### `init`  <sub>(func)</sub>

```go
func init()
```

### `kiterunner/cmd/kiterunner/cmd/wordlist.go`


#### `init`  <sub>(func)</sub>

```go
func init()
```

### `kiterunner/cmd/kiterunner/cmd/wordlistCache.go`


#### `init`  <sub>(func)</sub>

```go
func init()
```

### `kiterunner/cmd/kiterunner/cmd/wordlistList.go`


#### `init`  <sub>(func)</sub>

```go
func init()
```

### `kiterunner/cmd/kiterunner/main.go`


#### `main`  <sub>(func)</sub>

```go
func main()
```


## `cmd/testServer` <a id='pkg-cmd-testserver'></a>

*13 documented items*


### `kiterunner/cmd/testServer/main.go`


#### `count32`  <sub>(struct)</sub>

```go
type count32 struct
```

**Fields:**
```go
val uint32
```


#### `count32.increment`  <sub>(method)</sub>

```go
func (c *count32) increment()
```

#### `count32.get`  <sub>(method)</sub>

```go
func (c *count32) get() uint32
```

#### `PreRequest`  <sub>(func)</sub>

```go
func PreRequest()
```

#### `Index`  <sub>(func)</sub>

```go
func Index(ctx *fasthttp.RequestCtx)
```

#### `ASDFResponder`  <sub>(func)</sub>

```go
func ASDFResponder(ctx *fasthttp.RequestCtx)
```

#### `Hello`  <sub>(func)</sub>

```go
func Hello(ctx *fasthttp.RequestCtx)
```

#### `WildcardResponder`  <sub>(func)</sub>

```go
func WildcardResponder(ctx *fasthttp.RequestCtx)
```

#### `RedirectResponder`  <sub>(func)</sub>

```go
func RedirectResponder(ctx *fasthttp.RequestCtx)
```

#### `UserWildcardResponder`  <sub>(func)</sub>

```go
func UserWildcardResponder(ctx *fasthttp.RequestCtx)
```

#### `APIWildcardResponder`  <sub>(func)</sub>

```go
func APIWildcardResponder(ctx *fasthttp.RequestCtx)
```

#### `StatsFunc`  <sub>(func)</sub>

```go
func StatsFunc(end <-chan bool)
```

#### `main`  <sub>(func)</sub>

```go
func main()
```


## `internal/art` <a id='pkg-internal-art'></a>

*3 documented items*


### `kiterunner/internal/art/art.go`


#### `GetArtBytes`  <sub>(func)</sub>

```go
func GetArtBytes() []byte
```

#### `WriteArtBytes`  <sub>(func)</sub>

```go
func WriteArtBytes(w io.Writer)
```

#### `PrintArt`  <sub>(func)</sub>

```go
func PrintArt()
```


## `internal/kitebuilder` <a id='pkg-internal-kitebuilder'></a>

*19 documented items*


### `kiterunner/internal/kitebuilder/convert.go`


#### `ConvertFiles`  <sub>(func)</sub>

```go
func ConvertFiles(ctx context.Context, input string, output string) error
```

#### `FileType`  <sub>(type)</sub>

> go:generate stringer -type=FileType

```go
type FileType int
```

#### `FileTypeFromExtension`  <sub>(func)</sub>

```go
func FileTypeFromExtension(filename string) (FileType, error)
```

#### `convertFile`  <sub>(func)</sub>

> convertFile will convert everything into our intermediate proute format, then output the format in the desired output  
> the inputType and outputType must be different. If they're the same this will error (since no conversion should occur)

```go
func convertFile(input io.Reader, inputType FileType, inputFilename string, output io.Writer, outputType FileType) error
```

### `kiterunner/internal/kitebuilder/filetype_string.go`


#### `_`  <sub>(func)</sub>

```go
func _()
```

#### `FileType.String`  <sub>(method)</sub>

```go
func (i FileType) String() string
```

### `kiterunner/internal/kitebuilder/parse.go`


#### `ScanOptions`  <sub>(struct)</sub>

```go
type ScanOptions struct
```

**Fields:**
```go
Debug bool
```


#### `NewDefaultScanOptions`  <sub>(func)</sub>

```go
func NewDefaultScanOptions() *ScanOptions
```

#### `Debug`  <sub>(func)</sub>

```go
func Debug(enabled bool) ScanOption
```

#### `ScanOption`  <sub>(type)</sub>

```go
type ScanOption func(o *ScanOptions)
```

#### `ScanStdin`  <sub>(func)</sub>

```go
func ScanStdin(ctx context.Context, opts ...ScanOption) error
```

#### `ScanFile`  <sub>(func)</sub>

```go
func ScanFile(ctx context.Context, filename string, opts ...ScanOption) error
```

#### `fixOutputFilename`  <sub>(func)</sub>

```go
func fixOutputFilename(filename string) string
```

#### `CompileFile`  <sub>(func)</sub>

```go
func CompileFile(ctx context.Context, input string, outputFile string, opts ...ScanOption) error
```

#### `Compile`  <sub>(func)</sub>

```go
func Compile(ctx context.Context, r io.Reader, outputFile string, opts ...ScanOption) error
```

#### `DebugPrintReader`  <sub>(func)</sub>

```go
func DebugPrintReader(ctx context.Context, r io.Reader, opts ...ScanOption) error
```

#### `DebugPrintBytes`  <sub>(func)</sub>

```go
func DebugPrintBytes(data []byte) error
```

#### `PrintBytes`  <sub>(func)</sub>

```go
func PrintBytes(data []byte) error
```

### `kiterunner/internal/kitebuilder/replay.go`


#### `Replay`  <sub>(func)</sub>

```go
func Replay(ctx context.Context, kitefile string, kuid string, method string, path string, host string, proxy string) error
```


## `internal/scan` <a id='pkg-internal-scan'></a>

*49 documented items*


### `kiterunner/internal/scan/options.go`


#### `ScanOptions`  <sub>(struct)</sub>

```go
type ScanOptions struct
```

**Fields:**
```go
Routes                   []*http.Route
Headers                  []http.Header
PrecheckTargets          bool
MaxConnPerHost           int
MaxParallelHosts         int
Delay                    time.Duration
UserAgent                string
Timeout                  time.Duration
MaxRedirects             int
ForceMethod              string
ShowProgress             bool
WildcardDetection        bool
ContentLengthIgnoreRange []http.Range
ProgressBar              bool
QuarantineThreshold      int64
PreflightDepth           int64
KitebuilderFullScan bool
SuccessStatusCodes  map[int]interface{}
FailStatusCodes     map[int]interface{}
BlacklistRedirectDomains []string
FilterAPIs               map[string]interface{}
extensions                 []string    // internal fields for logging
dirsearchCompatabilityMode bool
kitebuilderAPINames        []string
assetnoteAPINames          []string
wordlistNames              []string
kitebuilderAPIs []kitebuilder.API    // interim struct for conversion
prouteAPIs []proute.API    // actual routes that we'll be scanning prior to conversion to Routes
```


#### `ScanOptions.KiterunnerOptions`  <sub>(method)</sub>

```go
func (s ScanOptions) KiterunnerOptions() []kiterunner.ConfigOption
```

#### `ScanOptions.String`  <sub>(method)</sub>

```go
func (s ScanOptions) String() string
```

#### `NewDefaultScanOptions`  <sub>(func)</sub>

```go
func NewDefaultScanOptions() *ScanOptions
```

#### `ScanOptions.FilteredRoutes`  <sub>(method)</sub>

> FilteredRoutes will return the routes that match have a source in s.FilterAPIs. If s.FilterAPIs is empty  
> then this will return s.Routes

```go
func (s ScanOptions) FilteredRoutes() (ret []*http.Route)
```

#### `ScanOptions.Validate`  <sub>(method)</sub>

> Validate will ensure the config is sane after all the flags and then  
> return an error if things dont make sense

```go
func (s ScanOptions) Validate() error
```

#### `ForceMethod`  <sub>(func)</sub>

```go
func ForceMethod(method string) ScanOption
```

#### `Precheck`  <sub>(func)</sub>

```go
func Precheck(v bool) ScanOption
```

#### `ShowProgress`  <sub>(func)</sub>

```go
func ShowProgress(n bool) ScanOption
```

#### `UserAgent`  <sub>(func)</sub>

```go
func UserAgent(n string) ScanOption
```

#### `Timeout`  <sub>(func)</sub>

```go
func Timeout(n time.Duration) ScanOption
```

#### `Delay`  <sub>(func)</sub>

```go
func Delay(n time.Duration) ScanOption
```

#### `MaxParallelHosts`  <sub>(func)</sub>

```go
func MaxParallelHosts(n int) ScanOption
```

#### `MaxRedirects`  <sub>(func)</sub>

```go
func MaxRedirects(n int) ScanOption
```

#### `MaxConnPerHost`  <sub>(func)</sub>

```go
func MaxConnPerHost(n int) ScanOption
```

#### `readLines`  <sub>(func)</sub>

> readLines reads all of the lines from a text file in to  
> a slice of strings, returning the slice and any error

```go
func readLines(filename string) ([][]byte, error)
```

#### `LoadTextWordlist`  <sub>(func)</sub>

> LoadTextWordlist will load the lines from the text wordlist, ensure the paths are valid with a prefixing slash  
> and append any required extensions to the wordlist  
> we do not lazy load the wordlist because thats hard and a pain for allocations

```go
func LoadTextWordlist(fns []string, extensions []string, dirsearchCompatabilityMode bool) ScanOption
```

#### `LoadAssetnoteWordlist`  <sub>(func)</sub>

```go
func LoadAssetnoteWordlist(fns []string, extensions []string, dirsearchCompatabilityMode bool) ScanOption
```

#### `LoadAssetnoteWordlistKitebuilder`  <sub>(func)</sub>

```go
func LoadAssetnoteWordlistKitebuilder(fns []string) ScanOption
```

#### `LoadKitebuilderFile`  <sub>(func)</sub>

```go
func LoadKitebuilderFile(fns []string) ScanOption
```

#### `FilterAPIs`  <sub>(func)</sub>

> FilterAPIs will modify the output of FilteredRoutes to return the routes that only match the apis set

```go
func FilterAPIs(apis []string) ScanOption
```

#### `AddHeaders`  <sub>(func)</sub>

```go
func AddHeaders(hs []string) ScanOption
```

#### `AddHeader`  <sub>(func)</sub>

```go
func AddHeader(h string) ScanOption
```

#### `FailStatusCodes`  <sub>(func)</sub>

```go
func FailStatusCodes(v []int) ScanOption
```

#### `SuccessStatusCodes`  <sub>(func)</sub>

```go
func SuccessStatusCodes(v []int) ScanOption
```

#### `BlacklistDomains`  <sub>(func)</sub>

```go
func BlacklistDomains(in []string) ScanOption
```

#### `KitebuilderFullScan`  <sub>(func)</sub>

```go
func KitebuilderFullScan(v bool) ScanOption
```

#### `WildcardDetection`  <sub>(func)</sub>

```go
func WildcardDetection(v bool) ScanOption
```

#### `ContentLengthIgnoreRanges`  <sub>(func)</sub>

```go
func ContentLengthIgnoreRanges(v []string) ScanOption
```

#### `ContentLengthIgnoreRange`  <sub>(func)</sub>

```go
func ContentLengthIgnoreRange(v string) ScanOption
```

#### `ProgressBarEnabled`  <sub>(func)</sub>

```go
func ProgressBarEnabled(v bool) ScanOption
```

#### `QuarantineThreshold`  <sub>(func)</sub>

```go
func QuarantineThreshold(n int64) ScanOption
```

#### `PreflightDepth`  <sub>(func)</sub>

```go
func PreflightDepth(n int64) ScanOption
```

#### `ScanOption`  <sub>(type)</sub>

```go
type ScanOption func(o *ScanOptions) error
```

### `kiterunner/internal/scan/parse.go`


#### `ErrInvalidProtocol`  <sub>(struct)</sub>

```go
type ErrInvalidProtocol struct
```

**Fields:**
```go
Protocol string
URL      string
```


#### `ErrInvalidProtocol.Error`  <sub>(method)</sub>

```go
func (e *ErrInvalidProtocol) Error() string
```

#### `FileLen`  <sub>(struct)</sub>

```go
type FileLen struct
```

**Fields:**
```go
Filename  string
MaxLength int
```


#### `ParseFileWithLen`  <sub>(func)</sub>

```go
func ParseFileWithLen(in string) (FileLen, error)
```

#### `ParseInput`  <sub>(func)</sub>

> ParseInput will attempt to extract all targets from a given input  
> We will attempt to find a file matching your provided <input>, and otherwise  
> attempt to parse it as a URI.  
> If protocol is missing, then we will assume from the port.  
> If the port is missing, then we will try both http:80 and https:443  
> "-" should not be passed to this, as we want to parse stdin asynchronously.

```go
func ParseInput(in string) ([]*http.Target, error)
```

#### `ParseFile`  <sub>(func)</sub>

> ParseFile will perform a ParseDomain on all lines in a file

```go
func ParseFile(filename string) ([]*http.Target, error)
```

#### `ParseStdin`  <sub>(func)</sub>

> ParseStdin will return a channel that will publish chunks of targets every second (if there are any targets)  
> This attempts to optimise against pipes that slowly write out the targets, allowing us to asynchronously to start  
> processing targets without waiting for all the input

```go
func ParseStdin(ctx context.Context) (chan []*http.Target, error)
```

#### `ParseDomain`  <sub>(func)</sub>

> ParseDomain will attempt to determine the target based off the input  
> The only support protocols are http, https  
> If protocol is missing, then we will assume from the port.  
> If the port is missing, then we will try both http:80 and https:443  
> we use net/url to parse the URL

```go
func ParseDomain(domain string) ([]*http.Target, error)
```

### `kiterunner/internal/scan/progress.go`


#### `ProgressBar`  <sub>(struct)</sub>

```go
type ProgressBar struct
```

**Fields:**
```go
Pb       *mpb.Progress
Requests *progressbar.ProgressBar
```


#### `NewProgress`  <sub>(func)</sub>

```go
func NewProgress(max int64) *ProgressBar
```

#### `ProgressBar.Incr`  <sub>(method)</sub>

```go
func (b *ProgressBar) Incr(n int64)
```

#### `ProgressBar.AddTotal`  <sub>(method)</sub>

```go
func (b *ProgressBar) AddTotal(n int64)
```

### `kiterunner/internal/scan/scan.go`


#### `runWithProgress`  <sub>(func)</sub>

```go
func runWithProgress(ctx context.Context, pbEnabled bool, routes http.RouteMap, targets []*http.Target, wcopts []kiterunner.ConfigOption) ([]*kiterunner.Result, error)
```

#### `ScanDomainOrFile`  <sub>(func)</sub>

> ScanDomainOrFile will perform a scan using the domain or file provided.  
> This will first attempt to read the file specified, and if not found, attempt to parse  
> the input as a target.  
> If you wish to read from stdin, use ScanStdin

```go
func ScanDomainOrFile(ctx context.Context, domainOrFile string, opts ...ScanOption) error
```

#### `ScanStdin`  <sub>(func)</sub>

> ScanStdin will perform a scan using the options provided, reading targets from stdin  
> TODO: figure out how to do phase scanning with stdin scan

```go
func ScanStdin(ctx context.Context, opts ...ScanOption) error
```


## `internal/wordlist` <a id='pkg-internal-wordlist'></a>

*27 documented items*


### `kiterunner/internal/wordlist/cache.go`


#### `Save`  <sub>(func)</sub>

> Save will save the specified names to the local cache.  
> TODO: implement special case for "all"

```go
func Save(ctx context.Context, nocache bool, names ...string) (ret []WordlistMetadata, err error)
```

#### `CheckAllCached`  <sub>(func)</sub>

```go
func CheckAllCached(in []WordlistMetadata) ([]WordlistMetadata, error)
```

### `kiterunner/internal/wordlist/get.go`


#### `Get`  <sub>(func)</sub>

> Get will retrieve all the names from remote or local sources

```go
func Get(ctx context.Context, names ...string) (ret []WordlistMetadata, err error)
```

### `kiterunner/internal/wordlist/list.go`


#### `Format`  <sub>(type)</sub>

```go
type Format int
```

#### `FormatFromString`  <sub>(func)</sub>

```go
func FormatFromString(in string) (Format, error)
```

#### `ListOptions`  <sub>(struct)</sub>

```go
type ListOptions struct
```

**Fields:**
```go
Output Format
```


#### `ListOption`  <sub>(type)</sub>

```go
type ListOption func(o *ListOptions)
```

#### `OutputFormat`  <sub>(func)</sub>

```go
func OutputFormat(v Format) ListOption
```

#### `NewListOptions`  <sub>(func)</sub>

```go
func NewListOptions(opts ...ListOption) *ListOptions
```

#### `List`  <sub>(func)</sub>

```go
func List(ctx context.Context, opts ...ListOption) error
```

#### `getDownloadLink`  <sub>(func)</sub>

> getDownloadLink will extract the url from the html embedded thing that shubs did  
> "Download": "<a href='https://s3.amazonaws.com/assetnote-wordlists/./data/automated/httparchive_aspx_asp_cfm_svc_ashx_asmx_2021_02_28.txt'>Download</a>"

```go
func getDownloadLink(html string) string
```

#### `nameConvert`  <sub>(func)</sub>

> nameConvert will convert the filename like httparchive_apiroutes_2021_01_28.txt into a shortname like  
> apiroutes-210128. If you supply it weird input, it'll do weird things.

```go
func nameConvert(in string) string
```

#### `TabString`  <sub>(func)</sub>

```go
func TabString(fields ...string) string
```

### `kiterunner/internal/wordlist/local.go`


#### `GetLocalDirPanic`  <sub>(func)</sub>

```go
func GetLocalDirPanic() (string)
```

#### `GetLocalDir`  <sub>(func)</sub>

```go
func GetLocalDir() (string, error)
```

#### `CreateLocalCache`  <sub>(func)</sub>

```go
func CreateLocalCache() error
```

#### `GetLocalDirListing`  <sub>(func)</sub>

```go
func GetLocalDirListing() ([]WordlistMetadata, error)
```

### `kiterunner/internal/wordlist/metadata.go`


#### `WordlistMetadata`  <sub>(struct)</sub>

```go
type WordlistMetadata struct
```

**Fields:**
```go
Shortname string  `json:"Shortname,omitempty"`
Filename  string  `json:"Filename,omitempty"`
LineCount int     `json:"Line Count,omitempty"`
FileSize  string  `json:"File Size,omitempty"`
Date      float64 `json:"Date,omitempty"`
Download  string  `json:"Download,omitempty"`
Source    string  `json:"Source,omitempty"`
Cached    bool    `json:"Cached"`
```


#### `GetRemoteWordlists`  <sub>(func)</sub>

```go
func GetRemoteWordlists() ([]WordlistMetadata, error)
```

#### `data`  <sub>(struct)</sub>

```go
type data struct
```

**Fields:**
```go
Data []WordlistMetadata `json:"data"`
```


#### `WordlistMetadata.LocalFilenamePanic`  <sub>(method)</sub>

```go
func (w WordlistMetadata) LocalFilenamePanic() string
```

#### `WordlistMetadata.LocalKiteFilename`  <sub>(method)</sub>

```go
func (w WordlistMetadata) LocalKiteFilename() (string, error)
```

#### `WordlistMetadata.LocalFilename`  <sub>(method)</sub>

```go
func (w WordlistMetadata) LocalFilename() (string, error)
```

#### `WordlistMetadata.Cache`  <sub>(method)</sub>

> Cache will downlad the file and save it to disk

```go
func (w *WordlistMetadata) Cache() error
```

#### `WordlistMetadata.APIS`  <sub>(method)</sub>

> API will atttempt to load a compiled kite API from disk.  
> If this doesnt exist, then we attempt to compile a local file to disk.  
> If this local file doesnt exist, then we download it first and cache it

```go
func (w WordlistMetadata) APIS() (ret proute.APIS, err error)
```

#### `WordlistMetadata.Words`  <sub>(method)</sub>

> words will attempt to retrieve the words from the cache. If it fails, it will

```go
func (w WordlistMetadata) Words() ([]string, error)
```

#### `WordlistMetadata.IsCached`  <sub>(method)</sub>

> Cached performs a single stat operation against the local directory to see if the file exists  
> on disk. This is not efficient, but a few dozen stat calls never hurt anyone.  
> If you want to go fast, use CheckAllCached

```go
func (w *WordlistMetadata) IsCached() bool
```


## `pkg/context` <a id='pkg-pkg-context'></a>

*4 documented items*


### `kiterunner/pkg/context/context.go`


#### `AddInterruptCancellation`  <sub>(func)</sub>

> AddInterruptCancellation will add an interrupt handler that will catch the first SIGTERM and cancel the context  
> upon second SIGTERM, the program will exit immediately  
> This wrapping allows for graceful shutdown of the application

```go
func AddInterruptCancellation(ctx context.Context, cancel context.CancelFunc)
```

#### `InitContext`  <sub>(func)</sub>

> InitContext will initialize the global context used to catch interrupts. This is automatically called  
> by Context and Cancel

```go
func InitContext()
```

#### `Context`  <sub>(func)</sub>

> Context will initialize the global context and attach the interrupt handler that will cancel the context  
> upon SIGTERM. This is safe to call from multiple goroutines and will always return the same context

```go
func Context() context.Context
```

#### `Cancel`  <sub>(func)</sub>

> Cancel will cancel the global context. Calling this multiple times is the equivalent of cancelling  
> the same context multiple times

```go
func Cancel()
```


## `pkg/convert` <a id='pkg-pkg-convert'></a>

*4 documented items*


### `kiterunner/pkg/convert/convert.go`


#### `IntMapToSlice`  <sub>(func)</sub>

> IntMapToSlice will return all the keys for the given intmap

```go
func IntMapToSlice(m map[int]interface{}) (ret []int)
```

#### `IntSliceToMap`  <sub>(func)</sub>

> IntSliceToMap will return a map with keys created from the slice

```go
func IntSliceToMap(v []int) map[int]interface{}
```

#### `StringMapToSlice`  <sub>(func)</sub>

> StringMapToSlice will return all the keys for the given string map

```go
func StringMapToSlice(m map[string]interface{}) (ret []string)
```

#### `UniqueStrings`  <sub>(func)</sub>

> UniqueStrings will remove duplicates preserving order of the input

```go
func UniqueStrings(in []string) (out []string)
```


## `pkg/errors` <a id='pkg-pkg-errors'></a>

*5 documented items*


### `kiterunner/pkg/errors/error.go`


#### `prefixFromDepth`  <sub>(func)</sub>

> prefixfromDepth will create the indent prefix for a certain depth  
> of string, e.g. 2 will yield "  " * 2 -> "    "

```go
func prefixFromDepth(depth int) string
```

#### `PrintError`  <sub>(func)</sub>

> PrintError will attempt to traverse the nested error and  
> recursively print out any nested ParserErrors found  
> If a multierror.Error is found, we will recurisvely print out  
> each error found

```go
func PrintError(err error, depth int)
```

#### `ParserError`  <sub>(struct)</sub>

> ParserError encapsulates the contextual error relating to parsing an API schema  
> The fields can be arbitrarily used to represent whatever information you wish

```go
type ParserError struct
```

**Fields:**
```go
ID      string
Method  string    // ID corresponds to the KSUID for the API allowing you to backref which API the error came from
Route   string    // Method corresponds to the method for the request at the top level of the API
RawJSON []byte    // Route corresponds to the route for the request
Err     error    // RawJSON optionally can include the raw json for the component if there was a JSON parsing error
Context string    // Err includes the error. If this is a wrapped error, then Error() will not print out this field
```


#### `ParserError.Error`  <sub>(method)</sub>

> Error will return the string representation of the error. If the error  
> is wrapped, we omit printing the wrapped error, allowing the user to determine  
> how to display the wrapped error.

```go
func (p *ParserError) Error() string
```

#### `ParserError.LogError`  <sub>(method)</sub>

> LogError will log to Debug() the context surrounding the error.  
> the depth argument modifies the indentation depth of the pretty printed error  
> If RawJSON is included, we will add the RawJSON to the logging

```go
func (p *ParserError) LogError(depth int)
```


## `pkg/http` <a id='pkg-pkg-http'></a>

*93 documented items*


### `kiterunner/pkg/http/client.go`


#### `HTTPClient`  <sub>(type)</sub>

> HTTPClient is a type alias for the actual host client we use.  
> We do this instead of using an interface to avoid reflecting

```go
type HTTPClient = fasthttp.HostClient
```

#### `BackupClient`  <sub>(type)</sub>

> BackupClient is a normal fasthttpClient that can adapt to different hosts  
> This is used to handle redirects that change the host/port of the request

```go
type BackupClient = fasthttp.Client
```

#### `NewHTTPClient`  <sub>(func)</sub>

> NewHTTPClient will create a http client configured specifically for requesting against the targetted host.  
> This is backed by the fasthttp.HostClient

```go
func NewHTTPClient(host string, tls bool) *HTTPClient
```

#### `Config`  <sub>(struct)</sub>

> Config provides all the options available to a request, this is used by DoClient

```go
type Config struct
```

**Fields:**
```go
Timeout      time.Duration `toml:"timeout" json:"timeout" mapstructure:"timeout"`    // Timeout is the duration to wait when performing a DoTimeout request
MaxRedirects int           `toml:"max_redirects" json:"max_redirects" mapstructure:"max_redirects"`    // redirects are followed
ReadBody    bool    // off the wire to perform the length and word count calculations
ReadHeaders bool    // to determine whether to follow redirects
BlacklistRedirects []string `toml:"blacklist_redirects" json:"blacklist_redirects" mapstructure:"blacklist_redirects"`    // if a string is blacklisted, e.g. okta.com, the redirect is not followed
ExtraHeaders []Header    // ExtraHeaders are added to the request last and will overwrite the route headers
backupClient *BackupClient
```


#### `Config.IsBlacklistedRedirect`  <sub>(method)</sub>

> IsBlacklistedRedirect will compare the stored hosts against the provided host.  
> We use a prefix match with linear probing to simplify the check.  
> e.g. blacklist[okta.com, onelogin.com] will match against okta.com:80

```go
func (c *Config) IsBlacklistedRedirect(host []byte) bool
```

#### `Config.BackupClient`  <sub>(method)</sub>

> BackupClient provides a generic http client that is not bound to a single host.  
> it is generated on demand based off the config options. This is done only once per config.  
> if you change the options after calling BackupClient(), you must call ResetBackupClient()

```go
func (c *Config) BackupClient() *BackupClient
```

#### `Config.ResetBackupClient`  <sub>(method)</sub>

> ResetBackupClient will update the settings on the BackupClient. This is to be called if the timeout  
> is changed on the config after calling BackupClient

```go
func (c *Config) ResetBackupClient()
```

#### `DoClient`  <sub>(func)</sub>

> DoClient performs the provided request. We recommend avoiding letting the Response escape to the heap  
> to prevent allocating where not necessary. This will handle the redirects for the request.  
> Redirect responses are added to the linked list in the Response  
> The returned response chain will have the Response.OriginRequest populated  
> This will always read the body from the wire, but will only copy the body into the Response if config.ReadBody is true  
> We will always perform the calculations for the BodyLength, Words and Lines, as these require 0 allocations  
> given the response body is already read into memory  
>   
> Responses part of the chain are allocated dynamically using AcquireResponse and should be appropriately released  
> when the response is no longer needed

```go
func DoClient(c *HTTPClient, req Request, config *Config) (Response, error)
```

#### `doRequestFollowRedirects`  <sub>(func)</sub>

> doRequestFollowRedirects will use the client provided and attempt to follow config.MaxRedirects number of redirects  
> This will just exit early if it hits a redirect  
> If a redirect is found, it builds out the response linked list  
> We use a concrete HTTPClient instead of an interface to avoid reflection and saving us a few cycles

```go
func doRequestFollowRedirects(c *HTTPClient, fastreq *fasthttp.Request, fastresp *fasthttp.Response, config *Config) (ret Response, err error)
```

#### `updateRedirectURL`  <sub>(func)</sub>

> getRedirectURL will construct the redirect URL based off the location header. This will also return if the  
> redirect is on the same host or not.

```go
func updateRedirectURL(base *fasthttp.URI, buf []byte, location []byte) bool
```

#### `StatusCodeIsRedirect`  <sub>(func)</sub>

> StatusCodeIsRedirect returns true if the status code indicates a redirect.

```go
func StatusCodeIsRedirect(statusCode int) bool
```

### `kiterunner/pkg/http/field.go`


#### `HeaderField`  <sub>(struct)</sub>

```go
type HeaderField struct
```

**Fields:**
```go
Key Field
Value Field
```


#### `FieldType`  <sub>(type)</sub>

```go
type FieldType int
```

#### `Field`  <sub>(struct)</sub>

```go
type Field struct
```

**Fields:**
```go
Key string `toml:"key" json:"key" mapstructure:"key"`
Type FieldType `toml:"type" json:"type" mapstructure:"type"`
```


#### `Field.Bytes`  <sub>(method)</sub>

```go
func (f *Field) Bytes() []byte
```

#### `StringToFields`  <sub>(func)</sub>

> StringToField will break down a URL path into fields  
> This is a convenience function to convert /foo/bar into []Field{{"foo"}, {"bar"}}

```go
func StringToFields(in string) (ret []Field)
```

#### `Field.AppendBytes`  <sub>(method)</sub>

> Write the string to the specified writer

```go
func (f *Field) AppendBytes(dst []byte) []byte
```

### `kiterunner/pkg/http/header.go`


#### `Header`  <sub>(struct)</sub>

> Header encapsulates a header key value entry  
> TODO: replace strings with byte slices

```go
type Header struct
```

**Fields:**
```go
Key   string
Value string
```


#### `Headers`  <sub>(type)</sub>

```go
type Headers []Header
```

#### `Headers.MarshalZerologArray`  <sub>(method)</sub>

```go
func (rr Headers) MarshalZerologArray(a *zerolog.Array)
```

#### `Header.MarshalZerologObject`  <sub>(method)</sub>

```go
func (h Header) MarshalZerologObject(e *zerolog.Event)
```

#### `Header.AppendBytes`  <sub>(method)</sub>

```go
func (h *Header) AppendBytes(b []byte) []byte
```

#### `Header.Write`  <sub>(method)</sub>

```go
func (h *Header) Write(buf io.Writer) (int, error)
```

#### `Header.String`  <sub>(method)</sub>

```go
func (h *Header) String() string
```

#### `Header.reset`  <sub>(method)</sub>

```go
func (h *Header) reset()
```

#### `AcquireHeader`  <sub>(func)</sub>

> AcquireHeader retrieves a host from the shared header pool

```go
func AcquireHeader() *Header
```

#### `ReleaseHeader`  <sub>(func)</sub>

> ReleaseHeader releases a host into the shared header pool

```go
func ReleaseHeader(h *Header)
```

### `kiterunner/pkg/http/range.go`


#### `Range`  <sub>(struct)</sub>

```go
type Range struct
```

**Fields:**
```go
Min int
Max int
```


#### `Range.String`  <sub>(method)</sub>

```go
func (r Range) String() string
```

#### `RangeFromString`  <sub>(func)</sub>

> RangeFromString will return a range from a string like 5-10

```go
func RangeFromString(in string) (ret Range, err error)
```

### `kiterunner/pkg/http/request.go`


#### `Request`  <sub>(struct)</sub>

```go
type Request struct
```

**Fields:**
```go
Target *Target
Route *Route
```


#### `Request.String`  <sub>(method)</sub>

```go
func (r *Request) String() string
```

#### `Request.reset`  <sub>(method)</sub>

```go
func (r *Request) reset()
```

#### `Request.WriteRequest`  <sub>(method)</sub>

> WriteRequest will populate the request object with all the required information from the  
> Target details, and from the route details  
> The path is construction with target.Basepath + basepath + route.Path  
> no slashes are added inbetween. we assume the user supplies everything

```go
func (r *Request) WriteRequest(dst *fasthttp.Request, basepath []byte)
```

#### `AcquireRequest`  <sub>(func)</sub>

> AcquireRequest retrieves a host from the shared header pool

```go
func AcquireRequest() *Request
```

#### `ReleaseRequest`  <sub>(func)</sub>

> ReleaseRequest releases a host into the shared header pool

```go
func ReleaseRequest(h *Request)
```

### `kiterunner/pkg/http/response.go`


#### `Response`  <sub>(struct)</sub>

```go
type Response struct
```

**Fields:**
```go
StatusCode  int
Words       int
Lines       int
BodyLength  int
HTTPVersion string
Headers []*Header    // these headers should not be used elsewhere, they are released into the pool on object release into the pool
Body    []byte
URI []byte    // This will only be set in secondary/subsequent requests in a redirect chain
OriginRequest Request    // OriginRequest corresponds to the original request used. this may not correlate to a redirected request
Next          *Response
Error         error
```


#### `Responses`  <sub>(type)</sub>

```go
type Responses []*Response
```

#### `Responses.MarshalZerologArray`  <sub>(method)</sub>

```go
func (rr Responses) MarshalZerologArray(a *zerolog.Array)
```

#### `Response.Flatten`  <sub>(method)</sub>

```go
func (r *Response) Flatten() (ret Responses)
```

#### `Response.MarshalZerologObject`  <sub>(method)</sub>

```go
func (r Response) MarshalZerologObject(e *zerolog.Event)
```

#### `Response.AppendRedirectChain`  <sub>(method)</sub>

```go
func (r *Response) AppendRedirectChain(b []byte) []byte
```

#### `Response.String`  <sub>(method)</sub>

```go
func (r *Response) String() string
```

#### `Response.AddHeader`  <sub>(method)</sub>

```go
func (r *Response) AddHeader(k, v []byte)
```

#### `Response.Reset`  <sub>(method)</sub>

```go
func (r *Response) Reset()
```

#### `AcquireResponse`  <sub>(func)</sub>

> AcquireResponse retrieves a host from the shared header pool

```go
func AcquireResponse() *Response
```

#### `ReleaseResponse`  <sub>(func)</sub>

> ReleaseResponse releases a host into the shared header pool

```go
func ReleaseResponse(h *Response)
```

### `kiterunner/pkg/http/route.go`


#### `Method`  <sub>(type)</sub>

```go
type Method []byte
```

#### `MethodFromString`  <sub>(func)</sub>

```go
func MethodFromString(m string) (Method, error)
```

#### `ChunkedRoutes`  <sub>(type)</sub>

```go
type ChunkedRoutes [][]*Route
```

#### `AcquireChunkedRoutes`  <sub>(func)</sub>

> acquireChunkedRoutes retrieves a host from the shared header pool

```go
func AcquireChunkedRoutes() *ChunkedRoutes
```

#### `ReleaseChunkedRoutes`  <sub>(func)</sub>

> releaseChunkedRoutes releases a host into the shared header pool

```go
func ReleaseChunkedRoutes(h *ChunkedRoutes)
```

#### `ChunkRoutes`  <sub>(func)</sub>

```go
func ChunkRoutes(items []*Route, src *ChunkedRoutes, chunks int) *ChunkedRoutes
```

#### `Route`  <sub>(struct)</sub>

```go
type Route struct
```

**Fields:**
```go
Headers []Header
Path    []byte
Query   []byte
Source  string
Method  Method
Body    []byte
```


#### `Route.String`  <sub>(method)</sub>

```go
func (r Route) String() string
```

#### `Route.MarshalZerologObject`  <sub>(method)</sub>

```go
func (r Route) MarshalZerologObject(e *zerolog.Event)
```

#### `Route.AppendShortBytes`  <sub>(method)</sub>

```go
func (r Route) AppendShortBytes(b []byte) []byte
```

#### `Route.AppendBytes`  <sub>(method)</sub>

```go
func (r Route) AppendBytes(b []byte) []byte
```

#### `Route.AppendPath`  <sub>(method)</sub>

```go
func (r *Route) AppendPath(dst []byte) []byte
```

#### `Route.AppendQuery`  <sub>(method)</sub>

```go
func (r *Route) AppendQuery(dst []byte) []byte
```

### `kiterunner/pkg/http/routes.go`


#### `getDepth`  <sub>(func)</sub>

> getDepth will return up to the N'th+1 slash in the path.  
> e.g. /foo/bar/baz depth 2 will return /foo/bar  
> If a prefix slash is not included, we will prepend the prefix slash,  
> e.g. foo/bar/baz depth 2 will return /foo/bar  
> If there is insufficient path elements, we will return the input element

```go
func getDepth(path string, depth int64) (ret string)
```

#### `RouteMap`  <sub>(type)</sub>

```go
type RouteMap map[string][]*Route
```

#### `RouteMap.FlattenCount`  <sub>(method)</sub>

```go
func (r RouteMap) FlattenCount() int
```

#### `RouteMap.Flatten`  <sub>(method)</sub>

```go
func (r RouteMap) Flatten() []*Route
```

#### `GroupRouteDepth`  <sub>(func)</sub>

> GroupRouteDepth will collate the routes into the corresponding depth of path

```go
func GroupRouteDepth(routes []*Route, depth int64) RouteMap
```

#### `UniqueSource`  <sub>(func)</sub>

> UniqueSource will unique the routes based on the ID

```go
func UniqueSource(routes []*Route) []*Route
```

#### `FilterSource`  <sub>(func)</sub>

> FilterSource will only return the routes that exist in the map provided

```go
func FilterSource(routes []*Route, want map[string]interface{}) []*Route
```

### `kiterunner/pkg/http/target.go`


#### `Target`  <sub>(struct)</sub>

> Target encapsulates the basic data required to scan any given application. This can be specified to attach to a specific  
> vhost or subpath configured webserver.  
>   
> When using target, you *MUST* call ParseHostHeader before calling AppendHostHeader to ensure the header is parsed.  
> This developer unfriendly design decision was made to avoid having to create locks around generating the host header  
> and to avoid allocating additional memory when requesting the host header. We recommend against changing the fields  
> of a target after ParseHostHeader has been called as this will result in unexpected behaviour.  
>   
> The request is sent to the value returned by Host(). this is derived from the IP:port or Hostname:port.  
>   
> This target can be used to instantiate its own client using HTTPClient(maxconns, timeout).  
>   
> The Target should only be instantiated once for each use and is passed around via a pointer to ensure that the  
> HTTPClient is reused.  
>   
> The Quarantine and Context features of a target can be used indicate to other readers across goroutines that the  
> target is responding in an unexpected behaviour (i.e. responding to too many requests unexpected) and/or the  
> target should be abandoned.  
>   
> A sync.Pool is provided for minimising allocations when creating/reusing targets. You can use AcquireTarget() and  
> ReleaseTarget() correspondingly to reuse targets. After a target has been released, it is not safe for reuse and can  
> result in race conditions

```go
type Target struct
```

**Fields:**
```go
Hostname string
HostHeader   []byte    // 2. Hostname:Port
muHostHeader sync.Mutex
IP       string
Port     int    // the IP is the address used to reach the server. If empty, Hostname will be used
IsTLS    bool    // Port will be the port used to reach the server.
BasePath string    // IsTLS defines whether to use a TLS dialer or normal dialer
Headers  []Header    // the Host header is separately governed by HostHeader
ctx       context.Context
ctxcancel func()
hits           int64
quarantineHits int64    // number of hits. if this gets too high, we'll quarantine the host
quarantined    int32
b []byte
httpClient *HTTPClient
```


#### `Target.SetContext`  <sub>(method)</sub>

> SetContext will overwrite this context and cancellation with the provided context

```go
func (t *Target) SetContext(c context.Context)
```

#### `Target.Context`  <sub>(method)</sub>

> Context will return the context of this object. If nil, it will initialize a context for this target  
> This context can be used across goroutines to assess the validity of the goroutine. This is independent  
> of the quarantine state.  
>   
> select {  
> case <-target.Context().Done():  
> return  
> case default:  
> resp, err := http.DoClient(target.HTTPClient(5, time.Second), req, &config.HTTP)  
> }

```go
func (t *Target) Context() context.Context
```

#### `Target.Cancel`  <sub>(method)</sub>

> Cancel will cancel the context associated with the host. This can be used to expire the target's validity across  
> multiple running goroutines. This will simply cancel the context associated with the host. It relies upon user  
> implementation to ensure that further interactions with the target do not occur

```go
func (t *Target) Cancel()
```

#### `Target.HitIncr`  <sub>(method)</sub>

> HitIncr will threadsafe increment the target hit counter.  
> This should be called whenever a request is performed against the host

```go
func (t *Target) HitIncr() int64
```

#### `Target.HitReset`  <sub>(method)</sub>

> Reset returns the old quarantine counter

```go
func (t *Target) HitReset() int64
```

#### `Target.Hits`  <sub>(method)</sub>

> Hits will return the value of the number of hits this target has been used for  
> use HitIncr to incremenet the hits

```go
func (t *Target) Hits() int64
```

#### `Target.QuarantineIncr`  <sub>(method)</sub>

> QuarantineIncr will threadsafe incremenet the quarantine counter  
> This will return the new value of the quarantine counter

```go
func (t *Target) QuarantineIncr() int64
```

#### `Target.QuarantineReset`  <sub>(method)</sub>

> QuarantineReset will threadsafe unquarantine the host

```go
func (t *Target) QuarantineReset() int64
```

#### `Target.Quarantine`  <sub>(method)</sub>

> Quarantine will threadsafe quarantine the host.  
> To reset the quarantine state use QuarantineReset

```go
func (t *Target) Quarantine()
```

#### `Target.Quarantined`  <sub>(method)</sub>

> Quarantined will return whether the target has been quarantined

```go
func (t *Target) Quarantined() bool
```

#### `Target.ParseHostHeader`  <sub>(method)</sub>

> ParseHostHeader will perform a thread safe update of t.HostHeader using the existing fields  
> If t.HostHeader is already set, this operation will just return t.HostHeader  
> otherwise, this will perform t.AppendHost(t.HostHeader[:0])

```go
func (t *Target) ParseHostHeader() []byte
```

#### `Target.AppendHostHeader`  <sub>(method)</sub>

> AppendHostHeader will return the HostHeader to be used in the HTTP Request. This will first prioritize  
> t.HostHeader, then t.Hostname:t.Port. You should always call ParseHostHeader before calling AppendHostHeader  
> otherwise there might not be a host header. We avoid acquiring a lock since this is in the hotpath for requests  
> we don't parse the host header here because that's expensive and requires acquiring a lock

```go
func (t *Target) AppendHostHeader(buf []byte) []byte
```

#### `Target.AppendScheme`  <sub>(method)</sub>

> AppendScheme will append the scheme to the host not including the ://

```go
func (t *Target) AppendScheme(buf []byte) []byte
```

#### `Target.appendColonPort`  <sub>(method)</sub>

> appendColonPort will append :1234 only if its not a standard port (i.e. http://:80 https://:443)  
> this avoids unexpected behaviour with random clients

```go
func (t *Target) appendColonPort(buf []byte) []byte
```

#### `Target.AppendIPOrHostname`  <sub>(method)</sub>

> AppendIPOrHostname will append the ip if set, otherwise the hostname  
> use this to determine where to send the request, not the host header  
> This does not include the port

```go
func (t *Target) AppendIPOrHostname(buf []byte) []byte
```

#### `Target.AppendHost`  <sub>(method)</sub>

> AppendHost will append the host to make the request including the port.  
> e.g. foo.com:80 or if t.IP is set, 1.1.1.1:80  
> this can be used for the HostHeader if HostHeader is not set

```go
func (t *Target) AppendHost(buf []byte) []byte
```

#### `Target.Host`  <sub>(method)</sub>

> Host will return the Host:Port or IP:Port of the target

```go
func (t *Target) Host() string
```

#### `Target.HTTPClient`  <sub>(method)</sub>

> HTTPClient will return a HTTPClient configured for the particular target with the configured  
> maxConnections and timeout.  
> This is cached after the first call, so subsequent changes to the Host and IsTLS after the  
> first call of HTTPClient will not be respected

```go
func (t *Target) HTTPClient(maxConnections int, timeout time.Duration) *HTTPClient
```

#### `Target.AppendBytes`  <sub>(method)</sub>

> AppendBytes will append the full request details including the headers and scheme to the provided buffer  
> e.g. http://google.com:80/foo {x-forwarded-for:127.0.0.1}

```go
func (t *Target) AppendBytes(b []byte) []byte
```

#### `Target.Write`  <sub>(method)</sub>

> Write will write the target out to the buffer specified

```go
func (t *Target) Write(b io.Writer) (int, error)
```

#### `Target.String`  <sub>(method)</sub>

> String will return a string representation of the target

```go
func (t *Target) String() string
```

#### `Target.Bytes`  <sub>(method)</sub>

> Bytes will return the same output as String. This is cached in t.b  
> If the target is changed after Bytes() is called, the changes will not be  
> reflected

```go
func (t *Target) Bytes() []byte
```

#### `Target.reset`  <sub>(method)</sub>

> reset will nil out all the values. This should only be internally called by ReleaseTarget

```go
func (t *Target) reset()
```

#### `AcquireTarget`  <sub>(func)</sub>

> AcquireTarget retrieves a host from the shared target pool

```go
func AcquireTarget() *Target
```

#### `ReleaseTarget`  <sub>(func)</sub>

> ReleaseTarget releases a host into the shared target pool

```go
func ReleaseTarget(h *Target)
```


## `pkg/kitebuilder` <a id='pkg-pkg-kitebuilder'></a>

*22 documented items*


### `kiterunner/pkg/kitebuilder/parse.go`


#### `LoadJSONFile`  <sub>(func)</sub>

> LoadJSONFile will load the json schema from the specified file

```go
func LoadJSONFile(filename string) (schema []API, err error)
```

#### `LoadJSONReader`  <sub>(func)</sub>

```go
func LoadJSONReader(r io.Reader) (schema []API, err error)
```

#### `SlowLoadJSONFile`  <sub>(func)</sub>

> SlowLoadJSONFile will load the json schema from the specified file

```go
func SlowLoadJSONFile(filename string) (schema []API, err error)
```

#### `LoadJSONBytes`  <sub>(func)</sub>

> LoadJSONBytes will load the schema from the provided bytes

```go
func LoadJSONBytes(buf []byte) ([]API, error)
```

#### `SlowLoadJSONBytes`  <sub>(func)</sub>

> SlowLoadJSONBytes will unmarshal the buf into an interface{} and attempt  
> to unmarshal each individual API while printing errors about the object  
> This is a painful manual process when the kitebuilder spec was non-finalised

```go
func SlowLoadJSONBytes(buf []byte) ([]API, error)
```

#### `unmarshalToAPI`  <sub>(func)</sub>

```go
func unmarshalToAPI(v map[string]interface{}) (API, error)
```

#### `unmarshalToParameters`  <sub>(func)</sub>

```go
func unmarshalToParameters(params interface{}, dst *[]Parameter) error
```

#### `UnmarshalJSONString`  <sub>(func)</sub>

```go
func UnmarshalJSONString(v map[string]interface{}, key string, dest interface{}) error
```

#### `GetMapString`  <sub>(func)</sub>

```go
func GetMapString(v map[string]interface{}, key string) (string, error)
```

#### `LoadJSONString`  <sub>(func)</sub>

> LoadString will load the schema from the provided string

```go
func LoadJSONString(buf string) ([]API, error)
```

### `kiterunner/pkg/kitebuilder/print.go`


#### `PrintAPIs`  <sub>(func)</sub>

```go
func PrintAPIs(apis []API)
```

### `kiterunner/pkg/kitebuilder/schema.go`


#### `Path`  <sub>(type)</sub>

```go
type Path string
```

#### `API`  <sub>(struct)</sub>

```go
type API struct
```

**Fields:**
```go
ID                  string                        `json:"ksuid,omitempty"`
URL                 string                        `json:"url"`
SecurityDefinitions map[string]SecurityDefinition `json:"securityDefinitions"`
Paths               map[Path]Operations           `json:"paths"`
```


#### `SecurityDefinition`  <sub>(struct)</sub>

```go
type SecurityDefinition struct
```

**Fields:**
```go
In   string `json:"in"`
Name string `json:"name"`
Type string `json:"type"`
```


#### `OperationTypes`  <sub>(type)</sub>

```go
type OperationTypes string
```

#### `Operations`  <sub>(type)</sub>

> Operations is a map of the verb to the operation data. technically we can have "parameters" here, but then we have  
> dual types for this one object which is fucked

```go
type Operations map[OperationTypes]Operation
```

#### `ContentType`  <sub>(type)</sub>

```go
type ContentType string
```

#### `Operation`  <sub>(struct)</sub>

```go
type Operation struct
```

**Fields:**
```go
Description string        `json:"description,omitempty"`
OperationID string        `json:"operationId,omitempty"`
Parameters  []Parameter   `json:"parameter,omitempty"`
Consumes    []ContentType `json:"consume,omitempty"`
Produces    []ContentType `json:"produce,omitempty"`
```


#### `Typer`  <sub>(interface)</sub>

> Typer provides an interface that makes parameter and schema generic. It provides a list of fields  
> that allow both to be accessed as the same kind of object when reconstructing proute crumbs  
> It doesnt totally fit over both, but its close/good enough with nillable types for determining whats  
> possible and whats not possible

```go
type Typer interface
```

**Members/Variants:**
```go
GetType() string
GetName() string
GetExample() interface{}
GetFormat() string
GetPattern() string
GetDefault() interface{}
GetIn() string
GetMinimum() float64    // exclusive to parameter, so a schema should return empty
GetMaximum() float64
GetProperties() map[string]Schema
GetAdditionalProperties() *Schema
GetSchema() *Schema
GetItems() *Schema
GetAllOf() []Schema
```


#### `Parameter`  <sub>(struct)</sub>

```go
type Parameter struct
```

**Fields:**
```go
Description string      `json:"description,omitempty"`
In          string      `json:"in,omitempty"`
Name        string      `json:"name,omitempty"`
Required    interface{} `json:"required,omitempty"`
Schema *Schema `json:"schema,omitempty"`    // some schemas use "true" instead of true. So we decided to ignore required entirely anyway
Type            string      `json:"type,omitempty"`    // if in != "body"
AllowEmptyValue bool        `json:"allowEmptyValue,omitempty"`
Pattern         string      `json:"pattern,omitempty"`
Format          string      `json:"format,omitempty"`
Example         interface{} `json:"example,omitempty"`
Minimum   float64 `json:"minimum,omitempty"`    // could really be anything. we should handle this with care.
Maximum   float64 `json:"maximum,omitempty"`
MaxLength uint64  `json:"maxLength,omitempty"`
MaxItems  uint64  `json:"maxItems,omitempty"`
MinLength uint64  `json:"minLength,omitempty"`
MinItems  uint64  `json:"minItems,omitempty"`
Enum    []interface{} `json:"enum,omitempty"`
Default interface{}   `json:"default,omitempty"`    // can be [1,2,3] ["a", "b", "c"]
Items *Schema `json:"items,omitempty"`
```


#### `Parameter.GetType`  <sub>(method)</sub>

```go
func (p Parameter) GetType() string                  { return p.Type }
    func (p Parameter) GetName() string                  { return p.Name }
    func (p Parameter) GetIn() string                    { return p.In }
    func (p Parameter) GetExample() interface{}          { return p.Example }
    func (p Parameter) GetMinimum() float64              { return p.Minimum }
    func (p Parameter) GetMaximum() float64              { return p.Maximum }
    func (p Parameter) GetDefault() interface{}          { return p.Default }
    func (p Parameter) GetFormat() string                { return p.Format }
    func (p Parameter) GetPattern() string               { return p.Pattern }
    func (p Parameter) GetProperties() map[string]Schema { return nil }
    func (p Parameter) GetSchema() *Schema               { return p.Schema }
    func (p Parameter) GetAdditionalProperties() *Schema { return nil }
    func (p Parameter) GetItems() *Schema                { return p.Items }
    func (p Parameter) GetAllOf() []Schema               { return nil }
    
    type Schema struct
```

#### `Schema.GetType`  <sub>(method)</sub>

```go
func (s Schema) GetType() string                  { return s.Type }
    func (s Schema) GetName() string                  { return s.Name }
    func (s Schema) GetIn() string                    { return "" }
    func (s Schema) GetExample() interface{}          { return s.Example }
    func (s Schema) GetMinimum() float64              { return s.Min }
    func (s Schema) GetMaximum() float64              { return s.Max }
    func (s Schema) GetDefault() interface{}          { return s.Default }
    func (s Schema) GetFormat() string                { return s.Format }
    func (s Schema) GetPattern() string               { return s.Pattern }
    func (s Schema) GetProperties() map[string]Schema { return s.Properties }
    func (s Schema) GetSchema() *Schema               { return nil }
    func (s Schema) GetAdditionalProperties() *Schema { return s.AdditionalProperties }
    func (s Schema) GetItems() *Schema                { return s.Items }
    func (s Schema) GetAllOf() []Schema               { return s.AllOf }
    
    func (s Schema) IsZero() bool
```


## `pkg/kiterunner` <a id='pkg-pkg-kiterunner'></a>

*102 documented items*


### `kiterunner/pkg/kiterunner/config.go`


#### `ProgressBar`  <sub>(interface)</sub>

```go
type ProgressBar interface
```

**Members/Variants:**
```go
Incr(n int64)
AddTotal(n int64)
```


#### `NullProgressBar`  <sub>(struct)</sub>

```go
type NullProgressBar struct
```

**Fields:**
```go
total int64
hits  int64
```


#### `NullProgressBar.Incr`  <sub>(method)</sub>

```go
func (n *NullProgressBar) Incr(v int64)
```

#### `NullProgressBar.AddTotal`  <sub>(method)</sub>

```go
func (n *NullProgressBar) AddTotal(v int64)
```

#### `Config`  <sub>(struct)</sub>

```go
type Config struct
```

**Fields:**
```go
MaxParallelHosts     int           `toml:"max_parallel_hosts" json:"max_parallel_hosts" mapstructure:"max_parallel_hosts"`
MaxConnPerHost       int           `toml:"max_conn_per_host" json:"max_conn_per_host" mapstructure:"max_conn_per_host"`
WildcardDetection    bool          `json:"wildcard_detection"`
Delay                time.Duration `toml:"delay_ms" json:"delay_ms" mapstructure:"delay_ms"`
HTTP                 http.Config   `toml:"http" json:"http" mapstructure:"http"`
QuarantineThreshold  int64
PreflightCheckRoutes []*http.Route
ProgressBar          ProgressBar    // these are the routes use to calculate the baseline. If the slice is empty, no baselines will be created so requests will match on the status codes
RequestValidators    []RequestValidator
```


#### `NewDefaultConfig`  <sub>(func)</sub>

```go
func NewDefaultConfig() *Config
```

#### `ErrBadConfig`  <sub>(struct)</sub>

```go
type ErrBadConfig struct
```

**Fields:**
```go
fields []string
```


#### `ErrBadConfig.Error`  <sub>(method)</sub>

```go
func (e *ErrBadConfig) Error() string
```

#### `Config.Validate`  <sub>(method)</sub>

```go
func (c *Config) Validate() error
```

#### `ConfigOption`  <sub>(type)</sub>

```go
type ConfigOption func(*Config)
```

#### `MaxTimeout`  <sub>(func)</sub>

```go
func MaxTimeout(n time.Duration) ConfigOption
```

#### `Delay`  <sub>(func)</sub>

```go
func Delay(n time.Duration) ConfigOption
```

#### `MaxRedirects`  <sub>(func)</sub>

```go
func MaxRedirects(n int) ConfigOption
```

#### `MaxConnPerHost`  <sub>(func)</sub>

```go
func MaxConnPerHost(v int) ConfigOption
```

#### `MaxParallelHosts`  <sub>(func)</sub>

```go
func MaxParallelHosts(v int) ConfigOption
```

#### `ReadBody`  <sub>(func)</sub>

```go
func ReadBody(v bool) ConfigOption
```

#### `ReadHeaders`  <sub>(func)</sub>

```go
func ReadHeaders(v bool) ConfigOption
```

#### `BlacklistDomains`  <sub>(func)</sub>

```go
func BlacklistDomains(in []string) ConfigOption
```

#### `WildcardDetection`  <sub>(func)</sub>

```go
func WildcardDetection(enabled bool) ConfigOption
```

#### `AddRequestFilter`  <sub>(func)</sub>

```go
func AddRequestFilter(f RequestValidator) ConfigOption
```

#### `SkipPreflight`  <sub>(func)</sub>

> SkipPreflight will zero out the preflight check routes

```go
func SkipPreflight(enabled bool) ConfigOption
```

#### `AddProgressBar`  <sub>(func)</sub>

```go
func AddProgressBar(p ProgressBar) ConfigOption
```

#### `TargetQuarantineThreshold`  <sub>(func)</sub>

```go
func TargetQuarantineThreshold(n int64) ConfigOption
```

#### `SetPreflightCheckRoutes`  <sub>(func)</sub>

```go
func SetPreflightCheckRoutes(r []*http.Route) ConfigOption
```

#### `HTTPExtraHeaders`  <sub>(func)</sub>

```go
func HTTPExtraHeaders(h []http.Header) ConfigOption
```

### `kiterunner/pkg/kiterunner/job.go`


#### `job`  <sub>(struct)</sub>

```go
type job struct
```

**Fields:**
```go
t     *http.Target
routes []*http.Route
wcr []WildcardResponse
client    *http.HTTPClient
```


#### `job.HTTPRequest`  <sub>(method)</sub>

```go
func (j *job) HTTPRequest() *http.Request
```

#### `job.reset`  <sub>(method)</sub>

```go
func (j *job) reset()
```

#### `acquireJob`  <sub>(func)</sub>

> acquireJob retrieves a host from the shared header pool

```go
func acquireJob() *job
```

#### `releaseJob`  <sub>(func)</sub>

> releaseJob releases a host into the shared header pool

```go
func releaseJob(h *job)
```

#### `subpathBaselineChan`  <sub>(type)</sub>

```go
type subpathBaselineChan chan *subpathBaseline
```

#### `acquireSubpathBaselineChan`  <sub>(func)</sub>

> acquireJobSem retrieves a job semaphore from the pool. this can only be initialized once  
> TODO: make this abstraction more logical and tied to the config value

```go
func acquireSubpathBaselineChan(size int) subpathBaselineChan
```

#### `releaseSubpathBaselineChan`  <sub>(func)</sub>

> releaseJob releases a host into the shared header pool

```go
func releaseSubpathBaselineChan(h subpathBaselineChan)
```

#### `subpathRoutesChan`  <sub>(type)</sub>

```go
type subpathRoutesChan chan subpathRoutes
```

#### `acquireSubpathRoutesChan`  <sub>(func)</sub>

> acquireJobSem retrieves a job semaphore from the pool. this can only be initialized once  
> TODO: make this abstraction more logical and tied to the config value

```go
func acquireSubpathRoutesChan(size int) subpathRoutesChan
```

#### `releaseSubpathRoutesChan`  <sub>(func)</sub>

> releaseJob releases a host into the shared header pool

```go
func releaseSubpathRoutesChan(h subpathRoutesChan)
```

#### `acquireWaitGroup`  <sub>(func)</sub>

```go
func acquireWaitGroup() *sync.WaitGroup
```

#### `releaseWaitGroup`  <sub>(func)</sub>

> releaseJob releases a host into the shared header pool

```go
func releaseWaitGroup(v *sync.WaitGroup)
```

### `kiterunner/pkg/kiterunner/kiterunner.go`


#### `Engine`  <sub>(struct)</sub>

> Engine provides a scan configuration with a set of routes and a specified configuration  
> Calling Run or RunAsync can be done concurrently. Each call will create its own threadpool  
> If you wish to access one threadpool from multiple workers, use RunAsync and communicate using  
> the provided channels  
> The options are non-configurable after instantiation as modifying the routes or config  
> during Run or RunAsync may lead to non-deterministic behaviour

```go
type Engine struct
```

**Fields:**
```go
config *Config
routes http.RouteMap
```


#### `NewEngine`  <sub>(func)</sub>

> NewEngine will create an engine with the defined routes and options.  
> If you require a different set of routes, you should instantiate a new engine

```go
func NewEngine(routes http.RouteMap, opts ...ConfigOption) *Engine
```

#### `Engine.Config`  <sub>(method)</sub>

> Config returns the config for the engine. Modifying this config will modify the config  
> for any currently running scans

```go
func (e *Engine) Config() *Config
```

#### `handleTarget`  <sub>(func)</sub>

> handleTarget will process the target provided. This will manage the number of concurrent requests  
> that can be sent to a particular host with a semaphore channel  
> this function processes preflight check asynchronously  
> This function will terminate when the context is cancelled.  
> the child goroutine will receive the context cancellation and close the subpathRouteChannel  
> the function will wait for the subpathRouteChannel to be closed before exiting to ensure no goroutines are left before exiting

```go
func handleTarget(ctx context.Context, target *http.Target, reqChan chan *ReqMsg, rm http.RouteMap, config *Config) error
```

#### `handleRequest`  <sub>(func)</sub>

> handleRequest is a convenience abstraction for handling the job passed to a job worker  
> this will consume the job, send the request and send the job back through its provided source channel  
> valid results will be passed to the result channel  
> this will process the slice of routes. we perform a slice of routes to avoid having to wait on the channel  
> for each individual route  
> HandleRequest will early exit from processing a slice of routes if the context is cancelled

```go
func handleRequest(ctx context.Context, j *job, resChan chan *Result, config *Config) error
```

#### `Engine.RunAsync`  <sub>(method)</sub>

> RunAsync will begin all the concurrent threads for scanning. Inputs should be fed to the tx (to transmit) and results  
> are read off rx (receive). This function can fail if the config provided to the engine is invalid  
> Each call instantiates its own set of workers, tx, and rx. Hence this is safe to call concurrently  
> The Engine will terminate when the context is cancelled, or when the tx channel is closed  
> When the rx channel is closed, all results have been returned.  
> The caller closing rx may panic and is considered unexpected behaviour

```go
func (e *Engine) RunAsync(ctx context.Context) (tx chan *http.Target, rx chan *Result, err error)
```

### `kiterunner/pkg/kiterunner/kiterunner_wrappers.go`


#### `Engine.Run`  <sub>(method)</sub>

> Run will perform the same operation as RunAsync. This wraps the channels with allocated structs and returns the results  
> This is safe to call from concurrent threads and will use separate worker pools for each call.  
> your callback will be invoked on each result received so you can asynchronously process the results if you wish  
> All the results will still be returned by the []*Result slice. Modifying the result in the callback is considered  
> undefined behaviour

```go
func (e *Engine) Run(ctx context.Context, input []*http.Target) ([]*Result, error)
```

#### `Engine.RunCallback`  <sub>(method)</sub>

> RunCallback will run the scan against the provided input, calling the provided callbacks on each result.  
> the callbacks can be used to log the error in realtime, or perform other processes. You should not  
> modify or use the Target, or route from the Result as this may have unintended side effects

```go
func (e *Engine) RunCallback(ctx context.Context, input []*http.Target, cb ...func(r *Result, c *Config)) ([]*Result, error)
```

#### `Engine.RunCallbackNoResult`  <sub>(method)</sub>

> RunCallbackNoResult will run the scan against the provided input, calling the provided callbacks on each result.  
> the callbacks can be used to log the error in realtime, or perform other processes. You should not  
> modify or use the Target, or route from the Result as this may have unintended side effects  
> This function does not return the results as they are released immediately after all callbacks are called.  
> It is unsafe to use the result after your callbacks return  
> Use this when you don't require using the result after the callback, e.g. writing to disk/printing to output

```go
func (e *Engine) RunCallbackNoResult(ctx context.Context, input []*http.Target, cb ...func(r *Result, c *Config)) (error)
```

### `kiterunner/pkg/kiterunner/preflight.go`


#### `WildcardResponse`  <sub>(struct)</sub>

```go
type WildcardResponse struct
```

**Fields:**
```go
DefaultStatusCode     int
DefaultContentLength  int
AdjustedContentLength int
AdjustmentScale       int    // adjustedContentLength is the content length adjusted for the length of the requested path
DefaultWordCount      int    // number of times the requested path appears in the request
DefaultLineCount      int    // number of spaces + 1
```


#### `WildcardResponses`  <sub>(type)</sub>

```go
type WildcardResponses []WildcardResponse
```

#### `WildcardResponses.UniqueAdd`  <sub>(method)</sub>

```go
func (w WildcardResponses) UniqueAdd(wr WildcardResponse) (WildcardResponses, bool)
```

#### `ReqMsgType`  <sub>(type)</sub>

```go
type ReqMsgType int
```

#### `ReqMsg`  <sub>(struct)</sub>

```go
type ReqMsg struct
```

**Fields:**
```go
typ       ReqMsgType
Preflight *subpathBaseline
Job       *job
```


#### `ReqMsg.reset`  <sub>(method)</sub>

```go
func (s *ReqMsg) reset()
```

#### `acquireReqMsg`  <sub>(func)</sub>

> AcquireReqMsg retrieves a host from the shared header pool

```go
func acquireReqMsg() *ReqMsg
```

#### `releaseReqMsg`  <sub>(func)</sub>

> ReleaseReqMsg releases a host into the shared header pool

```go
func releaseReqMsg(h *ReqMsg)
```

#### `subpathRoutes`  <sub>(struct)</sub>

```go
type subpathRoutes struct
```

**Fields:**
```go
routes    []*http.Route
responses chan *subpathBaseline
```


#### `subpathBaseline`  <sub>(struct)</sub>

```go
type subpathBaseline struct
```

**Fields:**
```go
base     []byte
target   *http.Target
route    *http.Route
baseline WildcardResponse
err      error
resp     chan *subpathBaseline
```


#### `subpathBaseline.reset`  <sub>(method)</sub>

```go
func (s *subpathBaseline) reset()
```

#### `acquireSubpathBaseline`  <sub>(func)</sub>

> AcquiresubpathBaseline retrieves a host from the shared header pool

```go
func acquireSubpathBaseline() *subpathBaseline
```

#### `releaseSubpathBaseline`  <sub>(func)</sub>

> ReleasesubpathBaseline releases a host into the shared header pool

```go
func releaseSubpathBaseline(h *subpathBaseline)
```

#### `ErrFailedPreflight`  <sub>(struct)</sub>

```go
type ErrFailedPreflight struct
```

**Fields:**
```go
err error
```


#### `ErrFailedPreflight.Error`  <sub>(method)</sub>

```go
func (e ErrFailedPreflight) Error() string
```

#### `ErrFailedPreflight.Unwrap`  <sub>(method)</sub>

```go
func (e ErrFailedPreflight) Unwrap() error
```

#### `targetWildcardDetection`  <sub>(func)</sub>

> targetWildcardDetection attempts to determine what elements of the response  
> correspond to a wildcard. This is then used when validating future requests against the target  
> to discard any erronous results

```go
func targetWildcardDetection(resp *fasthttp.Response, basepath string) WildcardResponse
```

#### `preflightCheck`  <sub>(func)</sub>

> preflightCheck will perform a basic http request against the target and return an error if it failed.  
> this will also perform wildcard detection and populate the target with the corresponding wildcardThreshold data  
> if nil is returned the request succeeded  
> PreflightCheck will perform a set of preflight checks against the provided basepath  
> This should be a '/' prefixed string, if this is empty, it will default to '/'  
> this will return the wildcard responses calculated from the preflight check routes  
> semaphore can be the jobsemaphore or nil. If its nil, then we instantiate one temporarily  
> this ensures the client doesn't exhaust all its available connections

```go
func preflightCheck(route *http.Route, t *http.Target, config *Config, basepath []byte) (ret WildcardResponse, err error)
```

### `kiterunner/pkg/kiterunner/result.go`


#### `Result`  <sub>(struct)</sub>

```go
type Result struct
```

**Fields:**
```go
Target   *http.Target
Route    *http.Route
Response http.Response
```


#### `Result.String`  <sub>(method)</sub>

```go
func (r *Result) String() string
```

#### `Result.reset`  <sub>(method)</sub>

```go
func (r *Result) reset()
```

#### `Result.Release`  <sub>(method)</sub>

> Release will place the called result back into the pool. After release it is not safe for use

```go
func (r *Result) Release()
```

#### `AcquireResult`  <sub>(func)</sub>

> AcquireResult retrieves a host from the shared header pool

```go
func AcquireResult() *Result
```

#### `ReleaseResult`  <sub>(func)</sub>

> ReleaseResult releases a host into the shared header pool

```go
func ReleaseResult(h *Result)
```

#### `leftpadAppendBytes`  <sub>(func)</sub>

```go
func leftpadAppendBytes(buf []byte, in []byte, pad int) []byte
```

#### `rightpadAppendBytes`  <sub>(func)</sub>

```go
func rightpadAppendBytes(buf []byte, in []byte, pad int) []byte
```

#### `Attribute`  <sub>(type)</sub>

```go
type Attribute int
```

#### `getColor`  <sub>(func)</sub>

```go
func getColor(sc int) Attribute
```

#### `appendColorStatusCode`  <sub>(func)</sub>

```go
func appendColorStatusCode(buf []byte, sc int) []byte
```

#### `appendColorStart`  <sub>(func)</sub>

```go
func appendColorStart(buf []byte, code Attribute) []byte
```

#### `appendColorEnd`  <sub>(func)</sub>

```go
func appendColorEnd(buf []byte) []byte
```

#### `appendColor`  <sub>(func)</sub>

```go
func appendColor(buf []byte, in []byte, code Attribute) []byte
```

#### `appendPaddedIntColor`  <sub>(func)</sub>

```go
func appendPaddedIntColor(b []byte, v int, padding int) []byte
```

#### `Result.AppendPrettyBytes`  <sub>(method)</sub>

> <METHOD> <STATUSCODE> [lines, words, lines] <URL> [redirects ...]

```go
func (r *Result) AppendPrettyBytes(b []byte) []byte
```

#### `Result.AppendBytes`  <sub>(method)</sub>

> <METHOD> <STATUSCODE> <URL> [redirects ...]

```go
func (r *Result) AppendBytes(b []byte) []byte
```

#### `LogResultsChan`  <sub>(func)</sub>

> LogResultsChan will output the results using the configured logger

```go
func LogResultsChan(ctx context.Context, res chan *Result, config *Config)
```

#### `LogResults`  <sub>(func)</sub>

> LogResults will output the results using the configured logger

```go
func LogResults(res []*Result, config *Config)
```

#### `LogResult`  <sub>(func)</sub>

```go
func LogResult(r *Result, config *Config)
```

### `kiterunner/pkg/kiterunner/validator.go`


#### `RequestValidator`  <sub>(interface)</sub>

> RequestValidator is an interface that lets you add custom validators for what are good and bad responses

```go
type RequestValidator interface
```

**Members/Variants:**
```go
Validate(r http.Response, wildcardResponses []WildcardResponse, c *Config) error
```


#### `KnownBadSitesValidator`  <sub>(struct)</sub>

```go
type KnownBadSitesValidator struct{}
```

#### `KnownBadSitesValidator.Validate`  <sub>(method)</sub>

```go
func (v *KnownBadSitesValidator) Validate(r http.Response, wildcardResponses []WildcardResponse, c *Config) error
```

#### `WildcardResponseValidator`  <sub>(struct)</sub>

```go
type WildcardResponseValidator struct{}
```

#### `WildcardResponseValidator.Validate`  <sub>(method)</sub>

```go
func (v *WildcardResponseValidator) Validate(r http.Response, wildcardResponses []WildcardResponse, c *Config) error
```

#### `ContentLengthValidator`  <sub>(struct)</sub>

```go
type ContentLengthValidator struct
```

**Fields:**
```go
IgnoreRanges []http.Range
```


#### `NewContentLengthValidator`  <sub>(func)</sub>

```go
func NewContentLengthValidator(ranges []http.Range) *ContentLengthValidator
```

#### `ContentLengthValidator.String`  <sub>(method)</sub>

```go
func (v ContentLengthValidator) String() string
```

#### `ContentLengthValidator.Validate`  <sub>(method)</sub>

```go
func (v *ContentLengthValidator) Validate(r http.Response, _ []WildcardResponse, _ *Config) error
```

#### `StatusCodeWhitelist`  <sub>(struct)</sub>

```go
type StatusCodeWhitelist struct
```

**Fields:**
```go
Codes map[int]interface{}
```


#### `NewStatusCodeWhitelist`  <sub>(func)</sub>

```go
func NewStatusCodeWhitelist(valid []int) *StatusCodeWhitelist
```

#### `StatusCodeWhitelist.String`  <sub>(method)</sub>

```go
func (v StatusCodeWhitelist) String() string
```

#### `StatusCodeWhitelist.Validate`  <sub>(method)</sub>

```go
func (v *StatusCodeWhitelist) Validate(r http.Response, _ []WildcardResponse, _ *Config) error
```

#### `StatusCodeBlacklist`  <sub>(struct)</sub>

```go
type StatusCodeBlacklist struct
```

**Fields:**
```go
Codes map[int]interface{}
```


#### `NewStatusCodeBlacklist`  <sub>(func)</sub>

```go
func NewStatusCodeBlacklist(valid []int) *StatusCodeBlacklist
```

#### `StatusCodeBlacklist.String`  <sub>(method)</sub>

```go
func (v StatusCodeBlacklist) String() string
```

#### `StatusCodeBlacklist.Validate`  <sub>(method)</sub>

```go
func (v *StatusCodeBlacklist) Validate(r http.Response, _ []WildcardResponse, _ *Config) error
```


## `pkg/log` <a id='pkg-pkg-log'></a>

*4 documented items*


### `kiterunner/pkg/log/log.go`


#### `LogFormat`  <sub>(type)</sub>

```go
type LogFormat string
```

#### `SetLevelString`  <sub>(func)</sub>

```go
func SetLevelString(level string) error
```

#### `GetLogFormat`  <sub>(func)</sub>

```go
func GetLogFormat() LogFormat
```

#### `SetFormat`  <sub>(func)</sub>

```go
func SetFormat(format string) error
```


## `pkg/proute` <a id='pkg-pkg-proute'></a>

*503 documented items*


### `kiterunner/pkg/proute/crumb.go`


#### `Crumb`  <sub>(interface)</sub>

> Crumb is a piece of a route, can be dynamically rendered or not

```go
type Crumb interface
```

**Members/Variants:**
```go
Value(...CrumbOption) string
RawValue(...CrumbOption) interface{}    // we pass opts so at runtime we can determine if we want to configure any behaviour
Key() string
protoCrumb() *ProtoCrumb
```


#### `CrumbOptions`  <sub>(struct)</sub>

> CrumbOptions provides options that can be used by any crumb when generating the values  
> TODO: add rng as a source from the crumb options

```go
type CrumbOptions struct
```

**Fields:**
```go
ContentType      ContentType
FormDataBoundary string
IsChild          bool
Random           bool
```


#### `CrumbOption`  <sub>(type)</sub>

```go
type CrumbOption func(o *CrumbOptions)
```

#### `DefaultCrumbOptions`  <sub>(func)</sub>

```go
func DefaultCrumbOptions() *CrumbOptions
```

#### `NewCrumbOptions`  <sub>(func)</sub>

```go
func NewCrumbOptions(opts ...CrumbOption) *CrumbOptions
```

#### `CrumbOptContentType`  <sub>(func)</sub>

```go
func CrumbOptContentType(v ContentType) CrumbOption
```

#### `CrumbOptFormDataBoundary`  <sub>(func)</sub>

```go
func CrumbOptFormDataBoundary(v string) CrumbOption
```

#### `CrumbOptIsChild`  <sub>(func)</sub>

```go
func CrumbOptIsChild(v bool) CrumbOption
```

#### `UUIDCrumb.Key`  <sub>(method)</sub>

```go
func (p UUIDCrumb) Key() string
```

#### `UUIDCrumb.Value`  <sub>(method)</sub>

```go
func (p UUIDCrumb) Value(...CrumbOption) string
```

#### `UUIDCrumb.RawValue`  <sub>(method)</sub>

```go
func (p UUIDCrumb) RawValue(...CrumbOption) interface{}
```

#### `UUIDCrumb.protoCrumb`  <sub>(method)</sub>

```go
func (p UUIDCrumb) protoCrumb() *ProtoCrumb
```

#### `StaticCrumb.Key`  <sub>(method)</sub>

```go
func (s StaticCrumb) Key() string
```

#### `StaticCrumb.Value`  <sub>(method)</sub>

```go
func (s StaticCrumb) Value(...CrumbOption) string
```

#### `StaticCrumb.RawValue`  <sub>(method)</sub>

```go
func (s StaticCrumb) RawValue(...CrumbOption) interface{}
```

#### `StaticCrumb.protoCrumb`  <sub>(method)</sub>

```go
func (p StaticCrumb) protoCrumb() *ProtoCrumb
```

#### `IntCrumb.Value`  <sub>(method)</sub>

```go
func (i IntCrumb) Value(opts ...CrumbOption) string
```

#### `IntCrumb.Key`  <sub>(method)</sub>

```go
func (i IntCrumb) Key() string
```

#### `IntCrumb.RawValue`  <sub>(method)</sub>

```go
func (i IntCrumb) RawValue(...CrumbOption) interface{}
```

#### `IntCrumb.protoCrumb`  <sub>(method)</sub>

```go
func (p IntCrumb) protoCrumb() *ProtoCrumb
```

#### `BoolCrumb.Value`  <sub>(method)</sub>

```go
func (b BoolCrumb) Value(opts ...CrumbOption) string
```

#### `BoolCrumb.Key`  <sub>(method)</sub>

```go
func (b BoolCrumb) Key() string
```

#### `BoolCrumb.RawValue`  <sub>(method)</sub>

```go
func (b BoolCrumb) RawValue(...CrumbOption) interface{}
```

#### `BoolCrumb.protoCrumb`  <sub>(method)</sub>

```go
func (p BoolCrumb) protoCrumb() *ProtoCrumb
```

#### `FloatCrumb.Value`  <sub>(method)</sub>

```go
func (i FloatCrumb) Value(opts ...CrumbOption) string
```

#### `FloatCrumb.Key`  <sub>(method)</sub>

```go
func (i FloatCrumb) Key() string
```

#### `FloatCrumb.RawValue`  <sub>(method)</sub>

```go
func (i FloatCrumb) RawValue(...CrumbOption) interface{}
```

#### `FloatCrumb.protoCrumb`  <sub>(method)</sub>

```go
func (p FloatCrumb) protoCrumb() *ProtoCrumb
```

#### `RandomStringCrumb.Key`  <sub>(method)</sub>

```go
func (s RandomStringCrumb) Key() string
```

#### `RandomStringCrumb.Value`  <sub>(method)</sub>

> Value(...CrumbOption) will return a random string value. For maximum compatability, this return a number

```go
func (s RandomStringCrumb) Value(...CrumbOption) string
```

#### `RandomStringCrumb.RawValue`  <sub>(method)</sub>

```go
func (s RandomStringCrumb) RawValue(opts ...CrumbOption) interface{}
```

#### `RandomStringCrumb.protoCrumb`  <sub>(method)</sub>

```go
func (p RandomStringCrumb) protoCrumb() *ProtoCrumb
```

#### `RegexStringCrumb.Validate`  <sub>(method)</sub>

```go
func (r RegexStringCrumb) Validate() error
```

#### `RegexStringCrumb.Value`  <sub>(method)</sub>

```go
func (r RegexStringCrumb) Value(...CrumbOption) string
```

#### `RegexStringCrumb.RawValue`  <sub>(method)</sub>

```go
func (r RegexStringCrumb) RawValue(opts ...CrumbOption) interface{}
```

#### `RegexStringCrumb.Key`  <sub>(method)</sub>

```go
func (r RegexStringCrumb) Key() string
```

#### `RegexStringCrumb.protoCrumb`  <sub>(method)</sub>

```go
func (p RegexStringCrumb) protoCrumb() *ProtoCrumb
```

#### `BasicAuthCrumb.Value`  <sub>(method)</sub>

```go
func (b BasicAuthCrumb) Value(...CrumbOption) string
```

#### `BasicAuthCrumb.RawValue`  <sub>(method)</sub>

```go
func (b BasicAuthCrumb) RawValue(opts ...CrumbOption) interface{}
```

#### `BasicAuthCrumb.Key`  <sub>(method)</sub>

```go
func (b BasicAuthCrumb) Key() string
```

#### `BasicAuthCrumb.protoCrumb`  <sub>(method)</sub>

```go
func (p BasicAuthCrumb) protoCrumb() *ProtoCrumb
```

#### `MarshalXMLCrumb`  <sub>(func)</sub>

> MarshalXMLCrumb will create an XML object containing just that crumb. The object name is set to the crumb name  
> and the child value is set to the child value of the crumb

```go
func MarshalXMLCrumb(c Crumb, opts ...CrumbOption) string
```

#### `XMLer`  <sub>(interface)</sub>

```go
type XMLer interface
```

**Members/Variants:**
```go
AddChild(t etree.Token)
SetText(string)
CreateElement(tag string) *etree.Element
```


#### `ObjectCrumb`  <sub>(struct)</sub>

```go
type ObjectCrumb struct
```

**Fields:**
```go
Name     string
Elements []Crumb
```


#### `XMLAddChildCrumb`  <sub>(func)</sub>

> XMLAddChildCrumb is a dirty method of building an XML document using strings  
> if the child is to be an object, we need to unmarshal it to get a etree elemtn we can append  
> otherwise it'll encode our xml object. Fuck this is stupid

```go
func XMLAddChildCrumb(xml XMLer, c Crumb, nameOverride string, opts ...CrumbOption) error
```

#### `ObjectCrumb.Value`  <sub>(method)</sub>

```go
func (o ObjectCrumb) Value(opts ...CrumbOption) string
```

#### `ObjectCrumb.Key`  <sub>(method)</sub>

```go
func (o ObjectCrumb) Key() string
```

#### `ObjectCrumb.RawValue`  <sub>(method)</sub>

```go
func (o ObjectCrumb) RawValue(opts ...CrumbOption) interface{}
```

#### `ObjectCrumb.MarshalJSONObject`  <sub>(method)</sub>

```go
func (o ObjectCrumb) MarshalJSONObject(enc *gojay.Encoder)
```

#### `ObjectCrumb.IsNil`  <sub>(method)</sub>

```go
func (o ObjectCrumb) IsNil() bool
```

#### `ObjectCrumb.protoCrumb`  <sub>(method)</sub>

```go
func (p ObjectCrumb) protoCrumb() *ProtoCrumb
```

#### `ObjectCrumb.protoObjectCrumb`  <sub>(method)</sub>

```go
func (p ObjectCrumb) protoObjectCrumb() *ProtoObjectCrumb
```

#### `ArrayCrumb`  <sub>(struct)</sub>

```go
type ArrayCrumb struct
```

**Fields:**
```go
Name    string
Element Crumb
```


#### `ArrayCrumb.Value`  <sub>(method)</sub>

```go
func (a ArrayCrumb) Value(opts ...CrumbOption) string
```

#### `ArrayCrumb.RawValue`  <sub>(method)</sub>

```go
func (a ArrayCrumb) RawValue(opts ...CrumbOption) interface{}
```

#### `ArrayCrumb.Key`  <sub>(method)</sub>

```go
func (a ArrayCrumb) Key() string
```

#### `ArrayCrumb.MarshalJSONArray`  <sub>(method)</sub>

```go
func (a ArrayCrumb) MarshalJSONArray(enc *gojay.Encoder)
```

#### `ArrayCrumb.IsNil`  <sub>(method)</sub>

```go
func (a ArrayCrumb) IsNil() bool
```

#### `ArrayCrumb.protoCrumb`  <sub>(method)</sub>

```go
func (p ArrayCrumb) protoCrumb() *ProtoCrumb
```

#### `ArrayCrumb.protoArrayCrumb`  <sub>(method)</sub>

```go
func (p ArrayCrumb) protoArrayCrumb() *ProtoArrayCrumb
```

#### `StringCrumbCrumb`  <sub>(struct)</sub>

```go
type StringCrumbCrumb struct
```

**Fields:**
```go
Name  string
Child Crumb
```


#### `StringCrumbCrumb.Value`  <sub>(method)</sub>

```go
func (s StringCrumbCrumb) Value(option ...CrumbOption) string
```

#### `StringCrumbCrumb.Key`  <sub>(method)</sub>

```go
func (s StringCrumbCrumb) Key() string
```

#### `StringCrumbCrumb.RawValue`  <sub>(method)</sub>

```go
func (s StringCrumbCrumb) RawValue(option ...CrumbOption) interface{}
```

#### `StringCrumbCrumb.protoCrumb`  <sub>(method)</sub>

```go
func (p StringCrumbCrumb) protoCrumb() *ProtoCrumb
```

#### `StringCrumbCrumb.protoStringCrumbCrumb`  <sub>(method)</sub>

```go
func (p StringCrumbCrumb) protoStringCrumbCrumb() *ProtoStringCrumbCrumb
```

### `kiterunner/pkg/proute/kitebuilder.go`


#### `floatCrumbFromInterface`  <sub>(func)</sub>

```go
func floatCrumbFromInterface(name string, v interface{}) (FloatCrumb, error)
```

#### `objectCrumbFromSchema`  <sub>(func)</sub>

> objectCrumbFromSchema will construct an object crumb based off the schema. this operates recursively since an  
> OpenAPI Schema can contain multiple nested schemas.  
> if the name is empty, we will attempt to deduce the name from the schema name.  
> Neither of these are always possible, if all else fails it just becomes a string.  
> This handles a Typer interface so it can handle the Parameter and the Schema types, (both are kinda similar)  
> This way we don't need to duplicate logic to handle different types. but it might be a bad idea later

```go
func objectCrumbFromSchema(name string, v kitebuilder.Typer) (Crumb, error)
```

#### `FromKitebuilderAPIs`  <sub>(func)</sub>

```go
func FromKitebuilderAPIs(src []kitebuilder.API) ([]API, error)
```

#### `FromKitebuilderAPI`  <sub>(func)</sub>

> FromKitebuilderAPI will convert an kitebuilder API to a proute API. Errors are swallowed and printed to stdout because  
> we hate ourselves other developers who try to use this. I can't believe I have to parse json types into usable data  
> future readers beware. https://www.youtube.com/watch?v=LZWnM_u7Vfg

```go
func FromKitebuilderAPI(src kitebuilder.API) (API, error)
```

#### `APIS.ToKiteBuilderAPIS`  <sub>(method)</sub>

```go
func (a APIS) ToKiteBuilderAPIS() ([]kitebuilder.API, error)
```

#### `CrumbToSchema`  <sub>(func)</sub>

```go
func CrumbToSchema(c Crumb) kitebuilder.Schema
```

#### `CrumbToParameter`  <sub>(func)</sub>

```go
func CrumbToParameter(c Crumb) kitebuilder.Parameter
```

#### `CrumbToSecurityDefinition`  <sub>(func)</sub>

```go
func CrumbToSecurityDefinition(c Crumb) kitebuilder.SecurityDefinition
```

#### `API.ToKitebuilderAPI`  <sub>(method)</sub>

```go
func (a API) ToKitebuilderAPI() (kitebuilder.API, error)
```

### `kiterunner/pkg/proute/kiterunner.go`


#### `APIsToKiterunnerRoutes`  <sub>(func)</sub>

```go
func APIsToKiterunnerRoutes(api []API) ([]*http.Route, error)
```

#### `ToKiterunnerRoutes`  <sub>(func)</sub>

```go
func ToKiterunnerRoutes(api API) ([]*http.Route, error)
```

#### `Route.ToKiterunner`  <sub>(method)</sub>

```go
func (r Route) ToKiterunner(extraHeaders ...KV) (*http.Route, error)
```

### `kiterunner/pkg/proute/print.go`


#### `crumbString`  <sub>(func)</sub>

```go
func crumbString(in []Crumb) string
```

### `kiterunner/pkg/proute/proute.go`


#### `API`  <sub>(struct)</sub>

> API encapsulates all the routes and required headers for the routes

```go
type API struct
```

**Fields:**
```go
URL    string
ID     string    // the URL source of the API
Routes []Route    // the Ksuid for this API which can be any UUID really... but KSUID is convenient
QueryCrumbs []Crumb    // Crumbs to add to all the Routes
queryParams []KV
HeaderCrumbs []Crumb
headers      []KV
BodyCrumbs []Crumb
bodyParams []KV
CookieCrumbs []Crumb
cookieParams []KV
```


#### `APIS`  <sub>(type)</sub>

> APIS are multiple APIs, defined as a type for convenience

```go
type APIS []API
```

#### `FromAPISlice`  <sub>(func)</sub>

```go
func FromAPISlice(a []API) APIS
```

#### `APIS.First`  <sub>(method)</sub>

```go
func (a APIS) First(n int) APIS
```

#### `API.DebugPrint`  <sub>(method)</sub>

```go
func (a API) DebugPrint()
```

#### `API.QueryParams`  <sub>(method)</sub>

```go
func (a API) QueryParams(generate bool) []KV
```

#### `API.CookieParams`  <sub>(method)</sub>

```go
func (a API) CookieParams(generate bool) []KV
```

#### `API.BodyParams`  <sub>(method)</sub>

```go
func (a API) BodyParams(generate bool) []KV
```

#### `API.Headers`  <sub>(method)</sub>

```go
func (a API) Headers(generate bool) []KV
```

#### `NewAPI`  <sub>(func)</sub>

```go
func NewAPI(id string, url string) API
```

### `kiterunner/pkg/proute/proute.pb.go`


#### `UUIDCrumb`  <sub>(struct)</sub>

> UUIDCrumb implements a crumb that renders a uuid when called

```go
type UUIDCrumb struct
```

**Fields:**
```go
Name string `protobuf:"bytes,1,opt,name=Name,proto3" json:"Name,omitempty"`
```


#### `UUIDCrumb.Reset`  <sub>(method)</sub>

```go
func (m *UUIDCrumb) Reset()      { *m = UUIDCrumb{} }
    func (*UUIDCrumb) ProtoMessage() {}
    func (*UUIDCrumb) Descriptor() ([]byte, []int)
```

#### `UUIDCrumb.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *UUIDCrumb) XXX_Unmarshal(b []byte) error
```

#### `UUIDCrumb.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *UUIDCrumb) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `UUIDCrumb.XXX_Merge`  <sub>(method)</sub>

```go
func (m *UUIDCrumb) XXX_Merge(src proto.Message)
```

#### `UUIDCrumb.XXX_Size`  <sub>(method)</sub>

```go
func (m *UUIDCrumb) XXX_Size() int
```

#### `UUIDCrumb.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *UUIDCrumb) XXX_DiscardUnknown()
```

#### `UUIDCrumb.GetName`  <sub>(method)</sub>

```go
func (m *UUIDCrumb) GetName() string
```

#### `StaticCrumb`  <sub>(struct)</sub>

> StaticCrumb implements a crumb that renders a static value when called

```go
type StaticCrumb struct
```

**Fields:**
```go
K string `protobuf:"bytes,1,opt,name=K,proto3" json:"K,omitempty"`
V string `protobuf:"bytes,2,opt,name=V,proto3" json:"V,omitempty"`
```


#### `StaticCrumb.Reset`  <sub>(method)</sub>

```go
func (m *StaticCrumb) Reset()      { *m = StaticCrumb{} }
    func (*StaticCrumb) ProtoMessage() {}
    func (*StaticCrumb) Descriptor() ([]byte, []int)
```

#### `StaticCrumb.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *StaticCrumb) XXX_Unmarshal(b []byte) error
```

#### `StaticCrumb.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *StaticCrumb) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `StaticCrumb.XXX_Merge`  <sub>(method)</sub>

```go
func (m *StaticCrumb) XXX_Merge(src proto.Message)
```

#### `StaticCrumb.XXX_Size`  <sub>(method)</sub>

```go
func (m *StaticCrumb) XXX_Size() int
```

#### `StaticCrumb.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *StaticCrumb) XXX_DiscardUnknown()
```

#### `StaticCrumb.GetK`  <sub>(method)</sub>

```go
func (m *StaticCrumb) GetK() string
```

#### `StaticCrumb.GetV`  <sub>(method)</sub>

```go
func (m *StaticCrumb) GetV() string
```

#### `IntCrumb`  <sub>(struct)</sub>

```go
type IntCrumb struct
```

**Fields:**
```go
Min   int64  `protobuf:"varint,1,opt,name=Min,proto3" json:"Min,omitempty"`
Max   int64  `protobuf:"varint,2,opt,name=Max,proto3" json:"Max,omitempty"`
Val   int64  `protobuf:"varint,3,opt,name=Val,proto3" json:"Val,omitempty"`
Fixed bool   `protobuf:"varint,4,opt,name=Fixed,proto3" json:"Fixed,omitempty"`
Name  string `protobuf:"bytes,5,opt,name=Name,proto3" json:"Name,omitempty"`
```


#### `IntCrumb.Reset`  <sub>(method)</sub>

```go
func (m *IntCrumb) Reset()      { *m = IntCrumb{} }
    func (*IntCrumb) ProtoMessage() {}
    func (*IntCrumb) Descriptor() ([]byte, []int)
```

#### `IntCrumb.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *IntCrumb) XXX_Unmarshal(b []byte) error
```

#### `IntCrumb.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *IntCrumb) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `IntCrumb.XXX_Merge`  <sub>(method)</sub>

```go
func (m *IntCrumb) XXX_Merge(src proto.Message)
```

#### `IntCrumb.XXX_Size`  <sub>(method)</sub>

```go
func (m *IntCrumb) XXX_Size() int
```

#### `IntCrumb.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *IntCrumb) XXX_DiscardUnknown()
```

#### `IntCrumb.GetMin`  <sub>(method)</sub>

```go
func (m *IntCrumb) GetMin() int64
```

#### `IntCrumb.GetMax`  <sub>(method)</sub>

```go
func (m *IntCrumb) GetMax() int64
```

#### `IntCrumb.GetVal`  <sub>(method)</sub>

```go
func (m *IntCrumb) GetVal() int64
```

#### `IntCrumb.GetFixed`  <sub>(method)</sub>

```go
func (m *IntCrumb) GetFixed() bool
```

#### `IntCrumb.GetName`  <sub>(method)</sub>

```go
func (m *IntCrumb) GetName() string
```

#### `BoolCrumb`  <sub>(struct)</sub>

```go
type BoolCrumb struct
```

**Fields:**
```go
Name  string `protobuf:"bytes,1,opt,name=Name,proto3" json:"Name,omitempty"`
Fixed bool   `protobuf:"varint,2,opt,name=Fixed,proto3" json:"Fixed,omitempty"`
Val   bool   `protobuf:"varint,3,opt,name=Val,proto3" json:"Val,omitempty"`
```


#### `BoolCrumb.Reset`  <sub>(method)</sub>

```go
func (m *BoolCrumb) Reset()      { *m = BoolCrumb{} }
    func (*BoolCrumb) ProtoMessage() {}
    func (*BoolCrumb) Descriptor() ([]byte, []int)
```

#### `BoolCrumb.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *BoolCrumb) XXX_Unmarshal(b []byte) error
```

#### `BoolCrumb.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *BoolCrumb) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `BoolCrumb.XXX_Merge`  <sub>(method)</sub>

```go
func (m *BoolCrumb) XXX_Merge(src proto.Message)
```

#### `BoolCrumb.XXX_Size`  <sub>(method)</sub>

```go
func (m *BoolCrumb) XXX_Size() int
```

#### `BoolCrumb.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *BoolCrumb) XXX_DiscardUnknown()
```

#### `BoolCrumb.GetName`  <sub>(method)</sub>

```go
func (m *BoolCrumb) GetName() string
```

#### `BoolCrumb.GetFixed`  <sub>(method)</sub>

```go
func (m *BoolCrumb) GetFixed() bool
```

#### `BoolCrumb.GetVal`  <sub>(method)</sub>

```go
func (m *BoolCrumb) GetVal() bool
```

#### `FloatCrumb`  <sub>(struct)</sub>

```go
type FloatCrumb struct
```

**Fields:**
```go
Name  string  `protobuf:"bytes,1,opt,name=Name,proto3" json:"Name,omitempty"`
Fixed bool    `protobuf:"varint,2,opt,name=Fixed,proto3" json:"Fixed,omitempty"`
Val   float64 `protobuf:"fixed64,3,opt,name=Val,proto3" json:"Val,omitempty"`
```


#### `FloatCrumb.Reset`  <sub>(method)</sub>

```go
func (m *FloatCrumb) Reset()      { *m = FloatCrumb{} }
    func (*FloatCrumb) ProtoMessage() {}
    func (*FloatCrumb) Descriptor() ([]byte, []int)
```

#### `FloatCrumb.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *FloatCrumb) XXX_Unmarshal(b []byte) error
```

#### `FloatCrumb.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *FloatCrumb) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `FloatCrumb.XXX_Merge`  <sub>(method)</sub>

```go
func (m *FloatCrumb) XXX_Merge(src proto.Message)
```

#### `FloatCrumb.XXX_Size`  <sub>(method)</sub>

```go
func (m *FloatCrumb) XXX_Size() int
```

#### `FloatCrumb.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *FloatCrumb) XXX_DiscardUnknown()
```

#### `FloatCrumb.GetName`  <sub>(method)</sub>

```go
func (m *FloatCrumb) GetName() string
```

#### `FloatCrumb.GetFixed`  <sub>(method)</sub>

```go
func (m *FloatCrumb) GetFixed() bool
```

#### `FloatCrumb.GetVal`  <sub>(method)</sub>

```go
func (m *FloatCrumb) GetVal() float64
```

#### `RandomStringCrumb`  <sub>(struct)</sub>

> RandomStringCrumb will return a random string of specified length with the specified charset

```go
type RandomStringCrumb struct
```

**Fields:**
```go
Name    string `protobuf:"bytes,1,opt,name=Name,proto3" json:"Name,omitempty"`
Charset string `protobuf:"bytes,2,opt,name=Charset,proto3" json:"Charset,omitempty"`
Length  int    `protobuf:"varint,3,opt,name=Length,proto3,casttype=int" json:"Length,omitempty"`
```


#### `RandomStringCrumb.Reset`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) Reset()      { *m = RandomStringCrumb{} }
    func (*RandomStringCrumb) ProtoMessage() {}
    func (*RandomStringCrumb) Descriptor() ([]byte, []int)
```

#### `RandomStringCrumb.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) XXX_Unmarshal(b []byte) error
```

#### `RandomStringCrumb.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `RandomStringCrumb.XXX_Merge`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) XXX_Merge(src proto.Message)
```

#### `RandomStringCrumb.XXX_Size`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) XXX_Size() int
```

#### `RandomStringCrumb.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) XXX_DiscardUnknown()
```

#### `RandomStringCrumb.GetName`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) GetName() string
```

#### `RandomStringCrumb.GetCharset`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) GetCharset() string
```

#### `RandomStringCrumb.GetLength`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) GetLength() int
```

#### `RegexStringCrumb`  <sub>(struct)</sub>

> RegexStringCrumb will generate a string based off a provided regex

```go
type RegexStringCrumb struct
```

**Fields:**
```go
Name  string `protobuf:"bytes,1,opt,name=Name,proto3" json:"Name,omitempty"`
Regex string `protobuf:"bytes,2,opt,name=Regex,proto3" json:"Regex,omitempty"`
```


#### `RegexStringCrumb.Reset`  <sub>(method)</sub>

```go
func (m *RegexStringCrumb) Reset()      { *m = RegexStringCrumb{} }
    func (*RegexStringCrumb) ProtoMessage() {}
    func (*RegexStringCrumb) Descriptor() ([]byte, []int)
```

#### `RegexStringCrumb.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *RegexStringCrumb) XXX_Unmarshal(b []byte) error
```

#### `RegexStringCrumb.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *RegexStringCrumb) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `RegexStringCrumb.XXX_Merge`  <sub>(method)</sub>

```go
func (m *RegexStringCrumb) XXX_Merge(src proto.Message)
```

#### `RegexStringCrumb.XXX_Size`  <sub>(method)</sub>

```go
func (m *RegexStringCrumb) XXX_Size() int
```

#### `RegexStringCrumb.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *RegexStringCrumb) XXX_DiscardUnknown()
```

#### `RegexStringCrumb.GetName`  <sub>(method)</sub>

```go
func (m *RegexStringCrumb) GetName() string
```

#### `RegexStringCrumb.GetRegex`  <sub>(method)</sub>

```go
func (m *RegexStringCrumb) GetRegex() string
```

#### `BasicAuthCrumb`  <sub>(struct)</sub>

```go
type BasicAuthCrumb struct
```

**Fields:**
```go
Name     string `protobuf:"bytes,1,opt,name=Name,proto3" json:"Name,omitempty"`
User     string `protobuf:"bytes,2,opt,name=User,proto3" json:"User,omitempty"`
Password string `protobuf:"bytes,3,opt,name=Password,proto3" json:"Password,omitempty"`
Random   bool   `protobuf:"varint,4,opt,name=Random,proto3" json:"Random,omitempty"`
```


#### `BasicAuthCrumb.Reset`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) Reset()      { *m = BasicAuthCrumb{} }
    func (*BasicAuthCrumb) ProtoMessage() {}
    func (*BasicAuthCrumb) Descriptor() ([]byte, []int)
```

#### `BasicAuthCrumb.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) XXX_Unmarshal(b []byte) error
```

#### `BasicAuthCrumb.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `BasicAuthCrumb.XXX_Merge`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) XXX_Merge(src proto.Message)
```

#### `BasicAuthCrumb.XXX_Size`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) XXX_Size() int
```

#### `BasicAuthCrumb.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) XXX_DiscardUnknown()
```

#### `BasicAuthCrumb.GetName`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) GetName() string
```

#### `BasicAuthCrumb.GetUser`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) GetUser() string
```

#### `BasicAuthCrumb.GetPassword`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) GetPassword() string
```

#### `BasicAuthCrumb.GetRandom`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) GetRandom() bool
```

#### `ProtoArrayCrumb`  <sub>(struct)</sub>

```go
type ProtoArrayCrumb struct
```

**Fields:**
```go
Name    string      `protobuf:"bytes,1,opt,name=Name,proto3" json:"Name,omitempty"`
Element *ProtoCrumb `protobuf:"bytes,2,opt,name=Element,proto3" json:"Element,omitempty"`
```


#### `ProtoArrayCrumb.Reset`  <sub>(method)</sub>

```go
func (m *ProtoArrayCrumb) Reset()      { *m = ProtoArrayCrumb{} }
    func (*ProtoArrayCrumb) ProtoMessage() {}
    func (*ProtoArrayCrumb) Descriptor() ([]byte, []int)
```

#### `ProtoArrayCrumb.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoArrayCrumb) XXX_Unmarshal(b []byte) error
```

#### `ProtoArrayCrumb.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *ProtoArrayCrumb) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `ProtoArrayCrumb.XXX_Merge`  <sub>(method)</sub>

```go
func (m *ProtoArrayCrumb) XXX_Merge(src proto.Message)
```

#### `ProtoArrayCrumb.XXX_Size`  <sub>(method)</sub>

```go
func (m *ProtoArrayCrumb) XXX_Size() int
```

#### `ProtoArrayCrumb.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *ProtoArrayCrumb) XXX_DiscardUnknown()
```

#### `ProtoArrayCrumb.GetName`  <sub>(method)</sub>

```go
func (m *ProtoArrayCrumb) GetName() string
```

#### `ProtoArrayCrumb.GetElement`  <sub>(method)</sub>

```go
func (m *ProtoArrayCrumb) GetElement() *ProtoCrumb
```

#### `ProtoObjectCrumb`  <sub>(struct)</sub>

```go
type ProtoObjectCrumb struct
```

**Fields:**
```go
Name     string       `protobuf:"bytes,1,opt,name=Name,proto3" json:"Name,omitempty"`
Elements []ProtoCrumb `protobuf:"bytes,2,rep,name=Elements,proto3" json:"Elements"`
```


#### `ProtoObjectCrumb.Reset`  <sub>(method)</sub>

```go
func (m *ProtoObjectCrumb) Reset()      { *m = ProtoObjectCrumb{} }
    func (*ProtoObjectCrumb) ProtoMessage() {}
    func (*ProtoObjectCrumb) Descriptor() ([]byte, []int)
```

#### `ProtoObjectCrumb.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoObjectCrumb) XXX_Unmarshal(b []byte) error
```

#### `ProtoObjectCrumb.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *ProtoObjectCrumb) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `ProtoObjectCrumb.XXX_Merge`  <sub>(method)</sub>

```go
func (m *ProtoObjectCrumb) XXX_Merge(src proto.Message)
```

#### `ProtoObjectCrumb.XXX_Size`  <sub>(method)</sub>

```go
func (m *ProtoObjectCrumb) XXX_Size() int
```

#### `ProtoObjectCrumb.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *ProtoObjectCrumb) XXX_DiscardUnknown()
```

#### `ProtoObjectCrumb.GetName`  <sub>(method)</sub>

```go
func (m *ProtoObjectCrumb) GetName() string
```

#### `ProtoObjectCrumb.GetElements`  <sub>(method)</sub>

```go
func (m *ProtoObjectCrumb) GetElements() []ProtoCrumb
```

#### `ProtoStringCrumbCrumb`  <sub>(struct)</sub>

> StringCrumbCrumb is a crumb that will generate the value of its child, then wrap it in a string

```go
type ProtoStringCrumbCrumb struct
```

**Fields:**
```go
Name  string      `protobuf:"bytes,1,opt,name=Name,proto3" json:"Name,omitempty"`
Child *ProtoCrumb `protobuf:"bytes,2,opt,name=Child,proto3" json:"Child,omitempty"`
```


#### `ProtoStringCrumbCrumb.Reset`  <sub>(method)</sub>

```go
func (m *ProtoStringCrumbCrumb) Reset()      { *m = ProtoStringCrumbCrumb{} }
    func (*ProtoStringCrumbCrumb) ProtoMessage() {}
    func (*ProtoStringCrumbCrumb) Descriptor() ([]byte, []int)
```

#### `ProtoStringCrumbCrumb.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoStringCrumbCrumb) XXX_Unmarshal(b []byte) error
```

#### `ProtoStringCrumbCrumb.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *ProtoStringCrumbCrumb) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `ProtoStringCrumbCrumb.XXX_Merge`  <sub>(method)</sub>

```go
func (m *ProtoStringCrumbCrumb) XXX_Merge(src proto.Message)
```

#### `ProtoStringCrumbCrumb.XXX_Size`  <sub>(method)</sub>

```go
func (m *ProtoStringCrumbCrumb) XXX_Size() int
```

#### `ProtoStringCrumbCrumb.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *ProtoStringCrumbCrumb) XXX_DiscardUnknown()
```

#### `ProtoStringCrumbCrumb.GetName`  <sub>(method)</sub>

```go
func (m *ProtoStringCrumbCrumb) GetName() string
```

#### `ProtoStringCrumbCrumb.GetChild`  <sub>(method)</sub>

```go
func (m *ProtoStringCrumbCrumb) GetChild() *ProtoCrumb
```

#### `ProtoCrumb`  <sub>(struct)</sub>

```go
type ProtoCrumb struct
```

**Fields:**
```go
Crumb isProtoCrumb_Crumb `protobuf_oneof:"crumb"`    // *ProtoCrumb_StringCrumbCrumb
```


#### `ProtoCrumb.Reset`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) Reset()      { *m = ProtoCrumb{} }
    func (*ProtoCrumb) ProtoMessage() {}
    func (*ProtoCrumb) Descriptor() ([]byte, []int)
```

#### `ProtoCrumb.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) XXX_Unmarshal(b []byte) error
```

#### `ProtoCrumb.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `ProtoCrumb.XXX_Merge`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) XXX_Merge(src proto.Message)
```

#### `ProtoCrumb.XXX_Size`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) XXX_Size() int
```

#### `ProtoCrumb.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) XXX_DiscardUnknown()
```

#### `isProtoCrumb_Crumb`  <sub>(interface)</sub>

```go
type isProtoCrumb_Crumb interface
```

**Members/Variants:**
```go
isProtoCrumb_Crumb()
Equal(interface{}) bool
MarshalTo([]byte) (int, error)
Size() int
```


#### `ProtoCrumb_UuidCrumb`  <sub>(struct)</sub>

```go
type ProtoCrumb_UuidCrumb struct
```

**Fields:**
```go
UuidCrumb *UUIDCrumb `protobuf:"bytes,1,opt,name=uuidCrumb,proto3,oneof" json:"uuidCrumb,omitempty"`
```


#### `ProtoCrumb_StaticCrumb`  <sub>(struct)</sub>

```go
type ProtoCrumb_StaticCrumb struct
```

**Fields:**
```go
StaticCrumb *StaticCrumb `protobuf:"bytes,2,opt,name=staticCrumb,proto3,oneof" json:"staticCrumb,omitempty"`
```


#### `ProtoCrumb_IntCrumb`  <sub>(struct)</sub>

```go
type ProtoCrumb_IntCrumb struct
```

**Fields:**
```go
IntCrumb *IntCrumb `protobuf:"bytes,3,opt,name=intCrumb,proto3,oneof" json:"intCrumb,omitempty"`
```


#### `ProtoCrumb_BoolCrumb`  <sub>(struct)</sub>

```go
type ProtoCrumb_BoolCrumb struct
```

**Fields:**
```go
BoolCrumb *BoolCrumb `protobuf:"bytes,4,opt,name=boolCrumb,proto3,oneof" json:"boolCrumb,omitempty"`
```


#### `ProtoCrumb_FloatCrumb`  <sub>(struct)</sub>

```go
type ProtoCrumb_FloatCrumb struct
```

**Fields:**
```go
FloatCrumb *FloatCrumb `protobuf:"bytes,5,opt,name=floatCrumb,proto3,oneof" json:"floatCrumb,omitempty"`
```


#### `ProtoCrumb_RandomStringCrumb`  <sub>(struct)</sub>

```go
type ProtoCrumb_RandomStringCrumb struct
```

**Fields:**
```go
RandomStringCrumb *RandomStringCrumb `protobuf:"bytes,6,opt,name=randomStringCrumb,proto3,oneof" json:"randomStringCrumb,omitempty"`
```


#### `ProtoCrumb_RegexStringCrumb`  <sub>(struct)</sub>

```go
type ProtoCrumb_RegexStringCrumb struct
```

**Fields:**
```go
RegexStringCrumb *RegexStringCrumb `protobuf:"bytes,7,opt,name=regexStringCrumb,proto3,oneof" json:"regexStringCrumb,omitempty"`
```


#### `ProtoCrumb_BasicAuthCrumb`  <sub>(struct)</sub>

```go
type ProtoCrumb_BasicAuthCrumb struct
```

**Fields:**
```go
BasicAuthCrumb *BasicAuthCrumb `protobuf:"bytes,8,opt,name=basicAuthCrumb,proto3,oneof" json:"basicAuthCrumb,omitempty"`
```


#### `ProtoCrumb_ArrayCrumb`  <sub>(struct)</sub>

```go
type ProtoCrumb_ArrayCrumb struct
```

**Fields:**
```go
ArrayCrumb *ProtoArrayCrumb `protobuf:"bytes,9,opt,name=arrayCrumb,proto3,oneof" json:"arrayCrumb,omitempty"`
```


#### `ProtoCrumb_ObjectCrumb`  <sub>(struct)</sub>

```go
type ProtoCrumb_ObjectCrumb struct
```

**Fields:**
```go
ObjectCrumb *ProtoObjectCrumb `protobuf:"bytes,10,opt,name=objectCrumb,proto3,oneof" json:"objectCrumb,omitempty"`
```


#### `ProtoCrumb_StringCrumbCrumb`  <sub>(struct)</sub>

```go
type ProtoCrumb_StringCrumbCrumb struct
```

**Fields:**
```go
StringCrumbCrumb *ProtoStringCrumbCrumb `protobuf:"bytes,11,opt,name=stringCrumbCrumb,proto3,oneof" json:"stringCrumbCrumb,omitempty"`
```


#### `ProtoCrumb_UuidCrumb.isProtoCrumb_Crumb`  <sub>(method)</sub>

```go
func (*ProtoCrumb_UuidCrumb) isProtoCrumb_Crumb()         {}
    func (*ProtoCrumb_StaticCrumb) isProtoCrumb_Crumb()       {}
    func (*ProtoCrumb_IntCrumb) isProtoCrumb_Crumb()          {}
    func (*ProtoCrumb_BoolCrumb) isProtoCrumb_Crumb()         {}
    func (*ProtoCrumb_FloatCrumb) isProtoCrumb_Crumb()        {}
    func (*ProtoCrumb_RandomStringCrumb) isProtoCrumb_Crumb() {}
    func (*ProtoCrumb_RegexStringCrumb) isProtoCrumb_Crumb()  {}
    func (*ProtoCrumb_BasicAuthCrumb) isProtoCrumb_Crumb()    {}
    func (*ProtoCrumb_ArrayCrumb) isProtoCrumb_Crumb()        {}
    func (*ProtoCrumb_ObjectCrumb) isProtoCrumb_Crumb()       {}
    func (*ProtoCrumb_StringCrumbCrumb) isProtoCrumb_Crumb()  {}
    
    func (m *ProtoCrumb) GetCrumb() isProtoCrumb_Crumb
```

#### `ProtoCrumb.GetUuidCrumb`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) GetUuidCrumb() *UUIDCrumb
```

#### `ProtoCrumb.GetStaticCrumb`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) GetStaticCrumb() *StaticCrumb
```

#### `ProtoCrumb.GetIntCrumb`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) GetIntCrumb() *IntCrumb
```

#### `ProtoCrumb.GetBoolCrumb`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) GetBoolCrumb() *BoolCrumb
```

#### `ProtoCrumb.GetFloatCrumb`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) GetFloatCrumb() *FloatCrumb
```

#### `ProtoCrumb.GetRandomStringCrumb`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) GetRandomStringCrumb() *RandomStringCrumb
```

#### `ProtoCrumb.GetRegexStringCrumb`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) GetRegexStringCrumb() *RegexStringCrumb
```

#### `ProtoCrumb.GetBasicAuthCrumb`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) GetBasicAuthCrumb() *BasicAuthCrumb
```

#### `ProtoCrumb.GetArrayCrumb`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) GetArrayCrumb() *ProtoArrayCrumb
```

#### `ProtoCrumb.GetObjectCrumb`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) GetObjectCrumb() *ProtoObjectCrumb
```

#### `ProtoCrumb.GetStringCrumbCrumb`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) GetStringCrumbCrumb() *ProtoStringCrumbCrumb
```

#### `ProtoCrumb.XXX_OneofWrappers`  <sub>(method)</sub>

> XXX_OneofWrappers is for the internal use of the proto package.

```go
func (*ProtoCrumb) XXX_OneofWrappers() []interface{}
```

#### `ProtoRoute`  <sub>(struct)</sub>

```go
type ProtoRoute struct
```

**Fields:**
```go
TemplatePath string        `protobuf:"bytes,1,opt,name=TemplatePath,proto3" json:"TemplatePath,omitempty"`
Method       string        `protobuf:"bytes,2,opt,name=Method,proto3" json:"Method,omitempty"`
PathCrumbs   []ProtoCrumb  `protobuf:"bytes,3,rep,name=PathCrumbs,proto3" json:"PathCrumbs"`
HeaderCrumbs []ProtoCrumb  `protobuf:"bytes,4,rep,name=HeaderCrumbs,proto3" json:"HeaderCrumbs"`
QueryCrumbs  []ProtoCrumb  `protobuf:"bytes,5,rep,name=QueryCrumbs,proto3" json:"QueryCrumbs"`
BodyCrumbs   []ProtoCrumb  `protobuf:"bytes,6,rep,name=BodyCrumbs,proto3" json:"BodyCrumbs"`
ContentType  []ContentType `protobuf:"bytes,7,rep,name=ContentType,proto3,casttype=ContentType" json:"ContentType,omitempty"`
```


#### `ProtoRoute.Reset`  <sub>(method)</sub>

```go
func (m *ProtoRoute) Reset()      { *m = ProtoRoute{} }
    func (*ProtoRoute) ProtoMessage() {}
    func (*ProtoRoute) Descriptor() ([]byte, []int)
```

#### `ProtoRoute.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoRoute) XXX_Unmarshal(b []byte) error
```

#### `ProtoRoute.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *ProtoRoute) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `ProtoRoute.XXX_Merge`  <sub>(method)</sub>

```go
func (m *ProtoRoute) XXX_Merge(src proto.Message)
```

#### `ProtoRoute.XXX_Size`  <sub>(method)</sub>

```go
func (m *ProtoRoute) XXX_Size() int
```

#### `ProtoRoute.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *ProtoRoute) XXX_DiscardUnknown()
```

#### `ProtoRoute.GetTemplatePath`  <sub>(method)</sub>

```go
func (m *ProtoRoute) GetTemplatePath() string
```

#### `ProtoRoute.GetMethod`  <sub>(method)</sub>

```go
func (m *ProtoRoute) GetMethod() string
```

#### `ProtoRoute.GetPathCrumbs`  <sub>(method)</sub>

```go
func (m *ProtoRoute) GetPathCrumbs() []ProtoCrumb
```

#### `ProtoRoute.GetHeaderCrumbs`  <sub>(method)</sub>

```go
func (m *ProtoRoute) GetHeaderCrumbs() []ProtoCrumb
```

#### `ProtoRoute.GetQueryCrumbs`  <sub>(method)</sub>

```go
func (m *ProtoRoute) GetQueryCrumbs() []ProtoCrumb
```

#### `ProtoRoute.GetBodyCrumbs`  <sub>(method)</sub>

```go
func (m *ProtoRoute) GetBodyCrumbs() []ProtoCrumb
```

#### `ProtoRoute.GetContentType`  <sub>(method)</sub>

```go
func (m *ProtoRoute) GetContentType() []ContentType
```

#### `ProtoAPI`  <sub>(struct)</sub>

```go
type ProtoAPI struct
```

**Fields:**
```go
URL          string       `protobuf:"bytes,1,opt,name=URL,proto3" json:"URL,omitempty"`
ID           string       `protobuf:"bytes,2,opt,name=ID,proto3" json:"ID,omitempty"`
Routes       []ProtoRoute `protobuf:"bytes,3,rep,name=Routes,proto3" json:"Routes"`
HeaderCrumbs []ProtoCrumb `protobuf:"bytes,4,rep,name=HeaderCrumbs,proto3" json:"HeaderCrumbs"`
QueryCrumbs  []ProtoCrumb `protobuf:"bytes,5,rep,name=QueryCrumbs,proto3" json:"QueryCrumbs"`
BodyCrumbs   []ProtoCrumb `protobuf:"bytes,6,rep,name=BodyCrumbs,proto3" json:"BodyCrumbs"`
CookieCrumbs []ProtoCrumb `protobuf:"bytes,7,rep,name=CookieCrumbs,proto3" json:"CookieCrumbs"`
```


#### `ProtoAPI.Reset`  <sub>(method)</sub>

```go
func (m *ProtoAPI) Reset()      { *m = ProtoAPI{} }
    func (*ProtoAPI) ProtoMessage() {}
    func (*ProtoAPI) Descriptor() ([]byte, []int)
```

#### `ProtoAPI.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoAPI) XXX_Unmarshal(b []byte) error
```

#### `ProtoAPI.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *ProtoAPI) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `ProtoAPI.XXX_Merge`  <sub>(method)</sub>

```go
func (m *ProtoAPI) XXX_Merge(src proto.Message)
```

#### `ProtoAPI.XXX_Size`  <sub>(method)</sub>

```go
func (m *ProtoAPI) XXX_Size() int
```

#### `ProtoAPI.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *ProtoAPI) XXX_DiscardUnknown()
```

#### `ProtoAPI.GetURL`  <sub>(method)</sub>

```go
func (m *ProtoAPI) GetURL() string
```

#### `ProtoAPI.GetID`  <sub>(method)</sub>

```go
func (m *ProtoAPI) GetID() string
```

#### `ProtoAPI.GetRoutes`  <sub>(method)</sub>

```go
func (m *ProtoAPI) GetRoutes() []ProtoRoute
```

#### `ProtoAPI.GetHeaderCrumbs`  <sub>(method)</sub>

```go
func (m *ProtoAPI) GetHeaderCrumbs() []ProtoCrumb
```

#### `ProtoAPI.GetQueryCrumbs`  <sub>(method)</sub>

```go
func (m *ProtoAPI) GetQueryCrumbs() []ProtoCrumb
```

#### `ProtoAPI.GetBodyCrumbs`  <sub>(method)</sub>

```go
func (m *ProtoAPI) GetBodyCrumbs() []ProtoCrumb
```

#### `ProtoAPI.GetCookieCrumbs`  <sub>(method)</sub>

```go
func (m *ProtoAPI) GetCookieCrumbs() []ProtoCrumb
```

#### `ProtoAPIS`  <sub>(struct)</sub>

```go
type ProtoAPIS struct
```

**Fields:**
```go
APIs []ProtoAPI `protobuf:"bytes,1,rep,name=APIs,proto3" json:"APIs"`
```


#### `ProtoAPIS.Reset`  <sub>(method)</sub>

```go
func (m *ProtoAPIS) Reset()      { *m = ProtoAPIS{} }
    func (*ProtoAPIS) ProtoMessage() {}
    func (*ProtoAPIS) Descriptor() ([]byte, []int)
```

#### `ProtoAPIS.XXX_Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoAPIS) XXX_Unmarshal(b []byte) error
```

#### `ProtoAPIS.XXX_Marshal`  <sub>(method)</sub>

```go
func (m *ProtoAPIS) XXX_Marshal(b []byte, deterministic bool) ([]byte, error)
```

#### `ProtoAPIS.XXX_Merge`  <sub>(method)</sub>

```go
func (m *ProtoAPIS) XXX_Merge(src proto.Message)
```

#### `ProtoAPIS.XXX_Size`  <sub>(method)</sub>

```go
func (m *ProtoAPIS) XXX_Size() int
```

#### `ProtoAPIS.XXX_DiscardUnknown`  <sub>(method)</sub>

```go
func (m *ProtoAPIS) XXX_DiscardUnknown()
```

#### `ProtoAPIS.GetAPIs`  <sub>(method)</sub>

```go
func (m *ProtoAPIS) GetAPIs() []ProtoAPI
```

#### `init`  <sub>(func)</sub>

```go
func init()
```

#### `init`  <sub>(func)</sub>

```go
func init() { proto.RegisterFile("proute.proto", fileDescriptor_ea0b90b390d3b351) }
    
    var fileDescriptor_ea0b90b390d3b351 = []byte
```

#### `UUIDCrumb.Equal`  <sub>(method)</sub>

```go
func (this *UUIDCrumb) Equal(that interface{}) bool
```

#### `StaticCrumb.Equal`  <sub>(method)</sub>

```go
func (this *StaticCrumb) Equal(that interface{}) bool
```

#### `IntCrumb.Equal`  <sub>(method)</sub>

```go
func (this *IntCrumb) Equal(that interface{}) bool
```

#### `BoolCrumb.Equal`  <sub>(method)</sub>

```go
func (this *BoolCrumb) Equal(that interface{}) bool
```

#### `FloatCrumb.Equal`  <sub>(method)</sub>

```go
func (this *FloatCrumb) Equal(that interface{}) bool
```

#### `RandomStringCrumb.Equal`  <sub>(method)</sub>

```go
func (this *RandomStringCrumb) Equal(that interface{}) bool
```

#### `RegexStringCrumb.Equal`  <sub>(method)</sub>

```go
func (this *RegexStringCrumb) Equal(that interface{}) bool
```

#### `BasicAuthCrumb.Equal`  <sub>(method)</sub>

```go
func (this *BasicAuthCrumb) Equal(that interface{}) bool
```

#### `ProtoArrayCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoArrayCrumb) Equal(that interface{}) bool
```

#### `ProtoObjectCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoObjectCrumb) Equal(that interface{}) bool
```

#### `ProtoStringCrumbCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoStringCrumbCrumb) Equal(that interface{}) bool
```

#### `ProtoCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoCrumb) Equal(that interface{}) bool
```

#### `ProtoCrumb_UuidCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_UuidCrumb) Equal(that interface{}) bool
```

#### `ProtoCrumb_StaticCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_StaticCrumb) Equal(that interface{}) bool
```

#### `ProtoCrumb_IntCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_IntCrumb) Equal(that interface{}) bool
```

#### `ProtoCrumb_BoolCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_BoolCrumb) Equal(that interface{}) bool
```

#### `ProtoCrumb_FloatCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_FloatCrumb) Equal(that interface{}) bool
```

#### `ProtoCrumb_RandomStringCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_RandomStringCrumb) Equal(that interface{}) bool
```

#### `ProtoCrumb_RegexStringCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_RegexStringCrumb) Equal(that interface{}) bool
```

#### `ProtoCrumb_BasicAuthCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_BasicAuthCrumb) Equal(that interface{}) bool
```

#### `ProtoCrumb_ArrayCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_ArrayCrumb) Equal(that interface{}) bool
```

#### `ProtoCrumb_ObjectCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_ObjectCrumb) Equal(that interface{}) bool
```

#### `ProtoCrumb_StringCrumbCrumb.Equal`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_StringCrumbCrumb) Equal(that interface{}) bool
```

#### `ProtoRoute.Equal`  <sub>(method)</sub>

```go
func (this *ProtoRoute) Equal(that interface{}) bool
```

#### `ProtoAPI.Equal`  <sub>(method)</sub>

```go
func (this *ProtoAPI) Equal(that interface{}) bool
```

#### `ProtoAPIS.Equal`  <sub>(method)</sub>

```go
func (this *ProtoAPIS) Equal(that interface{}) bool
```

#### `UUIDCrumb.GoString`  <sub>(method)</sub>

```go
func (this *UUIDCrumb) GoString() string
```

#### `StaticCrumb.GoString`  <sub>(method)</sub>

```go
func (this *StaticCrumb) GoString() string
```

#### `IntCrumb.GoString`  <sub>(method)</sub>

```go
func (this *IntCrumb) GoString() string
```

#### `BoolCrumb.GoString`  <sub>(method)</sub>

```go
func (this *BoolCrumb) GoString() string
```

#### `FloatCrumb.GoString`  <sub>(method)</sub>

```go
func (this *FloatCrumb) GoString() string
```

#### `RandomStringCrumb.GoString`  <sub>(method)</sub>

```go
func (this *RandomStringCrumb) GoString() string
```

#### `RegexStringCrumb.GoString`  <sub>(method)</sub>

```go
func (this *RegexStringCrumb) GoString() string
```

#### `BasicAuthCrumb.GoString`  <sub>(method)</sub>

```go
func (this *BasicAuthCrumb) GoString() string
```

#### `ProtoArrayCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoArrayCrumb) GoString() string
```

#### `ProtoObjectCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoObjectCrumb) GoString() string
```

#### `ProtoStringCrumbCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoStringCrumbCrumb) GoString() string
```

#### `ProtoCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoCrumb) GoString() string
```

#### `ProtoCrumb_UuidCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_UuidCrumb) GoString() string
```

#### `ProtoCrumb_StaticCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_StaticCrumb) GoString() string
```

#### `ProtoCrumb_IntCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_IntCrumb) GoString() string
```

#### `ProtoCrumb_BoolCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_BoolCrumb) GoString() string
```

#### `ProtoCrumb_FloatCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_FloatCrumb) GoString() string
```

#### `ProtoCrumb_RandomStringCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_RandomStringCrumb) GoString() string
```

#### `ProtoCrumb_RegexStringCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_RegexStringCrumb) GoString() string
```

#### `ProtoCrumb_BasicAuthCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_BasicAuthCrumb) GoString() string
```

#### `ProtoCrumb_ArrayCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_ArrayCrumb) GoString() string
```

#### `ProtoCrumb_ObjectCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_ObjectCrumb) GoString() string
```

#### `ProtoCrumb_StringCrumbCrumb.GoString`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_StringCrumbCrumb) GoString() string
```

#### `ProtoRoute.GoString`  <sub>(method)</sub>

```go
func (this *ProtoRoute) GoString() string
```

#### `ProtoAPI.GoString`  <sub>(method)</sub>

```go
func (this *ProtoAPI) GoString() string
```

#### `ProtoAPIS.GoString`  <sub>(method)</sub>

```go
func (this *ProtoAPIS) GoString() string
```

#### `valueToGoStringProute`  <sub>(func)</sub>

```go
func valueToGoStringProute(v interface{}, typ string) string
```

#### `UUIDCrumb.Marshal`  <sub>(method)</sub>

```go
func (m *UUIDCrumb) Marshal() (dAtA []byte, err error)
```

#### `UUIDCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *UUIDCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `UUIDCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *UUIDCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `StaticCrumb.Marshal`  <sub>(method)</sub>

```go
func (m *StaticCrumb) Marshal() (dAtA []byte, err error)
```

#### `StaticCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *StaticCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `StaticCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *StaticCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `IntCrumb.Marshal`  <sub>(method)</sub>

```go
func (m *IntCrumb) Marshal() (dAtA []byte, err error)
```

#### `IntCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *IntCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `IntCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *IntCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `BoolCrumb.Marshal`  <sub>(method)</sub>

```go
func (m *BoolCrumb) Marshal() (dAtA []byte, err error)
```

#### `BoolCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *BoolCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `BoolCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *BoolCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `FloatCrumb.Marshal`  <sub>(method)</sub>

```go
func (m *FloatCrumb) Marshal() (dAtA []byte, err error)
```

#### `FloatCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *FloatCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `FloatCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *FloatCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `RandomStringCrumb.Marshal`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) Marshal() (dAtA []byte, err error)
```

#### `RandomStringCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `RandomStringCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `RegexStringCrumb.Marshal`  <sub>(method)</sub>

```go
func (m *RegexStringCrumb) Marshal() (dAtA []byte, err error)
```

#### `RegexStringCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *RegexStringCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `RegexStringCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *RegexStringCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `BasicAuthCrumb.Marshal`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) Marshal() (dAtA []byte, err error)
```

#### `BasicAuthCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `BasicAuthCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoArrayCrumb.Marshal`  <sub>(method)</sub>

```go
func (m *ProtoArrayCrumb) Marshal() (dAtA []byte, err error)
```

#### `ProtoArrayCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoArrayCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoArrayCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoArrayCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoObjectCrumb.Marshal`  <sub>(method)</sub>

```go
func (m *ProtoObjectCrumb) Marshal() (dAtA []byte, err error)
```

#### `ProtoObjectCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoObjectCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoObjectCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoObjectCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoStringCrumbCrumb.Marshal`  <sub>(method)</sub>

```go
func (m *ProtoStringCrumbCrumb) Marshal() (dAtA []byte, err error)
```

#### `ProtoStringCrumbCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoStringCrumbCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoStringCrumbCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoStringCrumbCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoCrumb.Marshal`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) Marshal() (dAtA []byte, err error)
```

#### `ProtoCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoCrumb_UuidCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_UuidCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoCrumb_UuidCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_UuidCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoCrumb_StaticCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_StaticCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoCrumb_StaticCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_StaticCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoCrumb_IntCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_IntCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoCrumb_IntCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_IntCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoCrumb_BoolCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_BoolCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoCrumb_BoolCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_BoolCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoCrumb_FloatCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_FloatCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoCrumb_FloatCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_FloatCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoCrumb_RandomStringCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_RandomStringCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoCrumb_RandomStringCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_RandomStringCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoCrumb_RegexStringCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_RegexStringCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoCrumb_RegexStringCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_RegexStringCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoCrumb_BasicAuthCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_BasicAuthCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoCrumb_BasicAuthCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_BasicAuthCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoCrumb_ArrayCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_ArrayCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoCrumb_ArrayCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_ArrayCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoCrumb_ObjectCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_ObjectCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoCrumb_ObjectCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_ObjectCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoCrumb_StringCrumbCrumb.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_StringCrumbCrumb) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoCrumb_StringCrumbCrumb.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_StringCrumbCrumb) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoRoute.Marshal`  <sub>(method)</sub>

```go
func (m *ProtoRoute) Marshal() (dAtA []byte, err error)
```

#### `ProtoRoute.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoRoute) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoRoute.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoRoute) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoAPI.Marshal`  <sub>(method)</sub>

```go
func (m *ProtoAPI) Marshal() (dAtA []byte, err error)
```

#### `ProtoAPI.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoAPI) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoAPI.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoAPI) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `ProtoAPIS.Marshal`  <sub>(method)</sub>

```go
func (m *ProtoAPIS) Marshal() (dAtA []byte, err error)
```

#### `ProtoAPIS.MarshalTo`  <sub>(method)</sub>

```go
func (m *ProtoAPIS) MarshalTo(dAtA []byte) (int, error)
```

#### `ProtoAPIS.MarshalToSizedBuffer`  <sub>(method)</sub>

```go
func (m *ProtoAPIS) MarshalToSizedBuffer(dAtA []byte) (int, error)
```

#### `encodeVarintProute`  <sub>(func)</sub>

```go
func encodeVarintProute(dAtA []byte, offset int, v uint64) int
```

#### `UUIDCrumb.Size`  <sub>(method)</sub>

```go
func (m *UUIDCrumb) Size() (n int)
```

#### `StaticCrumb.Size`  <sub>(method)</sub>

```go
func (m *StaticCrumb) Size() (n int)
```

#### `IntCrumb.Size`  <sub>(method)</sub>

```go
func (m *IntCrumb) Size() (n int)
```

#### `BoolCrumb.Size`  <sub>(method)</sub>

```go
func (m *BoolCrumb) Size() (n int)
```

#### `FloatCrumb.Size`  <sub>(method)</sub>

```go
func (m *FloatCrumb) Size() (n int)
```

#### `RandomStringCrumb.Size`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) Size() (n int)
```

#### `RegexStringCrumb.Size`  <sub>(method)</sub>

```go
func (m *RegexStringCrumb) Size() (n int)
```

#### `BasicAuthCrumb.Size`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) Size() (n int)
```

#### `ProtoArrayCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoArrayCrumb) Size() (n int)
```

#### `ProtoObjectCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoObjectCrumb) Size() (n int)
```

#### `ProtoStringCrumbCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoStringCrumbCrumb) Size() (n int)
```

#### `ProtoCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) Size() (n int)
```

#### `ProtoCrumb_UuidCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_UuidCrumb) Size() (n int)
```

#### `ProtoCrumb_StaticCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_StaticCrumb) Size() (n int)
```

#### `ProtoCrumb_IntCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_IntCrumb) Size() (n int)
```

#### `ProtoCrumb_BoolCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_BoolCrumb) Size() (n int)
```

#### `ProtoCrumb_FloatCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_FloatCrumb) Size() (n int)
```

#### `ProtoCrumb_RandomStringCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_RandomStringCrumb) Size() (n int)
```

#### `ProtoCrumb_RegexStringCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_RegexStringCrumb) Size() (n int)
```

#### `ProtoCrumb_BasicAuthCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_BasicAuthCrumb) Size() (n int)
```

#### `ProtoCrumb_ArrayCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_ArrayCrumb) Size() (n int)
```

#### `ProtoCrumb_ObjectCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_ObjectCrumb) Size() (n int)
```

#### `ProtoCrumb_StringCrumbCrumb.Size`  <sub>(method)</sub>

```go
func (m *ProtoCrumb_StringCrumbCrumb) Size() (n int)
```

#### `ProtoRoute.Size`  <sub>(method)</sub>

```go
func (m *ProtoRoute) Size() (n int)
```

#### `ProtoAPI.Size`  <sub>(method)</sub>

```go
func (m *ProtoAPI) Size() (n int)
```

#### `ProtoAPIS.Size`  <sub>(method)</sub>

```go
func (m *ProtoAPIS) Size() (n int)
```

#### `sovProute`  <sub>(func)</sub>

```go
func sovProute(x uint64) (n int)
```

#### `sozProute`  <sub>(func)</sub>

```go
func sozProute(x uint64) (n int)
```

#### `UUIDCrumb.String`  <sub>(method)</sub>

```go
func (this *UUIDCrumb) String() string
```

#### `StaticCrumb.String`  <sub>(method)</sub>

```go
func (this *StaticCrumb) String() string
```

#### `IntCrumb.String`  <sub>(method)</sub>

```go
func (this *IntCrumb) String() string
```

#### `BoolCrumb.String`  <sub>(method)</sub>

```go
func (this *BoolCrumb) String() string
```

#### `FloatCrumb.String`  <sub>(method)</sub>

```go
func (this *FloatCrumb) String() string
```

#### `RandomStringCrumb.String`  <sub>(method)</sub>

```go
func (this *RandomStringCrumb) String() string
```

#### `RegexStringCrumb.String`  <sub>(method)</sub>

```go
func (this *RegexStringCrumb) String() string
```

#### `BasicAuthCrumb.String`  <sub>(method)</sub>

```go
func (this *BasicAuthCrumb) String() string
```

#### `ProtoArrayCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoArrayCrumb) String() string
```

#### `ProtoObjectCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoObjectCrumb) String() string
```

#### `ProtoStringCrumbCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoStringCrumbCrumb) String() string
```

#### `ProtoCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoCrumb) String() string
```

#### `ProtoCrumb_UuidCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_UuidCrumb) String() string
```

#### `ProtoCrumb_StaticCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_StaticCrumb) String() string
```

#### `ProtoCrumb_IntCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_IntCrumb) String() string
```

#### `ProtoCrumb_BoolCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_BoolCrumb) String() string
```

#### `ProtoCrumb_FloatCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_FloatCrumb) String() string
```

#### `ProtoCrumb_RandomStringCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_RandomStringCrumb) String() string
```

#### `ProtoCrumb_RegexStringCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_RegexStringCrumb) String() string
```

#### `ProtoCrumb_BasicAuthCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_BasicAuthCrumb) String() string
```

#### `ProtoCrumb_ArrayCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_ArrayCrumb) String() string
```

#### `ProtoCrumb_ObjectCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_ObjectCrumb) String() string
```

#### `ProtoCrumb_StringCrumbCrumb.String`  <sub>(method)</sub>

```go
func (this *ProtoCrumb_StringCrumbCrumb) String() string
```

#### `ProtoRoute.String`  <sub>(method)</sub>

```go
func (this *ProtoRoute) String() string
```

#### `ProtoAPI.String`  <sub>(method)</sub>

```go
func (this *ProtoAPI) String() string
```

#### `ProtoAPIS.String`  <sub>(method)</sub>

```go
func (this *ProtoAPIS) String() string
```

#### `valueToStringProute`  <sub>(func)</sub>

```go
func valueToStringProute(v interface{}) string
```

#### `UUIDCrumb.Unmarshal`  <sub>(method)</sub>

```go
func (m *UUIDCrumb) Unmarshal(dAtA []byte) error
```

#### `StaticCrumb.Unmarshal`  <sub>(method)</sub>

```go
func (m *StaticCrumb) Unmarshal(dAtA []byte) error
```

#### `IntCrumb.Unmarshal`  <sub>(method)</sub>

```go
func (m *IntCrumb) Unmarshal(dAtA []byte) error
```

#### `BoolCrumb.Unmarshal`  <sub>(method)</sub>

```go
func (m *BoolCrumb) Unmarshal(dAtA []byte) error
```

#### `FloatCrumb.Unmarshal`  <sub>(method)</sub>

```go
func (m *FloatCrumb) Unmarshal(dAtA []byte) error
```

#### `RandomStringCrumb.Unmarshal`  <sub>(method)</sub>

```go
func (m *RandomStringCrumb) Unmarshal(dAtA []byte) error
```

#### `RegexStringCrumb.Unmarshal`  <sub>(method)</sub>

```go
func (m *RegexStringCrumb) Unmarshal(dAtA []byte) error
```

#### `BasicAuthCrumb.Unmarshal`  <sub>(method)</sub>

```go
func (m *BasicAuthCrumb) Unmarshal(dAtA []byte) error
```

#### `ProtoArrayCrumb.Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoArrayCrumb) Unmarshal(dAtA []byte) error
```

#### `ProtoObjectCrumb.Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoObjectCrumb) Unmarshal(dAtA []byte) error
```

#### `ProtoStringCrumbCrumb.Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoStringCrumbCrumb) Unmarshal(dAtA []byte) error
```

#### `ProtoCrumb.Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) Unmarshal(dAtA []byte) error
```

#### `ProtoRoute.Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoRoute) Unmarshal(dAtA []byte) error
```

#### `ProtoAPI.Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoAPI) Unmarshal(dAtA []byte) error
```

#### `ProtoAPIS.Unmarshal`  <sub>(method)</sub>

```go
func (m *ProtoAPIS) Unmarshal(dAtA []byte) error
```

#### `skipProute`  <sub>(func)</sub>

```go
func skipProute(dAtA []byte) (n int, err error)
```

### `kiterunner/pkg/proute/proute.pbext.go`


#### `ProtoCrumb_UuidCrumb.UnwrapCrumb`  <sub>(method)</sub>

```go
func (p *ProtoCrumb_UuidCrumb) UnwrapCrumb() Crumb         { return *p.UuidCrumb }
    func (p *ProtoCrumb_StaticCrumb) UnwrapCrumb() Crumb       { return *p.StaticCrumb }
    func (p *ProtoCrumb_IntCrumb) UnwrapCrumb() Crumb          { return *p.IntCrumb }
    func (p *ProtoCrumb_BoolCrumb) UnwrapCrumb() Crumb         { return *p.BoolCrumb }
    func (p *ProtoCrumb_FloatCrumb) UnwrapCrumb() Crumb        { return *p.FloatCrumb }
    func (p *ProtoCrumb_RandomStringCrumb) UnwrapCrumb() Crumb { return *p.RandomStringCrumb }
    func (p *ProtoCrumb_RegexStringCrumb) UnwrapCrumb() Crumb  { return *p.RegexStringCrumb }
    func (p *ProtoCrumb_BasicAuthCrumb) UnwrapCrumb() Crumb    { return *p.BasicAuthCrumb }
    func (p *ProtoCrumb_ArrayCrumb) UnwrapCrumb() Crumb        { return *p.ProuteCrumb()}
    func (p *ProtoCrumb_ObjectCrumb) UnwrapCrumb() Crumb       { return *p.ProuteCrumb() }
    func (p *ProtoCrumb_StringCrumbCrumb) UnwrapCrumb() Crumb  { return *p.ProuteCrumb() }
    
    func (p *ProtoCrumb_ArrayCrumb) ProuteCrumb() *ArrayCrumb
```

#### `ProtoCrumb_ObjectCrumb.ProuteCrumb`  <sub>(method)</sub>

```go
func (p *ProtoCrumb_ObjectCrumb) ProuteCrumb() *ObjectCrumb
```

#### `ProtoCrumb_StringCrumbCrumb.ProuteCrumb`  <sub>(method)</sub>

```go
func (p *ProtoCrumb_StringCrumbCrumb) ProuteCrumb() *StringCrumbCrumb
```

#### `UnwrapCrumber`  <sub>(interface)</sub>

```go
type UnwrapCrumber interface
```

**Members/Variants:**
```go
UnwrapCrumb() Crumb
```


#### `FromCrumbs`  <sub>(func)</sub>

```go
func FromCrumbs(in []Crumb) (ret []ProtoCrumb)
```

#### `ProtoCrumb.GetRawCrumb`  <sub>(method)</sub>

```go
func (m *ProtoCrumb) GetRawCrumb() Crumb
```

#### `FromProtoCrumbs`  <sub>(func)</sub>

```go
func FromProtoCrumbs(in []ProtoCrumb) (ret []Crumb)
```

#### `ProtoRoute.Route`  <sub>(method)</sub>

```go
func (p ProtoRoute) Route() Route
```

#### `Route.ProtoRoute`  <sub>(method)</sub>

```go
func (p Route) ProtoRoute() ProtoRoute
```

#### `FromProtoRoutes`  <sub>(func)</sub>

```go
func FromProtoRoutes(in []ProtoRoute) (ret []Route)
```

#### `FromRoutes`  <sub>(func)</sub>

```go
func FromRoutes(in []Route) (ret []ProtoRoute)
```

#### `ProtoAPI.API`  <sub>(method)</sub>

```go
func (p ProtoAPI) API() API
```

#### `API.ProtoAPI`  <sub>(method)</sub>

```go
func (p API) ProtoAPI() ProtoAPI
```

#### `ProtoAPIS.APIS`  <sub>(method)</sub>

```go
func (p ProtoAPIS) APIS() APIS
```

### `kiterunner/pkg/proute/proute_encoders.go`


#### `RegisterGobs`  <sub>(func)</sub>

> RegisterGobs will register globally the gobs needed to serialize and deserialize an API

```go
func RegisterGobs()
```

#### `APIS.EncodeProtoFile`  <sub>(method)</sub>

> EncodeProtoFile will encode the APIS to the specified filename, overwriting any existing file

```go
func (a APIS) EncodeProtoFile(filename string) error
```

#### `APIS.EncodeGobFile`  <sub>(method)</sub>

> EncodeGobFile will encode the APIS to the specified filename, overwriting any existing file

```go
func (a APIS) EncodeGobFile(filename string) error
```

#### `APIS.EncodeProto`  <sub>(method)</sub>

> EncodeProto will encode the APIs into a gob format that can be easily and quickly decoded

```go
func (a APIS) EncodeProto(w io.Writer) error
```

#### `APIS.EncodeGob`  <sub>(method)</sub>

> EncodeGob will encode the APIs into a gob format that can be easily and quickly decoded

```go
func (a APIS) EncodeGob(w io.Writer) error
```

#### `DecodeAPIGobFile`  <sub>(func)</sub>

> DecodeAPIGobFile will decode the API from the given file

```go
func DecodeAPIGobFile(filename string) (APIS, error)
```

#### `DecodeAPIProtoFile`  <sub>(func)</sub>

> DecodeAPIProtoFile will decode the API from the given file

```go
func DecodeAPIProtoFile(filename string) (APIS, error)
```

#### `DecodeProtoAPI`  <sub>(func)</sub>

> DecodeProtoAPI will decode from the given reader, returning the decoded APIs

```go
func DecodeProtoAPI(r io.Reader) (APIS, error)
```

#### `DecodeGobAPI`  <sub>(func)</sub>

> DecodeGobAPI will decode from the given reader, returning the decoded APIs

```go
func DecodeGobAPI(r io.Reader) (APIS, error)
```

### `kiterunner/pkg/proute/random.go`


#### `RandomString`  <sub>(func)</sub>

```go
func RandomString(rng *rand.Rand, charset string, length int) string
```

### `kiterunner/pkg/proute/route.go`


#### `Route`  <sub>(struct)</sub>

> Route is a request to be made

```go
type Route struct
```

**Fields:**
```go
TemplatePath string
PathCrumbs   []Crumb    // the raw path with template locations. This should begin with a '/'
path         []byte    // Pieces of the path
query        []byte    // cached rendered path
Method       string    // cached rendered query
HeaderCrumbs []Crumb
headers      []KV    // can be static or various types
QueryCrumbs []Crumb
queryParams []KV
BodyCrumbs  []Crumb
body        []byte
ContentType []ContentType    // rendered body
```


#### `Route.QueryParams`  <sub>(method)</sub>

```go
func (r Route) QueryParams(generate bool) []KV
```

#### `ContentType`  <sub>(type)</sub>

```go
type ContentType string
```

#### `DefaultValTagFunc`  <sub>(func)</sub>

```go
func DefaultValTagFunc(defaultv []byte, m map[string][]byte) func(w io.Writer, tag string) (int, error)
```

#### `Route.Path`  <sub>(method)</sub>

> Path returns a rendered path (with all the elements populated as a string)  
> calling with generate=true will generate a new path using the crumbs provided. otherwise a cached  
> version will be returned.  
> we generate the path by substituting the values into the handlebars-esque template string provided

```go
func (r Route) Path(generate bool) (string, error)
```

#### `Route.Query`  <sub>(method)</sub>

> Query will generate the full query string not including the ? e.g. foo=bar&baz=boo  
> Query Parameters will be pulled from the route QueryCrumbs  
> We can add extra query params. These will be written first, and the route query params will override the provided

```go
func (r Route) Query(generate bool, extraParams ...KV) (string, error)
```

#### `Route.Body`  <sub>(method)</sub>

> Path returns a rendered body (with all the elements populated as a string)  
> calling with generate=true will generate a new path using the crumbs provided. otherwise a cached  
> version will be returned  
> ContentType can be a specified contentType, or ContentTypeAny. If Any, this will attempt to deduce  
> the content type based on the route data

```go
func (r Route) Body(generate bool, contentType ContentType) []byte
```

#### `Route.Headers`  <sub>(method)</sub>

```go
func (r Route) Headers(generate bool) []KV
```

#### `stripNonSpaceWhitespace`  <sub>(func)</sub>

```go
func stripNonSpaceWhitespace(v string) string
```

#### `KV`  <sub>(struct)</sub>

```go
type KV struct
```

**Fields:**
```go
Key   string
Value string
```


### `kiterunner/pkg/proute/slice.go`


#### `PRouteOptions`  <sub>(struct)</sub>

```go
type PRouteOptions struct
```

**Fields:**
```go
HeaderCrumbs []Crumb
QueryCrumbs  []Crumb
PathCrumbs   []Crumb
BodyCrumbs   []Crumb
Method       string
ID string
ContentType  []ContentType
```


#### `PRouteOption`  <sub>(type)</sub>

```go
type PRouteOption func(o *PRouteOptions)
```

#### `OptHeader`  <sub>(func)</sub>

```go
func OptHeader(c Crumb) PRouteOption
```

#### `OptQuery`  <sub>(func)</sub>

```go
func OptQuery(c Crumb) PRouteOption
```

#### `OptPath`  <sub>(func)</sub>

```go
func OptPath(c Crumb) PRouteOption
```

#### `OptBody`  <sub>(func)</sub>

```go
func OptBody(c Crumb) PRouteOption
```

#### `OptID`  <sub>(func)</sub>

```go
func OptID(ID string) PRouteOption
```

#### `OptMethod`  <sub>(func)</sub>

```go
func OptMethod(method string) PRouteOption
```

#### `OptContentType`  <sub>(func)</sub>

```go
func OptContentType(v string) PRouteOption
```

#### `FromStringSliceReader`  <sub>(func)</sub>

```go
func FromStringSliceReader(r io.Reader, source string, opts...PRouteOption) (API, error)
```

#### `FromStringSlice`  <sub>(func)</sub>

> FromStringSlice will convert a string slice of paths into a Proute API. You can provide options to configure  
> all the proutes at the same time, but not any individually

```go
func FromStringSlice(in []string, source string, opts ...PRouteOption) (API, error)
```

#### `APIS.EncodeStringSlice`  <sub>(method)</sub>

```go
func (a APIS) EncodeStringSlice(w io.Writer) (error)
```
