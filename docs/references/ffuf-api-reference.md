# ffuf — Complete API Reference

Exhaustive per-item reference for **every** type, function, method, constant, and macro in the ffuf source (`ffuf/ffuf` @ `8d65e03`, v2.1.0). Language: **GO**. Generated from a full source read; test files are excluded. Each item shows its full signature and doc comment; composite types list their fields/members.

**386 documented items** across 51 source files.


---

## Modules / packages

- [`(root)`](#pkg--root-) — 9 items
- [`pkg`](#pkg-pkg) — 377 items

---


## `(root)` <a id='pkg--root-'></a>

*9 documented items*


### `ffuf/help.go`


#### `UsageSection`  <sub>(struct)</sub>

```go
type UsageSection struct
```

**Fields:**
```go
Name        string
Description string
Flags       []UsageFlag
Hidden      bool
Key         string
```


#### `UsageSection.PrintSection`  <sub>(method)</sub>

> PrintSection prints out the section name, description and each of the flags

```go
func (u *UsageSection) PrintSection(max_length int, extended bool)
```

#### `UsageFlag`  <sub>(struct)</sub>

```go
type UsageFlag struct
```

**Fields:**
```go
Name        string
Description string
Default     string
```


#### `UsageFlag.PrintFlag`  <sub>(method)</sub>

> PrintFlag prints out the flag name, usage string and default value

```go
func (f *UsageFlag) PrintFlag(max_length int)
```

#### `Usage`  <sub>(func)</sub>

```go
func Usage()
```

### `ffuf/main.go`


#### `ParseFlags`  <sub>(func)</sub>

> ParseFlags registers every CLI flag from ffuf.RegisterFlags — the single source  
> of truth — onto the global flag set and parses the command line into opts.  
> Flag defaults are the current field values, so a value loaded from a config  
> file is used unless the same flag is given on the command line (file < CLI).

```go
func ParseFlags(opts *ffuf.ConfigOptions) *ffuf.ConfigOptions
```

#### `main`  <sub>(func)</sub>

```go
func main()
```

#### `SetupFilters`  <sub>(func)</sub>

> SetupFilters installs the matchers and filters onto conf.MatcherManager. The  
> matcher/filter construction itself lives in filter.FromConfig (pure, testable);  
> this shim owns only the one decision that genuinely needs the global CLI flag  
> set - whether to install the default status matcher - plus the -ignore-body  
> warning.

```go
func SetupFilters(parseOpts *ffuf.ConfigOptions, conf *ffuf.Config) error
```

#### `printSearchResults`  <sub>(func)</sub>

```go
func printSearchResults(conf *ffuf.Config, pos int, exectime time.Time, hash string)
```


## `pkg` <a id='pkg-pkg'></a>

*377 documented items*


### `ffuf/pkg/assembly/build.go`


#### `BuildJob`  <sub>(func)</sub>

> BuildJob constructs and wires a Job from conf. It is the single source of truth  
> for how a Job is assembled. Matchers/filters are NOT installed here (they need  
> the CLI flag state); see filter.FromConfig and main.SetupFilters.

```go
func BuildJob(conf *ffuf.Config) (*engine.Job, error)
```

### `ffuf/pkg/engine/autocalibration.go`


#### `Job.autoCalibrationStrings`  <sub>(method)</sub>

```go
func (j *Job) autoCalibrationStrings() map[string][]string
```

#### `mergeMaps`  <sub>(func)</sub>

```go
func mergeMaps(m1 map[string][]string, m2 map[string][]string) map[string][]string
```

#### `Job.calibrationRequest`  <sub>(method)</sub>

```go
func (j *Job) calibrationRequest(inputs map[string][]byte) (ffuf.Response, error)
```

#### `Job.CalibrateForHost`  <sub>(method)</sub>

> CalibrateForHost runs autocalibration for a specific host

```go
func (j *Job) CalibrateForHost(host string, baseinput map[string][]byte) error
```

#### `Job.Calibrate`  <sub>(method)</sub>

> CalibrateResponses returns slice of Responses for randomly generated filter autocalibration requests

```go
func (j *Job) Calibrate(input map[string][]byte) error
```

#### `Job.CalibrateIfNeeded`  <sub>(method)</sub>

> CalibrateIfNeeded runs a self-calibration task for filtering options (if needed) by requesting random resources and  
>   
> configuring the filters accordingly

```go
func (j *Job) CalibrateIfNeeded(host string, input map[string][]byte) error
```

#### `Job.calibrateFilters`  <sub>(method)</sub>

```go
func (j *Job) calibrateFilters(responses []ffuf.Response, perHost bool) error
```

### `ffuf/pkg/engine/history.go`


#### `ConfigOptionsHistory`  <sub>(struct)</sub>

```go
type ConfigOptionsHistory struct
```

**Fields:**
```go
ffuf.ConfigOptions
Time time.Time `json:"time"`
```


#### `WriteHistoryEntry`  <sub>(func)</sub>

```go
func WriteHistoryEntry(conf *ffuf.Config) (string, error)
```

#### `historyOptions`  <sub>(func)</sub>

> historyOptions projects the Config's retained source options into the form  
> persisted for FFUFHASH history. Most fields ride the retained snapshot  
> unchanged, but fields the engine mutates AFTER parsing must be refreshed from  
> the live Config — otherwise a `-search` reconstruction shows stale values:  
> - HTTP.URL: recursion rewrites conf.Url per queued job (job.go prepareQueueJob),  
> so the frozen snapshot would report the base URL, not the recursed path.  
> - Filter/Matcher.*: autocalibration installs filters at runtime, absent from  
> the raw input. Rebuilt from MatcherManager exactly as the old ToOptions did.

```go
func historyOptions(conf *ffuf.Config) ffuf.ConfigOptions
```

#### `calculateHistoryHash`  <sub>(func)</sub>

```go
func calculateHistoryHash(options []byte) string
```

#### `SearchHash`  <sub>(func)</sub>

```go
func SearchHash(hash string) ([]ConfigOptionsHistory, int, error)
```

#### `HistoryReplayable`  <sub>(func)</sub>

```go
func HistoryReplayable(conf *ffuf.Config) (bool, string)
```

#### `configFromHistory`  <sub>(func)</sub>

```go
func configFromHistory(dirname string) (ConfigOptionsHistory, error)
```

### `ffuf/pkg/engine/job.go`


#### `Job`  <sub>(struct)</sub>

> Job ties together Config, Runner, Input and Output

```go
type Job struct
```

**Fields:**
```go
counter              int64    // only through the helper methods below, never directly.
errorCounter         int64
spuriousErrorCounter int64
count403             int64
count429             int64
running    int32    // 32-bit atomic flags (0 = false, 1 = true). Access only through the helpers.
runningJob int32
skipQueue  int32
AuditLogger  ffuf.AuditLogger
Config       *ffuf.Config
Input        ffuf.InputProvider
Runner       ffuf.RunnerProvider
ReplayRunner ffuf.RunnerProvider
Scraper      ffuf.Scraper
Output       ffuf.OutputProvider
Jobhash      string
Total        int
Rate         *RateThrottle
queue     *jobQueue
recursion *recursionManager
startTime    time.Time
startTimeJob time.Time
errorMsg     string
paused       bool
timeMutex       sync.Mutex    // guarded by pauseStateMutex
errMutex        sync.Mutex    // guards startTime and startTimeJob
inputMutex      sync.Mutex    // guards errorMsg
calibMutex      sync.Mutex    // serializes main-loop input iteration vs interactive restart Reset
pauseStateMutex sync.Mutex    // serializes autocalibration
pauseMutex      sync.RWMutex    // makes the pause-flag flip and the pauseMutex Lock/Unlock one atomic step
```


#### `QueueJob`  <sub>(struct)</sub>

```go
type QueueJob struct
```

**Fields:**
```go
Url   string
depth int
req   ffuf.Request
```


#### `jobContext`  <sub>(struct)</sub>

> jobContext carries the per-queue-job values a worker needs, passed BY VALUE so  
> no worker reads mutable Job/Config state during execution. basereq is the  
> immutable base request for the job; depth is its recursion depth. This is what  
> makes the recursion-depth and base-request reads correct by construction rather  
> than by drain timing.

```go
type jobContext struct
```

**Fields:**
```go
basereq ffuf.Request
depth   int
```


#### `NewJob`  <sub>(func)</sub>

```go
func NewJob(conf *ffuf.Config) *Job
```

#### `Job.incCounter`  <sub>(method)</sub>

```go
func (j *Job) incCounter()      { atomic.AddInt64(&j.counter, 1) }
    func (j *Job) setCounter(n int) { atomic.StoreInt64(&j.counter, int64(n)) }
    func (j *Job) getCounter() int  { return int(atomic.LoadInt64(&j.counter)) }
    
    func (j *Job) getErrorCounter() int { return int(atomic.LoadInt64(&j.errorCounter)) }
    func (j *Job) getSpuriousErrorCounter() int
```

#### `Job.getCount403`  <sub>(method)</sub>

```go
func (j *Job) getCount403() int { return int(atomic.LoadInt64(&j.count403)) }
    func (j *Job) getCount429() int { return int(atomic.LoadInt64(&j.count429)) }
    
    func boolToInt32(b bool) int32
```

#### `Job.setRunning`  <sub>(method)</sub>

```go
func (j *Job) setRunning(v bool)    { atomic.StoreInt32(&j.running, boolToInt32(v)) }
    func (j *Job) isRunning() bool      { return atomic.LoadInt32(&j.running) == 1 }
    func (j *Job) setRunningJob(v bool) { atomic.StoreInt32(&j.runningJob, boolToInt32(v)) }
    func (j *Job) isRunningJob() bool   { return atomic.LoadInt32(&j.runningJob) == 1 }
    func (j *Job) setSkipQueue(v bool)  { atomic.StoreInt32(&j.skipQueue, boolToInt32(v)) }
    func (j *Job) isSkipQueue() bool    { return atomic.LoadInt32(&j.skipQueue) == 1 }
    
    func (j *Job) setError(s string) { j.errMutex.Lock(); j.errorMsg = s; j.errMutex.Unlock() }
    func (j *Job) getError() string  { j.errMutex.Lock(); defer j.errMutex.Unlock(); return j.errorMsg }
    
    func (j *Job) setStartTime(t time.Time) { j.timeMutex.Lock(); j.startTime = t; j.timeMutex.Unlock() }
    func (j *Job) getStartTime() time.Time
```

#### `Job.setStartTimeJob`  <sub>(method)</sub>

```go
func (j *Job) setStartTimeJob(t time.Time)
```

#### `Job.getStartTimeJob`  <sub>(method)</sub>

```go
func (j *Job) getStartTimeJob() time.Time
```

#### `Job.incError`  <sub>(method)</sub>

> incError increments the error counter

```go
func (j *Job) incError()
```

#### `Job.inc403`  <sub>(method)</sub>

> inc403 increments the 403 response counter

```go
func (j *Job) inc403() { atomic.AddInt64(&j.count403, 1) }
    
    // inc429 increments the 429 response counter
    func (j *Job) inc429() { atomic.AddInt64(&j.count429, 1) }
    
    // resetSpuriousErrors resets the spurious error counter
    func (j *Job) resetSpuriousErrors() { atomic.StoreInt64(&j.spuriousErrorCounter, 0) }
    
    // DeleteQueueItem deletes a recursion job from the queue by its index in the slice
    func (j *Job) DeleteQueueItem(index int)
```

#### `Job.QueuedJobs`  <sub>(method)</sub>

> QueuedJobs returns the slice of queued recursive jobs

```go
func (j *Job) QueuedJobs() []QueueJob
```

#### `Job.Start`  <sub>(method)</sub>

> Start the execution of the Job

```go
func (j *Job) Start()
```

#### `Job.Reset`  <sub>(method)</sub>

> Reset resets the counters and wordlist position for a job

```go
func (j *Job) Reset(cycle bool)
```

#### `Job.jobsInQueue`  <sub>(method)</sub>

```go
func (j *Job) jobsInQueue() bool
```

#### `Job.prepareQueueJob`  <sub>(method)</sub>

```go
func (j *Job) prepareQueueJob() jobContext
```

#### `Job.SkipQueue`  <sub>(method)</sub>

> SkipQueue allows to skip the current job and advance to the next queued recursion job

```go
func (j *Job) SkipQueue()
```

#### `Job.sleepIfNeeded`  <sub>(method)</sub>

```go
func (j *Job) sleepIfNeeded()
```

#### `Job.Pause`  <sub>(method)</sub>

> Pause pauses the job process. The pause gate is a sync.RWMutex: while Pause  
> holds the write lock, every worker blocks at its pauseCheckpoint (an RLock).  
>   
> pauseStateMutex makes the paused-flag flip and the pauseMutex Lock/Unlock a  
> single atomic step, so there is exactly one Lock per Unlock and no goroutine  
> ever unlocks an unlocked mutex (a fatal panic), however Pause, Resume, and the  
> SIGINT handler interleave.  
>   
> Pausing is refused once the job is stopping (isRunning is false). Together with  
> Stop() calling Resume(), that guarantees the gate always ends OPEN after a Stop  
> regardless of ordering, so a Pause racing a Stop can never strand a worker at  
> the checkpoint with no Resume left to wake it.

```go
func (j *Job) Pause()
```

#### `Job.Resume`  <sub>(method)</sub>

> Resume resumes the job process

```go
func (j *Job) Resume()
```

#### `Job.pauseCheckpoint`  <sub>(method)</sub>

> pauseCheckpoint blocks while the job is paused and returns immediately  
> otherwise. Acquiring and releasing the read lock is a cheap speed bump when no  
> pause is in effect.

```go
func (j *Job) pauseCheckpoint()
```

#### `Job.startExecution`  <sub>(method)</sub>

```go
func (j *Job) startExecution(ctx jobContext)
```

#### `Job.interruptMonitor`  <sub>(method)</sub>

```go
func (j *Job) interruptMonitor()
```

#### `Job.runBackgroundTasks`  <sub>(method)</sub>

```go
func (j *Job) runBackgroundTasks(wg *sync.WaitGroup)
```

#### `Job.updateProgress`  <sub>(method)</sub>

```go
func (j *Job) updateProgress()
```

#### `Job.isMatch`  <sub>(method)</sub>

> isMatch delegates the match/filter decision to the MatcherManager seam, adapting  
> the Config fields it needs. The logic itself now lives in filter.MatcherManager.

```go
func (j *Job) isMatch(resp ffuf.Response) bool
```

#### `Job.ffufHash`  <sub>(method)</sub>

```go
func (j *Job) ffufHash(pos int) []byte
```

#### `Job.runTask`  <sub>(method)</sub>

```go
func (j *Job) runTask(ctx jobContext, input map[string][]byte, position int, retried bool)
```

#### `Job.handleScraperResult`  <sub>(method)</sub>

```go
func (j *Job) handleScraperResult(resp *ffuf.Response, sres ffuf.ScraperResult)
```

#### `Job.CheckStop`  <sub>(method)</sub>

> CheckStop stops the job if stopping conditions are met

```go
func (j *Job) CheckStop()
```

#### `Job.Stop`  <sub>(method)</sub>

> Stop the execution of the Job

```go
func (j *Job) Stop()
```

#### `Job.Next`  <sub>(method)</sub>

> Stop current, resume to next

```go
func (j *Job) Next()
```

### `ffuf/pkg/engine/jobqueue.go`


#### `jobQueue`  <sub>(struct)</sub>

> jobQueue owns the recursion/sniper job list and the active position, together  
> with the mutex that guards them. Worker goroutines append to the queue (from  
> the recursion handlers) while the main loop advances through it and the  
> progress goroutine and the interactive handler read it, so the slice and the  
> position must never be touched outside these methods. The fields are private:  
> there is no way to append without the lock.

```go
type jobQueue struct
```

**Fields:**
```go
mu   sync.Mutex
jobs []QueueJob
pos  int
```


#### `newJobQueue`  <sub>(func)</sub>

```go
func newJobQueue() *jobQueue
```

#### `jobQueue.push`  <sub>(method)</sub>

> push appends a job to the queue.

```go
func (q *jobQueue) push(j QueueJob)
```

#### `jobQueue.hasNext`  <sub>(method)</sub>

> hasNext reports whether an unprocessed job remains.

```go
func (q *jobQueue) hasNext() bool
```

#### `jobQueue.advance`  <sub>(method)</sub>

> advance returns the job at the active position and moves the cursor past it,  
> returning the new position (1-based marker matching the previous queuepos).

```go
func (q *jobQueue) advance() (QueueJob, int)
```

#### `jobQueue.position`  <sub>(method)</sub>

> position returns the active position marker.

```go
func (q *jobQueue) position() int
```

#### `jobQueue.total`  <sub>(method)</sub>

> total returns the number of jobs currently in the queue.

```go
func (q *jobQueue) total() int
```

#### `jobQueue.remaining`  <sub>(method)</sub>

> remaining returns a copy of the jobs from the active position onward.

```go
func (q *jobQueue) remaining() []QueueJob
```

#### `jobQueue.removeAt`  <sub>(method)</sub>

> removeAt deletes the queued job at the given offset from the active position  
> (the 1-based offset the interactive queuedel command uses). It returns false  
> when the offset is out of range instead of panicking.

```go
func (q *jobQueue) removeAt(offset int) bool
```

### `ffuf/pkg/engine/rate.go`


#### `RateThrottle`  <sub>(struct)</sub>

```go
type RateThrottle struct
```

**Fields:**
```go
rateCounter    *ring.Ring
Config         *ffuf.Config
RateMutex      sync.Mutex
RateLimiter    *time.Ticker
lastAdjustment time.Time
```


#### `NewRateThrottle`  <sub>(func)</sub>

```go
func NewRateThrottle(conf *ffuf.Config) *RateThrottle
```

#### `RateThrottle.CurrentRate`  <sub>(method)</sub>

> CurrentRate calculates requests/second value from circular list of rate

```go
func (r *RateThrottle) CurrentRate() int64
```

#### `RateThrottle.CurrentConfiguredRate`  <sub>(method)</sub>

> CurrentConfiguredRate returns the configured rate under the lock, for callers  
> (e.g. the interactive handler) that only need to display it.

```go
func (r *RateThrottle) CurrentConfiguredRate() int64
```

#### `RateThrottle.ChangeRate`  <sub>(method)</sub>

```go
func (r *RateThrottle) ChangeRate(rate int)
```

#### `RateThrottle.Tick`  <sub>(method)</sub>

> rateTick adds a new duration measurement tick to rate counter

```go
func (r *RateThrottle) Tick(start, end time.Time)
```

### `ffuf/pkg/engine/recursion.go`


#### `recursionManager`  <sub>(struct)</sub>

> recursionManager owns the recursion policy: deciding whether a matched response  
> should spawn a deeper queue job and enqueuing it. It was lifted out of Job so the  
> greedy/default recursion logic lives in one place and is testable without a full  
> engine. It holds only what it needs: the config (for the depth limit and to build  
> recursion requests), the queue to push onto, and the output for its messages.

```go
type recursionManager struct
```

**Fields:**
```go
config *ffuf.Config
queue  *jobQueue
output ffuf.OutputProvider
```


#### `newRecursionManager`  <sub>(func)</sub>

```go
func newRecursionManager(conf *ffuf.Config, queue *jobQueue, output ffuf.OutputProvider) *recursionManager
```

#### `recursionManager.handleGreedy`  <sub>(method)</sub>

> handleGreedy queues a recursion job for a matched response (greedy strategy: a  
> match is enough), if the recursion depth allows. The match has been determined  
> by the caller.

```go
func (r *recursionManager) handleGreedy(ctx jobContext, resp ffuf.Response)
```

#### `recursionManager.handleDefault`  <sub>(method)</sub>

> handleDefault queues a recursion job when a matched response is a directory  
> (default strategy: the response redirects to its own path with a trailing  
> slash), if the recursion depth allows.

```go
func (r *recursionManager) handleDefault(ctx jobContext, resp ffuf.Response)
```

### `ffuf/pkg/ffuf/autocalibration_strategies.go`


#### `AutocalibrationStrategy`  <sub>(type)</sub>

```go
type AutocalibrationStrategy map[string][]string
```

#### `setupDefaultAutocalibrationStrategies`  <sub>(func)</sub>

```go
func setupDefaultAutocalibrationStrategies() error
```

### `ffuf/pkg/ffuf/config.go`


#### `VarExtract`  <sub>(struct)</sub>

> VarExtract names a variable to capture from a preflight/postflight response.  
> The value is either the first capture group of Regex, run against the body and  
> then, if that finds nothing, the headers; or, when Source is set, the form  
> field, meta tag, cookie or header called Key.  
> The captured value is substituted into the keyword Name wherever it appears in  
> later requests.

```go
type VarExtract struct
```

**Fields:**
```go
Name   string `json:"name" toml:"name"`
Regex  string `json:"regex" toml:"regex"`
Source string `json:"source,omitempty" toml:"source,omitempty"`
Key    string `json:"key,omitempty" toml:"key,omitempty"`
Compiled *regexp.Regexp `json:"-" toml:"-"`    // path never recompiles per request; nil when a VarExtract is built directly.
```


#### `PreflightConfig`  <sub>(struct)</sub>

> PreflightConfig is one raw HTTP request file executed around the fuzzing  
> request, with optional variable extractions from its response.

```go
type PreflightConfig struct
```

**Fields:**
```go
RequestFile string       `json:"request_file" toml:"request_file"`
Vars        []VarExtract `json:"vars" toml:"vars"`
```


#### `Config`  <sub>(struct)</sub>

```go
type Config struct
```

**Fields:**
```go
AuditLog                  string                `json:"auditlog"`
AutoCalibration           bool                  `json:"autocalibration"`
AutoCalibrationKeyword    string                `json:"autocalibration_keyword"`
AutoCalibrationPerHost    bool                  `json:"autocalibration_perhost"`
AutoCalibrationStrategies []string              `json:"autocalibration_strategies"`
AutoCalibrationStrings    []string              `json:"autocalibration_strings"`
Cancel                    context.CancelFunc    `json:"-"`
Colors                    bool                  `json:"colors"`
CommandKeywords           []string              `json:"-"`
CommandLine               string                `json:"cmdline"`
ConfigFile                string                `json:"configfile"`
Context                   context.Context       `json:"-"`
Data                      string                `json:"postdata"`
Debuglog                  string                `json:"debuglog"`
Delay                     optRange              `json:"delay"`
DirSearchCompat           bool                  `json:"dirsearch_compatibility"`
Encoders                  []string              `json:"encoders"`
Extensions                []string              `json:"extensions"`
FilterMode                string                `json:"fmode"`
FollowRedirects           bool                  `json:"follow_redirects"`
Headers                   map[string]string     `json:"headers"`
IgnoreBody                bool                  `json:"ignorebody"`
IgnoreWordlistComments    bool                  `json:"ignore_wordlist_comments"`
InputMode                 string                `json:"inputmode"`
InputNum                  int                   `json:"cmd_inputnum"`
InputProviders            []InputProviderConfig `json:"inputproviders"`
InputShell                string                `json:"inputshell"`
Json                      bool                  `json:"json"`
MatcherManager            MatcherManager        `json:"matchers"`
MatcherMode               string                `json:"mmode"`
MaxTime                   int                   `json:"maxtime"`
MaxTimeJob                int                   `json:"maxtime_job"`
Method                    string                `json:"method"`
Noninteractive            bool                  `json:"noninteractive"`
OutputDirectory           string                `json:"outputdirectory"`
OutputFile                string                `json:"outputfile"`
OutputFormat              string                `json:"outputformat"`
OutputSkipEmptyFile       bool                  `json:"OutputSkipEmptyFile"`
ProgressFrequency         int                   `json:"-"`
ProxyURL                  string                `json:"proxyurl"`
Quiet                     bool                  `json:"quiet"`
Rate                      int64                 `json:"rate"`
Raw                       bool                  `json:"raw"`
Recursion                 bool                  `json:"recursion"`
RecursionDepth            int                   `json:"recursion_depth"`
RecursionStrategy         string                `json:"recursion_strategy"`
ReplayProxyURL            string                `json:"replayproxyurl"`
RequestFile               string                `json:"requestfile"`
RequestProto              string                `json:"requestproto"`
ScraperFile               string                `json:"scraperfile"`
Scrapers                  string                `json:"scrapers"`
SNI                       string                `json:"sni"`
StopOn403                 bool                  `json:"stop_403"`
StopOnAll                 bool                  `json:"stop_all"`
StopOnErrors              bool                  `json:"stop_errors"`
Threads                   int                   `json:"threads"`
Timeout                   int                   `json:"timeout"`
Url                       string                `json:"url"`
Verbose                   bool                  `json:"verbose"`
Wordlists                 []string              `json:"wordlists"`
Http2                     bool                  `json:"http2"`
ClientCert                string                `json:"client-cert"`
ClientKey                 string                `json:"client-key"`
Preflights                []PreflightConfig     `json:"preflights"`
Postflights               []PreflightConfig     `json:"postflights"`
PreflightMode             string                `json:"preflight_mode"`
PreflightError            string                `json:"preflight_error"`
PreflightAnyHost          bool                  `json:"preflight_anyhost"`
RateLimitFunc func() `json:"-" toml:"-"`    // outside the dispatch loop) also honor -rate and -p. Nil means unmetered.
Options *ConfigOptions `json:"-"`    // nil for a Config assembled by other means.
```


#### `InputProviderConfig`  <sub>(struct)</sub>

```go
type InputProviderConfig struct
```

**Fields:**
```go
Name     string `json:"name"`
Keyword  string `json:"keyword"`
Value    string `json:"value"`
Encoders string `json:"encoders"`
Template string `json:"template"`
```


#### `NewConfig`  <sub>(func)</sub>

> NewConfig returns a Config ready for ConfigFromOptions to populate. It sets ONLY  
> what ConfigFromOptions does not: the context, non-nil slice/map fields (which are  
> appended to or indexed during parsing), and the few defaults for fields that are  
> not copied from the options. Every other default lives once in NewConfigOptions  
> and flows in through ConfigFromOptions — listing it here too would just duplicate  
> that single source.

```go
func NewConfig(ctx context.Context, cancel context.CancelFunc) Config
```

#### `Config.SetContext`  <sub>(method)</sub>

```go
func (c *Config) SetContext(ctx context.Context, cancel context.CancelFunc)
```

### `ffuf/pkg/ffuf/flags.go`


#### `isKnownSection`  <sub>(func)</sub>

> isKnownSection reports whether s is one of the defined help sections.

```go
func isKnownSection(s string) bool
```

#### `RegisteredFlag`  <sub>(struct)</sub>

> RegisteredFlag is one flag's help metadata.

```go
type RegisteredFlag struct
```

**Fields:**
```go
Name    string
Section string
Hidden  bool
```


#### `FlagRegistry`  <sub>(struct)</sub>

> FlagRegistry is the single source of truth describing every CLI flag, used to  
> drive the segmented help output. It is produced by RegisterFlags.

```go
type FlagRegistry struct
```

**Fields:**
```go
byName map[string]RegisteredFlag
```


#### `FlagRegistry.SectionOf`  <sub>(method)</sub>

> SectionOf returns the help section a flag belongs to.

```go
func (r *FlagRegistry) SectionOf(name string) (string, bool)
```

#### `multiStringValue`  <sub>(struct)</sub>

> multiStringValue appends each occurrence verbatim (e.g. -H "A: b" -H "C: d").

```go
type multiStringValue struct{ p *[]string }
```

#### `multiStringValue.String`  <sub>(method)</sub>

```go
func (m *multiStringValue) String() string { return "" }
    func (m *multiStringValue) Set(v string) error
```

#### `wordlistValue`  <sub>(struct)</sub>

> wordlistValue appends, comma-splitting a single occurrence (e.g. -w a.txt,b.txt).

```go
type wordlistValue struct{ p *[]string }
```

#### `wordlistValue.String`  <sub>(method)</sub>

```go
func (w *wordlistValue) String() string { return "" }
    func (w *wordlistValue) Set(v string) error
```

#### `csvReplaceValue`  <sub>(struct)</sub>

> csvReplaceValue drops any default on the first occurrence, then appends  
> comma-split values. Matches the legacy -acs semantics (replace, not append).

```go
type csvReplaceValue struct
```

**Fields:**
```go
p   *[]string
set bool
```


#### `csvReplaceValue.String`  <sub>(method)</sub>

```go
func (c *csvReplaceValue) String() string { return "" }
    func (c *csvReplaceValue) Set(v string) error
```

#### `extraFlag`  <sub>(struct)</sub>

> extraFlag is the explicit supplement for flags that can't be expressed as a  
> plain tagged field: cross-section aliases, dummy compatibility flags, and  
> flags whose usage text contains a backtick (illegal inside a struct tag).

```go
type extraFlag struct
```

**Fields:**
```go
Name    string
Section string
Hidden  bool
Bind    func(fs *flag.FlagSet, o *ConfigOptions)
```


#### `appendFlightVarAuto`  <sub>(func)</sub>

> appendFlightVarAuto attaches a "NAME:key" or "NAME:[source]key" extraction to  
> the last entry in a preflight/postflight chain. The source tag lives only on  
> this flag: "[header]X" is a valid regex, so -preflight-var can't take it.

```go
func appendFlightVarAuto(chain *[]PreflightConfig, varFlag, fileFlag, spec string) error
```

#### `ParseVarSelector`  <sub>(func)</sub>

> ParseVarSelector splits "[source]key" into its source and key, or returns  
> VarSourceAuto for a bare key. The tag is a prefix because field names can end  
> in brackets (PHP's "data[_Token][key]") and contain colons (JSF's  
> "loginForm:token"); only the first "]" closes the tag.

```go
func ParseVarSelector(sel string) (source, key string, err error)
```

#### `ValidateVarSource`  <sub>(func)</sub>

> ValidateVarSource checks a source/key pair, whether it came from the CLI or a  
> config file.

```go
func ValidateVarSource(source, key string) error
```

#### `appendFlightVar`  <sub>(func)</sub>

> appendFlightVar attaches a "NAME:regex" extraction to the last entry in a  
> preflight/postflight chain. It errors (so flag.Parse surfaces it) when there is  
> no preceding file flag or the spec is malformed, rather than silently dropping.

```go
func appendFlightVar(chain *[]PreflightConfig, varFlag, fileFlag, spec string) error
```

#### `parseVarSpec`  <sub>(func)</sub>

> parseVarSpec splits a "NAME:regex" spec on the FIRST colon, so the regex may  
> itself contain colons. Both the name and the regex must be non-empty.

```go
func parseVarSpec(spec string) (name, regex string, ok bool)
```

#### `RegisterFlags`  <sub>(func)</sub>

> RegisterFlags is the single source of truth for CLI flag registration. It walks  
> the `ffuf:`-tagged fields of ConfigOptions via reflection and registers each on  
> fs — using the field's CURRENT value as the default, which preserves the  
> config-file < command-line precedence — then applies the explicit supplement.  
> It returns a FlagRegistry describing every flag for the segmented help.

```go
func RegisterFlags(fs *flag.FlagSet, o *ConfigOptions) *FlagRegistry
```

#### `bindField`  <sub>(func)</sub>

> bindField registers one reflected field pointer as a flag of the matching type.  
> It panics on an unsupported type or a slice field without a valid kind — those  
> are programming errors caught by the registration test, not runtime conditions.

```go
func bindField(fs *flag.FlagSet, name, usage, kind string, ptr interface{})
```

#### `isVarSource`  <sub>(func)</sub>

```go
func isVarSource(source string) bool
```

### `ffuf/pkg/ffuf/interfaces.go`


#### `MatcherManager`  <sub>(interface)</sub>

> MatcherManager provides functions for managing matchers and filters

```go
type MatcherManager interface
```

**Members/Variants:**
```go
SetCalibrated(calibrated bool)
SetCalibratedForHost(host string, calibrated bool)
AddFilter(name string, option string, replace bool) error
AddPerDomainFilter(domain string, name string, option string) error
RemoveFilter(name string)
AddMatcher(name string, option string) error
GetFilters() map[string]FilterProvider
GetMatchers() map[string]FilterProvider
FiltersForDomain(domain string) map[string]FilterProvider
CalibratedForDomain(domain string) bool
Calibrated() bool
Matches(resp *Response, perHost bool, matcherMode string, filterMode string) bool    // and/or combination modes.
```


#### `FilterProvider`  <sub>(interface)</sub>

> FilterProvider is a generic interface for both Matchers and Filters

```go
type FilterProvider interface
```

**Members/Variants:**
```go
Filter(response *Response) (bool, error)
Repr() string
ReprVerbose() string
```


#### `RunnerProvider`  <sub>(interface)</sub>

> RunnerProvider is an interface for request executors

```go
type RunnerProvider interface
```

**Members/Variants:**
```go
Prepare(input map[string][]byte, basereq *Request) (Request, error)
Execute(req *Request) (Response, error)
Dump(req *Request) ([]byte, error)
```


#### `InputProvider`  <sub>(interface)</sub>

> InputProvider interface handles the input data for RunnerProvider

```go
type InputProvider interface
```

**Members/Variants:**
```go
ActivateKeywords([]string)
AddProvider(InputProviderConfig) error
Keywords() []string
Next() bool
Position() int
SetPosition(int)
Reset()
Value() map[string][]byte
Total() int
```


#### `InternalInputProvider`  <sub>(interface)</sub>

> InternalInputProvider interface handles providing input data to InputProvider

```go
type InternalInputProvider interface
```

**Members/Variants:**
```go
Keyword() string
Next() bool
Position() int
SetPosition(int)
ResetPosition()
IncrementPosition()
Value() []byte
Total() int
Active() bool
Enable()
Disable()
```


#### `OutputProvider`  <sub>(interface)</sub>

> OutputProvider is responsible of providing output from the RunnerProvider

```go
type OutputProvider interface
```

**Members/Variants:**
```go
Banner()
Finalize() error
Progress(status Progress)
Info(infostring string)
Error(errstring string)
Raw(output string)
Warning(warnstring string)
Result(resp Response)
PrintResult(res Result)
SaveFile(filename, format string) error
GetCurrentResults() []Result
SetCurrentResults(results []Result)
FilterCurrentResults(keep func(Result) bool)
SetPaused(paused bool)
PendingResults() int
Reset()
Cycle()
```


#### `AuditLogger`  <sub>(interface)</sub>

> AuditLogger is responsible for providing auditing output of every request/response  
> sent and recieved by FFUF

```go
type AuditLogger interface
```

**Members/Variants:**
```go
Close()
Write(data interface{}) error
```


#### `Scraper`  <sub>(interface)</sub>

```go
type Scraper interface
```

**Members/Variants:**
```go
Execute(resp *Response, matched bool) []ScraperResult
AppendFromFile(path string) error
```


#### `ScraperResult`  <sub>(struct)</sub>

```go
type ScraperResult struct
```

**Fields:**
```go
Name    string   `json:"name"`
Type    string   `json:"type"`
Action  []string `json:"action"`
Results []string `json:"results"`
```


#### `Result`  <sub>(struct)</sub>

```go
type Result struct
```

**Fields:**
```go
Input            map[string][]byte   `json:"input"`
Position         int                 `json:"position"`
StatusCode       int64               `json:"status"`
ContentLength    int64               `json:"length"`
ContentWords     int64               `json:"words"`
ContentLines     int64               `json:"lines"`
ContentType      string              `json:"content-type"`
RedirectLocation string              `json:"redirectlocation"`
Url              string              `json:"url"`
Duration         time.Duration       `json:"duration"`
ScraperData      map[string][]string `json:"scraper"`
ResultFile       string              `json:"resultfile"`
Host             string              `json:"host"`
HTMLColor        string              `json:"-"`
Printed bool `json:"-"`    // count. Not serialized to any output format.
```


### `ffuf/pkg/ffuf/multierror.go`


#### `Multierror`  <sub>(struct)</sub>

```go
type Multierror struct
```

**Fields:**
```go
errors []error
```


#### `NewMultierror`  <sub>(func)</sub>

> NewMultierror returns a new Multierror

```go
func NewMultierror() Multierror
```

#### `Multierror.Add`  <sub>(method)</sub>

```go
func (m *Multierror) Add(err error)
```

#### `Multierror.ErrorOrNil`  <sub>(method)</sub>

```go
func (m *Multierror) ErrorOrNil() error
```

### `ffuf/pkg/ffuf/optionsparser.go`


#### `ConfigOptions`  <sub>(struct)</sub>

```go
type ConfigOptions struct
```

**Fields:**
```go
Filter  FilterOptions  `json:"filters"`
General GeneralOptions `json:"general"`
HTTP    HTTPOptions    `json:"http"`
Input   InputOptions   `json:"input"`
Matcher MatcherOptions `json:"matchers"`
Output  OutputOptions  `json:"output"`
```


#### `HTTPOptions`  <sub>(struct)</sub>

> The `ffuf`, `section`, `usage`, `kind` and `alias` struct tags are the single  
> source of truth for CLI flags (see flags.go / RegisterFlags). A field with an  
> `ffuf` tag becomes a flag; the help section, usage text, (for []string) the value  
> kind, and any compatibility aliases come from the tags. The only flags NOT backed  
> by a field are the dummy `copy as curl` compat flags (-i/-k/-compressed), declared  
> explicitly in flags.go because they have no value to bind.

```go
type HTTPOptions struct
```

**Fields:**
```go
Cookies           []string `json:"-" ffuf:"b" alias:"cookie" kind:"multistring" section:"http" usage:"Cookie data \"NAME1=VALUE1; NAME2=VALUE2\" for copy as curl functionality."`
Data              string   `json:"data" ffuf:"d" alias:"data,data-ascii,data-binary" section:"http" usage:"POST data"`
FollowRedirects   bool     `json:"follow_redirects" ffuf:"r" section:"http" usage:"Follow redirects"`
Headers           []string `json:"headers" ffuf:"H" kind:"multistring" section:"http" usage:"Header \"Name: Value\", separated by colon. Multiple -H flags are accepted."`
IgnoreBody        bool     `json:"ignore_body" ffuf:"ignore-body" section:"http" usage:"Do not fetch the response content."`
Method            string   `json:"method" ffuf:"X" section:"http" usage:"HTTP method to use"`
ProxyURL          string   `json:"proxy_url" ffuf:"x" section:"http" usage:"Proxy URL (SOCKS5 or HTTP). For example: http:
Raw               bool     `json:"raw" ffuf:"raw" section:"http" usage:"Do not encode URI"`    // 127.0.0.1:8080 or socks5://127.0.0.1:8080"`
Recursion         bool     `json:"recursion" ffuf:"recursion" section:"http" usage:"Scan recursively. Only FUZZ keyword is supported, and URL (-u) has to end in it."`
RecursionDepth    int      `json:"recursion_depth" ffuf:"recursion-depth" section:"http" usage:"Maximum recursion depth."`
RecursionStrategy string   `json:"recursion_strategy" ffuf:"recursion-strategy" section:"http" usage:"Recursion strategy: \"default\" for a redirect based, and \"greedy\" to recurse on all matches"`
ReplayProxyURL    string   `json:"replay_proxy_url" ffuf:"replay-proxy" section:"http" usage:"Replay matched requests using this proxy."`
SNI               string   `json:"sni" ffuf:"sni" section:"http" usage:"Target TLS SNI, does not support FUZZ keyword"`
Timeout           int      `json:"timeout" ffuf:"timeout" section:"http" usage:"HTTP request timeout in seconds."`
URL               string   `json:"url" ffuf:"u" section:"http" usage:"Target URL"`
Http2             bool     `json:"http2" ffuf:"http2" section:"http" usage:"Use HTTP2 protocol"`
ClientCert        string   `json:"client-cert" ffuf:"cc" section:"http" usage:"Client cert for authentication. Client key needs to be defined as well for this to work"`
ClientKey         string   `json:"client-key" ffuf:"ck" section:"http" usage:"Client key for authentication. Client certificate needs to be defined as well for this to work"`
Preflights       []PreflightConfig `json:"preflights" toml:"preflights"`    // tagged field. The json/toml tags carry them through config files and history.
Postflights      []PreflightConfig `json:"postflights" toml:"postflights"`
PreflightMode    string            `json:"preflight_mode" toml:"preflight_mode" ffuf:"preflight-mode" section:"http" usage:"Preflight execution mode: \"per-request\" or \"per-thread\""`
PreflightError   string            `json:"preflight_error" toml:"preflight_error" ffuf:"preflight-error" section:"http" usage:"Preflight error handling: \"abort\" or \"ignore\""`
PreflightAnyHost bool              `json:"preflight_anyhost" toml:"preflight_anyhost" ffuf:"preflight-anyhost" section:"http" usage:"Allow a -preflight-var value to change a flight request's scheme, host or port. Off by default: those values come from the scanned target."`
```


#### `GeneralOptions`  <sub>(struct)</sub>

```go
type GeneralOptions struct
```

**Fields:**
```go
AutoCalibration           bool     `json:"autocalibration" ffuf:"ac" section:"general" usage:"Automatically calibrate filtering options"`
AutoCalibrationKeyword    string   `json:"autocalibration_keyword" ffuf:"ack" section:"general" usage:"Autocalibration keyword"`
AutoCalibrationPerHost    bool     `json:"autocalibration_per_host" ffuf:"ach" section:"general" usage:"Per host autocalibration"`
AutoCalibrationStrategies []string `json:"autocalibration_strategies" ffuf:"acs" kind:"csvreplace" section:"general" usage:"Custom auto-calibration strategies. Can be used multiple times. Implies -ac"`
AutoCalibrationStrings    []string `json:"autocalibration_strings" ffuf:"acc" kind:"multistring" section:"general" usage:"Custom auto-calibration string. Can be used multiple times. Implies -ac"`
Colors                    bool     `json:"colors" ffuf:"c" section:"general" usage:"Colorize output."`
ConfigFile                string   `toml:"-" json:"config_file" ffuf:"config" section:"general" usage:"Load configuration from a file"`
Delay                     string   `json:"delay" ffuf:"p" section:"general" usage:"Seconds of delay between requests, or a range of random delay. For example \"0.1\" or \"0.1-2.0\""`
Json                      bool     `json:"json" ffuf:"json" section:"general" usage:"JSON output, printing newline-delimited JSON records"`
MaxTime                   int      `json:"maxtime" ffuf:"maxtime" section:"general" usage:"Maximum running time in seconds for entire process."`
MaxTimeJob                int      `json:"maxtime_job" ffuf:"maxtime-job" section:"general" usage:"Maximum running time in seconds per job."`
Noninteractive            bool     `json:"noninteractive" ffuf:"noninteractive" section:"general" usage:"Disable the interactive console functionality"`
Quiet                     bool     `json:"quiet" ffuf:"s" section:"general" usage:"Do not print additional information (silent mode)"`
Rate                      int      `json:"rate" ffuf:"rate" section:"general" usage:"Rate of requests per second"`
ScraperFile               string   `json:"scraperfile" ffuf:"scraperfile" section:"general" usage:"Custom scraper file path"`
Scrapers                  string   `json:"scrapers" ffuf:"scrapers" section:"general" usage:"Active scraper groups"`
Searchhash                string   `json:"-" ffuf:"search" section:"general" usage:"Search for a FFUFHASH payload from ffuf history"`
ShowVersion               bool     `toml:"-" json:"-" ffuf:"V" section:"general" usage:"Show version information."`
StopOn403                 bool     `json:"stop_on_403" ffuf:"sf" section:"general" usage:"Stop when > 95% of responses return 403 Forbidden"`
StopOnAll                 bool     `json:"stop_on_all" ffuf:"sa" section:"general" usage:"Stop on all error cases. Implies -sf and -se."`
StopOnErrors              bool     `json:"stop_on_errors" ffuf:"se" section:"general" usage:"Stop on spurious errors"`
Threads                   int      `json:"threads" ffuf:"t" section:"general" usage:"Number of concurrent threads."`
Verbose                   bool     `json:"verbose" ffuf:"v" section:"general" usage:"Verbose output, printing full URL and redirect location (if any) with the results."`
```


#### `InputOptions`  <sub>(struct)</sub>

```go
type InputOptions struct
```

**Fields:**
```go
DirSearchCompat        bool     `json:"dirsearch_compat" ffuf:"D" section:"input" usage:"DirSearch wordlist compatibility mode. Used in conjunction with -e flag."`
Encoders               []string `json:"encoders" ffuf:"enc" kind:"wordlist" section:"input" usage:"Encoders for keywords, eg. 'FUZZ:urlencode b64encode'"`
Extensions             string   `json:"extensions" ffuf:"e" section:"input" usage:"Comma separated list of extensions. Extends FUZZ keyword."`
IgnoreWordlistComments bool     `json:"ignore_wordlist_comments" ffuf:"ic" section:"input" usage:"Ignore wordlist comments"`
InputMode              string   `json:"input_mode" ffuf:"mode" section:"input" usage:"Multi-wordlist operation mode. Available modes: clusterbomb, pitchfork, sniper"`
InputNum               int      `json:"input_num" ffuf:"input-num" section:"input" usage:"Number of inputs to test. Used in conjunction with --input-cmd."`
InputShell             string   `json:"input_shell" ffuf:"input-shell" section:"input" usage:"Shell to be used for running command"`
Inputcommands          []string `json:"input_commands" ffuf:"input-cmd" kind:"multistring" section:"input" usage:"Command producing the input. --input-num is required when using this input method. Overrides -w."`
Request                string   `json:"request_file" ffuf:"request" section:"input" usage:"File containing the raw http request"`
RequestProto           string   `json:"request_proto" ffuf:"request-proto" section:"input" usage:"Protocol to use along with raw request"`
Wordlists              []string `json:"wordlists" ffuf:"w" kind:"wordlist" section:"input" usage:"Wordlist file path and (optional) keyword separated by colon. eg. '/path/to/wordlist:KEYWORD'"`
```


#### `OutputOptions`  <sub>(struct)</sub>

```go
type OutputOptions struct
```

**Fields:**
```go
AuditLog            string `json:"audit_log" ffuf:"audit-log" section:"output" usage:"Write audit log containing all requests, responses and config"`
DebugLog            string `json:"debug_log" ffuf:"debug-log" section:"output" usage:"Write all of the internal logging to the specified file."`
OutputDirectory     string `json:"output_directory" ffuf:"od" section:"output" usage:"Directory path to store matched results to."`
OutputFile          string `json:"output_file" ffuf:"o" section:"output" usage:"Write output to file"`
OutputFormat        string `json:"output_format" ffuf:"of" section:"output" usage:"Output file format. Available formats: json, ejson, html, md, csv, ecsv (or, 'all' for all formats)"`
OutputSkipEmptyFile bool   `json:"output_skip_empty" ffuf:"or" section:"output" usage:"Don't create the output file if we don't have results"`
```


#### `FilterOptions`  <sub>(struct)</sub>

```go
type FilterOptions struct
```

**Fields:**
```go
Mode   string `json:"mode" ffuf:"fmode" section:"filter" usage:"Filter set operator. Either of: and, or"`
Lines  string `json:"lines" ffuf:"fl" section:"filter" usage:"Filter by amount of lines in response. Comma separated list of line counts and ranges"`
Regexp string `json:"regexp" ffuf:"fr" section:"filter" usage:"Filter regexp"`
Size   string `json:"size" ffuf:"fs" section:"filter" usage:"Filter HTTP response size. Comma separated list of sizes and ranges"`
Status string `json:"status" ffuf:"fc" section:"filter" usage:"Filter HTTP status codes from response. Comma separated list of codes and ranges"`
Time   string `json:"time" ffuf:"ft" section:"filter" usage:"Filter by number of milliseconds to the first response byte, either greater or less than. EG: >100 or <100"`
Words  string `json:"words" ffuf:"fw" section:"filter" usage:"Filter by amount of words in response. Comma separated list of word counts and ranges"`
```


#### `MatcherOptions`  <sub>(struct)</sub>

```go
type MatcherOptions struct
```

**Fields:**
```go
Mode   string `json:"mode" ffuf:"mmode" section:"matcher" usage:"Matcher set operator. Either of: and, or"`
Lines  string `json:"lines" ffuf:"ml" section:"matcher" usage:"Match amount of lines in response"`
Regexp string `json:"regexp" ffuf:"mr" section:"matcher" usage:"Match regexp"`
Size   string `json:"size" ffuf:"ms" section:"matcher" usage:"Match HTTP response size"`
Status string `json:"status" ffuf:"mc" section:"matcher" usage:"Match HTTP status codes, or \"all\" for everything."`
Time   string `json:"time" ffuf:"mt" section:"matcher" usage:"Match how many milliseconds to the first response byte, either greater or less than. EG: >100 or <100"`
Words  string `json:"words" ffuf:"mw" section:"matcher" usage:"Match amount of words in response"`
```


#### `NewConfigOptions`  <sub>(func)</sub>

> NewConfigOptions returns a newly created ConfigOptions struct with default values

```go
func NewConfigOptions() *ConfigOptions
```

#### `cloneStrings`  <sub>(func)</sub>

> cloneStrings returns a copy of s with its own backing array (nil stays nil), so  
> the retained options snapshot can't be mutated through the caller's slices.

```go
func cloneStrings(s []string) []string
```

#### `clonePreflights`  <sub>(func)</sub>

> clonePreflights deep-copies a preflight/postflight slice, including each entry's  
> Vars slice, so the retained options snapshot shares no backing array with the  
> caller's options (matching the cloneStrings invariant for the other slice fields).

```go
func clonePreflights(in []PreflightConfig) []PreflightConfig
```

#### `ConfigFromOptions`  <sub>(func)</sub>

> ConfigFromOptions parses the values in ConfigOptions struct, ensures that the values are sane,  
> and creates a Config struct out of them.

```go
func ConfigFromOptions(parseOpts *ConfigOptions, ctx context.Context, cancel context.CancelFunc) (*Config, error)
```

#### `parseRawRequest`  <sub>(func)</sub>

```go
func parseRawRequest(parseOpts *ConfigOptions, conf *Config) error
```

#### `keywordPresent`  <sub>(func)</sub>

```go
func keywordPresent(keyword string, conf *Config) bool
```

#### `templatePresent`  <sub>(func)</sub>

```go
func templatePresent(template string, conf *Config) bool
```

#### `ReadConfig`  <sub>(func)</sub>

```go
func ReadConfig(configFile string) (*ConfigOptions, error)
```

#### `ReadDefaultConfig`  <sub>(func)</sub>

```go
func ReadDefaultConfig() (*ConfigOptions, error)
```

### `ffuf/pkg/ffuf/optrange.go`


#### `optRange`  <sub>(struct)</sub>

> optRange stores either a single float, in which case the value is stored in min and IsRange is false,  
> or a range of floats, in which case IsRange is true

```go
type optRange struct
```

**Fields:**
```go
Min      float64
Max      float64
IsRange  bool
HasDelay bool
```


#### `optRangeJSON`  <sub>(struct)</sub>

```go
type optRangeJSON struct
```

**Fields:**
```go
Value string `json:"value"`
```


#### `optRange.MarshalJSON`  <sub>(method)</sub>

```go
func (o *optRange) MarshalJSON() ([]byte, error)
```

#### `optRange.UnmarshalJSON`  <sub>(method)</sub>

```go
func (o *optRange) UnmarshalJSON(b []byte) error
```

#### `optRange.Initialize`  <sub>(method)</sub>

> Initialize sets up the optRange from string value

```go
func (o *optRange) Initialize(value string) error
```

### `ffuf/pkg/ffuf/progress.go`


#### `Progress`  <sub>(struct)</sub>

```go
type Progress struct
```

**Fields:**
```go
StartedAt  time.Time
ReqCount   int
ReqTotal   int
ReqSec     int64
QueuePos   int
QueueTotal int
ErrorCount int
```


### `ffuf/pkg/ffuf/request.go`


#### `Request`  <sub>(struct)</sub>

> Request holds the meaningful data that is passed for runner for making the query

```go
type Request struct
```

**Fields:**
```go
Method    string
Host      string
Url       string
Headers   map[string]string
Data      []byte
Input     map[string][]byte
Position  int
Raw       string
Error     string
Timestamp time.Time
```


#### `NewRequest`  <sub>(func)</sub>

```go
func NewRequest(conf *Config) Request
```

#### `BaseRequest`  <sub>(func)</sub>

> BaseRequest returns a base request struct populated from the main config

```go
func BaseRequest(conf *Config) Request
```

#### `RecursionRequest`  <sub>(func)</sub>

> RecursionRequest returns a base request for a recursion target

```go
func RecursionRequest(conf *Config, path string) Request
```

#### `CopyRequest`  <sub>(func)</sub>

> CopyRequest performs a deep copy of a request and returns a new struct

```go
func CopyRequest(basereq *Request) Request
```

#### `SniperRequests`  <sub>(func)</sub>

> SniperRequests returns an array of requests, each with one of the templated locations replaced by a keyword

```go
func SniperRequests(basereq *Request, template string) []Request
```

#### `templateLocations`  <sub>(func)</sub>

> templateLocations returns an array of template character locations in input

```go
func templateLocations(template string, input string) []int
```

#### `injectKeyword`  <sub>(func)</sub>

> injectKeyword takes a string, a keyword, and a start/end offset. The data between  
> the start/end offset in string is removed, and replaced by keyword

```go
func injectKeyword(input string, keyword string, startOffset int, endOffset int) string
```

#### `scrubTemplates`  <sub>(func)</sub>

> scrubTemplates removes all template (§) strings from the request struct

```go
func scrubTemplates(req *Request, template string)
```

### `ffuf/pkg/ffuf/response.go`


#### `Response`  <sub>(struct)</sub>

> Response struct holds the meaningful data returned from request and is meant for passing to filters

```go
type Response struct
```

**Fields:**
```go
StatusCode    int64
Headers       map[string][]string
Data          []byte
ContentLength int64
ContentWords  int64
ContentLines  int64
ContentType   string
Cancelled     bool
Request       *Request
Raw           string
ResultFile    string
ScraperData   map[string][]string
Duration      time.Duration
Timestamp     time.Time
```


#### `Response.GetRedirectLocation`  <sub>(method)</sub>

> GetRedirectLocation returns the redirect location for a 3xx redirect HTTP response

```go
func (resp *Response) GetRedirectLocation(absolute bool) string
```

#### `UrlEqual`  <sub>(func)</sub>

```go
func UrlEqual(url1, url2 *url.URL) bool
```

#### `getUrlPort`  <sub>(func)</sub>

```go
func getUrlPort(url *url.URL) string
```

#### `NewResponse`  <sub>(func)</sub>

```go
func NewResponse(httpresp *http.Response, req *Request) Response
```

### `ffuf/pkg/ffuf/util.go`


#### `RandomString`  <sub>(func)</sub>

> RandomString returns a random string of length of parameter n

```go
func RandomString(n int) string
```

#### `UniqStringSlice`  <sub>(func)</sub>

> UniqStringSlice returns an unordered slice of unique strings. The duplicates are dropped

```go
func UniqStringSlice(inslice []string) []string
```

#### `FileExists`  <sub>(func)</sub>

> FileExists checks if the filepath exists and is not a directory.  
> Returns false in case it's not possible to describe the named file.

```go
func FileExists(path string) bool
```

#### `RequestContainsKeyword`  <sub>(func)</sub>

> RequestContainsKeyword checks if a keyword is present in any field of a request

```go
func RequestContainsKeyword(req Request, kw string) bool
```

#### `HostURLFromRequest`  <sub>(func)</sub>

> HostURLFromRequest gets a host + path without the filename or last part of the URL path

```go
func HostURLFromRequest(req Request) string
```

#### `Version`  <sub>(func)</sub>

> Version returns the ffuf version string.  
>   
> It resolves through four sources, in this order:  
>   
> 1. Release builds have VERSION injected via -ldflags by goreleaser and  
> VERSION_APPENDIX emptied, so they report a plain semantic version like  
> "2.2.0" with no manual constant bump required.  
> 2. Builds from a source checkout report a "git-<UTC date>-<short commit>"  
> identifier derived from the VCS metadata `go build` embeds, e.g.  
> "git-20260613-aabbccdd". VCS metadata takes precedence over the module  
> version below, because a working tree can sit on a tagged commit while  
> carrying uncommitted changes.  
> 3. Binaries produced by `go install github.com/ffuf/ffuf/v2@vX.Y.Z` carry no  
> VCS metadata, but the toolchain records the module version they were built  
> from, so report that.  
> 4. Failing all of those, VERSION+VERSION_APPENDIX, which is a placeholder  
> rather than a real version.

```go
func Version() string
```

#### `resolveVersion`  <sub>(func)</sub>

> resolveVersion resolves the version once and caches it. Version() sits on the  
> per-request path since it fills in the default User-Agent, and both metadata  
> lookups below parse data embedded in the binary, so resolving on every call  
> spends microseconds per request on a value that cannot change.

```go
func resolveVersion() (string, bool)
```

#### `selectVersion`  <sub>(func)</sub>

> selectVersion picks which of the available version sources to report, and  
> reports whether the result identifies a released version.  
>   
> VCS metadata deliberately outranks the module version: a working tree can sit  
> on a tagged commit while carrying uncommitted changes, and the toolchain  
> records the tag as the module version regardless, so preferring it would let a  
> dirty build claim to be a clean release.

```go
func selectVersion(injected, appendix, git, module string) (version string, released bool)
```

#### `moduleVersion`  <sub>(func)</sub>

> moduleVersion returns the version of the main module this binary was built  
> from, as recorded by `go install module@version`. It returns "" when there is  
> no real version to report, which is the case for a plain `go build` in a  
> working tree.

```go
func moduleVersion() string
```

#### `normalizeModuleVersion`  <sub>(func)</sub>

> normalizeModuleVersion strips the module version's leading "v" and rejects the  
> placeholders the toolchain uses when no version is available.

```go
func normalizeModuleVersion(v string) string
```

#### `gitVersion`  <sub>(func)</sub>

> gitVersion assembles a "git-<date>-<shorthash>" string from the VCS metadata  
> that `go build` stamps into the binary. It returns "" when the metadata is  
> missing (e.g. a module build outside of a repository).

```go
func gitVersion() string
```

#### `FormattedVersion`  <sub>(func)</sub>

> FormattedVersion returns the version prepared for display. Released versions  
> are prefixed with "v" (e.g. "v2.2.0"); development builds are returned  
> unprefixed (e.g. "git-20260613-aabbccdd") since a "v" reads as noise there.

```go
func FormattedVersion() string
```

#### `CheckOrCreateConfigDir`  <sub>(func)</sub>

```go
func CheckOrCreateConfigDir() error
```

#### `CreateConfigDir`  <sub>(func)</sub>

```go
func CreateConfigDir(path string) error
```

#### `StrInSlice`  <sub>(func)</sub>

```go
func StrInSlice(key string, slice []string) bool
```

### `ffuf/pkg/ffuf/valuerange.go`


#### `ValueRange`  <sub>(struct)</sub>

```go
type ValueRange struct
```

**Fields:**
```go
Min, Max int64
```


#### `ValueRangeFromString`  <sub>(func)</sub>

```go
func ValueRangeFromString(instr string) (ValueRange, error)
```

### `ffuf/pkg/filter/filter.go`


#### `MatcherManager`  <sub>(struct)</sub>

> MatcherManager handles both filters and matchers.  
>   
> Every access to the Matchers/Filters/PerDomainFilters maps goes through the  
> methods below under mu (an RWMutex). Readers RLock and return a *copy* of the  
> map: the request path (Job.isMatch) ranges the returned map after the method  
> has returned, so handing back the live map would let a worker iterate it while  
> autocalibration or an interactive filter command mutates it on another  
> goroutine, which the Go runtime aborts with "concurrent map read and map  
> write". Copying the map header (the FilterProvider values are immutable once  
> created) keeps the caller's iteration off the shared map.

```go
type MatcherManager struct
```

**Fields:**
```go
mu               sync.RWMutex
IsCalibrated     bool
Matchers         map[string]ffuf.FilterProvider
Filters          map[string]ffuf.FilterProvider
PerDomainFilters map[string]*PerDomainFilter
```


#### `PerDomainFilter`  <sub>(struct)</sub>

```go
type PerDomainFilter struct
```

**Fields:**
```go
IsCalibrated bool
Filters      map[string]ffuf.FilterProvider
```


#### `NewPerDomainFilter`  <sub>(func)</sub>

> NewPerDomainFilter copies the supplied (global) filter map. Storing the map by  
> reference aliased the global Filters, so a per-domain filter write mutated the  
> global filter set and leaked into every host.

```go
func NewPerDomainFilter(globfilters map[string]ffuf.FilterProvider) *PerDomainFilter
```

#### `PerDomainFilter.SetCalibrated`  <sub>(method)</sub>

```go
func (p *PerDomainFilter) SetCalibrated(value bool)
```

#### `copyFilterMap`  <sub>(func)</sub>

> copyFilterMap returns a shallow copy of a filter map. The FilterProvider values  
> are safe to share: they are replaced, never mutated, after creation.

```go
func copyFilterMap(in map[string]ffuf.FilterProvider) map[string]ffuf.FilterProvider
```

#### `NewMatcherManager`  <sub>(func)</sub>

```go
func NewMatcherManager() ffuf.MatcherManager
```

#### `MatcherManager.SetCalibrated`  <sub>(method)</sub>

```go
func (f *MatcherManager) SetCalibrated(value bool)
```

#### `MatcherManager.SetCalibratedForHost`  <sub>(method)</sub>

```go
func (f *MatcherManager) SetCalibratedForHost(host string, value bool)
```

#### `NewFilterByName`  <sub>(func)</sub>

```go
func NewFilterByName(name string, value string) (ffuf.FilterProvider, error)
```

#### `MatcherManager.AddFilter`  <sub>(method)</sub>

> AddFilter adds a new filter to MatcherManager

```go
func (f *MatcherManager) AddFilter(name string, option string, replace bool) error
```

#### `MatcherManager.AddPerDomainFilter`  <sub>(method)</sub>

> AddPerDomainFilter adds a new filter to PerDomainFilter configuration

```go
func (f *MatcherManager) AddPerDomainFilter(domain string, name string, option string) error
```

#### `MatcherManager.RemoveFilter`  <sub>(method)</sub>

> RemoveFilter removes a filter of a given type

```go
func (f *MatcherManager) RemoveFilter(name string)
```

#### `MatcherManager.AddMatcher`  <sub>(method)</sub>

> AddMatcher adds a new matcher to Config

```go
func (f *MatcherManager) AddMatcher(name string, option string) error
```

#### `MatcherManager.GetFilters`  <sub>(method)</sub>

```go
func (f *MatcherManager) GetFilters() map[string]ffuf.FilterProvider
```

#### `MatcherManager.GetMatchers`  <sub>(method)</sub>

```go
func (f *MatcherManager) GetMatchers() map[string]ffuf.FilterProvider
```

#### `MatcherManager.FiltersForDomain`  <sub>(method)</sub>

```go
func (f *MatcherManager) FiltersForDomain(domain string) map[string]ffuf.FilterProvider
```

#### `MatcherManager.CalibratedForDomain`  <sub>(method)</sub>

```go
func (f *MatcherManager) CalibratedForDomain(domain string) bool
```

#### `MatcherManager.Calibrated`  <sub>(method)</sub>

```go
func (f *MatcherManager) Calibrated() bool
```

#### `MatcherManager.Matches`  <sub>(method)</sub>

> Matches reports whether resp passes the configured matchers and filters. It is  
> the match decision that used to live in Job.isMatch, moved here so it sits with  
> the rule data it decides over and is unit-testable without an engine. perHost  
> selects the per-domain filter set (autocalibration -ach); matcherMode/filterMode  
> are the and/or combination modes. GetFilters/GetMatchers/FiltersForDomain each  
> take the read lock and return a copy, so the ranges below never touch the live  
> maps.

```go
func (f *MatcherManager) Matches(resp *ffuf.Response, perHost bool, matcherMode string, filterMode string) bool
```

### `ffuf/pkg/filter/fromconfig.go`


#### `FromConfig`  <sub>(func)</sub>

> FromConfig builds a MatcherManager from the parsed ConfigOptions. It is the pure,  
> global-free core of what main.SetupFilters used to do inline, so matcher/filter  
> setup is now unit-testable without the CLI flag package. The one CLI-explicit  
> decision - whether to install the default status matcher - needs the global flag  
> set, so it stays in main and is passed in as addDefaultStatusMatcher.

```go
func FromConfig(opts *ffuf.ConfigOptions, addDefaultStatusMatcher bool) (ffuf.MatcherManager, error)
```

### `ffuf/pkg/filter/lines.go`


#### `LineFilter`  <sub>(struct)</sub>

```go
type LineFilter struct
```

**Fields:**
```go
Value []ffuf.ValueRange
```


#### `NewLineFilter`  <sub>(func)</sub>

```go
func NewLineFilter(value string) (ffuf.FilterProvider, error)
```

#### `LineFilter.MarshalJSON`  <sub>(method)</sub>

```go
func (f *LineFilter) MarshalJSON() ([]byte, error)
```

#### `LineFilter.Filter`  <sub>(method)</sub>

```go
func (f *LineFilter) Filter(response *ffuf.Response) (bool, error)
```

#### `LineFilter.Repr`  <sub>(method)</sub>

```go
func (f *LineFilter) Repr() string
```

#### `LineFilter.ReprVerbose`  <sub>(method)</sub>

```go
func (f *LineFilter) ReprVerbose() string
```

### `ffuf/pkg/filter/regex.go`


#### `RegexpFilter`  <sub>(struct)</sub>

```go
type RegexpFilter struct
```

**Fields:**
```go
Value    *regexp.Regexp
valueRaw string
```


#### `NewRegexpFilter`  <sub>(func)</sub>

```go
func NewRegexpFilter(value string) (ffuf.FilterProvider, error)
```

#### `RegexpFilter.MarshalJSON`  <sub>(method)</sub>

```go
func (f *RegexpFilter) MarshalJSON() ([]byte, error)
```

#### `RegexpFilter.Filter`  <sub>(method)</sub>

```go
func (f *RegexpFilter) Filter(response *ffuf.Response) (bool, error)
```

#### `RegexpFilter.Repr`  <sub>(method)</sub>

```go
func (f *RegexpFilter) Repr() string
```

#### `RegexpFilter.ReprVerbose`  <sub>(method)</sub>

```go
func (f *RegexpFilter) ReprVerbose() string
```

### `ffuf/pkg/filter/size.go`


#### `SizeFilter`  <sub>(struct)</sub>

```go
type SizeFilter struct
```

**Fields:**
```go
Value []ffuf.ValueRange
```


#### `NewSizeFilter`  <sub>(func)</sub>

```go
func NewSizeFilter(value string) (ffuf.FilterProvider, error)
```

#### `SizeFilter.MarshalJSON`  <sub>(method)</sub>

```go
func (f *SizeFilter) MarshalJSON() ([]byte, error)
```

#### `SizeFilter.Filter`  <sub>(method)</sub>

```go
func (f *SizeFilter) Filter(response *ffuf.Response) (bool, error)
```

#### `SizeFilter.Repr`  <sub>(method)</sub>

```go
func (f *SizeFilter) Repr() string
```

#### `SizeFilter.ReprVerbose`  <sub>(method)</sub>

```go
func (f *SizeFilter) ReprVerbose() string
```

### `ffuf/pkg/filter/status.go`


#### `StatusFilter`  <sub>(struct)</sub>

```go
type StatusFilter struct
```

**Fields:**
```go
Value []ffuf.ValueRange
```


#### `NewStatusFilter`  <sub>(func)</sub>

```go
func NewStatusFilter(value string) (ffuf.FilterProvider, error)
```

#### `StatusFilter.MarshalJSON`  <sub>(method)</sub>

```go
func (f *StatusFilter) MarshalJSON() ([]byte, error)
```

#### `StatusFilter.Filter`  <sub>(method)</sub>

```go
func (f *StatusFilter) Filter(response *ffuf.Response) (bool, error)
```

#### `StatusFilter.Repr`  <sub>(method)</sub>

```go
func (f *StatusFilter) Repr() string
```

#### `StatusFilter.ReprVerbose`  <sub>(method)</sub>

```go
func (f *StatusFilter) ReprVerbose() string
```

### `ffuf/pkg/filter/time.go`


#### `TimeFilter`  <sub>(struct)</sub>

```go
type TimeFilter struct
```

**Fields:**
```go
ms       int64
gt       bool    // milliseconds since first response byte
lt       bool    // filter if response time is greater than
valueRaw string    // filter if response time is less than
```


#### `NewTimeFilter`  <sub>(func)</sub>

```go
func NewTimeFilter(value string) (ffuf.FilterProvider, error)
```

#### `TimeFilter.MarshalJSON`  <sub>(method)</sub>

```go
func (f *TimeFilter) MarshalJSON() ([]byte, error)
```

#### `TimeFilter.Filter`  <sub>(method)</sub>

```go
func (f *TimeFilter) Filter(response *ffuf.Response) (bool, error)
```

#### `TimeFilter.Repr`  <sub>(method)</sub>

```go
func (f *TimeFilter) Repr() string
```

#### `TimeFilter.ReprVerbose`  <sub>(method)</sub>

```go
func (f *TimeFilter) ReprVerbose() string
```

### `ffuf/pkg/filter/words.go`


#### `WordFilter`  <sub>(struct)</sub>

```go
type WordFilter struct
```

**Fields:**
```go
Value []ffuf.ValueRange
```


#### `NewWordFilter`  <sub>(func)</sub>

```go
func NewWordFilter(value string) (ffuf.FilterProvider, error)
```

#### `WordFilter.MarshalJSON`  <sub>(method)</sub>

```go
func (f *WordFilter) MarshalJSON() ([]byte, error)
```

#### `WordFilter.Filter`  <sub>(method)</sub>

```go
func (f *WordFilter) Filter(response *ffuf.Response) (bool, error)
```

#### `WordFilter.Repr`  <sub>(method)</sub>

```go
func (f *WordFilter) Repr() string
```

#### `WordFilter.ReprVerbose`  <sub>(method)</sub>

```go
func (f *WordFilter) ReprVerbose() string
```

### `ffuf/pkg/input/command.go`


#### `CommandInput`  <sub>(struct)</sub>

```go
type CommandInput struct
```

**Fields:**
```go
config  *ffuf.Config
count   int
active  bool
keyword string
command string
shell   string
```


#### `NewCommandInput`  <sub>(func)</sub>

```go
func NewCommandInput(keyword string, value string, conf *ffuf.Config) (*CommandInput, error)
```

#### `CommandInput.Keyword`  <sub>(method)</sub>

> Keyword returns the keyword assigned to this InternalInputProvider

```go
func (c *CommandInput) Keyword() string
```

#### `CommandInput.Position`  <sub>(method)</sub>

> Position will return the current position in the input list

```go
func (c *CommandInput) Position() int
```

#### `CommandInput.SetPosition`  <sub>(method)</sub>

> SetPosition will set the current position of the inputprovider

```go
func (c *CommandInput) SetPosition(pos int)
```

#### `CommandInput.ResetPosition`  <sub>(method)</sub>

> ResetPosition will reset the current position of the InternalInputProvider

```go
func (c *CommandInput) ResetPosition()
```

#### `CommandInput.IncrementPosition`  <sub>(method)</sub>

> IncrementPosition increments the current position in the inputprovider

```go
func (c *CommandInput) IncrementPosition()
```

#### `CommandInput.Next`  <sub>(method)</sub>

> Next will increment the cursor position, and return a boolean telling if there's iterations left

```go
func (c *CommandInput) Next() bool
```

#### `CommandInput.Value`  <sub>(method)</sub>

> Value returns the input from command stdoutput

```go
func (c *CommandInput) Value() []byte
```

#### `CommandInput.Total`  <sub>(method)</sub>

> Total returns the size of wordlist

```go
func (c *CommandInput) Total() int
```

#### `CommandInput.Active`  <sub>(method)</sub>

```go
func (c *CommandInput) Active() bool
```

#### `CommandInput.Enable`  <sub>(method)</sub>

```go
func (c *CommandInput) Enable()
```

#### `CommandInput.Disable`  <sub>(method)</sub>

```go
func (c *CommandInput) Disable()
```

### `ffuf/pkg/input/input.go`


#### `MainInputProvider`  <sub>(struct)</sub>

```go
type MainInputProvider struct
```

**Fields:**
```go
Providers   []ffuf.InternalInputProvider
Encoders    map[string]*pencode.Chain
Config      *ffuf.Config
position    int
msbIterator int
```


#### `NewInputProvider`  <sub>(func)</sub>

```go
func NewInputProvider(conf *ffuf.Config) (ffuf.InputProvider, ffuf.Multierror)
```

#### `MainInputProvider.AddProvider`  <sub>(method)</sub>

```go
func (i *MainInputProvider) AddProvider(provider ffuf.InputProviderConfig) error
```

#### `MainInputProvider.ActivateKeywords`  <sub>(method)</sub>

> ActivateKeywords enables / disables wordlists based on list of active keywords

```go
func (i *MainInputProvider) ActivateKeywords(kws []string)
```

#### `MainInputProvider.Position`  <sub>(method)</sub>

> Position will return the current position of progress

```go
func (i *MainInputProvider) Position() int
```

#### `MainInputProvider.SetPosition`  <sub>(method)</sub>

> SetPosition will reset the MainInputProvider to a specific position

```go
func (i *MainInputProvider) SetPosition(pos int)
```

#### `MainInputProvider.Keywords`  <sub>(method)</sub>

> Keywords returns a slice of all keywords in the inputprovider

```go
func (i *MainInputProvider) Keywords() []string
```

#### `MainInputProvider.Next`  <sub>(method)</sub>

> Next will increment the cursor position, and return a boolean telling if there's inputs left

```go
func (i *MainInputProvider) Next() bool
```

#### `MainInputProvider.Value`  <sub>(method)</sub>

> Value returns a map of inputs for keywords

```go
func (i *MainInputProvider) Value() map[string][]byte
```

#### `MainInputProvider.Reset`  <sub>(method)</sub>

> Reset resets all the inputproviders and counters

```go
func (i *MainInputProvider) Reset()
```

#### `MainInputProvider.pitchforkValue`  <sub>(method)</sub>

> pitchforkValue returns a map of keyword:value pairs including all inputs.  
> This mode will iterate through wordlists in lockstep.

```go
func (i *MainInputProvider) pitchforkValue() map[string][]byte
```

#### `MainInputProvider.setpitchforkPosition`  <sub>(method)</sub>

```go
func (i *MainInputProvider) setpitchforkPosition(pos int)
```

#### `MainInputProvider.clusterbombValue`  <sub>(method)</sub>

> clusterbombValue returns map of keyword:value pairs including all inputs.  
> this mode will iterate through all possible combinations.

```go
func (i *MainInputProvider) clusterbombValue() map[string][]byte
```

#### `MainInputProvider.setclusterbombPosition`  <sub>(method)</sub>

```go
func (i *MainInputProvider) setclusterbombPosition(pos int)
```

#### `MainInputProvider.clusterbombIteratorReset`  <sub>(method)</sub>

```go
func (i *MainInputProvider) clusterbombIteratorReset()
```

#### `MainInputProvider.Total`  <sub>(method)</sub>

> Total returns the amount of input combinations available

```go
func (i *MainInputProvider) Total() int
```

### `ffuf/pkg/input/wordlist.go`


#### `WordlistInput`  <sub>(struct)</sub>

```go
type WordlistInput struct
```

**Fields:**
```go
active   bool
config   *ffuf.Config
data     [][]byte
position int
keyword  string
```


#### `NewWordlistInput`  <sub>(func)</sub>

```go
func NewWordlistInput(keyword string, value string, conf *ffuf.Config) (*WordlistInput, error)
```

#### `WordlistInput.Position`  <sub>(method)</sub>

> Position will return the current position in the input list

```go
func (w *WordlistInput) Position() int
```

#### `WordlistInput.SetPosition`  <sub>(method)</sub>

> SetPosition sets the current position of the inputprovider

```go
func (w *WordlistInput) SetPosition(pos int)
```

#### `WordlistInput.ResetPosition`  <sub>(method)</sub>

> ResetPosition resets the position back to beginning of the wordlist.

```go
func (w *WordlistInput) ResetPosition()
```

#### `WordlistInput.Keyword`  <sub>(method)</sub>

> Keyword returns the keyword assigned to this InternalInputProvider

```go
func (w *WordlistInput) Keyword() string
```

#### `WordlistInput.Next`  <sub>(method)</sub>

> Next will return a boolean telling if there's words left in the list

```go
func (w *WordlistInput) Next() bool
```

#### `WordlistInput.IncrementPosition`  <sub>(method)</sub>

> IncrementPosition will increment the current position in the inputprovider data slice

```go
func (w *WordlistInput) IncrementPosition()
```

#### `WordlistInput.Value`  <sub>(method)</sub>

> Value returns the value from wordlist at current cursor position

```go
func (w *WordlistInput) Value() []byte
```

#### `WordlistInput.Total`  <sub>(method)</sub>

> Total returns the size of wordlist

```go
func (w *WordlistInput) Total() int
```

#### `WordlistInput.Active`  <sub>(method)</sub>

> Active returns boolean if the inputprovider is active

```go
func (w *WordlistInput) Active() bool
```

#### `WordlistInput.Enable`  <sub>(method)</sub>

> Enable sets the inputprovider as active

```go
func (w *WordlistInput) Enable()
```

#### `WordlistInput.Disable`  <sub>(method)</sub>

> Disable disables the inputprovider

```go
func (w *WordlistInput) Disable()
```

#### `WordlistInput.validFile`  <sub>(method)</sub>

> validFile checks that the wordlist file exists and can be read

```go
func (w *WordlistInput) validFile(path string) (bool, error)
```

#### `WordlistInput.readFile`  <sub>(method)</sub>

> readFile reads the file line by line to a byte slice

```go
func (w *WordlistInput) readFile(path string) error
```

#### `stripComments`  <sub>(func)</sub>

> stripComments removes all kind of comments from the word

```go
func stripComments(text string) (string, bool)
```

### `ffuf/pkg/interactive/posix.go`


#### `termHandle`  <sub>(func)</sub>

```go
func termHandle() (*os.File, error)
```

### `ffuf/pkg/interactive/termhandler.go`


#### `interactive`  <sub>(struct)</sub>

```go
type interactive struct
```

**Fields:**
```go
Job    *engine.Job
paused bool
```


#### `Handle`  <sub>(func)</sub>

```go
func Handle(job *engine.Job) error
```

#### `interactive.handleInput`  <sub>(method)</sub>

```go
func (i *interactive) handleInput(in []byte)
```

#### `resultProbe`  <sub>(func)</sub>

> resultProbe rebuilds the minimal Response needed to re-evaluate the active  
> filters against an already-collected result. ContentLines must come from the  
> result's line count, not its length: feeding ContentLength here made an  
> in-console "fl" (line count) filter match against the wrong number.

```go
func resultProbe(res ffuf.Result) *ffuf.Response
```

#### `interactive.refreshResults`  <sub>(method)</sub>

```go
func (i *interactive) refreshResults()
```

#### `interactive.updateFilter`  <sub>(method)</sub>

```go
func (i *interactive) updateFilter(name, value string, replace bool)
```

#### `interactive.appendFilter`  <sub>(method)</sub>

```go
func (i *interactive) appendFilter(name, value string)
```

#### `interactive.printQueue`  <sub>(method)</sub>

```go
func (i *interactive) printQueue()
```

#### `interactive.deleteQueue`  <sub>(method)</sub>

```go
func (i *interactive) deleteQueue(in string)
```

#### `interactive.resumeJob`  <sub>(method)</sub>

> resumeJob restores live result printing and resumes the job. If any matches  
> arrived while the console was open, it reports the count first so the user  
> knows there are new (recorded but unprinted) findings to inspect with "show".

```go
func (i *interactive) resumeJob()
```

#### `interactive.printBanner`  <sub>(method)</sub>

```go
func (i *interactive) printBanner()
```

#### `interactive.printPrompt`  <sub>(method)</sub>

```go
func (i *interactive) printPrompt()
```

#### `interactive.printHelp`  <sub>(method)</sub>

```go
func (i *interactive) printHelp()
```

### `ffuf/pkg/interactive/windows.go`


#### `termHandle`  <sub>(func)</sub>

```go
func termHandle() (*os.File, error)
```

### `ffuf/pkg/output/audit.go`


#### `AuditLogger`  <sub>(struct)</sub>

```go
type AuditLogger struct
```

**Fields:**
```go
file *os.File
lock sync.Mutex
```


#### `NewAuditLogger`  <sub>(func)</sub>

```go
func NewAuditLogger(filename string) (*AuditLogger, error)
```

#### `AuditLogger.Close`  <sub>(method)</sub>

```go
func (logger *AuditLogger) Close()
```

#### `AuditLogger.Write`  <sub>(method)</sub>

```go
func (logger *AuditLogger) Write(data interface{}) error
```

### `ffuf/pkg/output/file_csv.go`


#### `writeCSV`  <sub>(func)</sub>

```go
func writeCSV(filename string, config *ffuf.Config, res []ffuf.Result, encode bool) error
```

#### `base64encode`  <sub>(func)</sub>

```go
func base64encode(in []byte) string
```

#### `toCSV`  <sub>(func)</sub>

```go
func toCSV(r ffuf.Result) []string
```

### `ffuf/pkg/output/file_html.go`


#### `htmlResult`  <sub>(struct)</sub>

```go
type htmlResult struct
```

**Fields:**
```go
Input            map[string]string
Position         int
StatusCode       int64
ContentLength    int64
ContentWords     int64
ContentLines     int64
ContentType      string
RedirectLocation string
ScraperData      string
Duration         time.Duration
ResultFile       string
Url              string
Host             string
HTMLColor        string
FfufHash         string
```


#### `htmlFileOutput`  <sub>(struct)</sub>

```go
type htmlFileOutput struct
```

**Fields:**
```go
CommandLine string
Time        string
Keys        []string
Results     []htmlResult
```


#### `colorizeResults`  <sub>(func)</sub>

> colorizeResults returns a new slice with HTMLColor attribute

```go
func colorizeResults(results []ffuf.Result) []ffuf.Result
```

#### `writeHTML`  <sub>(func)</sub>

```go
func writeHTML(filename string, config *ffuf.Config, results []ffuf.Result) error
```

### `ffuf/pkg/output/file_json.go`


#### `ejsonFileOutput`  <sub>(struct)</sub>

```go
type ejsonFileOutput struct
```

**Fields:**
```go
CommandLine string        `json:"commandline"`
Time        string        `json:"time"`
Results     []ffuf.Result `json:"results"`
Config      *ffuf.Config  `json:"config"`
```


#### `JsonResult`  <sub>(struct)</sub>

```go
type JsonResult struct
```

**Fields:**
```go
Input            map[string]string   `json:"input"`
Position         int                 `json:"position"`
StatusCode       int64               `json:"status"`
ContentLength    int64               `json:"length"`
ContentWords     int64               `json:"words"`
ContentLines     int64               `json:"lines"`
ContentType      string              `json:"content-type"`
RedirectLocation string              `json:"redirectlocation"`
ScraperData      map[string][]string `json:"scraper"`
Duration         time.Duration       `json:"duration"`
ResultFile       string              `json:"resultfile"`
Url              string              `json:"url"`
Host             string              `json:"host"`
```


#### `jsonFileOutput`  <sub>(struct)</sub>

```go
type jsonFileOutput struct
```

**Fields:**
```go
CommandLine string       `json:"commandline"`
Time        string       `json:"time"`
Results     []JsonResult `json:"results"`
Config      *ffuf.Config `json:"config"`
```


#### `writeEJSON`  <sub>(func)</sub>

```go
func writeEJSON(filename string, config *ffuf.Config, res []ffuf.Result) error
```

#### `writeJSON`  <sub>(func)</sub>

```go
func writeJSON(filename string, config *ffuf.Config, res []ffuf.Result) error
```

### `ffuf/pkg/output/file_md.go`


#### `writeMarkdown`  <sub>(func)</sub>

```go
func writeMarkdown(filename string, config *ffuf.Config, results []ffuf.Result) error
```

### `ffuf/pkg/output/stdout.go`


#### `Stdoutput`  <sub>(struct)</sub>

```go
type Stdoutput struct
```

**Fields:**
```go
config           *ffuf.Config
fuzzkeywords     []string
resultMutex      sync.Mutex
Results          []ffuf.Result    // guards Results, CurrentResults and paused; Result is called from worker goroutines
CurrentResults   []ffuf.Result
stdoutIsTerminal bool
stderrIsTerminal bool
paused           bool
```


#### `NewStdoutput`  <sub>(func)</sub>

```go
func NewStdoutput(conf *ffuf.Config) *Stdoutput
```

#### `isTerminal`  <sub>(func)</sub>

> isTerminal reports whether f is an interactive terminal, as opposed to a  
> regular file or a pipe. This also treats other character devices (e.g.  
> /dev/null) as terminals, which is harmless here since it only affects  
> whether a handful of extra control bytes get written to a destination  
> that isn't going to be read back anyway.

```go
func isTerminal(f *os.File) bool
```

#### `Stdoutput.stderrClear`  <sub>(method)</sub>

> stderrClear returns the terminal control sequence used to redraw the  
> current line, or an empty string when stderr isn't an interactive  
> terminal. Without this, redirecting stderr to a file or another process  
> leaves raw control characters in the output.

```go
func (s *Stdoutput) stderrClear() string
```

#### `Stdoutput.stdoutClear`  <sub>(method)</sub>

> stdoutClear is the stdout equivalent of stderrClear, used when printing  
> results that overwrite the in-progress progress bar line.

```go
func (s *Stdoutput) stdoutClear() string
```

#### `Stdoutput.ansiClear`  <sub>(method)</sub>

> ansiClear returns the ANSI reset code used to close a color started by  
> colorize(), or an empty string when -c wasn't requested. Without this,  
> result lines carry a trailing reset code even when nothing was colored.

```go
func (s *Stdoutput) ansiClear() string
```

#### `Stdoutput.Banner`  <sub>(method)</sub>

```go
func (s *Stdoutput) Banner()
```

#### `Stdoutput.Reset`  <sub>(method)</sub>

> Reset resets the result slice

```go
func (s *Stdoutput) Reset()
```

#### `Stdoutput.Cycle`  <sub>(method)</sub>

> Cycle moves the CurrentResults to Results and resets the results slice

```go
func (s *Stdoutput) Cycle()
```

#### `Stdoutput.GetCurrentResults`  <sub>(method)</sub>

> GetResults returns the result slice

```go
func (s *Stdoutput) GetCurrentResults() []ffuf.Result
```

#### `Stdoutput.SetCurrentResults`  <sub>(method)</sub>

> SetResults sets the result slice

```go
func (s *Stdoutput) SetCurrentResults(results []ffuf.Result)
```

#### `Stdoutput.FilterCurrentResults`  <sub>(method)</sub>

> FilterCurrentResults keeps only the results for which keep returns true. The  
> read-filter-write runs under resultMutex, so a concurrent Result() append is  
> not lost through a stale snapshot the way a caller-side GetCurrentResults /  
> SetCurrentResults pair would drop it.

```go
func (s *Stdoutput) FilterCurrentResults(keep func(ffuf.Result) bool)
```

#### `Stdoutput.Progress`  <sub>(method)</sub>

```go
func (s *Stdoutput) Progress(status ffuf.Progress)
```

#### `Stdoutput.Info`  <sub>(method)</sub>

```go
func (s *Stdoutput) Info(infostring string)
```

#### `Stdoutput.Error`  <sub>(method)</sub>

```go
func (s *Stdoutput) Error(errstring string)
```

#### `Stdoutput.Warning`  <sub>(method)</sub>

```go
func (s *Stdoutput) Warning(warnstring string)
```

#### `Stdoutput.Raw`  <sub>(method)</sub>

```go
func (s *Stdoutput) Raw(output string)
```

#### `Stdoutput.writeToAll`  <sub>(method)</sub>

```go
func (s *Stdoutput) writeToAll(filename string, config *ffuf.Config, res []ffuf.Result) error
```

#### `Stdoutput.SaveFile`  <sub>(method)</sub>

> SaveFile saves the current results to a file of a given type

```go
func (s *Stdoutput) SaveFile(filename, format string) error
```

#### `Stdoutput.Finalize`  <sub>(method)</sub>

> Finalize gets run after all the ffuf jobs are completed

```go
func (s *Stdoutput) Finalize() error
```

#### `Stdoutput.Result`  <sub>(method)</sub>

```go
func (s *Stdoutput) Result(resp ffuf.Response)
```

#### `Stdoutput.SetPaused`  <sub>(method)</sub>

> SetPaused toggles whether Result streams matches to the live terminal. While  
> paused (the interactive console is open) matches are recorded but not printed,  
> so inflight requests completing cannot scroll the console off screen. Leaving  
> a pause (paused == false) marks every held result as shown, since resume  
> reports their count to the user; a subsequent pause then only counts matches  
> that are genuinely new.

```go
func (s *Stdoutput) SetPaused(paused bool)
```

#### `Stdoutput.PendingResults`  <sub>(method)</sub>

> PendingResults returns the number of held matches not yet shown to the user:  
> results recorded while paused that still survive the active filters. It is  
> derived from CurrentResults, so a filter change that prunes a held result  
> drops it from this count in the same operation.

```go
func (s *Stdoutput) PendingResults() int
```

#### `Stdoutput.writeResultToFile`  <sub>(method)</sub>

```go
func (s *Stdoutput) writeResultToFile(resp ffuf.Response) string
```

#### `Stdoutput.PrintResult`  <sub>(method)</sub>

```go
func (s *Stdoutput) PrintResult(res ffuf.Result)
```

#### `Stdoutput.prepareInputsOneLine`  <sub>(method)</sub>

```go
func (s *Stdoutput) prepareInputsOneLine(res ffuf.Result) string
```

#### `Stdoutput.resultQuiet`  <sub>(method)</sub>

```go
func (s *Stdoutput) resultQuiet(res ffuf.Result)
```

#### `Stdoutput.resultMultiline`  <sub>(method)</sub>

```go
func (s *Stdoutput) resultMultiline(res ffuf.Result)
```

#### `Stdoutput.resultNormal`  <sub>(method)</sub>

```go
func (s *Stdoutput) resultNormal(res ffuf.Result)
```

#### `Stdoutput.resultJson`  <sub>(method)</sub>

```go
func (s *Stdoutput) resultJson(res ffuf.Result)
```

#### `Stdoutput.colorize`  <sub>(method)</sub>

```go
func (s *Stdoutput) colorize(status int64) string
```

#### `printOption`  <sub>(func)</sub>

```go
func printOption(name []byte, value []byte)
```

### `ffuf/pkg/runner/flightvars.go`


#### `flightResponse`  <sub>(struct)</sub>

> flightResponse is what a variable can be read from: one preflight or  
> postflight response. The HTML is parsed at most once, and only when a form or  
> meta lookup needs it; the header block is likewise built on first use.

```go
type flightResponse struct
```

**Fields:**
```go
body      []byte
header    http.Header
doc       *goquery.Document
docErr    error
docDone   bool
headers   []byte
headersOK bool
```


#### `flightResponse.headerBlock`  <sub>(method)</sub>

> headerBlock returns the response headers as "Name: value\r\n" lines, one per  
> value, for -preflight-var regexes to search. Names are canonical  
> ("X-Csrf-Token") and sorted, since the wire order is not kept. Header.Write  
> turns any newline inside a value into a space.

```go
func (fr *flightResponse) headerBlock() []byte
```

#### `flightResponse.document`  <sub>(method)</sub>

```go
func (fr *flightResponse) document() (*goquery.Document, error)
```

#### `flightResponse.lookup`  <sub>(method)</sub>

> lookup returns the value of key in one source, and whether it was present.

```go
func (fr *flightResponse) lookup(source, key string) (string, bool)
```

#### `formValue`  <sub>(func)</sub>

> formValue returns the value a browser would submit for the first control named  
> key: an input's value attribute (skipping unchecked radios and checkboxes), a  
> textarea's text, or a select's selected (else first) option. goquery decodes  
> HTML entities, so "a&amp;b" comes back as "a&b", which a regex over the raw  
> markup would get wrong.

```go
func formValue(doc *goquery.Document, key string) (string, bool)
```

#### `metaValue`  <sub>(func)</sub>

> metaValue returns the content of the first <meta name=key>, the way Rails,  
> Laravel and Spring publish their CSRF token. Meta names compare  
> case-insensitively.

```go
func metaValue(doc *goquery.Document, key string) (string, bool)
```

#### `cookieValue`  <sub>(func)</sub>

> cookieValue returns the value of the cookie called name out of every  
> Set-Cookie header. Header.Get("Set-Cookie") would only see the first one. When  
> a response sets the same cookie twice the last one wins, as in a browser, and a  
> cookie being deleted (Max-Age<0) counts as absent. The value is returned as  
> sent, without percent-decoding.

```go
func cookieValue(header http.Header, name string) (string, bool)
```

#### `SimpleRunner.extractByName`  <sub>(method)</sub>

> extractByName resolves a -preflight-var-auto variable. A pinned source is read  
> directly. VarSourceAuto tries every source in ffuf.VarSources order; the first  
> response that resolves it pins the source for the rest of the run, so a later  
> error page carrying a same-named header can't quietly swap the value. If the  
> key is present in two sources with different values, it refuses to guess.

```go
func (r *SimpleRunner) extractByName(ve *ffuf.VarExtract, fr *flightResponse) (string, error)
```

### `ffuf/pkg/runner/simple.go`


#### `SimpleRunner`  <sub>(struct)</sub>

```go
type SimpleRunner struct
```

**Fields:**
```go
config *ffuf.Config
client *http.Client
lanes  *lanePool
autoPins sync.Map    // bare key first resolved to (see extractByName).
```


#### `preflightLane`  <sub>(struct)</sub>

> preflightLane holds the variables extracted by a per-thread preflight chain.  
> A lane is checked out of the pool for the duration of one Execute call, so its  
> map is only ever touched by a single goroutine at a time.

```go
type preflightLane struct
```

**Fields:**
```go
vars        map[string]string
initialized bool
```


#### `lanePool`  <sub>(struct)</sub>

> lanePool hands out preflightLanes for per-thread preflight mode. ffuf runs a  
> fresh goroutine per request (bounded to -t concurrent), so there is no  
> persistent worker to pin per-thread state to; instead a request borrows a lane  
> for the length of its Execute call and returns it. Lanes are reused, so a  
> per-thread preflight chain runs once per lane and its vars are amortized across  
> every request that later borrows that lane. The free list grows on demand, so a  
> checkout never blocks and cannot deadlock.

```go
type lanePool struct
```

**Fields:**
```go
mu   sync.Mutex
free []*preflightLane
```


#### `lanePool.get`  <sub>(method)</sub>

```go
func (p *lanePool) get() *preflightLane
```

#### `lanePool.put`  <sub>(method)</sub>

```go
func (p *lanePool) put(l *preflightLane)
```

#### `NewSimpleRunner`  <sub>(func)</sub>

```go
func NewSimpleRunner(conf *ffuf.Config, replay bool) ffuf.RunnerProvider
```

#### `SimpleRunner.Prepare`  <sub>(method)</sub>

```go
func (r *SimpleRunner) Prepare(input map[string][]byte, basereq *ffuf.Request) (ffuf.Request, error)
```

#### `SimpleRunner.Execute`  <sub>(method)</sub>

```go
func (r *SimpleRunner) Execute(req *ffuf.Request) (resp ffuf.Response, err error)
```

#### `SimpleRunner.parsePreflightRequest`  <sub>(method)</sub>

> parsePreflightRequest reads a Burp-style raw HTTP request file and builds an  
> *http.Request. Main-config headers are inherited (so auth carries over) and any  
> already-known vars are substituted into the request before it is built.

```go
func (r *SimpleRunner) parsePreflightRequest(filename string, vars map[string]string) (*http.Request, error)
```

#### `SimpleRunner.runPreflightChain`  <sub>(method)</sub>

> runPreflightChain executes an ordered chain of requests, accumulating extracted  
> variables. inheritVars seeds the map (e.g. with vars from an earlier step). The  
> result is returned; no shared state is touched. On error it honours  
> -preflight-error: "ignore" returns the vars gathered so far, "abort" errors.

```go
func (r *SimpleRunner) runPreflightChain(chain []ffuf.PreflightConfig, inheritVars map[string]string) (map[string]string, error)
```

#### `varsReplacer`  <sub>(func)</sub>

> varsReplacer builds one deterministic replacer for the extracted vars. Names  
> are applied longest-first (then lexicographically) so a shorter name that is a  
> prefix of a longer one can't win, and strings.Replacer does a single  
> simultaneous pass, so a replaced value is never rescanned. That makes  
> substitution independent of map iteration order and free of value-contains-name  
> cascades.

```go
func varsReplacer(vars map[string]string) *strings.Replacer
```

#### `SimpleRunner.flightAuthority`  <sub>(method)</sub>

> hostOf returns the authority (host[:port]) of a URL, or "" if it cannot be parsed.  
> flightAuthority reports the scheme and host a flight request would be sent to,  
> given the request-line path and Host header from its file. It mirrors the URL  
> construction below, so the two stay in step.

```go
func (r *SimpleRunner) flightAuthority(path, host string) (scheme, authority string)
```

#### `hostOf`  <sub>(func)</sub>

```go
func hostOf(rawurl string) string
```

#### `SimpleRunner.hostHasKeyword`  <sub>(method)</sub>

> hostHasKeyword reports whether host still contains an input fuzz keyword, which  
> means it was derived from an unsubstituted target template (e.g. vhost fuzzing  
> with a relative-path preflight).

```go
func (r *SimpleRunner) hostHasKeyword(host string) (string, bool)
```

#### `SimpleRunner.applyVars`  <sub>(method)</sub>

> applyVars substitutes extracted variable keywords into a single request's URL,  
> headers and body. It mutates only the request passed in.

```go
func (r *SimpleRunner) applyVars(req *ffuf.Request, vars map[string]string)
```

#### `SimpleRunner.runPreflights`  <sub>(method)</sub>

> runPreflights resolves the variables for this request per -preflight-mode  
> (fresh per request, or once per lane and amortized in per-thread mode) and  
> applies them. It returns the applied vars so postflight can chain off them, and  
> a cleanup func the caller must defer (it returns a borrowed lane, if any).

```go
func (r *SimpleRunner) runPreflights(req *ffuf.Request) (applied map[string]string, cleanup func(), err error)
```

#### `SimpleRunner.runPostflights`  <sub>(method)</sub>

> runPostflights executes the postflight chain after the main request, seeded  
> with the vars applied to it so chained extractions work. Postflight failure is  
> non-fatal: the main result is always kept, the error is only logged.

```go
func (r *SimpleRunner) runPostflights(seedVars map[string]string)
```

#### `SimpleRunner.Dump`  <sub>(method)</sub>

```go
func (r *SimpleRunner) Dump(req *ffuf.Request) ([]byte, error)
```

### `ffuf/pkg/scraper/scraper.go`


#### `ScraperRule`  <sub>(struct)</sub>

```go
type ScraperRule struct
```

**Fields:**
```go
Name         string `json:"name"`
Rule         string `json:"rule"`
Target       string `json:"target"`
compiledRule *regexp.Regexp
Type         string   `json:"type"`
OnlyMatched  bool     `json:"onlymatched"`
Action       []string `json:"action"`
```


#### `ScraperGroup`  <sub>(struct)</sub>

```go
type ScraperGroup struct
```

**Fields:**
```go
Rules  []*ScraperRule `json:"rules"`
Name   string         `json:"groupname"`
Active bool           `json:"active"`
```


#### `Scraper`  <sub>(struct)</sub>

```go
type Scraper struct
```

**Fields:**
```go
Rules []*ScraperRule
```


#### `readGroupFromFile`  <sub>(func)</sub>

```go
func readGroupFromFile(filename string) (ScraperGroup, error)
```

#### `FromDir`  <sub>(func)</sub>

```go
func FromDir(dirname string, activestr string) (ffuf.Scraper, ffuf.Multierror)
```

#### `Scraper.AppendFromFile`  <sub>(method)</sub>

> FromFile initializes a scraper instance and reads rules from a file

```go
func (s *Scraper) AppendFromFile(path string) error
```

#### `Scraper.Execute`  <sub>(method)</sub>

```go
func (s *Scraper) Execute(resp *ffuf.Response, matched bool) []ffuf.ScraperResult
```

#### `ScraperRule.init`  <sub>(method)</sub>

> init initializes the scraper rule, and returns an error in case there's an error in the syntax

```go
func (r *ScraperRule) init() error
```

#### `ScraperRule.Check`  <sub>(method)</sub>

```go
func (r *ScraperRule) Check(data string) []string
```

#### `ScraperRule.checkQuery`  <sub>(method)</sub>

```go
func (r *ScraperRule) checkQuery(data string) []string
```

#### `ScraperRule.checkRegexp`  <sub>(method)</sub>

```go
func (r *ScraperRule) checkRegexp(data string) []string
```

### `ffuf/pkg/scraper/util.go`


#### `headerString`  <sub>(func)</sub>

```go
func headerString(headers map[string][]string) string
```

#### `isActive`  <sub>(func)</sub>

```go
func isActive(name string, activegroups []string) bool
```

#### `parseActiveGroups`  <sub>(func)</sub>

```go
func parseActiveGroups(activestr string) []string
```

### `ffuf/pkg/testtarget/server.go`


#### `Recorded`  <sub>(struct)</sub>

> Recorded is one request the target received, exposed via Target.Requests so a  
> test can assert on what ffuf actually sent (method, headers, body, timing).

```go
type Recorded struct
```

**Fields:**
```go
Method string
Path   string
Query  string
Header http.Header
Body   string
At     time.Time
```


#### `Target`  <sub>(struct)</sub>

> Target wraps an httptest.Server with a thread-safe request recorder.

```go
type Target struct
```

**Fields:**
```go
*httptest.Server
mu       sync.Mutex
recorded []Recorded
```


#### `New`  <sub>(func)</sub>

> New starts a new deterministic target server. The caller must Close it.

```go
func New() *Target
```

#### `Target.Requests`  <sub>(method)</sub>

> Requests returns a snapshot copy of every request received so far.

```go
func (t *Target) Requests() []Recorded
```

#### `Target.Count`  <sub>(method)</sub>

> Count returns how many requests the target has received.

```go
func (t *Target) Count() int
```

#### `Target.handle`  <sub>(method)</sub>

```go
func (t *Target) handle(w http.ResponseWriter, r *http.Request)
```

#### `forbidden`  <sub>(func)</sub>

```go
func forbidden(w http.ResponseWriter)
```

#### `notFound`  <sub>(func)</sub>

```go
func notFound(w http.ResponseWriter) { w.WriteHeader(http.StatusNotFound); fmt.Fprint(w, "not found") }
    
    // maxTailInt bounds every path-derived integer. It is far above anything the
    // tests use and keeps the value that reaches size/word/line allocation from
    // being attacker-controlled (a request like /size/99999999999 is rejected
    // rather than allocating from an unbounded, path-supplied size).
    const maxTailInt = 1 << 20
    
    // tailInt parses the integer immediately following prefix in path. It requires
    // the remainder to be exactly an integer (no further path segments), so
    // /size/20 parses and /size/20/extra does not, and enforces 0 <= n <= maxTailInt.
    func tailInt(path, prefix string) (int, bool)
```

#### `filled`  <sub>(func)</sub>

> filled returns n bytes of 'A'. net/http sets Content-Length to n for these  
> small buffered bodies, which is what ffuf's size matcher reads.

```go
func filled(n int) []byte
```

#### `words`  <sub>(func)</sub>

> words returns a body of exactly n whitespace-separated tokens.

```go
func words(n int) string
```

#### `lines`  <sub>(func)</sub>

> lines returns a body that ffuf counts as exactly n lines. ffuf's line count  
> is len(strings.Split(data, "\n")), so n "L"s joined by newlines (no trailing  
> newline) yields exactly n, matching the /size and /words convention.

```go
func lines(n int) string
```

#### `drain`  <sub>(func)</sub>

```go
func drain(r *http.Request) string
```
