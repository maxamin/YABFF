# gobuster — Complete API Reference

Exhaustive per-item reference for **every** type, function, method, constant, and macro in the gobuster source (`OJ/gobuster` @ `c77583f`, v3.8.2). Language: **GO**. Generated from a full source read; test files are excluded. Each item shows its full signature and doc comment; composite types list their fields/members.

**217 documented items** across 47 source files.


---

## Modules / packages

- [`(root)`](#pkg--root-) — 1 items
- [`cli`](#pkg-cli) — 36 items
- [`gobusterdir`](#pkg-gobusterdir) — 15 items
- [`gobusterdns`](#pkg-gobusterdns) — 18 items
- [`gobusterfuzz`](#pkg-gobusterfuzz) — 15 items
- [`gobustergcs`](#pkg-gobustergcs) — 16 items
- [`gobusters3`](#pkg-gobusters3) — 16 items
- [`gobustertftp`](#pkg-gobustertftp) — 13 items
- [`gobustervhost`](#pkg-gobustervhost) — 13 items
- [`libgobuster`](#pkg-libgobuster) — 68 items
- [`vhs`](#pkg-vhs) — 6 items

---


## `(root)` <a id='pkg--root-'></a>

*1 documented items*


### `gobuster/main.go`


#### `main`  <sub>(func)</sub>

```go
func main()
```


## `cli` <a id='pkg-cli'></a>

*36 documented items*


### `gobuster/cli/dir/dir.go`


#### `Command`  <sub>(func)</sub>

```go
func Command() *cli.Command
```

#### `getFlags`  <sub>(func)</sub>

```go
func getFlags() []cli.Flag
```

#### `run`  <sub>(func)</sub>

```go
func run(c *cli.Context) error
```

### `gobuster/cli/dns/dns.go`


#### `Command`  <sub>(func)</sub>

```go
func Command() *cli.Command
```

#### `getFlags`  <sub>(func)</sub>

```go
func getFlags() []cli.Flag
```

#### `run`  <sub>(func)</sub>

```go
func run(c *cli.Context) error
```

### `gobuster/cli/fuzz/fuzz.go`


#### `Command`  <sub>(func)</sub>

```go
func Command() *cli.Command
```

#### `getFlags`  <sub>(func)</sub>

```go
func getFlags() []cli.Flag
```

#### `run`  <sub>(func)</sub>

```go
func run(c *cli.Context) error
```

#### `containsFuzzKeyword`  <sub>(func)</sub>

```go
func containsFuzzKeyword(pluginopts gobusterfuzz.OptionsFuzz) bool
```

### `gobuster/cli/gcs/gcs.go`


#### `Command`  <sub>(func)</sub>

```go
func Command() *cli.Command
```

#### `getFlags`  <sub>(func)</sub>

```go
func getFlags() []cli.Flag
```

#### `run`  <sub>(func)</sub>

```go
func run(c *cli.Context) error
```

### `gobuster/cli/gobuster.go`


#### `resultWorker`  <sub>(func)</sub>

> resultWorker outputs the results as they come in. This needs to be a range and should not handle  
> the context so the channel always has a receiver and libgobuster will not block.

```go
func resultWorker(g *libgobuster.Gobuster, filename string, wg *sync.WaitGroup)
```

#### `errorWorker`  <sub>(func)</sub>

> errorWorker outputs the errors as they come in. This needs to be a range and should not handle  
> the context so the channel always has a receiver and libgobuster will not block.

```go
func errorWorker(g *libgobuster.Gobuster, wg *sync.WaitGroup)
```

#### `messageWorker`  <sub>(func)</sub>

> messageWorker outputs messages as they come in. This needs to be a range and should not handle  
> the context so the channel always has a receiver and libgobuster will not block.

```go
func messageWorker(g *libgobuster.Gobuster, wg *sync.WaitGroup)
```

#### `printProgress`  <sub>(func)</sub>

```go
func printProgress(g *libgobuster.Gobuster)
```

#### `progressWorker`  <sub>(func)</sub>

> progressWorker outputs the progress every tick. It will stop once cancel() is called  
> on the context

```go
func progressWorker(ctx context.Context, g *libgobuster.Gobuster, wg *sync.WaitGroup)
```

#### `writeToFile`  <sub>(func)</sub>

```go
func writeToFile(f *os.File, output string) error
```

#### `Gobuster`  <sub>(func)</sub>

> Gobuster is the main entry point for the CLI

```go
func Gobuster(ctx context.Context, opts *libgobuster.Options, plugin libgobuster.GobusterPlugin, log *libgobuster.Logger) error
```

### `gobuster/cli/options.go`


#### `BasicHTTPOptions`  <sub>(func)</sub>

```go
func BasicHTTPOptions() []cli.Flag
```

#### `ParseBasicHTTPOptions`  <sub>(func)</sub>

```go
func ParseBasicHTTPOptions(c *cli.Context) (libgobuster.BasicHTTPOptions, error)
```

#### `CommonHTTPOptions`  <sub>(func)</sub>

```go
func CommonHTTPOptions() []cli.Flag
```

#### `ParseCommonHTTPOptions`  <sub>(func)</sub>

```go
func ParseCommonHTTPOptions(c *cli.Context) (libgobuster.HTTPOptions, error)
```

#### `GlobalOptions`  <sub>(func)</sub>

```go
func GlobalOptions() []cli.Flag
```

#### `ParseGlobalOptions`  <sub>(func)</sub>

```go
func ParseGlobalOptions(c *cli.Context) (libgobuster.Options, error)
```

#### `getLocalAddrFromInterface`  <sub>(func)</sub>

```go
func getLocalAddrFromInterface(iface string) (*net.TCPAddr, error)
```

### `gobuster/cli/s3/s3.go`


#### `Command`  <sub>(func)</sub>

```go
func Command() *cli.Command
```

#### `getFlags`  <sub>(func)</sub>

```go
func getFlags() []cli.Flag
```

#### `run`  <sub>(func)</sub>

```go
func run(c *cli.Context) error
```

### `gobuster/cli/tftp/tftp.go`


#### `Command`  <sub>(func)</sub>

```go
func Command() *cli.Command
```

#### `getFlags`  <sub>(func)</sub>

```go
func getFlags() []cli.Flag
```

#### `run`  <sub>(func)</sub>

```go
func run(c *cli.Context) error
```

### `gobuster/cli/vhost/vhost.go`


#### `Command`  <sub>(func)</sub>

```go
func Command() *cli.Command
```

#### `getFlags`  <sub>(func)</sub>

```go
func getFlags() []cli.Flag
```

#### `run`  <sub>(func)</sub>

```go
func run(c *cli.Context) error
```


## `gobusterdir` <a id='pkg-gobusterdir'></a>

*15 documented items*


### `gobuster/gobusterdir/gobusterdir.go`


#### `WildcardError`  <sub>(struct)</sub>

> WildcardError is returned if a wildcard response is found

```go
type WildcardError struct
```

**Fields:**
```go
url        string
location   string
statusCode int
length     int64
```


#### `WildcardError.Error`  <sub>(method)</sub>

> Error is the implementation of the error interface

```go
func (e *WildcardError) Error() string
```

#### `GobusterDir`  <sub>(struct)</sub>

> GobusterDir is the main type to implement the interface

```go
type GobusterDir struct
```

**Fields:**
```go
options    *OptionsDir
globalopts *libgobuster.Options
http       *libgobuster.HTTPClient
```


#### `New`  <sub>(func)</sub>

> New creates a new initialized GobusterDir

```go
func New(globalopts *libgobuster.Options, opts *OptionsDir, logger *libgobuster.Logger) (*GobusterDir, error)
```

#### `GobusterDir.Name`  <sub>(method)</sub>

> Name should return the name of the plugin

```go
func (d *GobusterDir) Name() string
```

#### `GobusterDir.PreRun`  <sub>(method)</sub>

> PreRun is the pre run implementation of gobusterdir

```go
func (d *GobusterDir) PreRun(ctx context.Context, pr *libgobuster.Progress) error
```

#### `GobusterDir.AdditionalSuccessWords`  <sub>(method)</sub>

```go
func (d *GobusterDir) AdditionalSuccessWords(word string) []string
```

#### `GobusterDir.AdditionalWordsLen`  <sub>(method)</sub>

```go
func (d *GobusterDir) AdditionalWordsLen() int
```

#### `GobusterDir.AdditionalWords`  <sub>(method)</sub>

```go
func (d *GobusterDir) AdditionalWords(word string) []string
```

#### `GobusterDir.ProcessWord`  <sub>(method)</sub>

> ProcessWord is the process implementation of gobusterdir

```go
func (d *GobusterDir) ProcessWord(ctx context.Context, word string, progress *libgobuster.Progress) (libgobuster.Result, error)
```

#### `GobusterDir.GetConfigString`  <sub>(method)</sub>

> GetConfigString returns the string representation of the current config

```go
func (d *GobusterDir) GetConfigString() (string, error)
```

### `gobuster/gobusterdir/options.go`


#### `OptionsDir`  <sub>(struct)</sub>

> OptionsDir is the struct to hold all options for this plugin

```go
type OptionsDir struct
```

**Fields:**
```go
libgobuster.HTTPOptions
Extensions                 string
ExtensionsParsed           libgobuster.Set[string]
ExtensionsFile             string
StatusCodes                string
StatusCodesParsed          libgobuster.Set[int]
StatusCodesBlacklist       string
StatusCodesBlacklistParsed libgobuster.Set[int]
UseSlash                   bool
HideLength                 bool
Expanded                   bool
NoStatus                   bool
DiscoverBackup             bool
ExcludeLength              string
ExcludeLengthParsed        libgobuster.Set[int]
Force                      bool
```


#### `NewOptions`  <sub>(func)</sub>

> NewOptions returns a new initialized OptionsDir

```go
func NewOptions() *OptionsDir
```

### `gobuster/gobusterdir/result.go`


#### `Result`  <sub>(struct)</sub>

> Result represents a single result

```go
type Result struct
```

**Fields:**
```go
Path       string
Header     http.Header
StatusCode int
Size       int64
```


#### `Result.ResultToString`  <sub>(method)</sub>

> ResultToString converts the Result to its textual representation

```go
func (r Result) ResultToString() (string, error)
```


## `gobusterdns` <a id='pkg-gobusterdns'></a>

*18 documented items*


### `gobuster/gobusterdns/gobusterdns.go`


#### `WildcardError`  <sub>(struct)</sub>

> WildcardError is returned if a wildcard response is found

```go
type WildcardError struct
```

**Fields:**
```go
wildcardIps libgobuster.Set[netip.Addr]
```


#### `WildcardError.Error`  <sub>(method)</sub>

> Error is the implementation of the error interface

```go
func (e *WildcardError) Error() string
```

#### `GobusterDNS`  <sub>(struct)</sub>

> GobusterDNS is the main type to implement the interface

```go
type GobusterDNS struct
```

**Fields:**
```go
resolver    *net.Resolver
globalopts  *libgobuster.Options
options     *OptionsDNS
isWildcard  bool
wildcardIps libgobuster.Set[netip.Addr]
```


#### `newCustomDialer`  <sub>(func)</sub>

```go
func newCustomDialer(server string, protocol string) func(ctx context.Context, network, address string) (net.Conn, error)
```

#### `New`  <sub>(func)</sub>

> New creates a new initialized GobusterDNS

```go
func New(globalopts *libgobuster.Options, opts *OptionsDNS) (*GobusterDNS, error)
```

#### `GobusterDNS.Name`  <sub>(method)</sub>

> Name should return the name of the plugin

```go
func (d *GobusterDNS) Name() string
```

#### `GobusterDNS.PreRun`  <sub>(method)</sub>

> PreRun is the pre run implementation of gobusterdns

```go
func (d *GobusterDNS) PreRun(ctx context.Context, progress *libgobuster.Progress) error
```

#### `GobusterDNS.ProcessWord`  <sub>(method)</sub>

> ProcessWord is the process implementation of gobusterdns

```go
func (d *GobusterDNS) ProcessWord(ctx context.Context, word string, progress *libgobuster.Progress) (libgobuster.Result, error)
```

#### `GobusterDNS.AdditionalWordsLen`  <sub>(method)</sub>

```go
func (d *GobusterDNS) AdditionalWordsLen() int
```

#### `GobusterDNS.AdditionalWords`  <sub>(method)</sub>

```go
func (d *GobusterDNS) AdditionalWords(_ string) []string
```

#### `GobusterDNS.AdditionalSuccessWords`  <sub>(method)</sub>

```go
func (d *GobusterDNS) AdditionalSuccessWords(_ string) []string
```

#### `GobusterDNS.GetConfigString`  <sub>(method)</sub>

> GetConfigString returns the string representation of the current config

```go
func (d *GobusterDNS) GetConfigString() (string, error)
```

#### `GobusterDNS.dnsLookup`  <sub>(method)</sub>

```go
func (d *GobusterDNS) dnsLookup(ctx context.Context, domain string) ([]netip.Addr, error)
```

#### `GobusterDNS.dnsLookupCname`  <sub>(method)</sub>

```go
func (d *GobusterDNS) dnsLookupCname(ctx context.Context, domain string) (string, error)
```

### `gobuster/gobusterdns/options.go`


#### `OptionsDNS`  <sub>(struct)</sub>

> OptionsDNS holds all options for the dns plugin

```go
type OptionsDNS struct
```

**Fields:**
```go
Domain         string
CheckCNAME     bool
WildcardForced bool
Resolver       string
Protocol       string
NoFQDN         bool
Timeout        time.Duration
```


#### `NewOptions`  <sub>(func)</sub>

> NewOptions returns a new initialized OptionsDNS

```go
func NewOptions() *OptionsDNS
```

### `gobuster/gobusterdns/result.go`


#### `Result`  <sub>(struct)</sub>

> Result represents a single result

```go
type Result struct
```

**Fields:**
```go
Subdomain string
IPs       []netip.Addr
CNAME     string
```


#### `Result.ResultToString`  <sub>(method)</sub>

> ResultToString converts the Result to its textual representation

```go
func (r Result) ResultToString() (string, error)
```


## `gobusterfuzz` <a id='pkg-gobusterfuzz'></a>

*15 documented items*


### `gobuster/gobusterfuzz/gobusterfuzz.go`


#### `WildcardError`  <sub>(struct)</sub>

> WildcardError is returned if a wildcard response is found

```go
type WildcardError struct
```

**Fields:**
```go
url        string
statusCode int
```


#### `WildcardError.Error`  <sub>(method)</sub>

> Error is the implementation of the error interface

```go
func (e *WildcardError) Error() string
```

#### `GobusterFuzz`  <sub>(struct)</sub>

> GobusterFuzz is the main type to implement the interface

```go
type GobusterFuzz struct
```

**Fields:**
```go
options    *OptionsFuzz
globalopts *libgobuster.Options
http       *libgobuster.HTTPClient
```


#### `New`  <sub>(func)</sub>

> New creates a new initialized GobusterFuzz

```go
func New(globalopts *libgobuster.Options, opts *OptionsFuzz, logger *libgobuster.Logger) (*GobusterFuzz, error)
```

#### `GobusterFuzz.Name`  <sub>(method)</sub>

> Name should return the name of the plugin

```go
func (d *GobusterFuzz) Name() string
```

#### `GobusterFuzz.PreRun`  <sub>(method)</sub>

> PreRun is the pre run implementation of gobusterfuzz

```go
func (d *GobusterFuzz) PreRun(_ context.Context, _ *libgobuster.Progress) error
```

#### `GobusterFuzz.ProcessWord`  <sub>(method)</sub>

> ProcessWord is the process implementation of gobusterfuzz

```go
func (d *GobusterFuzz) ProcessWord(ctx context.Context, word string, progress *libgobuster.Progress) (libgobuster.Result, error)
```

#### `GobusterFuzz.AdditionalWordsLen`  <sub>(method)</sub>

```go
func (d *GobusterFuzz) AdditionalWordsLen() int
```

#### `GobusterFuzz.AdditionalWords`  <sub>(method)</sub>

```go
func (d *GobusterFuzz) AdditionalWords(_ string) []string
```

#### `GobusterFuzz.AdditionalSuccessWords`  <sub>(method)</sub>

```go
func (d *GobusterFuzz) AdditionalSuccessWords(_ string) []string
```

#### `GobusterFuzz.GetConfigString`  <sub>(method)</sub>

> GetConfigString returns the string representation of the current config

```go
func (d *GobusterFuzz) GetConfigString() (string, error)
```

### `gobuster/gobusterfuzz/options.go`


#### `OptionsFuzz`  <sub>(struct)</sub>

> OptionsFuzz is the struct to hold all options for this plugin

```go
type OptionsFuzz struct
```

**Fields:**
```go
libgobuster.HTTPOptions
ExcludedStatusCodes       string
ExcludedStatusCodesParsed libgobuster.Set[int]
ExcludeLength             string
ExcludeLengthParsed       libgobuster.Set[int]
RequestBody               string
```


#### `NewOptions`  <sub>(func)</sub>

> NewOptions returns a new initialized OptionsFuzz

```go
func NewOptions() *OptionsFuzz
```

### `gobuster/gobusterfuzz/result.go`


#### `Result`  <sub>(struct)</sub>

> Result represents a single result

```go
type Result struct
```

**Fields:**
```go
Word       string
Path       string
StatusCode int
Size       int64
Header     http.Header
```


#### `Result.ResultToString`  <sub>(method)</sub>

> ResultToString converts the Result to its textual representation

```go
func (r Result) ResultToString() (string, error)
```


## `gobustergcs` <a id='pkg-gobustergcs'></a>

*16 documented items*


### `gobuster/gobustergcs/gobustersgcs.go`


#### `GobusterGCS`  <sub>(struct)</sub>

> GobusterGCS is the main type to implement the interface

```go
type GobusterGCS struct
```

**Fields:**
```go
options     *OptionsGCS
globalopts  *libgobuster.Options
http        *libgobuster.HTTPClient
bucketRegex *regexp.Regexp
```


#### `New`  <sub>(func)</sub>

> New creates a new initialized GobusterGCS

```go
func New(globalopts *libgobuster.Options, opts *OptionsGCS, logger *libgobuster.Logger) (*GobusterGCS, error)
```

#### `GobusterGCS.Name`  <sub>(method)</sub>

> Name should return the name of the plugin

```go
func (s *GobusterGCS) Name() string
```

#### `GobusterGCS.PreRun`  <sub>(method)</sub>

> PreRun is the pre run implementation of GobusterS3

```go
func (s *GobusterGCS) PreRun(_ context.Context, _ *libgobuster.Progress) error
```

#### `GobusterGCS.ProcessWord`  <sub>(method)</sub>

> ProcessWord is the process implementation of GobusterS3

```go
func (s *GobusterGCS) ProcessWord(ctx context.Context, word string, progress *libgobuster.Progress) (libgobuster.Result, error)
```

#### `GobusterGCS.AdditionalWordsLen`  <sub>(method)</sub>

```go
func (s *GobusterGCS) AdditionalWordsLen() int
```

#### `GobusterGCS.AdditionalWords`  <sub>(method)</sub>

```go
func (s *GobusterGCS) AdditionalWords(_ string) []string
```

#### `GobusterGCS.AdditionalSuccessWords`  <sub>(method)</sub>

```go
func (s *GobusterGCS) AdditionalSuccessWords(_ string) []string
```

#### `GobusterGCS.GetConfigString`  <sub>(method)</sub>

> GetConfigString returns the string representation of the current config

```go
func (s *GobusterGCS) GetConfigString() (string, error)
```

#### `GobusterGCS.isValidBucketName`  <sub>(method)</sub>

> https://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html

```go
func (s *GobusterGCS) isValidBucketName(bucketName string) bool
```

### `gobuster/gobustergcs/options.go`


#### `OptionsGCS`  <sub>(struct)</sub>

> OptionsGCS is the struct to hold all options for this plugin

```go
type OptionsGCS struct
```

**Fields:**
```go
libgobuster.BasicHTTPOptions
MaxFilesToList int
ShowFiles      bool
```


#### `NewOptions`  <sub>(func)</sub>

> NewOptions returns a new initialized OptionsS3

```go
func NewOptions() *OptionsGCS
```

### `gobuster/gobustergcs/result.go`


#### `Result`  <sub>(struct)</sub>

> Result represents a single result

```go
type Result struct
```

**Fields:**
```go
Found      bool
BucketName string
Status     string
```


#### `Result.ResultToString`  <sub>(method)</sub>

> ResultToString converts the Result to its textual representation

```go
func (r Result) ResultToString() (string, error)
```

### `gobuster/gobustergcs/types.go`


#### `GCSError`  <sub>(struct)</sub>

> GCSError represents a returned error from GCS

```go
type GCSError struct
```

**Fields:**
```go
Error struct {
Code    int    `json:"code"`
Message string `json:"message"`
Errors  []struct {
Message      string `json:"message"`
Reason       string `json:"reason"`
LocationType string `json:"locationType"`
Location     string `json:"location"`
} `json:"errors"`
} `json:"error"`
```


#### `GCSListing`  <sub>(struct)</sub>

> GCSListing contains only a subset of returned properties

```go
type GCSListing struct
```

**Fields:**
```go
IsTruncated string `json:"nextPageToken"`
Items       []struct {
Name         string `json:"name"`
LastModified string `json:"updated"`
Size         string `json:"size"`
} `json:"items"`
```



## `gobusters3` <a id='pkg-gobusters3'></a>

*16 documented items*


### `gobuster/gobusters3/gobusters3.go`


#### `GobusterS3`  <sub>(struct)</sub>

> GobusterS3 is the main type to implement the interface

```go
type GobusterS3 struct
```

**Fields:**
```go
options     *OptionsS3
globalopts  *libgobuster.Options
http        *libgobuster.HTTPClient
bucketRegex *regexp.Regexp
```


#### `New`  <sub>(func)</sub>

> New creates a new initialized GobusterS3

```go
func New(globalopts *libgobuster.Options, opts *OptionsS3, logger *libgobuster.Logger) (*GobusterS3, error)
```

#### `GobusterS3.Name`  <sub>(method)</sub>

> Name should return the name of the plugin

```go
func (s *GobusterS3) Name() string
```

#### `GobusterS3.PreRun`  <sub>(method)</sub>

> PreRun is the pre run implementation of GobusterS3

```go
func (s *GobusterS3) PreRun(_ context.Context, _ *libgobuster.Progress) error
```

#### `GobusterS3.ProcessWord`  <sub>(method)</sub>

> ProcessWord is the process implementation of GobusterS3

```go
func (s *GobusterS3) ProcessWord(ctx context.Context, word string, progress *libgobuster.Progress) (libgobuster.Result, error)
```

#### `GobusterS3.AdditionalWordsLen`  <sub>(method)</sub>

```go
func (s *GobusterS3) AdditionalWordsLen() int
```

#### `GobusterS3.AdditionalWords`  <sub>(method)</sub>

```go
func (s *GobusterS3) AdditionalWords(_ string) []string
```

#### `GobusterS3.AdditionalSuccessWords`  <sub>(method)</sub>

```go
func (s *GobusterS3) AdditionalSuccessWords(_ string) []string
```

#### `GobusterS3.GetConfigString`  <sub>(method)</sub>

> GetConfigString returns the string representation of the current config

```go
func (s *GobusterS3) GetConfigString() (string, error)
```

#### `GobusterS3.isValidBucketName`  <sub>(method)</sub>

> https://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html

```go
func (s *GobusterS3) isValidBucketName(bucketName string) bool
```

### `gobuster/gobusters3/options.go`


#### `OptionsS3`  <sub>(struct)</sub>

> OptionsS3 is the struct to hold all options for this plugin

```go
type OptionsS3 struct
```

**Fields:**
```go
libgobuster.BasicHTTPOptions
MaxFilesToList int
ShowFiles      bool
```


#### `NewOptions`  <sub>(func)</sub>

> NewOptions returns a new initialized OptionsS3

```go
func NewOptions() *OptionsS3
```

### `gobuster/gobusters3/result.go`


#### `Result`  <sub>(struct)</sub>

> Result represents a single result

```go
type Result struct
```

**Fields:**
```go
Found      bool
BucketName string
Status     string
```


#### `Result.ResultToString`  <sub>(method)</sub>

> ResultToString converts the Result to its textual representation

```go
func (r Result) ResultToString() (string, error)
```

### `gobuster/gobusters3/types.go`


#### `AWSError`  <sub>(struct)</sub>

> AWSError represents a returned error from AWS

```go
type AWSError struct
```

**Fields:**
```go
XMLName   xml.Name `xml:"Error"`
Code      string   `xml:"Code"`
Message   string   `xml:"Message"`
RequestID string   `xml:"RequestId"`
HostID    string   `xml:"HostId"`
```


#### `AWSListing`  <sub>(struct)</sub>

> AWSListing contains only a subset of returned properties

```go
type AWSListing struct
```

**Fields:**
```go
XMLName     xml.Name `xml:"ListBucketResult"`
Name        string   `xml:"Name"`
IsTruncated string   `xml:"IsTruncated"`
Contents    []struct {
Key          string `xml:"Key"`
LastModified string `xml:"LastModified"`
Size         int    `xml:"Size"`
} `xml:"Contents"`
```



## `gobustertftp` <a id='pkg-gobustertftp'></a>

*13 documented items*


### `gobuster/gobustertftp/gobustertftp.go`


#### `GobusterTFTP`  <sub>(struct)</sub>

> GobusterTFTP is the main type to implement the interface

```go
type GobusterTFTP struct
```

**Fields:**
```go
globalopts *libgobuster.Options
options    *OptionsTFTP
```


#### `New`  <sub>(func)</sub>

> New creates a new initialized NewGobusterTFTP

```go
func New(globalopts *libgobuster.Options, opts *OptionsTFTP) (*GobusterTFTP, error)
```

#### `GobusterTFTP.Name`  <sub>(method)</sub>

> Name should return the name of the plugin

```go
func (d *GobusterTFTP) Name() string
```

#### `GobusterTFTP.PreRun`  <sub>(method)</sub>

> PreRun is the pre run implementation of gobustertftp

```go
func (d *GobusterTFTP) PreRun(_ context.Context, _ *libgobuster.Progress) error
```

#### `GobusterTFTP.ProcessWord`  <sub>(method)</sub>

> ProcessWord is the process implementation of gobustertftp

```go
func (d *GobusterTFTP) ProcessWord(_ context.Context, word string, progress *libgobuster.Progress) (libgobuster.Result, error)
```

#### `GobusterTFTP.AdditionalWordsLen`  <sub>(method)</sub>

```go
func (d *GobusterTFTP) AdditionalWordsLen() int
```

#### `GobusterTFTP.AdditionalWords`  <sub>(method)</sub>

```go
func (d *GobusterTFTP) AdditionalWords(_ string) []string
```

#### `GobusterTFTP.AdditionalSuccessWords`  <sub>(method)</sub>

```go
func (d *GobusterTFTP) AdditionalSuccessWords(_ string) []string
```

#### `GobusterTFTP.GetConfigString`  <sub>(method)</sub>

> GetConfigString returns the string representation of the current config

```go
func (d *GobusterTFTP) GetConfigString() (string, error)
```

### `gobuster/gobustertftp/options.go`


#### `OptionsTFTP`  <sub>(struct)</sub>

> OptionsTFTP holds all options for the tftp plugin

```go
type OptionsTFTP struct
```

**Fields:**
```go
Server  string
Timeout time.Duration
```


#### `NewOptions`  <sub>(func)</sub>

> NewOptions returns a new initialized OptionsTFTP

```go
func NewOptions() *OptionsTFTP
```

### `gobuster/gobustertftp/result.go`


#### `Result`  <sub>(struct)</sub>

> Result represents a single result

```go
type Result struct
```

**Fields:**
```go
Filename     string
Size         int64
ErrorMessage string
```


#### `Result.ResultToString`  <sub>(method)</sub>

> ResultToString converts the Result to its textual representation

```go
func (r Result) ResultToString() (string, error)
```


## `gobustervhost` <a id='pkg-gobustervhost'></a>

*13 documented items*


### `gobuster/gobustervhost/gobustervhost.go`


#### `GobusterVhost`  <sub>(struct)</sub>

> GobusterVhost is the main type to implement the interface

```go
type GobusterVhost struct
```

**Fields:**
```go
options      *OptionsVhost
globalopts   *libgobuster.Options
http         *libgobuster.HTTPClient
domain       string
normalBody   []byte
abnormalBody []byte
once         sync.Once
```


#### `New`  <sub>(func)</sub>

> New creates a new initialized GobusterDir

```go
func New(globalopts *libgobuster.Options, opts *OptionsVhost, logger *libgobuster.Logger) (*GobusterVhost, error)
```

#### `GobusterVhost.Name`  <sub>(method)</sub>

> Name should return the name of the plugin

```go
func (v *GobusterVhost) Name() string
```

#### `GobusterVhost.PreRun`  <sub>(method)</sub>

> PreRun is the pre run implementation of gobusterdir

```go
func (v *GobusterVhost) PreRun(ctx context.Context, _ *libgobuster.Progress) error
```

#### `GobusterVhost.ProcessWord`  <sub>(method)</sub>

> ProcessWord is the process implementation of gobusterdir

```go
func (v *GobusterVhost) ProcessWord(ctx context.Context, word string, progress *libgobuster.Progress) (libgobuster.Result, error)
```

#### `GobusterVhost.AdditionalWordsLen`  <sub>(method)</sub>

```go
func (v *GobusterVhost) AdditionalWordsLen() int
```

#### `GobusterVhost.AdditionalWords`  <sub>(method)</sub>

```go
func (v *GobusterVhost) AdditionalWords(_ string) []string
```

#### `GobusterVhost.AdditionalSuccessWords`  <sub>(method)</sub>

```go
func (v *GobusterVhost) AdditionalSuccessWords(_ string) []string
```

#### `GobusterVhost.GetConfigString`  <sub>(method)</sub>

> GetConfigString returns the string representation of the current config

```go
func (v *GobusterVhost) GetConfigString() (string, error)
```

### `gobuster/gobustervhost/options.go`


#### `OptionsVhost`  <sub>(struct)</sub>

> OptionsVhost is the struct to hold all options for this plugin

```go
type OptionsVhost struct
```

**Fields:**
```go
libgobuster.HTTPOptions
AppendDomain          bool
ExcludeLength         string
ExcludeLengthParsed   libgobuster.Set[int]
ExcludeStatus         string
ExcludeStatusParsed   libgobuster.Set[int]
Domain                string
ExcludeHostnameLength bool
```


#### `NewOptions`  <sub>(func)</sub>

> NewOptions returns a new initialized OptionsVhost

```go
func NewOptions() *OptionsVhost
```

### `gobuster/gobustervhost/result.go`


#### `Result`  <sub>(struct)</sub>

> Result represents a single result

```go
type Result struct
```

**Fields:**
```go
Vhost      string
StatusCode int
Size       int64
Header     http.Header
```


#### `Result.ResultToString`  <sub>(method)</sub>

> ResultToString converts the Result to its textual representation

```go
func (r Result) ResultToString() (string, error)
```


## `libgobuster` <a id='pkg-libgobuster'></a>

*68 documented items*


### `gobuster/libgobuster/helpers.go`


#### `NewSet`  <sub>(func)</sub>

> NewSet creates a new initialized Set

```go
func NewSet[T comparable]() Set[T]
```

#### `Set[T].Add`  <sub>(method)</sub>

> Add an element to a set

```go
func (set *Set[T]) Add(s T) bool
```

#### `Set[T].AddRange`  <sub>(method)</sub>

> AddRange adds a list of elements to a set

```go
func (set *Set[T]) AddRange(ss []T)
```

#### `Set[T].Contains`  <sub>(method)</sub>

> Contains tests if an element is in a set

```go
func (set *Set[T]) Contains(s T) bool
```

#### `Set[T].ContainsAny`  <sub>(method)</sub>

> ContainsAny checks if any of the elements exist

```go
func (set *Set[T]) ContainsAny(ss []T) bool
```

#### `Set[T].Length`  <sub>(method)</sub>

> Length returns the length of the Set

```go
func (set *Set[T]) Length() int
```

#### `Set[T].Stringify`  <sub>(method)</sub>

> Stringify the set

```go
func (set *Set[T]) Stringify() string
```

#### `lineCounter`  <sub>(func)</sub>

> this method is much faster than lineCounter_slow but has the following errors:  
> - empty files are reported as 1 line  
> - files only containing a newline are reported as 1 line  
> - also counts lines with comments

```go
func lineCounter(r io.Reader) (int, error)
```

#### `lineCounterSlow`  <sub>(func)</sub>

```go
func lineCounterSlow(r io.Reader) (int, error)
```

#### `DefaultUserAgent`  <sub>(func)</sub>

> DefaultUserAgent returns the default user agent to use in HTTP requests

```go
func DefaultUserAgent() string
```

#### `ParseExtensions`  <sub>(func)</sub>

> ParseExtensions parses the extensions provided as a comma separated list

```go
func ParseExtensions(extensions string) (Set[string], error)
```

#### `ParseExtensionsFile`  <sub>(func)</sub>

```go
func ParseExtensionsFile(file string) ([]string, error)
```

#### `ParseCommaSeparatedInt`  <sub>(func)</sub>

> ParseCommaSeparatedInt parses the status codes provided as a comma separated list

```go
func ParseCommaSeparatedInt(inputString string) (Set[int], error)
```

### `gobuster/libgobuster/http.go`


#### `HTTPHeader`  <sub>(struct)</sub>

> HTTPHeader holds a single key value pair of a HTTP header

```go
type HTTPHeader struct
```

**Fields:**
```go
Name  string
Value string
```


#### `HTTPClient`  <sub>(struct)</sub>

> HTTPClient represents a http object

```go
type HTTPClient struct
```

**Fields:**
```go
client                *http.Client
userAgent             string
defaultUserAgent      string
username              string
password              string
headers               []HTTPHeader
noCanonicalizeHeaders bool
cookies               string
method                string
host                  string
logger                *Logger
```


#### `RequestOptions`  <sub>(struct)</sub>

> RequestOptions is used to pass options to a single individual request

```go
type RequestOptions struct
```

**Fields:**
```go
Host                     string
Body                     io.Reader
ReturnBody               bool
ModifiedHeaders          []HTTPHeader
UpdatedBasicAuthUsername string
UpdatedBasicAuthPassword string
```


#### `NewHTTPClient`  <sub>(func)</sub>

> NewHTTPClient returns a new HTTPClient

```go
func NewHTTPClient(opt *HTTPOptions, logger *Logger) (*HTTPClient, error)
```

#### `HTTPClient.Request`  <sub>(method)</sub>

> Request makes a http request and returns the status, the content length, the headers, the body and an error  
> if you want the body returned set the corresponding property inside RequestOptions

```go
func (client *HTTPClient) Request(ctx context.Context, fullURL url.URL, opts RequestOptions) (int, int64, http.Header, []byte, error)
```

#### `HTTPClient.makeRequest`  <sub>(method)</sub>

```go
func (client *HTTPClient) makeRequest(ctx context.Context, fullURL url.URL, opts RequestOptions) (*http.Response, error)
```

### `gobuster/libgobuster/interfaces.go`


#### `GobusterPlugin`  <sub>(interface)</sub>

> GobusterPlugin is an interface which plugins must implement

```go
type GobusterPlugin interface
```

**Members/Variants:**
```go
Name() string
PreRun(context.Context, *Progress) error
ProcessWord(context.Context, string, *Progress) (Result, error)
AdditionalWords(string) []string
AdditionalWordsLen() int
AdditionalSuccessWords(string) []string
GetConfigString() (string, error)
```


#### `Result`  <sub>(interface)</sub>

> Result is an interface for the Result object

```go
type Result interface
```

**Members/Variants:**
```go
ResultToString() (string, error)
```


### `gobuster/libgobuster/libgobuster.go`


#### `SetupFunc`  <sub>(type)</sub>

> SetupFunc is the "setup" function prototype for implementations

```go
type SetupFunc func(*Gobuster) error
```

#### `ProcessFunc`  <sub>(type)</sub>

> ProcessFunc is the "process" function prototype for implementations

```go
type ProcessFunc func(*Gobuster, string) ([]Result, error)
```

#### `ResultToStringFunc`  <sub>(type)</sub>

> ResultToStringFunc is the "to string" function prototype for implementations

```go
type ResultToStringFunc func(*Gobuster, *Result) (*string, error)
```

#### `Gobuster`  <sub>(struct)</sub>

> Gobuster is the main object when creating a new run

```go
type Gobuster struct
```

**Fields:**
```go
Opts     *Options
Logger   *Logger
plugin   GobusterPlugin
Progress *Progress
```


#### `Guess`  <sub>(struct)</sub>

```go
type Guess struct
```

**Fields:**
```go
word              string
discoverOnSuccess bool
```


#### `Wordlist`  <sub>(struct)</sub>

```go
type Wordlist struct
```

**Fields:**
```go
scanner        *bufio.Scanner
guessesPerLine int
isStream       bool
```


#### `NewGobuster`  <sub>(func)</sub>

> NewGobuster returns a new Gobuster object

```go
func NewGobuster(opts *Options, plugin GobusterPlugin, logger *Logger) (*Gobuster, error)
```

#### `Gobuster.worker`  <sub>(method)</sub>

```go
func (g *Gobuster) worker(ctx context.Context, guessChan <-chan *Guess, successChan chan<- *Guess, wg *sync.WaitGroup)
```

#### `feed`  <sub>(func)</sub>

```go
func feed(ctx context.Context, guessChan chan<- *Guess, words []string, discoverOnSuccess bool)
```

#### `Gobuster.feeder`  <sub>(method)</sub>

```go
func (g *Gobuster) feeder(ctx context.Context, guessChan chan<- *Guess, words []string, discoverOnSuccess bool, wg *sync.WaitGroup)
```

#### `Gobuster.feedWordlist`  <sub>(method)</sub>

```go
func (g *Gobuster) feedWordlist(ctx context.Context, guessChan chan<- *Guess, wordlist *Wordlist, wg *sync.WaitGroup)
```

#### `Gobuster.getWordlist`  <sub>(method)</sub>

```go
func (g *Gobuster) getWordlist(wordlist io.ReadSeeker) (*Wordlist, error)
```

#### `Gobuster.Run`  <sub>(method)</sub>

> Run the busting of the website with the given  
> set of settings from the command line.

```go
func (g *Gobuster) Run(ctx context.Context) error
```

#### `Gobuster.GetConfigString`  <sub>(method)</sub>

> GetConfigString returns the current config as a printable string

```go
func (g *Gobuster) GetConfigString() (string, error)
```

#### `Gobuster.processPatterns`  <sub>(method)</sub>

```go
func (g *Gobuster) processPatterns(word string) []string
```

#### `Gobuster.processDiscoverPatterns`  <sub>(method)</sub>

```go
func (g *Gobuster) processDiscoverPatterns(word string) []string
```

#### `Gobuster.applyPatterns`  <sub>(method)</sub>

```go
func (g *Gobuster) applyPatterns(word string, patterns []string) []string
```

### `gobuster/libgobuster/logger.go`


#### `Logger`  <sub>(struct)</sub>

```go
type Logger struct
```

**Fields:**
```go
log      *log.Logger
errorLog *log.Logger
debugLog *log.Logger
warnLog  *log.Logger
infoLog  *log.Logger
debug    bool
```


#### `NewLogger`  <sub>(func)</sub>

```go
func NewLogger(debug bool) *Logger
```

#### `Logger.Debug`  <sub>(method)</sub>

```go
func (l Logger) Debug(v ...any)
```

#### `Logger.Debugf`  <sub>(method)</sub>

```go
func (l Logger) Debugf(format string, v ...any)
```

#### `Logger.Warn`  <sub>(method)</sub>

```go
func (l Logger) Warn(v ...any)
```

#### `Logger.Warnf`  <sub>(method)</sub>

```go
func (l Logger) Warnf(format string, v ...any)
```

#### `Logger.Info`  <sub>(method)</sub>

```go
func (l Logger) Info(v ...any)
```

#### `Logger.Infof`  <sub>(method)</sub>

```go
func (l Logger) Infof(format string, v ...any)
```

#### `Logger.Print`  <sub>(method)</sub>

```go
func (l Logger) Print(v ...any)
```

#### `Logger.Printf`  <sub>(method)</sub>

```go
func (l Logger) Printf(format string, v ...any)
```

#### `Logger.Println`  <sub>(method)</sub>

```go
func (l Logger) Println(v ...any)
```

#### `Logger.Error`  <sub>(method)</sub>

```go
func (l Logger) Error(v ...any)
```

#### `Logger.Errorf`  <sub>(method)</sub>

```go
func (l Logger) Errorf(format string, v ...any)
```

#### `Logger.Fatal`  <sub>(method)</sub>

```go
func (l Logger) Fatal(v ...any)
```

#### `Logger.Fatalf`  <sub>(method)</sub>

```go
func (l Logger) Fatalf(format string, v ...any)
```

#### `Logger.Fatalln`  <sub>(method)</sub>

```go
func (l Logger) Fatalln(v ...any)
```

### `gobuster/libgobuster/options.go`


#### `Options`  <sub>(struct)</sub>

> Options holds all options that can be passed to libgobuster

```go
type Options struct
```

**Fields:**
```go
Threads             int
Debug               bool
Wordlist            string
WordlistOffset      int
PatternFile         string
DiscoverPatternFile string
Patterns            []string
DiscoverPatterns    []string
OutputFilename      string
NoProgress          bool
NoError             bool
Quiet               bool
Delay               time.Duration
```


### `gobuster/libgobuster/options_http.go`


#### `BasicHTTPOptions`  <sub>(struct)</sub>

> BasicHTTPOptions defines only core http options

```go
type BasicHTTPOptions struct
```

**Fields:**
```go
UserAgent        string
Proxy            string
NoTLSValidation  bool
Timeout          time.Duration
RetryOnTimeout   bool
RetryAttempts    int
TLSCertificate   *tls.Certificate
TLSRenegotiation bool
LocalAddr        *net.TCPAddr
```


#### `HTTPOptions`  <sub>(struct)</sub>

> HTTPOptions is the struct to pass in all http options to Gobuster

```go
type HTTPOptions struct
```

**Fields:**
```go
BasicHTTPOptions
Password              string
URL                   *url.URL
Username              string
Cookies               string
Headers               []HTTPHeader
NoCanonicalizeHeaders bool
FollowRedirect        bool
Method                string
```


### `gobuster/libgobuster/progress.go`


#### `MessageLevel`  <sub>(type)</sub>

```go
type MessageLevel int
```

#### `Message`  <sub>(struct)</sub>

```go
type Message struct
```

**Fields:**
```go
Level   MessageLevel
Message string
```


#### `Progress`  <sub>(struct)</sub>

```go
type Progress struct
```

**Fields:**
```go
requestsExpectedMutex *sync.RWMutex
requestsExpected      int
requestsCountMutex    *sync.RWMutex
requestsIssued        int
ResultChan            chan Result
ErrorChan             chan error
MessageChan           chan Message
```


#### `NewProgress`  <sub>(func)</sub>

```go
func NewProgress() *Progress
```

#### `Progress.RequestsExpected`  <sub>(method)</sub>

```go
func (p *Progress) RequestsExpected() int
```

#### `Progress.RequestsIssued`  <sub>(method)</sub>

```go
func (p *Progress) RequestsIssued() int
```

#### `Progress.incrementRequestsIssues`  <sub>(method)</sub>

```go
func (p *Progress) incrementRequestsIssues(by int)
```

#### `Progress.incrementRequests`  <sub>(method)</sub>

```go
func (p *Progress) incrementRequests()
```

#### `Progress.IncrementTotalRequests`  <sub>(method)</sub>

```go
func (p *Progress) IncrementTotalRequests(by int)
```

### `gobuster/libgobuster/useragents.go`


#### `GetRandomUserAgent`  <sub>(func)</sub>

> GetRandomUserAgent picks a random user agent from a predefined list

```go
func GetRandomUserAgent() (string, error)
```

### `gobuster/libgobuster/version.go`


#### `GetVersion`  <sub>(func)</sub>

```go
func GetVersion() string
```


## `vhs` <a id='pkg-vhs'></a>

*6 documented items*


### `gobuster/vhs/server.go`


#### `route`  <sub>(struct)</sub>

```go
type route struct
```

**Fields:**
```go
pattern *regexp.Regexp
handler http.Handler
```


#### `RegexpHandler`  <sub>(struct)</sub>

```go
type RegexpHandler struct
```

**Fields:**
```go
routes []*route
```


#### `RegexpHandler.Handler`  <sub>(method)</sub>

```go
func (h *RegexpHandler) Handler(pattern *regexp.Regexp, handler http.Handler)
```

#### `RegexpHandler.HandleFunc`  <sub>(method)</sub>

```go
func (h *RegexpHandler) HandleFunc(pattern *regexp.Regexp, handler func(http.ResponseWriter, *http.Request))
```

#### `RegexpHandler.ServeHTTP`  <sub>(method)</sub>

```go
func (h *RegexpHandler) ServeHTTP(w http.ResponseWriter, r *http.Request)
```

#### `main`  <sub>(func)</sub>

```go
func main()
```
