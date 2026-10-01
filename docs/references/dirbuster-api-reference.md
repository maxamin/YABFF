# DirBuster — Complete API Reference

Exhaustive per-item reference for **every** type, function, method, constant, and macro in the DirBuster source (`KajanM/DirBuster` @ `634d166`, v1.0-RC1). Language: **JAVA**. Generated from a full source read; test files are excluded. Each item shows its full signature and doc comment; composite types list their fields/members.

**1448 documented items** across 74 source files.


---

## Modules / packages

- [`src/com/sittinglittleduck/DirBuster`](#pkg-src-com-sittinglittleduck-dirbuster) — 393 items
- [`src/com/sittinglittleduck/DirBuster/bruteForceIterator`](#pkg-src-com-sittinglittleduck-dirbuster-bruteforceiterator) — 24 items
- [`src/com/sittinglittleduck/DirBuster/diff`](#pkg-src-com-sittinglittleduck-dirbuster-diff) — 99 items
- [`src/com/sittinglittleduck/DirBuster/gui`](#pkg-src-com-sittinglittleduck-dirbuster-gui) — 505 items
- [`src/com/sittinglittleduck/DirBuster/gui/JTableTree`](#pkg-src-com-sittinglittleduck-dirbuster-gui-jtabletree) — 110 items
- [`src/com/sittinglittleduck/DirBuster/gui/documentListeners`](#pkg-src-com-sittinglittleduck-dirbuster-gui-documentlisteners) — 20 items
- [`src/com/sittinglittleduck/DirBuster/gui/tableModels`](#pkg-src-com-sittinglittleduck-dirbuster-gui-tablemodels) — 105 items
- [`src/com/sittinglittleduck/DirBuster/gui/tree`](#pkg-src-com-sittinglittleduck-dirbuster-gui-tree) — 33 items
- [`src/com/sittinglittleduck/DirBuster/headless`](#pkg-src-com-sittinglittleduck-dirbuster-headless) — 2 items
- [`src/com/sittinglittleduck/DirBuster/monitorThreads`](#pkg-src-com-sittinglittleduck-dirbuster-monitorthreads) — 15 items
- [`src/com/sittinglittleduck/DirBuster/report`](#pkg-src-com-sittinglittleduck-dirbuster-report) — 16 items
- [`src/com/sittinglittleduck/DirBuster/utils`](#pkg-src-com-sittinglittleduck-dirbuster-utils) — 5 items
- [`src/com/sittinglittleduck/DirBuster/workGenerators`](#pkg-src-com-sittinglittleduck-dirbuster-workgenerators) — 121 items

---


## `src/com/sittinglittleduck/DirBuster` <a id='pkg-src-com-sittinglittleduck-dirbuster'></a>

*393 documented items*


### `dirbuster/src/com/sittinglittleduck/DirBuster/BaseCase.java`


#### `BaseCase`  <sub>(class)</sub>

> Used to store information about a base case.

```java
public class BaseCase
```

#### `private URL baseCaseURL;`  <sub>(method)</sub>

```java
private URL baseCaseURL;
```

#### `private int returnCode = 0;`  <sub>(method)</sub>

```java
private int returnCode = 0;
```

#### `private String baseCase = null;`  <sub>(method)</sub>

```java
private String baseCase = null;
```

#### `private boolean dir = true;`  <sub>(method)</sub>

```java
private boolean dir = true;
```

#### `private URL baseCaseRequestURL;`  <sub>(method)</sub>

```java
private URL baseCaseRequestURL;
```

#### `private String fileExt = null;`  <sub>(method)</sub>

```java
private String fileExt = null;
```

#### `private boolean useRegexInstead = false;`  <sub>(method)</sub>

```java
private boolean useRegexInstead = false;
```

#### `private String regex;`  <sub>(method)</sub>

```java
private String regex;
```

#### `BaseCase`  <sub>(method)</sub>

> Creates a new instance of BaseCase

```java
    public BaseCase(URL baseCaseURL, int returnCode, boolean dir, URL baseCaseRequestURL, String baseCase, String fileExt, boolean useRegexInstead, String regex)
```

#### `getBaseCase`  <sub>(method)</sub>

```java
    public String getBaseCase()
```

#### `getBaseCaseRequestURL`  <sub>(method)</sub>

```java
    public URL getBaseCaseRequestURL()
```

#### `getBaseCaseURL`  <sub>(method)</sub>

```java
    public URL getBaseCaseURL()
```

#### `getFailCode`  <sub>(method)</sub>

```java
    public int getFailCode()
```

#### `isDir`  <sub>(method)</sub>

```java
    public boolean isDir()
```

#### `setBaseCaseURL`  <sub>(method)</sub>

```java
    public void setBaseCaseURL(URL baseCaseURL)
```

#### `setDir`  <sub>(method)</sub>

```java
    public void setDir(boolean dir)
```

#### `useContentAnalysisMode`  <sub>(method)</sub>

```java
    public boolean useContentAnalysisMode()
```

#### `getFileExt`  <sub>(method)</sub>

```java
    public String getFileExt()
```

#### `getRegex`  <sub>(method)</sub>

```java
    public String getRegex()
```

#### `getReturnCode`  <sub>(method)</sub>

```java
    public int getReturnCode()
```

#### `isUseRegexInstead`  <sub>(method)</sub>

```java
    public boolean isUseRegexInstead()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/CheckForUpdates.java`


#### `CheckForUpdates`  <sub>(class)</sub>

```java
public class CheckForUpdates implements Runnable
```

#### `private Manager manager;`  <sub>(method)</sub>

```java
private Manager manager;
```

#### `private HttpClient httpclient;`  <sub>(method)</sub>

```java
private HttpClient httpclient;
```

#### `CheckForUpdates`  <sub>(method)</sub>

```java
    public CheckForUpdates(boolean informUser)
```

#### `run`  <sub>(method)</sub>

```java
    public void run()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/Config.java`


#### `Config`  <sub>(class)</sub>

> Stores basic configuration detials

```java
public class Config
```

#### `public static final String version = "1.0-RC1";`  <sub>(method)</sub>

```java
public static final String version = "1.0-RC1";
```

#### `public static final String versionDate = "06/05/`  <sub>(method)</sub>

```java
public static final String versionDate = "06/05/2008";
```

#### `public static String userAgent = "DirBuster-" + `  <sub>(method)</sub>

```java
public static String userAgent = "DirBuster-" + version + " (http://www.owasp.org/index.php/Category:OWASP_DirBuster_Project)";
```

#### `public static boolean debug = false;`  <sub>(method)</sub>

```java
public static boolean debug = false;
```

#### `public static boolean followRedirects = false;`  <sub>(method)</sub>

```java
public static boolean followRedirects = false;
```

#### `public static int connectionTimeout = 30;`  <sub>(method)</sub>

```java
public static int connectionTimeout = 30;
```

#### `public static String failCaseString = "thereIsNo`  <sub>(method)</sub>

```java
public static String failCaseString = "thereIsNoWayThat-You-CanBeThere";
```

#### `public static boolean parseHTML = true;`  <sub>(method)</sub>

```java
public static boolean parseHTML = true;
```

#### `public static boolean parseHTMLa = true;`  <sub>(method)</sub>

```java
public static boolean parseHTMLa = true;
```

#### `public static boolean parseHTMLimg = true;`  <sub>(method)</sub>

```java
public static boolean parseHTMLimg = true;
```

#### `public static boolean parseHTMLform = true;`  <sub>(method)</sub>

```java
public static boolean parseHTMLform = true;
```

#### `public static boolean caseInsensativeMode = fals`  <sub>(method)</sub>

```java
public static boolean caseInsensativeMode = false;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/DirToCheck.java`


#### `DirToCheck`  <sub>(class)</sub>

```java
public class DirToCheck
```

#### `Vector`  <sub>(method)</sub>

```java
private Vector exts = new Vector(10,10);
```

#### `DirToCheck`  <sub>(method)</sub>

> Creates a new instance of DirToCheck

```java
    public DirToCheck(String name, Vector exts)
```

#### `getName`  <sub>(method)</sub>

```java
    public String getName()
```

#### `getExts`  <sub>(method)</sub>

```java
    public Vector getExts()
```

#### `setExts`  <sub>(method)</sub>

```java
    public void setExts(Vector exts)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/EasySSLProtocolSocketFactory.java`


#### `EasySSLProtocolSocketFactory`  <sub>(class)</sub>

> <p>  
> EasySSLProtocolSocketFactory can be used to creats SSL {@link Socket}s  
> that accept self-signed certificates.  
> </p>  
> <p>  
> This socket factory SHOULD NOT be used for productive systems  
> due to security reasons, unless it is a concious decision and  
> you are perfectly aware of security implications of accepting  
> self-signed certificates  
> </p>  
> <p>  
> Example of using custom protocol socket factory for a specific host:  
> <pre>  
> Protocol easyhttps = new Protocol("https", new EasySSLProtocolSocketFactory(), 443);  
> HttpClient client = new HttpClient();  
> client.getHostConfiguration().setHost("localhost", 443, easyhttps);  
> // use relative url only  
> GetMethod httpget = new GetMethod("/");  
> client.executeMethod(httpget);  
> </pre>  
> </p>  
> <p>  
> Example of using custom protocol socket factory per default instead of the standard one:  
> <pre>  
> Protocol easyhttps = new Protocol("https", new EasySSLProtocolSocketFactory(), 443);  
> Protocol.registerProtocol("https", easyhttps);  
> HttpClient client = new HttpClient();  
> GetMethod httpget = new GetMethod("https://localhost/");  
> client.executeMethod(httpget);  
> </pre>  
> </p>  
> <p>  
> DISCLAIMER: HttpClient developers DO NOT actively support this component.  
> The component is provided as a reference material, which may be inappropriate  
> for use without additional customization.  
> </p>

```java
public class EasySSLProtocolSocketFactory implements SecureProtocolSocketFactory
```

#### `private SSLContext sslcontext = null;`  <sub>(method)</sub>

```java
private SSLContext sslcontext = null;
```

#### `EasySSLProtocolSocketFactory`  <sub>(method)</sub>

> Constructor for EasySSLProtocolSocketFactory.

```java
    public EasySSLProtocolSocketFactory()
```

#### `createEasySSLContext`  <sub>(method)</sub>

```java
    private static SSLContext createEasySSLContext()
```

#### `getSSLContext`  <sub>(method)</sub>

```java
    private SSLContext getSSLContext()
```

#### `createSocket`  <sub>(method)</sub>

```java
    public Socket createSocket(
            String host,
            int port,
            InetAddress clientHost,
            int clientPort)
```

#### `createSocket`  <sub>(method)</sub>

> Attempts to get a new socket connection to the given host within the given time limit.  
> <p>  
> To circumvent the limitations of older JREs that do not support connect timeout a  
> controller thread is executed. The controller thread attempts to create a new socket  
> within the given limit of time. If socket constructor does not return until the  
> timeout expires, the controller terminates and throws an {@link ConnectTimeoutException}  
> </p>  
> determined

```java
    public Socket createSocket(
            final String host,
            final int port,
            final InetAddress localAddress,
            final int localPort,
            final HttpConnectionParams params
    ) throws IOException, UnknownHostException, ConnectTimeoutException
```

#### `createSocket`  <sub>(method)</sub>

```java
    public Socket createSocket(String host, int port)
```

#### `createSocket`  <sub>(method)</sub>

```java
    public Socket createSocket(
            Socket socket,
            String host,
            int port,
            boolean autoClose)
```

#### `equals`  <sub>(method)</sub>

```java
    public boolean equals(Object obj)
```

#### `hashCode`  <sub>(method)</sub>

```java
    public int hashCode()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/EasyX509TrustManager.java`


#### `EasyX509TrustManager`  <sub>(class)</sub>

> EasyX509TrustManager unlike default {@link X509TrustManager} accepts * self-signed certificates. *  
> This trust manager SHOULD NOT be used for productive systems * due to security reasons, unless it is a concious decision and * you are perfectly aware of security implications of accepting * self-signed certificates *  
> DISCLAIMER: HttpClient developers DO NOT actively support this component.  
> The component is provided as a reference material, which may be inappropriate  
> to be used without additional customization.

```java
public class EasyX509TrustManager implements X509TrustManager
```

#### `private X509TrustManager standardTrustManager = `  <sub>(method)</sub>

```java
private X509TrustManager standardTrustManager = null;
```

#### `EasyX509TrustManager`  <sub>(method)</sub>

> Constructor for EasyX509TrustManager.

```java
    public EasyX509TrustManager(KeyStore keystore) throws NoSuchAlgorithmException, KeyStoreException
```

#### `getAcceptedIssuers`  <sub>(method)</sub>

```java
    public java.security.cert.X509Certificate[] getAcceptedIssuers()
```

#### `checkClientTrusted`  <sub>(method)</sub>

```java
    public void checkClientTrusted(java.security.cert.X509Certificate[] certs, String authType)
```

#### `checkServerTrusted`  <sub>(method)</sub>

```java
    public void checkServerTrusted(java.security.cert.X509Certificate[] certs, String authType)
```

#### `isServerTrusted`  <sub>(method)</sub>

```java
    public boolean isServerTrusted(java.security.cert.X509Certificate[] certs)
```

#### `isClientTrusted`  <sub>(method)</sub>

```java
    public boolean isClientTrusted(java.security.cert.X509Certificate[] certs)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/ExtToCheck.java`


#### `ExtToCheck`  <sub>(class)</sub>

```java
public class ExtToCheck
```

#### `public static final String BLANK_EXT = "BLANK";`  <sub>(method)</sub>

```java
public static final String BLANK_EXT = "BLANK";
```

#### `private String name;`  <sub>(method)</sub>

```java
private String name;
```

#### `private boolean toCheck;`  <sub>(method)</sub>

```java
private boolean toCheck;
```

#### `ExtToCheck`  <sub>(method)</sub>

> Creates a new instance of ExtToCheck

```java
    public ExtToCheck(String name, boolean toCheck)
```

#### `getName`  <sub>(method)</sub>

```java
    public String getName()
```

#### `toCheck`  <sub>(method)</sub>

```java
    public boolean toCheck()
```

#### `setToCheck`  <sub>(method)</sub>

```java
    public void setToCheck(boolean toCheck)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/FilterResponce.java`


#### `FilterResponce`  <sub>(class)</sub>

> Util class to normliaze http responces

```java
public class FilterResponce
```

#### `FilterResponce`  <sub>(method)</sub>

> Creates a new instance of FilterResponce

```java
    public FilterResponce()
```

#### `CleanResponce`  <sub>(method)</sub>

> Clean the responce of a work unit

```java
    public static String CleanResponce(String toclean, WorkUnit work)
```

#### `CleanResponce`  <sub>(method)</sub>

> Clean the responce of a work based on a URL

```java
    public static String CleanResponce(String toclean, URL url, String itemChecked)
```

#### `removeItemCheckedFor`  <sub>(method)</sub>

```java
    public static String removeItemCheckedFor(String toclean, String itemToCheckFor)
```

#### `RegexSafe`  <sub>(method)</sub>

```java
    private static String RegexSafe(String toMakeSafe)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/GenBaseCase.java`


#### `GenBaseCase`  <sub>(class)</sub>

> Generates a base case for a dir or file rangle that is about to be scanned

```java
public class GenBaseCase
```

#### `GenBaseCase`  <sub>(method)</sub>

> Creates a new instance of GenBaseCase

```java
    public GenBaseCase()
```

#### `genBaseCase`  <sub>(method)</sub>

> Generates the base case

```java
    public static synchronized BaseCase genBaseCase(String url, boolean isDir, String fileExtention) throws MalformedURLException, IOException
```

#### `genURLFuzzBaseCase`  <sub>(method)</sub>

> Used to generate a basecase when we are URL fuzzing

```java
    public static BaseCase genURLFuzzBaseCase(String fuzzStart, String FuzzEnd) throws MalformedURLException, IOException
```

#### `getBaseCaseAgain`  <sub>(method)</sub>

> this function is used to get base case again, so we can check that the base case is consitent.

```java
    private static String getBaseCaseAgain(URL failurl, String failString) throws IOException
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/HTMLelementToParse.java`


#### `HTMLelementToParse`  <sub>(class)</sub>

```java
public class HTMLelementToParse
```

#### `private String tag;`  <sub>(method)</sub>

```java
private String tag;
```

#### `private String attr;`  <sub>(method)</sub>

```java
private String attr;
```

#### `HTMLelementToParse`  <sub>(method)</sub>

> Creates a new instance of HTMLelementToParse

```java
    public HTMLelementToParse(String tag, String attr)
```

#### `getAttr`  <sub>(method)</sub>

```java
    public String getAttr()
```

#### `getTag`  <sub>(method)</sub>

```java
    public String getTag()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/HTMLparse.java`


#### `HTMLparse`  <sub>(class)</sub>

> This class is to paser the returned html pages and extract other dirs and files from them

```java
public class HTMLparse extends Thread
```

#### `private String sourceAsString = null;`  <sub>(method)</sub>

```java
private String sourceAsString = null;
```

#### `private WorkUnit work = null;`  <sub>(method)</sub>

```java
private WorkUnit work = null;
```

#### `private Manager manager;`  <sub>(method)</sub>

```java
private Manager manager;
```

#### `private boolean continueWorking = true;`  <sub>(method)</sub>

```java
private boolean continueWorking = true;
```

#### `HTMLparse`  <sub>(method)</sub>

> Creates a new instance of HTMLparse

```java
    public HTMLparse()
```

#### `stopWorking`  <sub>(method)</sub>

```java
    public void stopWorking()
```

#### `run`  <sub>(method)</sub>

```java
    public void run()
```

#### `processURL`  <sub>(method)</sub>

> Splits up the URL found

```java
    private Vector processURL(URL url)
```

#### `findBaseCasePoint`  <sub>(method)</sub>

```java
    private BaseCase findBaseCasePoint(String item)
```

#### `isWorking`  <sub>(method)</sub>

```java
    public boolean isWorking()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/HTMLparseWorkUnit.java`


#### `HTMLparseWorkUnit`  <sub>(class)</sub>

```java
public class HTMLparseWorkUnit
```

#### `private String htmlToParse;`  <sub>(method)</sub>

```java
private String htmlToParse;
```

#### `private WorkUnit workUnit;`  <sub>(method)</sub>

```java
private WorkUnit workUnit;
```

#### `HTMLparseWorkUnit`  <sub>(method)</sub>

> Creates a new instance of HTMLparseWorkUnit

```java
    public HTMLparseWorkUnit(String htmlToParse, WorkUnit workUnit)
```

#### `getHtmlToParse`  <sub>(method)</sub>

```java
    public String getHtmlToParse()
```

#### `getWorkUnit`  <sub>(method)</sub>

```java
    public WorkUnit getWorkUnit()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/HTTPHeader.java`


#### `HTTPHeader`  <sub>(class)</sub>

```java
public class HTTPHeader
```

#### `private String header;`  <sub>(method)</sub>

```java
private String header;
```

#### `private String value;`  <sub>(method)</sub>

```java
private String value;
```

#### `HTTPHeader`  <sub>(method)</sub>

> Creates a new instance of HTTPHeader

```java
    public HTTPHeader(String header, String value)
```

#### `getHeader`  <sub>(method)</sub>

```java
    public String getHeader()
```

#### `getValue`  <sub>(method)</sub>

```java
    public String getValue()
```

#### `setHeader`  <sub>(method)</sub>

```java
    public void setHeader(String header)
```

#### `setValue`  <sub>(method)</sub>

```java
    public void setValue(String value)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/ImageCreator.java`


#### `ImageCreator`  <sub>(class)</sub>

```java
public class ImageCreator
```

#### `Base64`  <sub>(method)</sub>

```java
private static Base64 base64 = new Base64();
```

#### `ImageIcon`  <sub>(method)</sub>

```java
public static final ImageIcon OWASP_IMAGE = new ImageIcon(base64.decode("iVBORw0KGgoAAAANSUhEUgAAAGAAAABYCAMAAAA9bwLKAAAAAXNSR0ICQMB9xQAAAqxQTFRFAAAAExMTGxsbCwsLFBkbFBUaCgkGDxASBggFBwgKAAETAAEaAwkUDhIbGBYYEBAOBgsZDhAPCQwaGBkWCAcFCw0SAQIjExkiCAs1DBE8CxEhBgkvAwUrCAsuExUhGx0hDRREFR1WERZGHSIkHCMqHiQwGSJaGCBHHCdlHy5xHzR3IBwbICAeOzs7KyssLjAuMjIzIiMiNTg2JSksKS0wNTk7KScoMTEvKjAzNDU5KCgmJSUpODc8KzI4PD1BJTFrMDx2JjV4ISxqJC50KDyIJj6RMT2HPkM/PUJEKkGMNEWMNEuYOVKcKkWVNU+jPFWgQD4/U1NTSktLQkNDWVdYW1tbWFZXUFJPRkhHTU5RTVBTVFVZVlhbRkhKSEdMRUVJVlhXXVxgQE6VQ1abRFSPVF2cRFujXWBfXWBhXmObS2OnVWmrW3CuWnSyWW2wTXGtTnSxXYG3YF5fc3JzY2Nja2tre3p7bnBvaGpnb3Fyd3h5ZWlrdXV4eXd4aGZneHZ3YmqqZnm0dHy2YnOtdnarfoGAdoa6aoO5e5C9e4zAe5TDbYrBgH+Ai4uMg4OEmpqbiYeIkpKTmJiWiomHkJCOlpiVkpiYlpaYjo6Qh4iKj5CRkI6PhIu8i5C4kZC6go3Bh5fFkZzInqCfi6HLl6bMmqvRnrHSjqXRoJ6foZ+goKCeu7u7q6uso6OjrbCvs7OzuLq3p6mmtri3p6ipt7i5qKqnr7CxpqWosK+xoK3Ooa3Rp7XVs7vZo7DOucLcv8DBvMXgwL/C29vbw8PDy8vL09PT0M7Q1tjXzNDR1tbYyczVxsbIx8jKyMbH2dnW2NbXyMfKw8rhy9Hk1t/w3d7h3uHg4d/g4ODe8/Pz6+vr4+Pj8PDu+Pj29vb46efo6ubl7u7w6Ojm7vDu9vj3GyzEYgAAAAF0Uk5TAEDm2GYAAAAJcEhZcwAACdgAAAnYAcegua0AAAAZdEVYdFNvZnR3YXJlAE1pY3Jvc29mdCBPZmZpY2V/7TVxAAAQ+ElEQVRo3s1Z+19TZ5oniY4nJxbdtkSozG6QGY1WzhLH67Q7GkioyclFYCJqLzubSNzmJAGagFbnwgmaGDDJorTdCzRhkkC3SSbjTre7kJQyJHF2OrvdxdmuAafby/wj+zzvCYrWWmv9YZ/wAfl82u/3eb7P5X2fl6qq/zdWXiik0plMOv2LbKpYWnq02KV0yHOqvaOjs7Oj/YcWb/9AOBTJFh4VSyHU3sT8eXPTkc6Ooxavt/+0Z2DgQvhvQqORdOqbcyxnLIxIxTQ3NYHz7ZZXvP39gB8Oh0YvR1CvVGn5m8AvDDSLRWKmubnpuc6OLvD/DIG/cmUM8dPp7C9TqWLxoaNYCDMSkVhF8I91tR+1/NDyVyBR/6tnw2ORzFWCXygUS6WF8kNldrQZ4HcS/M5jx47sYr7XKF6zds2axsbdew63nwtd/SeAL1wD/OXlhwji2vFvAb54L7NvV1NTs2odJZVKaZmcmIym1z+2af9fj6ULpdJvFm4sL9382jGEVRT6zzD7mhmxBLDltYoG5cGDhw6pNertyq3b6uS07Kk93teK4P5S+eviL1koSlItblQxjGoDRdFyhVKt07NW1mowmFgDGKvTtWyTyzZ+58SvPioj/h/KSw/OUmyiqHXVYjHDPP0nlFSmUB4ysgYrAtvsDofw5bDbDfoWhVzW+HwGMvAxsCw9aCZSDOCD/oxKgvDgO/HajLCcg+OcTpfL7XZznMNu0G2Tyb57rgQ1cXPpf5eXHyiIlJiSgv8qlYiiZIrtRtZoYs02O8Aj9iDP8z6fP+DnAzzv4hy2Q1vkG3/w+idlxF94kILNion/KnR/s9KI6GZA5wB9iB/i/X5/MDg+HI0FY8Gg3z/k5AyHamWbzv3bp4D/m9LCV8pUEAn+A42sYYfJyJrMiO52usF1BI/GJiYn4/ABm4gF/Tzn0DfIHj/+DvhfKhVLXxFDiaGkIqhPkKdeqTWaBHjOXYG/OB6LTcYTYDPEEpOxcb+Lsx+spfe8DgTXioXifRmWOinpBtCnGuTZgfIQ5Qk8j9IA/CSiz+YEm52djU8G/S6Hrpbe/cZvS8VCIVW6H0E/qU9Ir7R2B6pPxHe7KuoI8DMIn8/n5/Jo88nkZAx0stbJdv9tEQmy92HISAD/CdBfqtCYtCYT+u92D0HdYG4F/OQKPlp+bn4+N5uIj/NOq4LePVr8dSqVzX7pBC+pKCG/0lqNFnpL8J93gv8BxI/FgWB2VnAf4BcX5+fmF/M5TEXAdbKOPhCBALLp7JeUUrmd4O98kuiP1UnKBwTy+/xQmiQAxEeGRYTHr+tzkIvE5Ljfba3deDiNBJnCvQnSEkk1qX/5dlL+pHOdUP2rMgD474H/19H/D+avL+aBYC4PKmGq9fKNL/wC8DNvL9wzgE7A3/l9MUVv1QIBJtgBE+FWhqMkAySAxcW5uQ8W56//fi43jwT595CBdx6kNw1k/znz1uXsvQgiWJ97VU9KFYDPkgoCiVxCAP5gFAigA2Zzi6gPJuD6f1/P5efn4d9EpIkgz22j949l3oq8NnqPSlpuFsF83ieCBECDsSa7kOJBfoWA5HiGpHgRqmdu8ffz8E/AnsPvM4n4RT9/snb9iUwkMhqKfLHdRqF/n9mnougGIQBzpcdQooBAABLlZ2awgqCGFq+D/1CwuSn8AQSxmD/AtdCNFyCA0JXi3fg3u0QqOH9F1GaNVighMiM4Icc8DAnCkM/D13sf5ICBEAD21FQO05BPxII+3lYnfTEyFgpdytwdwu/EiP99SgYTCAhw/JMkOEkZVTSCNEPVz83m80nQ6bowLqaSU2Rm5GLBEd55SLbGAwGEwzfuIvixWAXXhyeoWn23VmfUg0gGg9VgtXXbgGUQSALIkJicQWeTeSyneQI/NZWcjidzszAxon7eZa+jX/y7K+ELnrsKafnATmbfs80SWsme727TWuEABjtps8Ep2Y1HZE9PT2+f/yLGsDgXn55OJJOzyeQUwZ+OJaFMkSAwxKnpxrPhCwNnL928s8nw/nOEoepbdTbfeatOb8WP1WyzOczkCO5x9Lp6+1x9IxeH4/n5eDyeTE7PJnNJ+JGEX2YSiTgeDjD1vr3eEh4Y8HjubOcBVfOzTR3VlMLYpreen3hZr9efRIVsNsxFJYJeV1/fSGBkZBjw44npJLHp6Xg8l0zgyQAELs7cQO259BOP58zVO+8pzK6mzk5QSI/28sT5bvgBDFBNWE+Y7h4noQB8MGCYTiSmp+cXc8l4bnYGFYr5MQKHmt50+qzntPfK6pFX6oD7YQcDNdqGwNaTQxPj3Ua8AJnxvBc0cgWG49NQMdBqhAEbezqfn8ol4WiLxyAFft7J2dlaacfZ02e8no9W3yTgev5S106pog0IQHxWb+djPgdcsAxmohFwRKGZklN4Ts7k44QBDdIwOzMDUyQWDfoDvBOuMtuk+z1nvF7v6l7LNHce6+oQSbe2EYlYqFLWMTQ+7uO5kxgHhDAUwGYYH/YHAoH4VEWmYagf0IcINE4U4uwmJd3o7fda2lcX6ijuF101shadpk0HbQD4cNkyc77YBObuYvyiE+5ByBDoAyNpAIsBSQLhYU6MYwCDcBlj1bK1sAlZuiKrJrUH8I++VCNvbdO1QZfhMGJNJoPVZOeG8Lz0wZVo6HwgEON7SK2OxKemUabhZB4KCSYR3JMwACfnMBs0sprjXoul49LtaVH2Hms/+krn+nq1TqPT6fSEwMAKCSbnDowMmEnRmT4Oa7WvLxDP54aH5+PD0zk8bqCCsAkwx3ZWU7++09ve3uW9TbDU3tVu8R6Do1iDBDojiQEZSJFWCJyTCR7wkSHQN5KcHRmZmp9KxtCikGG/DwOwG1htnXSPpb294+jtOl2G/c7b/xxdp75FwJJpJNSQg1wcnbEJp6OHExq6b2Q2OTISyydx8CVjw4Af4AUCIxDsfx6W3o7bt4sl2O9+2n8YLitCCBoMwWQ0CTNViMEZneRIv7l6e/v6eodn4oG+4akktMZsbhLSL2QAcswaFVKmvQuW6k9WNTIsdz/rpAkBmNaIMxuONVDJZBM6ORrtJv3W0+PGIIKzcVBqMpdDeaL+8QBfCcBg1G0DAsA/sioCwPcMHKO3VPDBhBhui+SbdJBIIAbA7x2JJWKQ61gsmYtHg8GgLxBA/G67mbXqFDSD+E2rCQD/wotSResKg5EYlCpcv1CnoUmuMvW4nh5HDxwOyREX1CvcNaaxhX0BfsgNAdhNVn3bFqnqJcDftYqg3/OTgfAJuu52BEQjkmi4wJgdk4NshQBlcsFqMAsEvQGo/3EhwUNuJ9cNRarX6xSUqvO5pubm2wTlAc9AOPTC45vVd2pkgHYjy9O4D0qWaIUULrwEzAR6e3t5rH+Cz7uFAEwGvW6zVNXZtKv5yGe3CUIDF8KhM49tBonUq0KAIPS4UvJv2oHAWpncHF5V/Qkec43lQ/AxACAwQwSazdLvwW7NdKya1xEI4PLZTfLtd0aAaQaf7BMO1mTFpgMGmwPvMXws6caCugPfARkAgtZ6imlq3rf3lVUXi2w4NHb5yjOyFo1auxKBlrQCpMHEnxeWY0LRgzcx3peIYboxGJ4kgCydZtDUqG2RUXD+MuJLqzfj8Njo26/tkW2tRLBjJQQW8uCYtLGkrQHA7nAO4lQKJgZ7MN284P+KQCbWqNdulUoAf684svphZXQ0krl6glZoNdqK/wKBAdotOAhzw4D1ZIUEuHHsDU36Ad7tcJBwhnBrht8Bn9UadQqpaD+zdyez+jz4LDOayWTP1tQSeVYlGRh6fCwrDFeDnUw9p8vpJ2MDRHcL+E4OM0AE0mg2U6J9e1Xi5tUX4HI2kkln394vUxp1AK0VZgUmwWjyd1cIDMLU45wudywqtISd452wY6H/UKIG+D80GiVNqRhY4zs+u2M9Bvxs4QXpFr3xtkGOTcaeCQN0NHysZE6QucolonacHHY7BuQk+sAUYo3gmkYhlQC+WOS583UL8FP/Hv7Teo0eT0uoTfiYcNhNcHrWpDcZBJcR340EBtLXZoeTE+Sp4Gs0UKRrQaAnqjN3rje/zKYKxfQBmZIVSnLF+vwGcjxXBp1gjpgP8W3QFPAL1o/NQBIABEoZVa36M/EG5q5ds5QtFIrFc4/X6QxmkxlLjmjgCL5sIpcK8tDidkC5u+ETHLfjfwHjowcmKMgDsULeNDqNpk4qVYnFIonn7h0fn/cW0s/IWwS8HuKxw+W3mSuOYy9xWJDwxUcd0BVoDiG9Br0Jy0+ngxSLxOJqkeg/vrAkF/B579WNtTrhGIZBwPVwUR95quC4QTdiD3ECgftNjsCbDQBvJ/JggnUada2UQnxJx80v7GiI//n7B2RbyQMIVgesyAHoqkHoVOhV8o0YEEbdZgPiw4RCeIIPp62mQUatQ3zJ2D2eST+8sby09OOn6jWVDRw2myF+lSEH7yZjh/PzBjN5uwOeFf+BoHUzRVWDPtT+/7nHnnzjxtLnVb89LNuGO7jwhIBjDU4rHw40NPcKAR8VigwrjuALASikVA3iU5fv+ZZDnj/f/Y5MaSIzgexmAO9HjsAKAcwiLM3oy4iO1ydjRR8gaKApKeBLqGf/cC+CP5Ln1c8HnpLvAAYnWWBxw/TjyPSRqenCsUkay/cmTDYWL7HIoBVaQE5RIsSnMvd5Ua4qPb+xVgPlhzkGhQiDL3BrbOLBgqUZ5bQ4mwm8lgTQWktRNYAvpY5//KXw5fKn5Xd+IKuDWnU4K0kAAkEhEKjiP5T+4PjK2NIKBFihUtE6iYQSX/ty/8ufLi0t/Wo3rTCa7ZUtnPevVJGQX4IPJTTuIEMXvmEDa3bUAf7aDZL7CwSJ/mR5+aPR3XCDMdkcXCWIlTIV5j4cjYDPOt5ktdo2TC987dhRBwkWEfxX7vsq+Bk+ry68/l26Vm0iQQDH0K1ji8ADPuZWe/68pk3bJhyyrajPug2gP9V0/6fTpY8WFv6zVBr7C7peaYSpR+a/cwj7uiIP6SwczazPWoHXNNRTFLUO9aeeKd3/4bS8/GGpVCoW//HwRnmDhmXN9ltDWlDfAO6bhM7tHm9rxfS2KmRCfsF/0a+/6um3vADwxULxX048Rtduh8NMOGoEcTC5pHPh3IbJNni+Ta1RKzfTFCUB/CcB/1+/+vG6XCr+rlBIvf+uZzctV6j1eGMhW4LJjvd5Io/GiINBrbFaW1sVcim+dhN9qlMP8vxeLgF+KpV95+9fXEPj8z7LVo59E7at3qjTVaTXVOBr1gr4qgfCB4YiPq++m3735/17HsM/fxwiI4HMNEBvq+CrW7cqvg3wUlG1iNTPgWLVg1opm02nr2bezrzxKlDQ9XUtag2st3qjVicYDB6lQi5D76vFIgG//ev8LWrhnXQ68/NI5I3XQ97Oxo00La/dslWpbFG3HjzYotzaoKiV0+j8OvHTcL5gfkXhqq9lnxcykcjl0dF/CIWueE4dXrNOKqVpmazyZy7Axmd0lUr1NIxn0bcoqvNa1de1Unp0dHQsFAqH8X3J8pfNTOPaGklNjQRMtFasYpi9cL9CfIraG3qYPwaWi5FQ+ArCn/W8err/3Kkfkb/4NoHt2YX3Z5WqGs5HmJ6XFqoezsqpywAP+Kf7z3hPnbLA/n684yWyfzXD9RDvP9Q6xvOw8GQ0lTLh0/2n+71eL8LD+g7rI+DvY1R7QR7p05bIN4EndiM75jnntfyoHbb3FXwGnBeLmc7wf31c9Qjsj8vvZ8bOei0dnYLvDLOvqeOn4cy1ctUjtPJyMZWOkNJ9K5368JFifxP7P3WlcRVYxi88AAAAAElFTkSuQmCC".getBytes()));
```

#### `ImageCreator`  <sub>(method)</sub>

> Creates a new instance of ImageCreator

```java
    public ImageCreator()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/Manager.java`


#### `Manager`  <sub>(class)</sub>

```java
public class Manager implements ProcessChecker.ProcessUpdate
```

#### `public static final int LIST_BASED = 0;`  <sub>(method)</sub>

```java
public static final int LIST_BASED = 0;
```

#### `public static final int LIST_BASED_FUZZ = 1;`  <sub>(method)</sub>

```java
public static final int LIST_BASED_FUZZ = 1;
```

#### `public static final int BRUTE_BASED = 2;`  <sub>(method)</sub>

```java
public static final int BRUTE_BASED = 2;
```

#### `public static final int BRUTE_BASED_FUZZ = 3;`  <sub>(method)</sub>

```java
public static final int BRUTE_BASED_FUZZ = 3;
```

#### `public BlockingQueue<WorkUnit> workQueue;`  <sub>(method)</sub>

```java
public BlockingQueue<WorkUnit> workQueue;
```

#### `public BlockingQueue<DirToCheck> dirQueue;`  <sub>(method)</sub>

```java
public BlockingQueue<DirToCheck> dirQueue;
```

#### `public BlockingQueue<HTMLparseWorkUnit> parseQue`  <sub>(method)</sub>

```java
public BlockingQueue<HTMLparseWorkUnit> parseQueue;
```

#### `public Thread workGenThread;`  <sub>(method)</sub>

```java
public Thread workGenThread;
```

#### `public WorkerGenerator workGen;`  <sub>(method)</sub>

```java
public WorkerGenerator workGen;
```

#### `public BruteForceWorkGenerator workGenBrute;`  <sub>(method)</sub>

```java
public BruteForceWorkGenerator workGenBrute;
```

#### `public WorkerGeneratorURLFuzz workGenFuzz;`  <sub>(method)</sub>

```java
public WorkerGeneratorURLFuzz workGenFuzz;
```

#### `public BruteForceURLFuzz workGenBruteFuzz;`  <sub>(method)</sub>

```java
public BruteForceURLFuzz workGenBruteFuzz;
```

#### `private String inputFile;`  <sub>(method)</sub>

```java
private String inputFile;
```

#### `private String firstPartOfURL;`  <sub>(method)</sub>

```java
private String firstPartOfURL;
```

#### `private String extention;`  <sub>(method)</sub>

```java
private String extention;
```

#### `private Timer timer;`  <sub>(method)</sub>

```java
private Timer timer;
```

#### `private ProcessChecker task;`  <sub>(method)</sub>

```java
private ProcessChecker task;
```

#### `private ProcessEnd task2;`  <sub>(method)</sub>

```java
private ProcessEnd task2;
```

#### `private String protocol;`  <sub>(method)</sub>

```java
private String protocol;
```

#### `private String host;`  <sub>(method)</sub>

```java
private String host;
```

#### `private int port;`  <sub>(method)</sub>

```java
private int port;
```

#### `private String startPoint;`  <sub>(method)</sub>

```java
private String startPoint;
```

#### `public StartGUI gui;`  <sub>(method)</sub>

```java
public StartGUI gui;
```

#### `private boolean doDirs,  doFiles;`  <sub>(method)</sub>

```java
private boolean doDirs,  doFiles;
```

#### `private int totalDone = 0;`  <sub>(method)</sub>

```java
private int totalDone = 0;
```

#### `Vector`  <sub>(method)</sub>

```java
private Vector workers = new Vector(100, 10);
```

#### `Vector`  <sub>(method)</sub>

```java
private Vector parseWorkers = new Vector(100, 10);
```

#### `private String[] charSet;`  <sub>(method)</sub>

```java
private String[] charSet;
```

#### `private int maxLen,  minLen;`  <sub>(method)</sub>

```java
private int maxLen,  minLen;
```

#### `private boolean auto = true;`  <sub>(method)</sub>

```java
private boolean auto = true;
```

#### `private double totalPass;`  <sub>(method)</sub>

```java
private double totalPass;
```

#### `private int totalDirsFound = 1;`  <sub>(method)</sub>

```java
private int totalDirsFound = 1;
```

#### `Vector`  <sub>(method)</sub>

```java
public Vector HTTPheaders = new Vector(10, 5);
```

#### `private boolean useProxy = false;`  <sub>(method)</sub>

```java
private boolean useProxy = false;
```

#### `private String proxyHost = "";`  <sub>(method)</sub>

```java
private String proxyHost = "";
```

#### `private int proxyPort = 0;`  <sub>(method)</sub>

```java
private int proxyPort = 0;
```

#### `private boolean useProxyAuth = false;`  <sub>(method)</sub>

```java
private boolean useProxyAuth = false;
```

#### `private String proxyUsername = "";`  <sub>(method)</sub>

```java
private String proxyUsername = "";
```

#### `private String proxyPassword = "";`  <sub>(method)</sub>

```java
private String proxyPassword = "";
```

#### `private String proxyRealm = "";`  <sub>(method)</sub>

```java
private String proxyRealm = "";
```

#### `private boolean blankExt = false;`  <sub>(method)</sub>

```java
private boolean blankExt = false;
```

#### `Vector`  <sub>(method)</sub>

```java
private Vector extToUse = new Vector(10, 5);
```

#### `private HttpClient httpclient;`  <sub>(method)</sub>

```java
private HttpClient httpclient;
```

#### `private HttpState initialState;`  <sub>(method)</sub>

```java
private HttpState initialState;
```

#### `private static Manager manager = null;`  <sub>(method)</sub>

```java
private static Manager manager = null;
```

#### `Vector`  <sub>(method)</sub>

```java
private Vector producedBasesCases = new Vector(10, 10);
```

#### `Vector`  <sub>(method)</sub>

```java
private Vector processedLinks = new Vector(100, 100);
```

#### `private int baseCaseCounterCorrection = 0;`  <sub>(method)</sub>

```java
private int baseCaseCounterCorrection = 0;
```

#### `private int workAmountCorrection = 0;`  <sub>(method)</sub>

```java
private int workAmountCorrection = 0;
```

#### `private int parsedLinksProcessed = 0;`  <sub>(method)</sub>

```java
private int parsedLinksProcessed = 0;
```

#### `private int numberOfBaseCasesProduced = 0;`  <sub>(method)</sub>

```java
private int numberOfBaseCasesProduced = 0;
```

#### `Vector`  <sub>(method)</sub>

```java
public Vector extsToMiss = new Vector(10, 10);
```

#### `Vector`  <sub>(method)</sub>

```java
public Vector elementsToParse = new Vector(10, 10);
```

#### `private String currentlyProcessing = "";`  <sub>(method)</sub>

```java
private String currentlyProcessing = "";
```

#### `private boolean useHTTPauth = false;`  <sub>(method)</sub>

```java
private boolean useHTTPauth = false;
```

#### `private String userName = "";`  <sub>(method)</sub>

```java
private String userName = "";
```

#### `private String password = "";`  <sub>(method)</sub>

```java
private String password = "";
```

#### `private String realmDomain = "";`  <sub>(method)</sub>

```java
private String realmDomain = "";
```

#### `private String authType = "";`  <sub>(method)</sub>

```java
private String authType = "";
```

#### `private String urlFuzzStart;`  <sub>(method)</sub>

```java
private String urlFuzzStart;
```

#### `private String urlFuzzEnd;`  <sub>(method)</sub>

```java
private String urlFuzzEnd;
```

#### `private boolean urlFuzzGenFinished = false;`  <sub>(method)</sub>

```java
private boolean urlFuzzGenFinished = false;
```

#### `private long timestarted;`  <sub>(method)</sub>

```java
private long timestarted;
```

#### `private boolean limitRequests = false;`  <sub>(method)</sub>

```java
private boolean limitRequests = false;
```

#### `private int limitRequestsTo = 50;`  <sub>(method)</sub>

```java
private int limitRequestsTo = 50;
```

#### `private boolean checkForUpdates = true;`  <sub>(method)</sub>

```java
private boolean checkForUpdates = true;
```

#### `private boolean failCaseDialogVisable = false;`  <sub>(method)</sub>

```java
private boolean failCaseDialogVisable = false;
```

#### `private int defaultNoThreads;`  <sub>(method)</sub>

```java
private int defaultNoThreads;
```

#### `private String defaultList;`  <sub>(method)</sub>

```java
private String defaultList;
```

#### `private String defaultExts;`  <sub>(method)</sub>

```java
private String defaultExts;
```

#### `isHeadLessMode`  <sub>(method)</sub>

```java
    public boolean isHeadLessMode()
```

#### `setHeadLessMode`  <sub>(method)</sub>

```java
    public void setHeadLessMode(boolean headLessMode)
```

#### `Vector`  <sub>(method)</sub>

```java
private Vector<String> failCaseRegexes = new Vector(10, 10);
```

#### `private boolean headLessMode = false;`  <sub>(method)</sub>

```java
private boolean headLessMode = false;
```

#### `private URL targetURL = null;`  <sub>(method)</sub>

```java
private URL targetURL = null;
```

#### `private String fileLocation = null;`  <sub>(method)</sub>

```java
private String fileLocation = null;
```

#### `private String reportLocation = null;`  <sub>(method)</sub>

```java
private String reportLocation = null;
```

#### `private String fileExtentions = null;`  <sub>(method)</sub>

```java
private String fileExtentions = null;
```

#### `private String pointToStartFrom = null;`  <sub>(method)</sub>

```java
private String pointToStartFrom = null;
```

#### `public Vector<ErrorTableObject> errors = new Vec`  <sub>(method)</sub>

```java
public Vector<ErrorTableObject> errors = new Vector<ErrorTableObject>(10,10);
```

#### `public Vector<MultiThreadedGenerator> genThreads`  <sub>(method)</sub>

```java
public Vector<MultiThreadedGenerator> genThreads = new Vector<MultiThreadedGenerator>(100,100);
```

#### `private int scanType;`  <sub>(method)</sub>

```java
private int scanType;
```

#### `private int numOfFiles = 0;`  <sub>(method)</sub>

```java
private int numOfFiles = 0;
```

#### `private int numOfDirs = 0;`  <sub>(method)</sub>

```java
private int numOfDirs = 0;
```

#### `public Vector<Result> results = new Vector<Resul`  <sub>(method)</sub>

```java
public Vector<Result> results = new Vector<Result>(100,100);
```

#### `Manager`  <sub>(method)</sub>

```java
    private Manager()
```

#### `getInstance`  <sub>(method)</sub>

```java
    public static Manager getInstance()
```

#### `setupManager`  <sub>(method)</sub>

```java
    public void setupManager(String startPoint,
                String inputFile,
                String protocol,
                String host,
                int port,
                String extention,
                StartGUI gui,
                int ThreadNumber,
                boolean doDirs,
                boolean doFiles,
                boolean recursive,
                boolean blankExt,
                Vector extToUse)
```

#### `getHttpclient`  <sub>(method)</sub>

```java
    public HttpClient getHttpclient()
```

#### `setupManager`  <sub>(method)</sub>

```java
    public void setupManager(String startPoint,
                String[] charSet,
                int minLen,
                int maxLen,
                String protocol,
                String host,
                int port,
                String extention,
                StartGUI gui,
                int ThreadNumber,
                boolean doDirs,
                boolean doFiles,
                boolean recursive,
                boolean blankExt,
                Vector extToUse)
```

#### `setUpManager`  <sub>(method)</sub>

> Used to setup the manager when we are URL fuzzing

```java
    public void setUpManager(String inputFile,
                String protocol,
                String host,
                int port,
                StartGUI gui,
                int ThreadNumber,
                String urlFuzzStart,
                String urlFuzzEnd)
```

#### `setUpManager`  <sub>(method)</sub>

> set up manager for bruteforce fuzzing

```java
    public void setUpManager(String[] charSet,
                int minLen,
                int maxLen,
                String protocol,
                String host,
                int port,
                StartGUI gui,
                int ThreadNumber,
                String urlFuzzStart,
                String urlFuzzEnd)
```

#### `createHttpClient`  <sub>(method)</sub>

```java
    private void createHttpClient()
```

#### `setpUpHttpClient`  <sub>(method)</sub>

```java
    private void setpUpHttpClient()
```

#### `createTheThreads`  <sub>(method)</sub>

```java
    private void createTheThreads()
```

#### `start`  <sub>(method)</sub>

```java
    public void start()
```

#### `hasWorkLeft`  <sub>(method)</sub>

```java
    public boolean hasWorkLeft()
```

#### `getWorkQueue`  <sub>(method)</sub>

```java
    public BlockingQueue<WorkUnit> getWorkQueue()
```

#### `getDirQueue`  <sub>(method)</sub>

```java
    public BlockingQueue<DirToCheck> getDirQueue()
```

#### `foundDir`  <sub>(method)</sub>

```java
    public synchronized void foundDir(URL url, int statusCode, String responseHeader, String responseBody, BaseCase baseCaseObj)
```

#### `updateTable`  <sub>(method)</sub>

```java
    public void updateTable(String finished, String started)
```

#### `foundFile`  <sub>(method)</sub>

> public void updateTable(String finished, String started)  
> {  
> if(!headLessMode)  
> {  
> gui.updateTable(finished, started);  
> }  
> }

```java
    public synchronized void foundFile(URL url, int statusCode, String responseHeader, String responseBody, BaseCase baseCaseObj)
```

#### `foundError`  <sub>(method)</sub>

```java
    public synchronized void foundError(URL url, String reason)
```

#### `updateProgress`  <sub>(method)</sub>

```java
    public void updateProgress(String current, String average, String total, String timeLeft, String parseQueueLength)
```

#### `getInputFile`  <sub>(method)</sub>

```java
    public String getInputFile()
```

#### `getFirstPartOfURL`  <sub>(method)</sub>

```java
    public String getFirstPartOfURL()
```

#### `getFileExtention`  <sub>(method)</sub>

```java
    public String getFileExtention()
```

#### `isAlive`  <sub>(method)</sub>

```java
    public void isAlive()
```

#### `workDone`  <sub>(method)</sub>

```java
    public synchronized void workDone()
```

#### `getTotalDone`  <sub>(method)</sub>

```java
    public synchronized int getTotalDone()
```

#### `getProtocol`  <sub>(method)</sub>

```java
    public String getProtocol()
```

#### `getHost`  <sub>(method)</sub>

```java
    public String getHost()
```

#### `getPort`  <sub>(method)</sub>

```java
    public int getPort()
```

#### `getDoDirs`  <sub>(method)</sub>

```java
    public boolean getDoDirs()
```

#### `getDoFiles`  <sub>(method)</sub>

```java
    public boolean getDoFiles()
```

#### `pause`  <sub>(method)</sub>

```java
    public void pause()
```

#### `pauseAllWorkGen`  <sub>(method)</sub>

```java
    public void pauseAllWorkGen()
```

#### `unPause`  <sub>(method)</sub>

```java
    public void unPause()
```

#### `unPauseAllWorkGen`  <sub>(method)</sub>

```java
    public void unPauseAllWorkGen()
```

#### `pauseWorkGen`  <sub>(method)</sub>

> pause one of the work gen threads

```java
    public void pauseWorkGen(int id)
```

#### `unPauseWorkGen`  <sub>(method)</sub>

> unpause one of the work gen threads

```java
    public void unPauseWorkGen(int id)
```

#### `setStatus`  <sub>(method)</sub>

```java
    public void setStatus(String status)
```

#### `getMinLen`  <sub>(method)</sub>

```java
    public int getMinLen()
```

#### `getMaxLen`  <sub>(method)</sub>

```java
    public int getMaxLen()
```

#### `getCharSet`  <sub>(method)</sub>

```java
    public String[] getCharSet()
```

#### `isRecursive`  <sub>(method)</sub>

```java
    public boolean isRecursive()
```

#### `youAreFinished`  <sub>(method)</sub>

```java
    public void youAreFinished()
```

#### `getTotalPass`  <sub>(method)</sub>

```java
    public double getTotalPass()
```

#### `setTotalPass`  <sub>(method)</sub>

```java
    public void setTotalPass(double totalPass)
```

#### `getTotalDirsFound`  <sub>(method)</sub>

```java
    public int getTotalDirsFound()
```

#### `getTotalFilesFound`  <sub>(method)</sub>

```java
    public int getTotalFilesFound()
```

#### `getWorkerCount`  <sub>(method)</sub>

```java
    public int getWorkerCount()
```

#### `getWorkers`  <sub>(method)</sub>

```java
    public Vector getWorkers()
```

#### `getAuto`  <sub>(method)</sub>

```java
    public boolean getAuto()
```

#### `setAuto`  <sub>(method)</sub>

```java
    public void setAuto(boolean b)
```

#### `addWrokers`  <sub>(method)</sub>

> used to add extra workers to the queue

```java
    public void addWrokers(int number)
```

#### `removeWorkers`  <sub>(method)</sub>

> used to remove extra workers from the queue

```java
    public void removeWorkers(int number)
```

#### `removeFromDirQueue`  <sub>(method)</sub>

```java
    public synchronized void removeFromDirQueue(String dir)
```

#### `addToDirQueue`  <sub>(method)</sub>

```java
    public synchronized void addToDirQueue(String dir)
```

#### `addHTMLToParseQueue`  <sub>(method)</sub>

```java
    public synchronized void addHTMLToParseQueue(HTMLparseWorkUnit parseWorkUnit)
```

#### `isUseProxy`  <sub>(method)</sub>

```java
    public boolean isUseProxy()
```

#### `getProxyHost`  <sub>(method)</sub>

```java
    public String getProxyHost()
```

#### `setProxyHost`  <sub>(method)</sub>

```java
    public void setProxyHost(String proxyHost)
```

#### `setUseProxy`  <sub>(method)</sub>

```java
    public void setUseProxy(boolean useProxy)
```

#### `getProxyPort`  <sub>(method)</sub>

```java
    public int getProxyPort()
```

#### `setProxyPort`  <sub>(method)</sub>

```java
    public void setProxyPort(int proxyPort)
```

#### `isUseProxyAuth`  <sub>(method)</sub>

```java
    public boolean isUseProxyAuth()
```

#### `getProxyUsername`  <sub>(method)</sub>

```java
    public String getProxyUsername()
```

#### `getProxyPassword`  <sub>(method)</sub>

```java
    public String getProxyPassword()
```

#### `getProxyRealm`  <sub>(method)</sub>

```java
    public String getProxyRealm()
```

#### `setProxyPassword`  <sub>(method)</sub>

```java
    public void setProxyPassword(String proxyPassword)
```

#### `setProxyRealm`  <sub>(method)</sub>

```java
    public void setProxyRealm(String proxyRealm)
```

#### `setProxyUsername`  <sub>(method)</sub>

```java
    public void setProxyUsername(String proxyUsername)
```

#### `setUseProxyAuth`  <sub>(method)</sub>

```java
    public void setUseProxyAuth(boolean useProxyAuth)
```

#### `isBlankExt`  <sub>(method)</sub>

```java
    public boolean isBlankExt()
```

#### `addHTTPheader`  <sub>(method)</sub>

```java
    public void addHTTPheader(HTTPHeader header)
```

#### `getHTTPHeaders`  <sub>(method)</sub>

```java
    public Vector getHTTPHeaders()
```

#### `addExt`  <sub>(method)</sub>

```java
    public void addExt(ExtToCheck ext)
```

#### `getExtToUse`  <sub>(method)</sub>

```java
    public Vector getExtToUse()
```

#### `getBaseCase`  <sub>(method)</sub>

```java
    public synchronized BaseCase getBaseCase(String base, boolean isDir, String fileExt)
```

#### `addBaseCase`  <sub>(method)</sub>

```java
    public synchronized void addBaseCase(BaseCase baseCase)
```

#### `hasLinkBeenDone`  <sub>(method)</sub>

```java
    public synchronized boolean hasLinkBeenDone(String link)
```

#### `getBaseCaseCounterCorrection`  <sub>(method)</sub>

```java
    public int getBaseCaseCounterCorrection()
```

#### `getParsedLinksProcessed`  <sub>(method)</sub>

```java
    public int getParsedLinksProcessed()
```

#### `addParsedLink`  <sub>(method)</sub>

```java
    public synchronized boolean addParsedLink(String link)
```

#### `addParsedLinksProcessed`  <sub>(method)</sub>

```java
    public synchronized void addParsedLinksProcessed()
```

#### `getNumberOfBaseCasesProduced`  <sub>(method)</sub>

```java
    public int getNumberOfBaseCasesProduced()
```

#### `addBaseCaseCounterCorrection`  <sub>(method)</sub>

```java
    public synchronized void addBaseCaseCounterCorrection()
```

#### `getElementsToParse`  <sub>(method)</sub>

```java
    public Vector getElementsToParse()
```

#### `addNumberOfBaseCasesProduced`  <sub>(method)</sub>

```java
    public synchronized void addNumberOfBaseCasesProduced()
```

#### `getParseWorkers`  <sub>(method)</sub>

```java
    public Vector getParseWorkers()
```

#### `skipCurrentWork`  <sub>(method)</sub>

```java
    public void skipCurrentWork()
```

#### `setCurrentlyProcessing`  <sub>(method)</sub>

```java
    public void setCurrentlyProcessing(String currentlyProcessing)
```

#### `addToWorkCorrection`  <sub>(method)</sub>

```java
    public void addToWorkCorrection(int amount)
```

#### `getAuthType`  <sub>(method)</sub>

```java
    public String getAuthType()
```

#### `getPassword`  <sub>(method)</sub>

```java
    public String getPassword()
```

#### `getRealmDomain`  <sub>(method)</sub>

```java
    public String getRealmDomain()
```

#### `isUseHTTPauth`  <sub>(method)</sub>

```java
    public boolean isUseHTTPauth()
```

#### `getUserName`  <sub>(method)</sub>

```java
    public String getUserName()
```

#### `getWorkAmountCorrection`  <sub>(method)</sub>

```java
    public int getWorkAmountCorrection()
```

#### `setAuthDetails`  <sub>(method)</sub>

```java
    public void setAuthDetails(String username, String password, String realmDomain, String type)
```

#### `setDoNotUseAuth`  <sub>(method)</sub>

```java
    public void setDoNotUseAuth()
```

#### `getUrlFuzzEnd`  <sub>(method)</sub>

```java
    public String getUrlFuzzEnd()
```

#### `getUrlFuzzStart`  <sub>(method)</sub>

```java
    public String getUrlFuzzStart()
```

#### `isURLFuzzGenFinished`  <sub>(method)</sub>

```java
    public boolean isURLFuzzGenFinished()
```

#### `setURLFuzzGenFinished`  <sub>(method)</sub>

```java
    public void setURLFuzzGenFinished(boolean urlFuzzGenFinished)
```

#### `getTimestarted`  <sub>(method)</sub>

```java
    public long getTimestarted()
```

#### `isLimitRequests`  <sub>(method)</sub>

```java
    public boolean isLimitRequests()
```

#### `setLimitRequests`  <sub>(method)</sub>

```java
    public void setLimitRequests(boolean limitRequests)
```

#### `getLimitRequestsTo`  <sub>(method)</sub>

```java
    public int getLimitRequestsTo()
```

#### `setLimitRequestsTo`  <sub>(method)</sub>

```java
    public void setLimitRequestsTo(int limitRequestsTo)
```

#### `addToTree`  <sub>(method)</sub>

> add items to the table tree view

```java
    private void addToTree(Result result)
```

#### `areWorkersAlive`  <sub>(method)</sub>

```java
    public boolean areWorkersAlive()
```

#### `loadPrefs`  <sub>(method)</sub>

> this loads the user prefs for set for DirBuster

```java
    public void loadPrefs()
```

#### `isCheckForUpdates`  <sub>(method)</sub>

```java
    public boolean isCheckForUpdates()
```

#### `setCheckForUpdates`  <sub>(method)</sub>

```java
    public void setCheckForUpdates(boolean checkForUpdates)
```

#### `isFailCaseDialogVisable`  <sub>(method)</sub>

```java
    public boolean isFailCaseDialogVisable()
```

#### `setFailCaseDialogVisable`  <sub>(method)</sub>

```java
    public void setFailCaseDialogVisable(boolean failCaseDialogVisable)
```

#### `checkForUpdates`  <sub>(method)</sub>

> function to check for updates

```java
    public void checkForUpdates(boolean informUser)
```

#### `getFailCaseRegexes`  <sub>(method)</sub>

> returns a vector of all the regexes we have already used

```java
    public Vector<String> getFailCaseRegexes()
```

#### `addFailCaseRegex`  <sub>(method)</sub>

> adds a new regex fail case

```java
    public void addFailCaseRegex(String regex)
```

#### `getTargetURL`  <sub>(method)</sub>

```java
    public URL getTargetURL()
```

#### `setTargetURL`  <sub>(method)</sub>

```java
    public void setTargetURL(URL targetURL)
```

#### `getFileLocation`  <sub>(method)</sub>

```java
    public String getFileLocation()
```

#### `setFileLocation`  <sub>(method)</sub>

```java
    public void setFileLocation(String fileLocation)
```

#### `getReportLocation`  <sub>(method)</sub>

```java
    public String getReportLocation()
```

#### `setReportLocation`  <sub>(method)</sub>

```java
    public void setReportLocation(String reportLocation)
```

#### `getFileExtentions`  <sub>(method)</sub>

```java
    public String getFileExtentions()
```

#### `setFileExtentions`  <sub>(method)</sub>

```java
    public void setFileExtentions(String fileExtentions)
```

#### `getPointToStartFrom`  <sub>(method)</sub>

```java
    public String getPointToStartFrom()
```

#### `setPointToStartFrom`  <sub>(method)</sub>

```java
    public void setPointToStartFrom(String pointToStartFrom)
```

#### `getDefaultExts`  <sub>(method)</sub>

```java
    public String getDefaultExts()
```

#### `setDefaultExts`  <sub>(method)</sub>

```java
    public void setDefaultExts(String defaultExts)
```

#### `getDefaultList`  <sub>(method)</sub>

```java
    public String getDefaultList()
```

#### `setDefaultList`  <sub>(method)</sub>

```java
    public void setDefaultList(String defaultList)
```

#### `getDefaultNoThreads`  <sub>(method)</sub>

```java
    public int getDefaultNoThreads()
```

#### `setDefaultNoThreads`  <sub>(method)</sub>

```java
    public void setDefaultNoThreads(int defaultNoThreads)
```

#### `resetConErrorCounter`  <sub>(method)</sub>

```java
    public void resetConErrorCounter()
```

#### `getScanType`  <sub>(method)</sub>

```java
    public int getScanType()
```

#### `setGUI`  <sub>(method)</sub>

```java
    public void setGUI(StartGUI gui)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/Result.java`


#### `Result`  <sub>(class)</sub>

```java
public class Result
```

#### `public static final int DIR = 0;`  <sub>(method)</sub>

```java
public static final int DIR = 0;
```

#### `public static final int FILE = 1;`  <sub>(method)</sub>

```java
public static final int FILE = 1;
```

#### `private int type = -1;`  <sub>(method)</sub>

```java
private int type = -1;
```

#### `private URL itemFound;`  <sub>(method)</sub>

```java
private URL itemFound;
```

#### `private int responceCode = 0;`  <sub>(method)</sub>

```java
private int responceCode = 0;
```

#### `private String responseHeader = "";`  <sub>(method)</sub>

```java
private String responseHeader = "";
```

#### `private String responseBody = "";`  <sub>(method)</sub>

```java
private String responseBody = "";
```

#### `private BaseCase baseCaseObj = null;`  <sub>(method)</sub>

```java
private BaseCase baseCaseObj = null;
```

#### `Result`  <sub>(method)</sub>

```java
    public Result(int type, URL itemFound, int responceCode, String responceHeader, String responseBody, BaseCase baseCaseObj)
```

#### `getBaseCaseObj`  <sub>(method)</sub>

```java
    public BaseCase getBaseCaseObj()
```

#### `getItemFound`  <sub>(method)</sub>

```java
    public URL getItemFound()
```

#### `getResponseBody`  <sub>(method)</sub>

```java
    public String getResponseBody()
```

#### `getResponseHeader`  <sub>(method)</sub>

```java
    public String getResponseHeader()
```

#### `getResponceCode`  <sub>(method)</sub>

```java
    public int getResponceCode()
```

#### `getType`  <sub>(method)</sub>

```java
    public int getType()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/Start.java`


#### `Start`  <sub>(class)</sub>

```java
public class Start
```

#### `main`  <sub>(method)</sub>

```java
    public static void main(String[] args)
```

#### `run`  <sub>(method)</sub>

```java
                public void run()
```

#### `run`  <sub>(method)</sub>

```java
                    public void run()
```

#### `printUsage`  <sub>(method)</sub>

```java
    public static void printUsage()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/WorkUnit.java`


#### `WorkUnit`  <sub>(class)</sub>

```java
public class WorkUnit
```

#### `private URL urlToGet;`  <sub>(method)</sub>

```java
private URL urlToGet;
```

#### `private boolean isDir;`  <sub>(method)</sub>

```java
private boolean isDir;
```

#### `private String method;`  <sub>(method)</sub>

```java
private String method;
```

#### `private BaseCase baseCaseObj;`  <sub>(method)</sub>

```java
private BaseCase baseCaseObj;
```

#### `private String itemToCheck;`  <sub>(method)</sub>

```java
private String itemToCheck;
```

#### `WorkUnit`  <sub>(method)</sub>

> Creates a new instance of WorkUnit

```java
    public WorkUnit(URL url, boolean isDir, String method, BaseCase baseCaseObj, String itemToCheck)
```

#### `getWork`  <sub>(method)</sub>

```java
    public URL getWork()
```

#### `isDir`  <sub>(method)</sub>

```java
    public boolean isDir()
```

#### `getMethod`  <sub>(method)</sub>

```java
    public String getMethod()
```

#### `getBaseCaseObj`  <sub>(method)</sub>

```java
    public BaseCase getBaseCaseObj()
```

#### `getItemToCheck`  <sub>(method)</sub>

```java
    public String getItemToCheck()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/Worker.java`


#### `Worker`  <sub>(class)</sub>

> This class process workunit and determines if the link has been found or not

```java
public class Worker implements Runnable
```

#### `private BlockingQueue<WorkUnit> queue;`  <sub>(method)</sub>

```java
private BlockingQueue<WorkUnit> queue;
```

#### `private URL url;`  <sub>(method)</sub>

```java
private URL url;
```

#### `private WorkUnit work;`  <sub>(method)</sub>

```java
private WorkUnit work;
```

#### `private Manager manager;`  <sub>(method)</sub>

```java
private Manager manager;
```

#### `private HttpClient httpclient;`  <sub>(method)</sub>

```java
private HttpClient httpclient;
```

#### `private boolean pleaseWait = false;`  <sub>(method)</sub>

```java
private boolean pleaseWait = false;
```

#### `private int threadId;`  <sub>(method)</sub>

```java
private int threadId;
```

#### `private boolean working;`  <sub>(method)</sub>

```java
private boolean working;
```

#### `private boolean stop = false;`  <sub>(method)</sub>

```java
private boolean stop = false;
```

#### `Worker`  <sub>(method)</sub>

> Creates a new instance of Worker

```java
    public Worker(int threadId)
```

#### `run`  <sub>(method)</sub>

> Run method of the thread

```java
    public void run()
```

#### `pause`  <sub>(method)</sub>

> Method to call to pause the thread

```java
    public void pause()
```

#### `unPause`  <sub>(method)</sub>

> Method to call to unpause the thread

```java
    public void unPause()
```

#### `isWorking`  <sub>(method)</sub>

> Return a boolean based on if the thread is working

```java
    public boolean isWorking()
```

#### `stopThread`  <sub>(method)</sub>

> Method to call to stop the thread

```java
    public void stopThread()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/urlTest.java`


#### `urlTest`  <sub>(class)</sub>

```java
public class urlTest
```

#### `urlTest`  <sub>(method)</sub>

> Creates a new instance of urlTest

```java
    public urlTest()
```

#### `main`  <sub>(method)</sub>

```java
    public static void main(String[] args)
```

#### `convertSecsToTime`  <sub>(method)</sub>

```java
    private static String convertSecsToTime(long secs)
```


## `src/com/sittinglittleduck/DirBuster/bruteForceIterator` <a id='pkg-src-com-sittinglittleduck-dirbuster-bruteforceiterator'></a>

*24 documented items*


### `dirbuster/src/com/sittinglittleduck/DirBuster/bruteForceIterator/BruteForceGenThread.java`


#### `BruteForceGenThread`  <sub>(class)</sub>

```java
public class BruteForceGenThread implements Runnable
```

#### `private int min;`  <sub>(method)</sub>

```java
private int min;
```

#### `private int max;`  <sub>(method)</sub>

```java
private int max;
```

#### `private String[] list;`  <sub>(method)</sub>

```java
private String[] list;
```

#### `private BlockingQueue<String> queue;`  <sub>(method)</sub>

```java
private BlockingQueue<String> queue;
```

#### `private int[] listindex;`  <sub>(method)</sub>

```java
private int[] listindex;
```

#### `BruteForceGenThread`  <sub>(method)</sub>

```java
    public BruteForceGenThread(int min, int max, String[] list, BlockingQueue<String> queue)
```

#### `run`  <sub>(method)</sub>

```java
    public void run()
```

#### `makeList`  <sub>(method)</sub>

```java
    private void makeList(int minLen, int maxLen)
```

#### `showString`  <sub>(method)</sub>

```java
    private void showString(int len)
```

#### `incrementCounter`  <sub>(method)</sub>

```java
    private void incrementCounter(int len)
```

#### `initIndex`  <sub>(method)</sub>

```java
    private void initIndex()
```

#### `isWorking`  <sub>(method)</sub>

```java
    public boolean isWorking()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/bruteForceIterator/BruteForceIterator.java`


#### `BruteForceIterator`  <sub>(class)</sub>

```java
public class BruteForceIterator implements Iterator<String>
```

#### `private int min;`  <sub>(method)</sub>

```java
private int min;
```

#### `private int max;`  <sub>(method)</sub>

```java
private int max;
```

#### `private String[] list;`  <sub>(method)</sub>

```java
private String[] list;
```

#### `private BlockingQueue<String> queue;`  <sub>(method)</sub>

```java
private BlockingQueue<String> queue;
```

#### `private BruteForceGenThread genThread;`  <sub>(method)</sub>

```java
private BruteForceGenThread genThread;
```

#### `BruteForceIterator`  <sub>(method)</sub>

```java
    public BruteForceIterator(int min, int max, String[] list)
```

#### `hasNext`  <sub>(method)</sub>

```java
    public boolean hasNext()
```

#### `next`  <sub>(method)</sub>

```java
    public String next()
```

#### `remove`  <sub>(method)</sub>

```java
    public void remove()
```

#### `getTotalToDo`  <sub>(method)</sub>

```java
    public double getTotalToDo()
```


## `src/com/sittinglittleduck/DirBuster/diff` <a id='pkg-src-com-sittinglittleduck-dirbuster-diff'></a>

*99 documented items*


### `dirbuster/src/com/sittinglittleduck/DirBuster/diff/Diff.java`


#### `Diff`  <sub>(class)</sub>

> A class to compare vectors of objects.  The result of comparison  
> is a list of <code>change</code> objects which form an  
> edit script.  The objects compared are traditionally lines  
> of text from two files.  Comparison options such as "ignore  
> whitespace" are implemented by modifying the <code>equals</code>  
> and <code>hashcode</code> methods for the objects compared.  
> <p>  
> The basic algorithm is described in: </br>  
> "An O(ND) Difference Algorithm and its Variations", Eugene Myers,  
> Algorithmica Vol. 1 No. 2, 1986, p 251.  
> <p>  
> This class outputs different results from GNU diff 1.15 on some  
> inputs.  Our results are actually better (smaller change list, smaller  
> total size of changes), but it would be nice to know why.  Perhaps  
> there is a memory overwrite bug in GNU diff 1.15.  
> Copyright (C) 2000  Business Management Systems, Inc.  
> <p>  
> This program is free software; you can redistribute it and/or modify  
> it under the terms of the GNU General Public License as published by  
> the Free Software Foundation; either version 1, or (at your option)  
> any later version.  
> <p>  
> This program is distributed in the hope that it will be useful,  
> but WITHOUT ANY WARRANTY; without even the implied warranty of  
> MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the  
> GNU General Public License for more details.  
> <p>  
> You should have received a copy of the <a href=COPYING.txt>  
> GNU General Public License</a>  
> along with this program; if not, write to the Free Software  
> Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.

```java
public class Diff
```

#### `Diff`  <sub>(method)</sub>

> Prepare to find differences between two arrays.  Each element of  
> the arrays is translated to an "equivalence number" based on  
> the result of <code>equals</code>.  The original Object arrays  
> are no longer needed for computing the differences.  They will  
> be needed again later to print the results of the comparison as  
> an edit script, if desired.

```java
  public Diff(Object[] a,Object[] b)
```

#### `private int equiv_max = 1;`  <sub>(method)</sub>

```java
private int equiv_max = 1;
```

#### `public boolean heuristic = false;`  <sub>(method)</sub>

```java
public boolean heuristic = false;
```

#### `public boolean no_discards = false;`  <sub>(method)</sub>

```java
public boolean no_discards = false;
```

#### `private int[] xvec, yvec;	/* Vectors being compa`  <sub>(method)</sub>

```java
private int[] xvec, yvec;	/* Vectors being compared. */
```

#### `private int[] fdiag;		/* Vector, indexed by diag`  <sub>(method)</sub>

```java
private int[] fdiag;		/* Vector, indexed by diagonal, containing
```

#### `private int[] bdiag;		/* Vector, indexed by diag`  <sub>(method)</sub>

```java
private int[] bdiag;		/* Vector, indexed by diagonal, containing
```

#### `private int fdiagoff, bdiagoff;`  <sub>(method)</sub>

```java
private int fdiagoff, bdiagoff;
```

#### `private final file_data[] filevec = new file_dat`  <sub>(method)</sub>

```java
private final file_data[] filevec = new file_data[2];
```

#### `private int cost;`  <sub>(method)</sub>

```java
private int cost;
```

#### `diag`  <sub>(method)</sub>

> Find the midpoint of the shortest edit script for a specified  
> portion of the two files.  
> We scan from the beginnings of the files, and simultaneously from the ends,  
> doing a breadth-first search through the space of edit-sequence.  
> When the two searches meet, we have found the midpoint of the shortest  
> edit sequence.  
> The value returned is the number of the diagonal on which the midpoint lies.  
> The diagonal number equals the number of inserted lines minus the number  
> of deleted lines (counting only lines before the midpoint).  
> The edit cost is stored into COST; this is the total number of  
> lines inserted or deleted (counting only lines before the midpoint).  
> This function assumes that the first lines of the specified portions  
> of the two files do not match, and likewise that the last lines do not  
> match.  The caller must trim matching lines from the beginning and end  
> of the portions it is going to specify.  
> Note that if we return the "wrong" diagonal value, or if  
> the value of bdiag at that diagonal is "wrong",  
> the worst this can do is cause suboptimal diff output.  
> It cannot cause incorrect diff output.

```java
  private int diag (int xoff, int xlim, int yoff, int ylim)
```

#### `final int[] fd = fdiag;	// Give the compiler a c`  <sub>(method)</sub>

```java
final int[] fd = fdiag;	// Give the compiler a chance.
```

#### `final int[] bd = bdiag;	// Additional help for t`  <sub>(method)</sub>

```java
final int[] bd = bdiag;	// Additional help for the compiler.
```

#### `final int[] xv = xvec;		// Still more help for t`  <sub>(method)</sub>

```java
final int[] xv = xvec;		// Still more help for the compiler.
```

#### `final int[] yv = yvec;		// And more and more . .`  <sub>(method)</sub>

```java
final int[] yv = yvec;		// And more and more . . .
```

#### `final int dmin = xoff - ylim;	// Minimum valid d`  <sub>(method)</sub>

```java
final int dmin = xoff - ylim;	// Minimum valid diagonal.
```

#### `final int dmax = xlim - yoff;	// Maximum valid d`  <sub>(method)</sub>

```java
final int dmax = xlim - yoff;	// Maximum valid diagonal.
```

#### `final int fmid = xoff - yoff;	// Center diagonal`  <sub>(method)</sub>

```java
final int fmid = xoff - yoff;	// Center diagonal of top-down search.
```

#### `final int bmid = xlim - ylim;	// Center diagonal`  <sub>(method)</sub>

```java
final int bmid = xlim - ylim;	// Center diagonal of bottom-up search.
```

#### `final boolean odd = (fmid - bmid & 1) != 0;`  <sub>(method)</sub>

```java
final boolean odd = (fmid - bmid & 1) != 0;
```

#### `compareseq`  <sub>(method)</sub>

> Compare in detail contiguous subsequences of the two files  
> which are known, as a whole, to match each other.  
> The results are recorded in the vectors filevec[N].changed_flag, by  
> storing a 1 in the element for each line that is an insertion or deletion.  
> The subsequence of file 0 is [XOFF, XLIM) and likewise for file 1.  
> Note that XLIM, YLIM are exclusive bounds.  
> All line numbers are origin-0 and discarded lines are not counted.

```java
  private void compareseq (int xoff, int xlim, int yoff, int ylim)
```

#### `discard_confusing_lines`  <sub>(method)</sub>

> Discard lines from one file that have no matches in the other file.

```java
  private void discard_confusing_lines()
```

#### `private boolean inhibit = false;`  <sub>(method)</sub>

```java
private boolean inhibit = false;
```

#### `shift_boundaries`  <sub>(method)</sub>

> Adjust inserts/deletes of blank lines to join changes  
> as much as possible.

```java
  private void shift_boundaries()
```

#### `ScriptBuilder`  <sub>(interface)</sub>

```java
public interface ScriptBuilder
```

#### `ReverseScript`  <sub>(class)</sub>

> Scan the tables of which lines are inserted and deleted,  
> producing an edit script in reverse order.

```java
static class ReverseScript implements ScriptBuilder
```

#### `build_script`  <sub>(method)</sub>

```java
    public  change build_script(
    	final boolean[] changed0,int len0,
    	final boolean[] changed1,int len1)
```

#### `ForwardScript`  <sub>(class)</sub>

```java
static class ForwardScript implements ScriptBuilder
```

#### `build_script`  <sub>(method)</sub>

> Scan the tables of which lines are inserted and deleted,  
> producing an edit script in forward order.

```java
    public change build_script(
    	  final boolean[] changed0,int len0,
    	  final boolean[] changed1,int len1)
```

#### `diff_2`  <sub>(method)</sub>

> Report the differences of two files.  DEPTH is the current directory  
> depth.

```java
  public final change diff_2(final boolean reverse)
```

#### `diff`  <sub>(method)</sub>

> Get the results of comparison as an edit script.  The script  
> is described by a list of changes.  The standard ScriptBuilder  
> implementations provide for forward and reverse edit scripts.  
> Alternate implementations could, for instance, list common elements  
> instead of differences.

```java
  public change diff(final ScriptBuilder bld)
```

#### `change`  <sub>(class)</sub>

> The result of comparison is an "edit script": a chain of change objects.  
> Each change represents one place where some lines are deleted  
> and some are inserted.  
> LINE0 and LINE1 are the first affected lines in the two files (origin 0).  
> DELETED is the number of lines deleted here from file 0.  
> INSERTED is the number of lines inserted here in file 1.  
> If DELETED is 0 then LINE0 is the number of the line before  
> which the insertion was done; vice versa for INSERTED and LINE1.

```java
public static class change
```

#### `public change link;`  <sub>(method)</sub>

```java
public change link;
```

#### `public final int inserted;`  <sub>(method)</sub>

```java
public final int inserted;
```

#### `public final int deleted;`  <sub>(method)</sub>

```java
public final int deleted;
```

#### `public final int line0;`  <sub>(method)</sub>

```java
public final int line0;
```

#### `public final int line1;`  <sub>(method)</sub>

```java
public final int line1;
```

#### `change`  <sub>(method)</sub>

> Cons an additional entry onto the front of an edit script OLD.  
> LINE0 and LINE1 are the first affected lines in the two files (origin 0).  
> DELETED is the number of lines deleted here from file 0.  
> INSERTED is the number of lines inserted here in file 1.  
> If DELETED is 0 then LINE0 is the number of the line before  
> which the insertion was done; vice versa for INSERTED and LINE1.

```java
    public change(int line0, int line1, int deleted, int inserted, change old)
```

#### `file_data`  <sub>(class)</sub>

> Data on one input file being compared.

```java
class file_data
```

#### `clear`  <sub>(method)</sub>

> Allocate changed array for the results of comparison.

```java
    void clear()
```

#### `equivCount`  <sub>(method)</sub>

> Return equiv_count[I] as the number of lines in this file  
> that fall in equivalence class I.

```java
    int[] equivCount()
```

#### `discard_confusing_lines`  <sub>(method)</sub>

> Discard lines that have no matches in another file.  
> A line which is discarded will not be considered by the actual  
> comparison algorithm; it will be as if that line were not in the file.  
> The file's `realindexes' table maps virtual line numbers  
> (which don't count the discarded lines) into real line numbers;  
> this is how the actual comparison algorithm produces results  
> that are comprehensible when the discarded lines are counted.  
> <p>  
> When we discard a line, we also mark it as a deletion or insertion  
> so that it will be printed in the output.

```java
    void discard_confusing_lines(file_data f)
```

#### `discardable`  <sub>(method)</sub>

```java
final byte[] discarded = discardable(f.equivCount());
```

#### `discardable`  <sub>(method)</sub>

> Mark to be discarded each line that matches no line of another file.  
> If a line matches many lines, mark it as provisionally discardable.  
> for each line

```java
    private byte[] discardable(final int[] counts)
```

#### `final int end = buffered_lines;`  <sub>(method)</sub>

```java
final int end = buffered_lines;
```

#### `final byte[] discards = new byte[end];`  <sub>(method)</sub>

```java
final byte[] discards = new byte[end];
```

#### `final int[] equivs = this.equivs;`  <sub>(method)</sub>

```java
final int[] equivs = this.equivs;
```

#### `filterDiscards`  <sub>(method)</sub>

> Don't really discard the provisional lines except when they occur  
> in a run of discardables, with nonprovisionals at the beginning  
> and end.

```java
    private void filterDiscards(final byte[] discards)
```

#### `final int end = buffered_lines;`  <sub>(method)</sub>

```java
final int end = buffered_lines;
```

#### `discard`  <sub>(method)</sub>

> Actually discard the lines.

```java
    private void discard(final byte[] discards)
```

#### `final int end = buffered_lines;`  <sub>(method)</sub>

```java
final int end = buffered_lines;
```

#### `file_data`  <sub>(method)</sub>

```java
    file_data(Object[] data,Hashtable h)
```

#### `shift_boundaries`  <sub>(method)</sub>

> Adjust inserts/deletes of blank lines to join changes  
> as much as possible.  
> We do something when a run of changed lines include a blank  
> line at one end and have an excluded blank line at the other.  
> We are free to choose which blank line is included.  
> `compareseq' always chooses the one at the beginning,  
> but usually it is cleaner to consider the following blank line  
> to be the "change".  The only exception is if the preceding blank line  
> would join this change to other changes.

```java
    void shift_boundaries(file_data f)
```

#### `final boolean[] changed = changed_flag;`  <sub>(method)</sub>

```java
final boolean[] changed = changed_flag;
```

#### `final boolean[] other_changed = f.changed_flag;`  <sub>(method)</sub>

```java
final boolean[] other_changed = f.changed_flag;
```

#### `final int buffered_lines;`  <sub>(method)</sub>

```java
final int buffered_lines;
```

#### `private final int[]	    equivs;`  <sub>(method)</sub>

```java
private final int[]	    equivs;
```

#### `final int[]	   undiscarded;`  <sub>(method)</sub>

```java
final int[]	   undiscarded;
```

#### `final int[]	   realindexes;`  <sub>(method)</sub>

```java
final int[]	   realindexes;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/diff/DiffPrint.java`


#### `UnaryPredicate`  <sub>(interface)</sub>

```java
interface UnaryPredicate
```

#### `DiffPrint`  <sub>(class)</sub>

> A simple framework for printing change lists produced by <code>Diff</code>.  
> Copyright (C) 2000 Business Management Systems, Inc.  
> <p>  
> This program is free software; you can redistribute it and/or modify  
> it under the terms of the GNU General Public License as published by  
> the Free Software Foundation; either version 1, or (at your option)  
> any later version.  
> <p>  
> This program is distributed in the hope that it will be useful,  
> but WITHOUT ANY WARRANTY; without even the implied warranty of  
> MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the  
> GNU General Public License for more details.  
> <p>  
> You should have received a copy of the GNU General Public License  
> along with this program; if not, write to the Free Software  
> Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.

```java
public class DiffPrint
```

#### `Base`  <sub>(class)</sub>

> A Base class for printing edit scripts produced by Diff.  
> This class divides the change list into "hunks", and calls  
> <code>print_hunk</code> for each hunk.  Various utility methods  
> are provided as well.

```java
public static abstract class Base
```

#### `protected PrintWriter outfile;`  <sub>(method)</sub>

```java
protected PrintWriter outfile;
```

#### `StringBuffer`  <sub>(method)</sub>

```java
protected StringBuffer stringOut = new StringBuffer();
```

#### `setOutput`  <sub>(method)</sub>

```java
        public void setOutput(Writer wtr)
```

#### `setupOutput`  <sub>(method)</sub>

```java
        private void setupOutput()
```

#### `Base`  <sub>(method)</sub>

```java
        protected Base(Object[] a,Object[] b)
```

#### `protected UnaryPredicate ignore = null;`  <sub>(method)</sub>

```java
protected UnaryPredicate ignore = null;
```

#### `protected Object[] file0, file1;`  <sub>(method)</sub>

```java
protected Object[] file0, file1;
```

#### `print_script`  <sub>(method)</sub>

> Divide SCRIPT into pieces by calling HUNKFUN and  
> print each piece with PRINTFUN.  
> Both functions take one arg, an edit script.  
> PRINTFUN takes a subscript which belongs together (with a null  
> link at the end) and prints it.

```java
        public String print_script(Diff.change script)
```

#### `hunkfun`  <sub>(method)</sub>

> Called with the tail of the script  
> and returns the last link that belongs together with the start  
> of the tail.

```java
        protected Diff.change hunkfun(Diff.change hunk)
```

#### `protected int first0, last0, first1, last1, dele`  <sub>(method)</sub>

```java
protected int first0, last0, first1, last1, deletes, inserts;
```

#### `analyze_hunk`  <sub>(method)</sub>

> Look at a hunk of edit script and report the range of lines in each file  
> that it applies to.  HUNK is the start of the hunk, which is a chain  
> of `struct change'.  The first and last line numbers of file 0 are stored  
> in *FIRST0 and *LAST0, and likewise for file 1 in *FIRST1 and *LAST1.  
> Note that these are internal line numbers that count from 0.  
> If no lines from file 0 are deleted, then FIRST0 is LAST0+1.  
> Also set *DELETES nonzero if any lines of file 0 are deleted  
> and set *INSERTS nonzero if any lines of file 1 are inserted.  
> If only ignorable lines are inserted or deleted, both are  
> set to 0.

```java
        protected void analyze_hunk(Diff.change hunk)
```

#### `print_header`  <sub>(method)</sub>

> Print the script header which identifies the files compared.

```java
        protected void print_header(String filea, String fileb)
```

#### `print_1_line`  <sub>(method)</sub>

```java
        protected void print_1_line(String pre,Object linbuf)
```

#### `print_number_range`  <sub>(method)</sub>

> Print a pair of line numbers with SEPCHAR, translated for file FILE.  
> If the two numbers are identical, print just one number.  
> Args A and B are internal line numbers.  
> We print the translated (real) line numbers.

```java
        protected void print_number_range(char sepchar, int a, int b)
```

#### `change_letter`  <sub>(method)</sub>

```java
        public static char change_letter(int inserts, int deletes)
```

#### `NormalPrint`  <sub>(class)</sub>

> Print a change list in the standard diff format.

```java
public static class NormalPrint extends Base
```

#### `NormalPrint`  <sub>(method)</sub>

```java
        public NormalPrint(Object[] a,Object[] b)
```

#### `print_hunk`  <sub>(method)</sub>

> Print a hunk of a normal diff.  
> This is a contiguous portion of a complete edit script,  
> describing changes in consecutive lines.

```java
        protected void print_hunk(Diff.change hunk)
```

#### `EdPrint`  <sub>(class)</sub>

> Prints an edit script in a format suitable for input to <code>ed</code>.  
> The edit script must be generated with the reverse option to  
> be useful as actual <code>ed</code> input.

```java
public static class EdPrint extends Base
```

#### `EdPrint`  <sub>(method)</sub>

```java
        public EdPrint(Object[] a,Object[] b)
```

#### `print_hunk`  <sub>(method)</sub>

> Print a hunk of an ed diff

```java
        protected void print_hunk(Diff.change hunk)
```

#### `ContextPrint`  <sub>(class)</sub>

> Prints an edit script in context diff format.  This and its  
> 'unified' variation is used for source code patches.

```java
public static class ContextPrint extends Base
```

#### `protected int context = 3;`  <sub>(method)</sub>

```java
protected int context = 3;
```

#### `ContextPrint`  <sub>(method)</sub>

```java
        public ContextPrint(Object[] a,Object[] b)
```

#### `print_context_label`  <sub>(method)</sub>

```java
        protected void print_context_label(String mark, File inf, String label)
```

#### `print_header`  <sub>(method)</sub>

```java
        public void print_header(String filea,String fileb)
```

#### `find_function`  <sub>(method)</sub>

> If function_regexp defined, search for start of function.

```java
        private String find_function(Object[] lines, int start)
```

#### `print_function`  <sub>(method)</sub>

```java
        protected void print_function(Object[] file,int start)
```

#### `print_hunk`  <sub>(method)</sub>

```java
        protected void print_hunk(Diff.change hunk)
```

#### `UnifiedPrint`  <sub>(class)</sub>

> Prints an edit script in context diff format.  This and its  
> 'unified' variation is used for source code patches.

```java
public static class UnifiedPrint extends ContextPrint
```

#### `UnifiedPrint`  <sub>(method)</sub>

```java
        public UnifiedPrint(Object[] a,Object[] b)
```

#### `print_header`  <sub>(method)</sub>

```java
        public void print_header(String filea,String fileb)
```

#### `print_number_range`  <sub>(method)</sub>

```java
        private void print_number_range(int a, int b)
```

#### `print_hunk`  <sub>(method)</sub>

```java
        protected void print_hunk(Diff.change hunk)
```

#### `slurp`  <sub>(method)</sub>

> Read a text file into an array of String.  This provides basic diff  
> functionality.  A more advanced diff utility will use specialized  
> objects to represent the text lines, with options to, for example,  
> convert sequences of whitespace to a single space for comparison  
> purposes.

```java
    static String[] slurp(String file) throws IOException
```

#### `main`  <sub>(method)</sub>

```java
    public static void main(String[] argv) throws IOException
```


## `src/com/sittinglittleduck/DirBuster/gui` <a id='pkg-src-com-sittinglittleduck-dirbuster-gui'></a>

*505 documented items*


### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JDialogAdvSetup.java`


#### `JDialogAdvSetup`  <sub>(class)</sub>

```java
public class JDialogAdvSetup extends javax.swing.JDialog
```

#### `private HTTPHeaderTableModel httpHeaderTableMode`  <sub>(method)</sub>

```java
private HTTPHeaderTableModel httpHeaderTableModel;
```

#### `private HTMLParseTableModel htmlParseTableModel;`  <sub>(method)</sub>

```java
private HTMLParseTableModel htmlParseTableModel;
```

#### `private Manager manager;`  <sub>(method)</sub>

```java
private Manager manager;
```

#### `JDialogAdvSetup`  <sub>(method)</sub>

> Creates new form JDialogAdvSetup

```java
    public JDialogAdvSetup(java.awt.Frame parent, boolean modal)
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `mouseClicked`  <sub>(method)</sub>

```java
            public void mouseClicked(java.awt.event.MouseEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `mouseClicked`  <sub>(method)</sub>

```java
            public void mouseClicked(java.awt.event.MouseEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `jButton1ActionPerformed`  <sub>(method)</sub>

```java
private void jButton1ActionPerformed(java.awt.event.ActionEvent evt) {//GEN-FIRST:event_jButton1ActionPerformed
```

#### `updateTable`  <sub>(method)</sub>

```java
    private void updateTable()
```

#### `private javax.swing.JButton jButton1;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButton1;
```

#### `private javax.swing.JButton jButtonAddHTMLToPars`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonAddHTMLToParse;
```

#### `private javax.swing.JButton jButtonAddHeader;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonAddHeader;
```

#### `private javax.swing.JButton jButtonCancel;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonCancel;
```

#### `private javax.swing.JButton jButtonCheckNow;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonCheckNow;
```

#### `private javax.swing.JButton jButtonOk;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonOk;
```

#### `private javax.swing.JCheckBox jCheckBoxAllowUpda`  <sub>(method)</sub>

```java
private javax.swing.JCheckBox jCheckBoxAllowUpdates;
```

#### `private javax.swing.JCheckBox jCheckBoxLimitReqS`  <sub>(method)</sub>

```java
private javax.swing.JCheckBox jCheckBoxLimitReqSec;
```

#### `private javax.swing.JCheckBox jCheckBoxRunViaPro`  <sub>(method)</sub>

```java
private javax.swing.JCheckBox jCheckBoxRunViaProxy;
```

#### `private javax.swing.JCheckBox jCheckBoxUseAuth;`  <sub>(method)</sub>

```java
private javax.swing.JCheckBox jCheckBoxUseAuth;
```

#### `private javax.swing.JCheckBox jCheckBoxUseProxyA`  <sub>(method)</sub>

```java
private javax.swing.JCheckBox jCheckBoxUseProxyAuth;
```

#### `private javax.swing.JComboBox jComboBoxAuthType;`  <sub>(method)</sub>

```java
private javax.swing.JComboBox jComboBoxAuthType;
```

#### `private javax.swing.JLabel jLabel1;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel1;
```

#### `private javax.swing.JLabel jLabel10;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel10;
```

#### `private javax.swing.JLabel jLabel11;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel11;
```

#### `private javax.swing.JLabel jLabel12;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel12;
```

#### `private javax.swing.JLabel jLabel13;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel13;
```

#### `private javax.swing.JLabel jLabel14;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel14;
```

#### `private javax.swing.JLabel jLabel15;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel15;
```

#### `private javax.swing.JLabel jLabel16;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel16;
```

#### `private javax.swing.JLabel jLabel17;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel17;
```

#### `private javax.swing.JLabel jLabel18;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel18;
```

#### `private javax.swing.JLabel jLabel19;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel19;
```

#### `private javax.swing.JLabel jLabel2;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel2;
```

#### `private javax.swing.JLabel jLabel20;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel20;
```

#### `private javax.swing.JLabel jLabel21;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel21;
```

#### `private javax.swing.JLabel jLabel22;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel22;
```

#### `private javax.swing.JLabel jLabel23;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel23;
```

#### `private javax.swing.JLabel jLabel24;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel24;
```

#### `private javax.swing.JLabel jLabel25;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel25;
```

#### `private javax.swing.JLabel jLabel26;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel26;
```

#### `private javax.swing.JLabel jLabel27;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel27;
```

#### `private javax.swing.JLabel jLabel3;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel3;
```

#### `private javax.swing.JLabel jLabel4;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel4;
```

#### `private javax.swing.JLabel jLabel5;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel5;
```

#### `private javax.swing.JLabel jLabel6;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel6;
```

#### `private javax.swing.JLabel jLabel7;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel7;
```

#### `private javax.swing.JLabel jLabel8;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel8;
```

#### `private javax.swing.JLabel jLabel9;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel9;
```

#### `private javax.swing.JLabel jLabelLimitReqSec;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabelLimitReqSec;
```

#### `private javax.swing.JMenuItem jMenuItemDelete;`  <sub>(method)</sub>

```java
private javax.swing.JMenuItem jMenuItemDelete;
```

#### `private javax.swing.JMenuItem jMenuItemDeleteEle`  <sub>(method)</sub>

```java
private javax.swing.JMenuItem jMenuItemDeleteElement;
```

#### `private javax.swing.JPanel jPanel1;`  <sub>(method)</sub>

```java
private javax.swing.JPanel jPanel1;
```

#### `private javax.swing.JPanel jPanel2;`  <sub>(method)</sub>

```java
private javax.swing.JPanel jPanel2;
```

#### `private javax.swing.JPanel jPanel3;`  <sub>(method)</sub>

```java
private javax.swing.JPanel jPanel3;
```

#### `private javax.swing.JPanel jPanelAuthDetails;`  <sub>(method)</sub>

```java
private javax.swing.JPanel jPanelAuthDetails;
```

#### `private javax.swing.JPanel jPanelHTMLParse;`  <sub>(method)</sub>

```java
private javax.swing.JPanel jPanelHTMLParse;
```

#### `private javax.swing.JPanel jPanelScanOpts;`  <sub>(method)</sub>

```java
private javax.swing.JPanel jPanelScanOpts;
```

#### `private javax.swing.JPasswordField jPasswordFiel`  <sub>(method)</sub>

```java
private javax.swing.JPasswordField jPasswordFieldAuth;
```

#### `private javax.swing.JPopupMenu jPopupMenuHTMLpar`  <sub>(method)</sub>

```java
private javax.swing.JPopupMenu jPopupMenuHTMLparseElements;
```

#### `private javax.swing.JPopupMenu jPopupMenuHTTPHea`  <sub>(method)</sub>

```java
private javax.swing.JPopupMenu jPopupMenuHTTPHeaders;
```

#### `private javax.swing.JScrollPane jScrollPane1;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane1;
```

#### `private javax.swing.JScrollPane jScrollPane2;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane2;
```

#### `private javax.swing.JSeparator jSeparator1;`  <sub>(method)</sub>

```java
private javax.swing.JSeparator jSeparator1;
```

#### `private javax.swing.JSeparator jSeparator2;`  <sub>(method)</sub>

```java
private javax.swing.JSeparator jSeparator2;
```

#### `private javax.swing.JSeparator jSeparator3;`  <sub>(method)</sub>

```java
private javax.swing.JSeparator jSeparator3;
```

#### `private javax.swing.JSeparator jSeparator4;`  <sub>(method)</sub>

```java
private javax.swing.JSeparator jSeparator4;
```

#### `private javax.swing.JSeparator jSeparator5;`  <sub>(method)</sub>

```java
private javax.swing.JSeparator jSeparator5;
```

#### `private javax.swing.JSeparator jSeparator6;`  <sub>(method)</sub>

```java
private javax.swing.JSeparator jSeparator6;
```

#### `private javax.swing.JTabbedPane jTabbedPane1;`  <sub>(method)</sub>

```java
private javax.swing.JTabbedPane jTabbedPane1;
```

#### `private javax.swing.JTable jTableHTMLParseElemen`  <sub>(method)</sub>

```java
private javax.swing.JTable jTableHTMLParseElements;
```

#### `private javax.swing.JTable jTableHTTPHeaders;`  <sub>(method)</sub>

```java
private javax.swing.JTable jTableHTTPHeaders;
```

#### `private javax.swing.JTextField jTextFieldAuthUse`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldAuthUserName;
```

#### `private javax.swing.JTextField jTextFieldDefault`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldDefaultExts;
```

#### `private javax.swing.JTextField jTextFieldDefault`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldDefaultList;
```

#### `private javax.swing.JTextField jTextFieldDefault`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldDefaultNoThreads;
```

#### `private javax.swing.JTextField jTextFieldExtToMi`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldExtToMiss;
```

#### `private javax.swing.JTextField jTextFieldFailCas`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldFailCaseString;
```

#### `private javax.swing.JTextField jTextFieldHTMLAtt`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldHTMLAttr;
```

#### `private javax.swing.JTextField jTextFieldHTMLTag`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldHTMLTag;
```

#### `private javax.swing.JTextField jTextFieldHTTPHea`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldHTTPHeader;
```

#### `private javax.swing.JTextField jTextFieldHTTPHea`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldHTTPHeaderValue;
```

#### `private javax.swing.JTextField jTextFieldLimitRe`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldLimitReqSec;
```

#### `private javax.swing.JTextField jTextFieldProxyHo`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldProxyHost;
```

#### `private javax.swing.JTextField jTextFieldProxyPa`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldProxyPassword;
```

#### `private javax.swing.JTextField jTextFieldProxyPo`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldProxyPort;
```

#### `private javax.swing.JTextField jTextFieldProxyRe`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldProxyRealm;
```

#### `private javax.swing.JTextField jTextFieldProxyUs`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldProxyUserName;
```

#### `private javax.swing.JTextField jTextFieldRealmDo`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldRealmDomain;
```

#### `private javax.swing.JTextField jTextFieldTimeout`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldTimeout;
```

#### `private javax.swing.JTextField jTextFieldUserAge`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldUserAgent;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JDialogDisplayErrors.java`


#### `JDialogDisplayErrors`  <sub>(class)</sub>

```java
public class JDialogDisplayErrors extends javax.swing.JDialog
```

#### `private Vector errors;`  <sub>(method)</sub>

```java
private Vector errors;
```

#### `JDialogDisplayErrors`  <sub>(method)</sub>

> Creates new form JDialogDisplayErrors

```java
    public JDialogDisplayErrors(java.awt.Frame parent, boolean modal, Vector errors)
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `public javax.swing.JButton jButtonOK;`  <sub>(method)</sub>

```java
public javax.swing.JButton jButtonOK;
```

#### `public javax.swing.JLabel jLabel1;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabel1;
```

#### `public javax.swing.JLabel jLabel2;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabel2;
```

#### `public javax.swing.JList jListErrors;`  <sub>(method)</sub>

```java
public javax.swing.JList jListErrors;
```

#### `public javax.swing.JScrollPane jScrollPane1;`  <sub>(method)</sub>

```java
public javax.swing.JScrollPane jScrollPane1;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JDialogInconsistentFailCodes.java`


#### `JDialogInconsistentFailCodes`  <sub>(class)</sub>

```java
public class JDialogInconsistentFailCodes extends javax.swing.JDialog
```

#### `private String response1;`  <sub>(method)</sub>

```java
private String response1;
```

#### `private String response2;`  <sub>(method)</sub>

```java
private String response2;
```

#### `private String response3;`  <sub>(method)</sub>

```java
private String response3;
```

#### `private String url;`  <sub>(method)</sub>

```java
private String url;
```

#### `JDialogInconsistentFailCodes`  <sub>(method)</sub>

> Creates new form JDialogInconsistentFailCodes

```java
    public JDialogInconsistentFailCodes(java.awt.Frame parent, boolean modal, String response1, String responce2, String responce3, String url)
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `jButtonCancelActionPerformed`  <sub>(method)</sub>

```java
private void jButtonCancelActionPerformed(java.awt.event.ActionEvent evt) {//GEN-FIRST:event_jButtonCancelActionPerformed
```

#### `jButtonOkActionPerformed`  <sub>(method)</sub>

```java
private void jButtonOkActionPerformed(java.awt.event.ActionEvent evt) {//GEN-FIRST:event_jButtonOkActionPerformed
```

#### `jButtonTestActionPerformed`  <sub>(method)</sub>

```java
private void jButtonTestActionPerformed(java.awt.event.ActionEvent evt) {//GEN-FIRST:event_jButtonTestActionPerformed
```

#### `showAndGetRegex`  <sub>(method)</sub>

> this is called to show the diaglog and get the regex that was entered

```java
public String showAndGetRegex()
```

#### `private javax.swing.JButton jButtonCancel;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonCancel;
```

#### `private javax.swing.JButton jButtonOk;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonOk;
```

#### `private javax.swing.JButton jButtonTest;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonTest;
```

#### `private javax.swing.JLabel jLabel1;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel1;
```

#### `private javax.swing.JLabel jLabel2;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel2;
```

#### `private javax.swing.JLabel jLabel4;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel4;
```

#### `private javax.swing.JLabel jLabel5;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel5;
```

#### `private javax.swing.JLabel jLabel6;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel6;
```

#### `private javax.swing.JLabel jLabel7;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel7;
```

#### `private javax.swing.JLabel jLabelFailURL;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabelFailURL;
```

#### `private javax.swing.JScrollPane jScrollPane1;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane1;
```

#### `private javax.swing.JScrollPane jScrollPane2;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane2;
```

#### `private javax.swing.JScrollPane jScrollPane3;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane3;
```

#### `private javax.swing.JScrollPane jScrollPane4;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane4;
```

#### `private javax.swing.JScrollPane jScrollPane5;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane5;
```

#### `private javax.swing.JScrollPane jScrollPane6;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane6;
```

#### `private javax.swing.JTabbedPane jTabbedPane1;`  <sub>(method)</sub>

```java
private javax.swing.JTabbedPane jTabbedPane1;
```

#### `private javax.swing.JTextArea jTextAreaDiff1_2;`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaDiff1_2;
```

#### `private javax.swing.JTextArea jTextAreaDiff1_3;`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaDiff1_3;
```

#### `private javax.swing.JTextArea jTextAreaDiff2_3;`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaDiff2_3;
```

#### `private javax.swing.JTextArea jTextAreaResponse1`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaResponse1;
```

#### `private javax.swing.JTextArea jTextAreaResponse2`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaResponse2;
```

#### `private javax.swing.JTextArea jTextAreaRsponce3;`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaRsponce3;
```

#### `private javax.swing.JTextField jTextFieldRegex;`  <sub>(method)</sub>

```java
private javax.swing.JTextField jTextFieldRegex;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JDialogVersion.java`


#### `JDialogVersion`  <sub>(class)</sub>

```java
public class JDialogVersion extends javax.swing.JDialog
```

#### `JDialogVersion`  <sub>(method)</sub>

> Creates new form JDialogVersion

```java
    public JDialogVersion(java.awt.Frame parent, boolean modal)
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `private javax.swing.JButton jButtonOk;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonOk;
```

#### `private javax.swing.JLabel jLabel1;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel1;
```

#### `private javax.swing.JLabel jLabel2;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel2;
```

#### `private javax.swing.JLabel jLabel3;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel3;
```

#### `private javax.swing.JLabel jLabel4;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel4;
```

#### `private javax.swing.JLabel jLabel5;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel5;
```

#### `private javax.swing.JLabel jLabel6;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel6;
```

#### `private javax.swing.JLabel jLabel7;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel7;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JDialogViewLicence.java`


#### `JDialogViewLicence`  <sub>(class)</sub>

```java
public class JDialogViewLicence extends javax.swing.JDialog
```

#### `Copyright`  <sub>(interface)</sub>

```java
private final String lgpl = "\t\t  GNU LESSER GENERAL PUBLIC LICENSE\n\t\t       Version 2.1, February 1999\n\n Copyright (C) 1991, 1999 Free Software Foundation, Inc.\n 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA\n Everyone is permitted to copy and distribute verbatim copies\n of this license document, but changing it is not allowed.\n\n[This is the first released version of the Lesser GPL.  It also counts\n as the successor of the GNU Library Public License, version 2, hence\n the version number 2.1.]\n\n\t\t\t    Preamble\n\n  The licenses for most software are designed to take away your\nfreedom to share and change it.  By contrast, the GNU General Public\nLicenses are intended to guarantee your freedom to share and change\nfree software--to make sure the software is free for all its users.\n\n  This license, the Lesser General Public License, applies to some\nspecially designated software packages--typically libraries--of the\nFree Software Foundation and other authors who decide to use it.  You\ncan use it too, but we suggest you first think carefully about whether\nthis license or the ordinary General Public License is the better\nstrategy to use in any particular case, based on the explanations below.\n\n  When we speak of free software, we are referring to freedom of use,\nnot price.  Our General Public Licenses are designed to make sure that\nyou have the freedom to distribute copies of free software (and charge\nfor this service if you wish); that you receive source code or can get\nit if you want it; that you can change the software and use pieces of\nit in new free programs; and that you are informed that you can do\nthese things.\n\n  To protect your rights, we need to make restrictions that forbid\ndistributors to deny you these rights or to ask you to surrender these\nrights.  These restrictions translate to certain responsibilities for\nyou if you distribute copies of the library or if you modify it.\n\n  For example, if you distribute copies of the library, whether gratis\nor for a fee, you must give the recipients all the rights that we gave\nyou.  You must make sure that they, too, receive or can get the source\ncode.  If you link other code with the library, you must provide\ncomplete object files to the recipients, so that they can relink them\nwith the library after making changes to the library and recompiling\nit.  And you must show them these terms so they know their rights.\n\n  We protect your rights with a two-step method: (1) we copyright the\nlibrary, and (2) we offer you this license, which gives you legal\npermission to copy, distribute and/or modify the library.\n\n  To protect each distributor, we want to make it very clear that\nthere is no warranty for the free library.  Also, if the library is\nmodified by someone else and passed on, the recipients should know\nthat what they have is not the original version, so that the original\nauthor's reputation will not be affected by problems that might be\nintroduced by others.\n\f\n  Finally, software patents pose a constant threat to the existence of\nany free program.  We wish to make sure that a company cannot\neffectively restrict the users of a free program by obtaining a\nrestrictive license from a patent holder.  Therefore, we insist that\nany patent license obtained for a version of the library must be\nconsistent with the full freedom of use specified in this license.\n\n  Most GNU software, including some libraries, is covered by the\nordinary GNU General Public License.  This license, the GNU Lesser\nGeneral Public License, applies to certain designated libraries, and\nis quite different from the ordinary General Public License.  We use\nthis license for certain libraries in order to permit linking those\nlibraries into non-free programs.\n\n  When a program is linked with a library, whether statically or using\na shared library, the combination of the two is legally speaking a\ncombined work, a derivative of the original library.  The ordinary\nGeneral Public License therefore permits such linking only if the\nentire combination fits its criteria of freedom.  The Lesser General\nPublic License permits more lax criteria for linking other code with\nthe library.\n\n  We call this license the \"Lesser\" General Public License because it\ndoes Less to protect the user's freedom than the ordinary General\nPublic License.  It also provides other free software developers Less\nof an advantage over competing non-free programs.  These disadvantages\nare the reason we use the ordinary General Public License for many\nlibraries.  However, the Lesser license provides advantages in certain\nspecial circumstances.\n\n  For example, on rare occasions, there may be a special need to\nencourage the widest possible use of a certain library, so that it becomes\na de-facto standard.  To achieve this, non-free programs must be\nallowed to use the library.  A more frequent case is that a free\nlibrary does the same job as widely used non-free libraries.  In this\ncase, there is little to gain by limiting the free library to free\nsoftware only, so we use the Lesser General Public License.\n\n  In other cases, permission to use a particular library in non-free\nprograms enables a greater number of people to use a large body of\nfree software.  For example, permission to use the GNU C Library in\nnon-free programs enables many more people to use the whole GNU\noperating system, as well as its variant, the GNU/Linux operating\nsystem.\n\n  Although the Lesser General Public License is Less protective of the\nusers' freedom, it does ensure that the user of a program that is\nlinked with the Library has the freedom and the wherewithal to run\nthat program using a modified version of the Library.\n\n  The precise terms and conditions for copying, distribution and\nmodification follow.  Pay close attention to the difference between a\n\"work based on the library\" and a \"work that uses the library\".  The\nformer contains code derived from the library, whereas the latter must\nbe combined with the library in order to run.\n\f\n\t\t  GNU LESSER GENERAL PUBLIC LICENSE\n   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION\n\n  0. This License Agreement applies to any software library or other\nprogram which contains a notice placed by the copyright holder or\nother authorized party saying it may be distributed under the terms of\nthis Lesser General Public License (also called \"this License\").\nEach licensee is addressed as \"you\".\n\n  A \"library\" means a collection of software functions and/or data\nprepared so as to be conveniently linked with application programs\n(which use some of those functions and data) to form executables.\n\n  The \"Library\", below, refers to any such software library or work\nwhich has been distributed under these terms.  A \"work based on the\nLibrary\" means either the Library or any derivative work under\ncopyright law: that is to say, a work containing the Library or a\nportion of it, either verbatim or with modifications and/or translated\nstraightforwardly into another language.  (Hereinafter, translation is\nincluded without limitation in the term \"modification\".)\n\n  \"Source code\" for a work means the preferred form of the work for\nmaking modifications to it.  For a library, complete source code means\nall the source code for all modules it contains, plus any associated\ninterface definition files, plus the scripts used to control compilation\nand installation of the library.\n\n  Activities other than copying, distribution and modification are not\ncovered by this License; they are outside its scope.  The act of\nrunning a program using the Library is not restricted, and output from\nsuch a program is covered only if its contents constitute a work based\non the Library (independent of the use of the Library in a tool for\nwriting it).  Whether that is true depends on what the Library does\nand what the program that uses the Library does.\n  \n  1. You may copy and distribute verbatim copies of the Library's\ncomplete source code as you receive it, in any medium, provided that\nyou conspicuously and appropriately publish on each copy an\nappropriate copyright notice and disclaimer of warranty; keep intact\nall the notices that refer to this License and to the absence of any\nwarranty; and distribute a copy of this License along with the\nLibrary.\n\n  You may charge a fee for the physical act of transferring a copy,\nand you may at your option offer warranty protection in exchange for a\nfee.\n\f\n  2. You may modify your copy or copies of the Library or any portion\nof it, thus forming a work based on the Library, and copy and\ndistribute such modifications or work under the terms of Section 1\nabove, provided that you also meet all of these conditions:\n\n    a) The modified work must itself be a software library.\n\n    b) You must cause the files modified to carry prominent notices\n    stating that you changed the files and the date of any change.\n\n    c) You must cause the whole of the work to be licensed at no\n    charge to all third parties under the terms of this License.\n\n    d) If a facility in the modified Library refers to a function or a\n    table of data to be supplied by an application program that uses\n    the facility, other than as an argument passed when the facility\n    is invoked, then you must make a good faith effort to ensure that,\n    in the event an application does not supply such function or\n    table, the facility still operates, and performs whatever part of\n    its purpose remains meaningful.\n\n    (For example, a function in a library to compute square roots has\n    a purpose that is entirely well-defined independent of the\n    application.  Therefore, Subsection 2d requires that any\n    application-supplied function or table used by this function must\n    be optional: if the application does not supply it, the square\n    root function must still compute square roots.)\n\nThese requirements apply to the modified work as a whole.  If\nidentifiable sections of that work are not derived from the Library,\nand can be reasonably considered independent and separate works in\nthemselves, then this License, and its terms, do not apply to those\nsections when you distribute them as separate works.  But when you\ndistribute the same sections as part of a whole which is a work based\non the Library, the distribution of the whole must be on the terms of\nthis License, whose permissions for other licensees extend to the\nentire whole, and thus to each and every part regardless of who wrote\nit.\n\nThus, it is not the intent of this section to claim rights or contest\nyour rights to work written entirely by you; rather, the intent is to\nexercise the right to control the distribution of derivative or\ncollective works based on the Library.\n\nIn addition, mere aggregation of another work not based on the Library\nwith the Library (or with a work based on the Library) on a volume of\na storage or distribution medium does not bring the other work under\nthe scope of this License.\n\n  3. You may opt to apply the terms of the ordinary GNU General Public\nLicense instead of this License to a given copy of the Library.  To do\nthis, you must alter all the notices that refer to this License, so\nthat they refer to the ordinary GNU General Public License, version 2,\ninstead of to this License.  (If a newer version than version 2 of the\nordinary GNU General Public License has appeared, then you can specify\nthat version instead if you wish.)  Do not make any other change in\nthese notices.\n\f\n  Once this change is made in a given copy, it is irreversible for\nthat copy, so the ordinary GNU General Public License applies to all\nsubsequent copies and derivative works made from that copy.\n\n  This option is useful when you wish to copy part of the code of\nthe Library into a program that is not a library.\n\n  4. You may copy and distribute the Library (or a portion or\nderivative of it, under Section 2) in object code or executable form\nunder the terms of Sections 1 and 2 above provided that you accompany\nit with the complete corresponding machine-readable source code, which\nmust be distributed under the terms of Sections 1 and 2 above on a\nmedium customarily used for software interchange.\n\n  If distribution of object code is made by offering access to copy\nfrom a designated place, then offering equivalent access to copy the\nsource code from the same place satisfies the requirement to\ndistribute the source code, even though third parties are not\ncompelled to copy the source along with the object code.\n\n  5. A program that contains no derivative of any portion of the\nLibrary, but is designed to work with the Library by being compiled or\nlinked with it, is called a \"work that uses the Library\".  Such a\nwork, in isolation, is not a derivative work of the Library, and\ntherefore falls outside the scope of this License.\n\n  However, linking a \"work that uses the Library\" with the Library\ncreates an executable that is a derivative of the Library (because it\ncontains portions of the Library), rather than a \"work that uses the\nlibrary\".  The executable is therefore covered by this License.\nSection 6 states terms for distribution of such executables.\n\n  When a \"work that uses the Library\" uses material from a header file\nthat is part of the Library, the object code for the work may be a\nderivative work of the Library even though the source code is not.\nWhether this is true is especially significant if the work can be\nlinked without the Library, or if the work is itself a library.  The\nthreshold for this to be true is not precisely defined by law.\n\n  If such an object file uses only numerical parameters, data\nstructure layouts and accessors, and small macros and small inline\nfunctions (ten lines or less in length), then the use of the object\nfile is unrestricted, regardless of whether it is legally a derivative\nwork.  (Executables containing this object code plus portions of the\nLibrary will still fall under Section 6.)\n\n  Otherwise, if the work is a derivative of the Library, you may\ndistribute the object code for the work under the terms of Section 6.\nAny executables containing that work also fall under Section 6,\nwhether or not they are linked directly with the Library itself.\n\f\n  6. As an exception to the Sections above, you may also combine or\nlink a \"work that uses the Library\" with the Library to produce a\nwork containing portions of the Library, and distribute that work\nunder terms of your choice, provided that the terms permit\nmodification of the work for the customer's own use and reverse\nengineering for debugging such modifications.\n\n  You must give prominent notice with each copy of the work that the\nLibrary is used in it and that the Library and its use are covered by\nthis License.  You must supply a copy of this License.  If the work\nduring execution displays copyright notices, you must include the\ncopyright notice for the Library among them, as well as a reference\ndirecting the user to the copy of this License.  Also, you must do one\nof these things:\n\n    a) Accompany the work with the complete corresponding\n    machine-readable source code for the Library including whatever\n    changes were used in the work (which must be distributed under\n    Sections 1 and 2 above); and, if the work is an executable linked\n    with the Library, with the complete machine-readable \"work that\n    uses the Library\", as object code and/or source code, so that the\n    user can modify the Library and then relink to produce a modified\n    executable containing the modified Library.  (It is understood\n    that the user who changes the contents of definitions files in the\n    Library will not necessarily be able to recompile the application\n    to use the modified definitions.)\n\n    b) Use a suitable shared library mechanism for linking with the\n    Library.  A suitable mechanism is one that (1) uses at run time a\n    copy of the library already present on the user's computer system,\n    rather than copying library functions into the executable, and (2)\n    will operate properly with a modified version of the library, if\n    the user installs one, as long as the modified version is\n    interface-compatible with the version that the work was made with.\n\n    c) Accompany the work with a written offer, valid for at\n    least three years, to give the same user the materials\n    specified in Subsection 6a, above, for a charge no more\n    than the cost of performing this distribution.\n\n    d) If distribution of the work is made by offering access to copy\n    from a designated place, offer equivalent access to copy the above\n    specified materials from the same place.\n\n    e) Verify that the user has already received a copy of these\n    materials or that you have already sent this user a copy.\n\n  For an executable, the required form of the \"work that uses the\nLibrary\" must include any data and utility programs needed for\nreproducing the executable from it.  However, as a special exception,\nthe materials to be distributed need not include anything that is\nnormally distributed (in either source or binary form) with the major\ncomponents (compiler, kernel, and so on) of the operating system on\nwhich the executable runs, unless that component itself accompanies\nthe executable.\n\n  It may happen that this requirement contradicts the license\nrestrictions of other proprietary libraries that do not normally\naccompany the operating system.  Such a contradiction means you cannot\nuse both them and the Library together in an executable that you\ndistribute.\n\f\n  7. You may place library facilities that are a work based on the\nLibrary side-by-side in a single library together with other library\nfacilities not covered by this License, and distribute such a combined\nlibrary, provided that the separate distribution of the work based on\nthe Library and of the other library facilities is otherwise\npermitted, and provided that you do these two things:\n\n    a) Accompany the combined library with a copy of the same work\n    based on the Library, uncombined with any other library\n    facilities.  This must be distributed under the terms of the\n    Sections above.\n\n    b) Give prominent notice with the combined library of the fact\n    that part of it is a work based on the Library, and explaining\n    where to find the accompanying uncombined form of the same work.\n\n  8. You may not copy, modify, sublicense, link with, or distribute\nthe Library except as expressly provided under this License.  Any\nattempt otherwise to copy, modify, sublicense, link with, or\ndistribute the Library is void, and will automatically terminate your\nrights under this License.  However, parties who have received copies,\nor rights, from you under this License will not have their licenses\nterminated so long as such parties remain in full compliance.\n\n  9. You are not required to accept this License, since you have not\nsigned it.  However, nothing else grants you permission to modify or\ndistribute the Library or its derivative works.  These actions are\nprohibited by law if you do not accept this License.  Therefore, by\nmodifying or distributing the Library (or any work based on the\nLibrary), you indicate your acceptance of this License to do so, and\nall its terms and conditions for copying, distributing or modifying\nthe Library or works based on it.\n\n  10. Each time you redistribute the Library (or any work based on the\nLibrary), the recipient automatically receives a license from the\noriginal licensor to copy, distribute, link with or modify the Library\nsubject to these terms and conditions.  You may not impose any further\nrestrictions on the recipients' exercise of the rights granted herein.\nYou are not responsible for enforcing compliance by third parties with\nthis License.\n\f\n  11. If, as a consequence of a court judgment or allegation of patent\ninfringement or for any other reason (not limited to patent issues),\nconditions are imposed on you (whether by court order, agreement or\notherwise) that contradict the conditions of this License, they do not\nexcuse you from the conditions of this License.  If you cannot\ndistribute so as to satisfy simultaneously your obligations under this\nLicense and any other pertinent obligations, then as a consequence you\nmay not distribute the Library at all.  For example, if a patent\nlicense would not permit royalty-free redistribution of the Library by\nall those who receive copies directly or indirectly through you, then\nthe only way you could satisfy both it and this License would be to\nrefrain entirely from distribution of the Library.\n\nIf any portion of this section is held invalid or unenforceable under any\nparticular circumstance, the balance of the section is intended to apply,\nand the section as a whole is intended to apply in other circumstances.\n\nIt is not the purpose of this section to induce you to infringe any\npatents or other property right claims or to contest validity of any\nsuch claims; this section has the sole purpose of protecting the\nintegrity of the free software distribution system which is\nimplemented by public license practices.  Many people have made\ngenerous contributions to the wide range of software distributed\nthrough that system in reliance on consistent application of that\nsystem; it is up to the author/donor to decide if he or she is willing\nto distribute software through any other system and a licensee cannot\nimpose that choice.\n\nThis section is intended to make thoroughly clear what is believed to\nbe a consequence of the rest of this License.\n\n  12. If the distribution and/or use of the Library is restricted in\ncertain countries either by patents or by copyrighted interfaces, the\noriginal copyright holder who places the Library under this License may add\nan explicit geographical distribution limitation excluding those countries,\nso that distribution is permitted only in or among countries not thus\nexcluded.  In such case, this License incorporates the limitation as if\nwritten in the body of this License.\n\n  13. The Free Software Foundation may publish revised and/or new\nversions of the Lesser General Public License from time to time.\nSuch new versions will be similar in spirit to the present version,\nbut may differ in detail to address new problems or concerns.\n\nEach version is given a distinguishing version number.  If the Library\nspecifies a version number of this License which applies to it and\n\"any later version\", you have the option of following the terms and\nconditions either of that version or of any later version published by\nthe Free Software Foundation.  If the Library does not specify a\nlicense version number, you may choose any version ever published by\nthe Free Software Foundation.\n\f\n  14. If you wish to incorporate parts of the Library into other free\nprograms whose distribution conditions are incompatible with these,\nwrite to the author to ask for permission.  For software which is\ncopyrighted by the Free Software Foundation, write to the Free\nSoftware Foundation; we sometimes make exceptions for this.  Our\ndecision will be guided by the two goals of preserving the free status\nof all derivatives of our free software and of promoting the sharing\nand reuse of software generally.\n\n\t\t\t    NO WARRANTY\n\n  15. BECAUSE THE LIBRARY IS LICENSED FREE OF CHARGE, THERE IS NO\nWARRANTY FOR THE LIBRARY, TO THE EXTENT PERMITTED BY APPLICABLE LAW.\nEXCEPT WHEN OTHERWISE STATED IN WRITING THE COPYRIGHT HOLDERS AND/OR\nOTHER PARTIES PROVIDE THE LIBRARY \"AS IS\" WITHOUT WARRANTY OF ANY\nKIND, EITHER EXPRESSED OR IMPLIED, INCLUDING, BUT NOT LIMITED TO, THE\nIMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR\nPURPOSE.  THE ENTIRE RISK AS TO THE QUALITY AND PERFORMANCE OF THE\nLIBRARY IS WITH YOU.  SHOULD THE LIBRARY PROVE DEFECTIVE, YOU ASSUME\nTHE COST OF ALL NECESSARY SERVICING, REPAIR OR CORRECTION.\n\n  16. IN NO EVENT UNLESS REQUIRED BY APPLICABLE LAW OR AGREED TO IN\nWRITING WILL ANY COPYRIGHT HOLDER, OR ANY OTHER PARTY WHO MAY MODIFY\nAND/OR REDISTRIBUTE THE LIBRARY AS PERMITTED ABOVE, BE LIABLE TO YOU\nFOR DAMAGES, INCLUDING ANY GENERAL, SPECIAL, INCIDENTAL OR\nCONSEQUENTIAL DAMAGES ARISING OUT OF THE USE OR INABILITY TO USE THE\nLIBRARY (INCLUDING BUT NOT LIMITED TO LOSS OF DATA OR DATA BEING\nRENDERED INACCURATE OR LOSSES SUSTAINED BY YOU OR THIRD PARTIES OR A\nFAILURE OF THE LIBRARY TO OPERATE WITH ANY OTHER SOFTWARE), EVEN IF\nSUCH HOLDER OR OTHER PARTY HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH\nDAMAGES.\n\n\t\t     END OF TERMS AND CONDITIONS\n";
```

#### `JDialogViewLicence`  <sub>(method)</sub>

> Creates new form JDialogViewReport

```java
    public JDialogViewLicence(java.awt.Frame parent, boolean modal)
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `private javax.swing.JButton jButton1;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButton1;
```

#### `private javax.swing.JScrollPane jScrollPane1;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane1;
```

#### `private javax.swing.JTextArea jTextAreaReport;`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaReport;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JDialogViewListInfo.java`


#### `JDialogViewListInfo`  <sub>(class)</sub>

```java
public class JDialogViewListInfo extends javax.swing.JDialog
```

#### `JDialogViewListInfo`  <sub>(method)</sub>

> Creates new form JDialogViewReport

```java
    public JDialogViewListInfo(java.awt.Frame parent, boolean modal)
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `private javax.swing.JButton jButton1;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButton1;
```

#### `private javax.swing.JScrollPane jScrollPane1;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane1;
```

#### `private javax.swing.JTextArea jTextAreaReport;`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaReport;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JDialogViewReport.java`


#### `JDialogViewReport`  <sub>(class)</sub>

```java
public class JDialogViewReport extends javax.swing.JDialog
```

#### `private String saveLoc;`  <sub>(method)</sub>

```java
private String saveLoc;
```

#### `JDialogViewReport`  <sub>(method)</sub>

> Creates new form JDialogViewReport

```java
    public JDialogViewReport(java.awt.Frame parent, boolean modal, String saveLoc)
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `propertyChange`  <sub>(method)</sub>

```java
            public void propertyChange(java.beans.PropertyChangeEvent evt)
```

#### `stateChanged`  <sub>(method)</sub>

```java
            public void stateChanged(javax.swing.event.ChangeEvent evt)
```

#### `private javax.swing.JButton jButton1;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButton1;
```

#### `private javax.swing.JLabel jLabelSaveLoc;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabelSaveLoc;
```

#### `private javax.swing.JScrollPane jScrollPane1;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane1;
```

#### `private javax.swing.JScrollPane jScrollPane2;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane2;
```

#### `private javax.swing.JScrollPane jScrollPane3;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane3;
```

#### `private javax.swing.JScrollPane jScrollPane4;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane4;
```

#### `private javax.swing.JTabbedPane jTabbedPane1;`  <sub>(method)</sub>

```java
private javax.swing.JTabbedPane jTabbedPane1;
```

#### `private javax.swing.JTextArea jTextAreaCSVReport`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaCSVReport;
```

#### `private javax.swing.JTextArea jTextAreaReport;`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaReport;
```

#### `private javax.swing.JTextArea jTextAreaSimpleRep`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaSimpleReport;
```

#### `private javax.swing.JTextArea jTextAreaXMLReport`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaXMLReport;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JDialogViewResponse.java`


#### `JDialogViewResponse`  <sub>(class)</sub>

```java
public class JDialogViewResponse extends javax.swing.JDialog
```

#### `private BaseCase baseCaseObj;`  <sub>(method)</sub>

```java
private BaseCase baseCaseObj;
```

#### `JDialogViewResponse`  <sub>(method)</sub>

> Creates new form JDialogViewResponce

```java
    public JDialogViewResponse(java.awt.Frame parent, boolean modal, Result result)
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `private javax.swing.JButton jButton1;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButton1;
```

#### `private javax.swing.JButton jButtonClose;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonClose;
```

#### `private javax.swing.JLabel jLabel1;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel1;
```

#### `private javax.swing.JLabel jLabel2;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel2;
```

#### `private javax.swing.JLabel jLabelBaseCaseURL;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabelBaseCaseURL;
```

#### `private javax.swing.JLabel jLabelHTTPResponceCod`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabelHTTPResponceCode;
```

#### `private javax.swing.JLabel jLabelURL;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabelURL;
```

#### `private javax.swing.JPanel jPanelBaseCase;`  <sub>(method)</sub>

```java
private javax.swing.JPanel jPanelBaseCase;
```

#### `private javax.swing.JScrollPane jScrollPane1;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane1;
```

#### `private javax.swing.JScrollPane jScrollPane2;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane2;
```

#### `private javax.swing.JScrollPane jScrollPane3;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPane3;
```

#### `private javax.swing.JScrollPane jScrollPaneDiff;`  <sub>(method)</sub>

```java
private javax.swing.JScrollPane jScrollPaneDiff;
```

#### `private javax.swing.JSplitPane jSplitPane1;`  <sub>(method)</sub>

```java
private javax.swing.JSplitPane jSplitPane1;
```

#### `private javax.swing.JTabbedPane jTabbedPane1;`  <sub>(method)</sub>

```java
private javax.swing.JTabbedPane jTabbedPane1;
```

#### `private javax.swing.JTextArea jTextAreaBaseCompe`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaBaseCompear;
```

#### `private javax.swing.JTextArea jTextAreaResponce;`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaResponce;
```

#### `private javax.swing.JTextArea jTextAreaResponceC`  <sub>(method)</sub>

```java
private javax.swing.JTextArea jTextAreaResponceCompear;
```

#### `private javax.swing.JTextPane jTextPaneDiff;`  <sub>(method)</sub>

```java
private javax.swing.JTextPane jTextPaneDiff;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JFrameHelp.java`


#### `JFrameHelp`  <sub>(class)</sub>

```java
public class JFrameHelp extends javax.swing.JFrame
```

#### `JFrameHelp`  <sub>(method)</sub>

> Creates new form jFrameHelp

```java
    public JFrameHelp(JHelp help)
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `main`  <sub>(method)</sub>

```java
    public static void main(String args[])
```

#### `run`  <sub>(method)</sub>

```java
            public void run()
```

#### `private javax.help.JHelp jHelp1;`  <sub>(method)</sub>

```java
private javax.help.JHelp jHelp1;
```

#### `private javax.help.JHelp jHelp2;`  <sub>(method)</sub>

```java
private javax.help.JHelp jHelp2;
```

#### `private javax.help.JHelp jHelp3;`  <sub>(method)</sub>

```java
private javax.help.JHelp jHelp3;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JPanelReport.java`


#### `JPanelReport`  <sub>(class)</sub>

```java
public class JPanelReport extends javax.swing.JPanel
```

#### `private StartGUI parent;`  <sub>(method)</sub>

```java
private StartGUI parent;
```

#### `public static final String reportExt = ".txt";`  <sub>(method)</sub>

```java
public static final String reportExt = ".txt";
```

#### `public static final String reportSimpleExt = "-s`  <sub>(method)</sub>

```java
public static final String reportSimpleExt = "-simple.txt";
```

#### `public static final String reportXMLExt = ".xml"`  <sub>(method)</sub>

```java
public static final String reportXMLExt = ".xml";
```

#### `public static final String reportCSVExt = ".csv"`  <sub>(method)</sub>

```java
public static final String reportCSVExt = ".csv";
```

#### `JPanelReport`  <sub>(method)</sub>

> Creates new form JPanelReport

```java
    public JPanelReport(StartGUI parent)
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `jButtonReportExitActionPerformed`  <sub>(method)</sub>

```java
    private void jButtonReportExitActionPerformed(java.awt.event.ActionEvent evt) {//GEN-FIRST:event_jButtonReportExitActionPerformed
```

#### `jButtonReportBrowseActionPerformed`  <sub>(method)</sub>

```java
    private void jButtonReportBrowseActionPerformed(java.awt.event.ActionEvent evt) {//GEN-FIRST:event_jButtonReportBrowseActionPerformed
```

#### `jButtonGenerateActionPerformed`  <sub>(method)</sub>

```java
    private void jButtonGenerateActionPerformed(java.awt.event.ActionEvent evt) {//GEN-FIRST:event_jButtonGenerateActionPerformed
```

#### `jButtonBackActionPerformed`  <sub>(method)</sub>

```java
private void jButtonBackActionPerformed(java.awt.event.ActionEvent evt) {//GEN-FIRST:event_jButtonBackActionPerformed
```

#### `private javax.swing.JButton jButtonBack;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonBack;
```

#### `private javax.swing.JButton jButtonGenerate;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonGenerate;
```

#### `private javax.swing.JButton jButtonReportBrowse;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonReportBrowse;
```

#### `private javax.swing.JButton jButtonReportExit;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonReportExit;
```

#### `private javax.swing.JButton jButtonReportNew;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonReportNew;
```

#### `private javax.swing.JCheckBox jCheckBoxCSVReport`  <sub>(method)</sub>

```java
private javax.swing.JCheckBox jCheckBoxCSVReport;
```

#### `private javax.swing.JCheckBox jCheckBoxFullTextR`  <sub>(method)</sub>

```java
private javax.swing.JCheckBox jCheckBoxFullTextReport;
```

#### `private javax.swing.JCheckBox jCheckBoxReportXML`  <sub>(method)</sub>

```java
private javax.swing.JCheckBox jCheckBoxReportXML;
```

#### `private javax.swing.JCheckBox jCheckBoxSimpleLis`  <sub>(method)</sub>

```java
private javax.swing.JCheckBox jCheckBoxSimpleList;
```

#### `private javax.swing.JComboBox jComboBoxSimpleLis`  <sub>(method)</sub>

```java
private javax.swing.JComboBox jComboBoxSimpleListType;
```

#### `private javax.swing.JLabel jLabel1;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel1;
```

#### `private javax.swing.JLabel jLabel2;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabel2;
```

#### `public javax.swing.JTextField jTextFieldReportLo`  <sub>(method)</sub>

```java
public javax.swing.JTextField jTextFieldReportLocation;
```

#### `public javax.swing.JTextField jTextFieldReportNa`  <sub>(method)</sub>

```java
public javax.swing.JTextField jTextFieldReportName;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JPanelRunning.java`


#### `JPanelRunning`  <sub>(class)</sub>

```java
public class JPanelRunning extends javax.swing.JPanel implements ClipboardOwner
```

#### `public ResultsTableModel resultsTableModel;`  <sub>(method)</sub>

```java
public ResultsTableModel resultsTableModel;
```

#### `public ErrorTableModel errorTableModel;`  <sub>(method)</sub>

```java
public ErrorTableModel errorTableModel;
```

#### `private DirToCheck localDirToCheck;`  <sub>(method)</sub>

```java
private DirToCheck localDirToCheck;
```

#### `getInstance`  <sub>(method)</sub>

```java
private Manager manager = Manager.getInstance();
```

#### `public ResultsNode rootNode;`  <sub>(method)</sub>

```java
public ResultsNode rootNode;
```

#### `public JTreeTable jTableTreeResults;`  <sub>(method)</sub>

```java
public JTreeTable jTableTreeResults;
```

#### `public ResultsTableTreeModel tableTreeModel;`  <sub>(method)</sub>

```java
public ResultsTableTreeModel tableTreeModel;
```

#### `public ScanInfoTableModel scanInfoTableModel;`  <sub>(method)</sub>

```java
public ScanInfoTableModel scanInfoTableModel;
```

#### `JPanelRunning`  <sub>(method)</sub>

> Creates new form JPanelRunning

```java
    public JPanelRunning(StartGUI Parent)
```

#### `mouseClicked`  <sub>(method)</sub>

```java
            public void mouseClicked(java.awt.event.MouseEvent e)
```

#### `lostOwnership`  <sub>(method)</sub>

```java
    public void lostOwnership(Clipboard parClipboard, Transferable parTransferable)
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `menuSelected`  <sub>(method)</sub>

```java
            public void menuSelected(javax.swing.event.MenuEvent evt)
```

#### `menuDeselected`  <sub>(method)</sub>

```java
            public void menuDeselected(javax.swing.event.MenuEvent evt)
```

#### `menuCanceled`  <sub>(method)</sub>

```java
            public void menuCanceled(javax.swing.event.MenuEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `mouseClicked`  <sub>(method)</sub>

```java
                public void mouseClicked(java.awt.event.MouseEvent e)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
                        public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `enableReport`  <sub>(method)</sub>

```java
    public void enableReport()
```

#### `dymanicMenuAction`  <sub>(method)</sub>

> Function to process the selection and deselection of extentions to be scanned

```java
    private synchronized void dymanicMenuAction(java.awt.event.ActionEvent evt)
```

#### `upDateResult`  <sub>(method)</sub>

```java
    public void upDateResult(String finished, String started)
```

#### `setCurrentSpeed`  <sub>(method)</sub>

```java
    public void setCurrentSpeed(String progress)
```

#### `setAverageSpeed`  <sub>(method)</sub>

```java
    public void setAverageSpeed(String progress)
```

#### `setTotalRequests`  <sub>(method)</sub>

```java
    public void setTotalRequests(String progress)
```

#### `addScanInfoObject`  <sub>(method)</sub>

```java
    public void addScanInfoObject(JPanelScanInfo scanInfoPanel)
```

#### `removeAllScanInfoObjects`  <sub>(method)</sub>

```java
    public void removeAllScanInfoObjects()
```

#### `public javax.swing.JButton jButtonBack;`  <sub>(method)</sub>

```java
public javax.swing.JButton jButtonBack;
```

#### `public javax.swing.JButton jButtonChangeThreads;`  <sub>(method)</sub>

```java
public javax.swing.JButton jButtonChangeThreads;
```

#### `public javax.swing.JButton jButtonReport;`  <sub>(method)</sub>

```java
public javax.swing.JButton jButtonReport;
```

#### `public javax.swing.JButton jButtonStop;`  <sub>(method)</sub>

```java
public javax.swing.JButton jButtonStop;
```

#### `public javax.swing.JLabel jLabel1;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabel1;
```

#### `public javax.swing.JLabel jLabelAverageSpeed;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelAverageSpeed;
```

#### `public javax.swing.JLabel jLabelCurrentSpeed;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelCurrentSpeed;
```

#### `public javax.swing.JLabel jLabelCurrentTarget;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelCurrentTarget;
```

#### `public javax.swing.JLabel jLabelNumThreads;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelNumThreads;
```

#### `public javax.swing.JLabel jLabelParseQueueLength`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelParseQueueLength;
```

#### `public javax.swing.JLabel jLabelTimeLeft;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelTimeLeft;
```

#### `public javax.swing.JLabel jLabelTotalRequests;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelTotalRequests;
```

#### `public javax.swing.JMenu jMenuExtList;`  <sub>(method)</sub>

```java
public javax.swing.JMenu jMenuExtList;
```

#### `public javax.swing.JMenuItem jMenuItem1;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItem1;
```

#### `public javax.swing.JMenuItem jMenuItemCopy;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemCopy;
```

#### `public javax.swing.JMenuItem jMenuItemCopyURL;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemCopyURL;
```

#### `public javax.swing.JMenuItem jMenuItemOpen;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemOpen;
```

#### `public javax.swing.JMenuItem jMenuItemOpenInBrow`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemOpenInBrowser;
```

#### `public javax.swing.JMenuItem jMenuItemSkip;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemSkip;
```

#### `public javax.swing.JMenuItem jMenuItemSkipSubDir`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemSkipSubDirs;
```

#### `public javax.swing.JMenuItem jMenuItemView;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemView;
```

#### `public javax.swing.JMenuItem jMenuItemViewRespon`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemViewResponse;
```

#### `public javax.swing.JPanel jPanelScanInfoBase;`  <sub>(method)</sub>

```java
public javax.swing.JPanel jPanelScanInfoBase;
```

#### `public javax.swing.JPopupMenu jPopupMenuOpenBrow`  <sub>(method)</sub>

```java
public javax.swing.JPopupMenu jPopupMenuOpenBrowser;
```

#### `public javax.swing.JPopupMenu jPopupMenuTreeTabl`  <sub>(method)</sub>

```java
public javax.swing.JPopupMenu jPopupMenuTreeTableResults;
```

#### `public javax.swing.JScrollPane jScrollPaneErrorV`  <sub>(method)</sub>

```java
public javax.swing.JScrollPane jScrollPaneErrorView;
```

#### `public javax.swing.JScrollPane jScrollPaneListRe`  <sub>(method)</sub>

```java
public javax.swing.JScrollPane jScrollPaneListResults;
```

#### `public javax.swing.JScrollPane jScrollPaneScanIn`  <sub>(method)</sub>

```java
public javax.swing.JScrollPane jScrollPaneScanInformation;
```

#### `public javax.swing.JScrollPane jScrollPaneTreeVi`  <sub>(method)</sub>

```java
public javax.swing.JScrollPane jScrollPaneTreeView;
```

#### `public javax.swing.JSeparator jSeparator1;`  <sub>(method)</sub>

```java
public javax.swing.JSeparator jSeparator1;
```

#### `public javax.swing.JTabbedPane jTabbedPaneViewRe`  <sub>(method)</sub>

```java
public javax.swing.JTabbedPane jTabbedPaneViewResults;
```

#### `public javax.swing.JTable jTableErrors;`  <sub>(method)</sub>

```java
public javax.swing.JTable jTableErrors;
```

#### `public javax.swing.JTable jTableResults;`  <sub>(method)</sub>

```java
public javax.swing.JTable jTableResults;
```

#### `public javax.swing.JTextField jTextFieldNumberOf`  <sub>(method)</sub>

```java
public javax.swing.JTextField jTextFieldNumberOfThreads;
```

#### `public javax.swing.JToggleButton jToggleButtonPa`  <sub>(method)</sub>

```java
public javax.swing.JToggleButton jToggleButtonPause;
```

#### `setShowTarget`  <sub>(method)</sub>

```java
    public void setShowTarget(String target)
```

#### `getSelectedItemInTable`  <sub>(method)</sub>

```java
    public int getSelectedItemInTable()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JPanelScanInfo.java`


#### `JPanelScanInfo`  <sub>(class)</sub>

```java
public class JPanelScanInfo extends javax.swing.JPanel
```

#### `private int id;`  <sub>(method)</sub>

```java
private int id;
```

#### `getInstance`  <sub>(method)</sub>

```java
private Manager manager = Manager.getInstance();
```

#### `private MultiThreadedGenerator gen;`  <sub>(method)</sub>

```java
private MultiThreadedGenerator gen;
```

#### `private int type;`  <sub>(method)</sub>

```java
private int type;
```

#### `JPanelScanInfo`  <sub>(method)</sub>

> Creates new form JPanelScanInfo

```java
    public JPanelScanInfo(int ThreadId, int type)
```

#### `setProgressMax`  <sub>(method)</sub>

```java
    public void setProgressMax(int max)
```

#### `setProgressText`  <sub>(method)</sub>

```java
    public void setProgressText(String text)
```

#### `setProgressCurrentValue`  <sub>(method)</sub>

```java
    public void setProgressCurrentValue(int value)
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `private javax.swing.JButton jButtonStop;`  <sub>(method)</sub>

```java
private javax.swing.JButton jButtonStop;
```

#### `private javax.swing.JLabel jLabelText;`  <sub>(method)</sub>

```java
private javax.swing.JLabel jLabelText;
```

#### `private javax.swing.JProgressBar jProgressBar1;`  <sub>(method)</sub>

```java
private javax.swing.JProgressBar jProgressBar1;
```

#### `private javax.swing.JSeparator jSeparator1;`  <sub>(method)</sub>

```java
private javax.swing.JSeparator jSeparator1;
```

#### `private javax.swing.JToggleButton jToggleButtonP`  <sub>(method)</sub>

```java
private javax.swing.JToggleButton jToggleButtonPause;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JPanelSetup.java`


#### `JPanelSetup`  <sub>(class)</sub>

```java
public class JPanelSetup extends javax.swing.JPanel
```

#### `private String host;`  <sub>(method)</sub>

```java
private String host;
```

#### `private int port;`  <sub>(method)</sub>

```java
private int port;
```

#### `private String protocol;`  <sub>(method)</sub>

```java
private String protocol;
```

#### `private String startPoint;`  <sub>(method)</sub>

```java
private String startPoint;
```

#### `private String[] charSet1 =`  <sub>(method)</sub>

```java
private String[] charSet1 =
```

#### `private String[] charSet0 =`  <sub>(method)</sub>

```java
private String[] charSet0 =
```

#### `private String[] charSet2 =`  <sub>(method)</sub>

```java
private String[] charSet2 =
```

#### `private String[] charSet3 =`  <sub>(method)</sub>

```java
private String[] charSet3 =
```

#### `private boolean fileExtChanged = false;`  <sub>(method)</sub>

```java
private boolean fileExtChanged = false;
```

#### `private boolean dirToStartWithChanged = false;`  <sub>(method)</sub>

```java
private boolean dirToStartWithChanged = false;
```

#### `private boolean urlFuzzChanged = false;`  <sub>(method)</sub>

```java
private boolean urlFuzzChanged = false;
```

#### `private String fuzzStart = "";`  <sub>(method)</sub>

```java
private String fuzzStart = "";
```

#### `private String fuzzEnd = "";`  <sub>(method)</sub>

```java
private String fuzzEnd = "";
```

#### `private Manager manager;`  <sub>(method)</sub>

```java
private Manager manager;
```

#### `JPanelSetup`  <sub>(method)</sub>

> Creates new form NewJPanelSetup

```java
    public JPanelSetup(StartGUI Parent)
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `stateChanged`  <sub>(method)</sub>

```java
            public void stateChanged(javax.swing.event.ChangeEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `processTarget`  <sub>(method)</sub>

> Process the target url, and extract all the required information

```java
    public void processTarget()
```

#### `fileExtUpdated`  <sub>(method)</sub>

```java
    public void fileExtUpdated()
```

#### `dirToStartWithUpdated`  <sub>(method)</sub>

```java
    public void dirToStartWithUpdated()
```

#### `urlFuzzUpdated`  <sub>(method)</sub>

```java
    public void urlFuzzUpdated()
```

#### `public javax.swing.ButtonGroup buttonGroup1;`  <sub>(method)</sub>

```java
public javax.swing.ButtonGroup buttonGroup1;
```

#### `public javax.swing.ButtonGroup buttonGroupStartP`  <sub>(method)</sub>

```java
public javax.swing.ButtonGroup buttonGroupStartPointConfig;
```

#### `public javax.swing.ButtonGroup buttonGroupWorkMe`  <sub>(method)</sub>

```java
public javax.swing.ButtonGroup buttonGroupWorkMethod;
```

#### `public javax.swing.JButton jButtonBrowse;`  <sub>(method)</sub>

```java
public javax.swing.JButton jButtonBrowse;
```

#### `public javax.swing.JButton jButtonExit;`  <sub>(method)</sub>

```java
public javax.swing.JButton jButtonExit;
```

#### `public javax.swing.JButton jButtonListInfo;`  <sub>(method)</sub>

```java
public javax.swing.JButton jButtonListInfo;
```

#### `public javax.swing.JButton jButtonStart;`  <sub>(method)</sub>

```java
public javax.swing.JButton jButtonStart;
```

#### `public javax.swing.JCheckBox jCheckBoxDoFiles;`  <sub>(method)</sub>

```java
public javax.swing.JCheckBox jCheckBoxDoFiles;
```

#### `public javax.swing.JCheckBox jCheckBoxGoFast;`  <sub>(method)</sub>

```java
public javax.swing.JCheckBox jCheckBoxGoFast;
```

#### `public javax.swing.JCheckBox jCheckBoxRecursive;`  <sub>(method)</sub>

```java
public javax.swing.JCheckBox jCheckBoxRecursive;
```

#### `public javax.swing.JCheckBox jCheckBoxUseBlankEx`  <sub>(method)</sub>

```java
public javax.swing.JCheckBox jCheckBoxUseBlankExt;
```

#### `public javax.swing.JCheckBox jCheckBoxdoDirs;`  <sub>(method)</sub>

```java
public javax.swing.JCheckBox jCheckBoxdoDirs;
```

#### `public javax.swing.JComboBox jComboBoxCharSet;`  <sub>(method)</sub>

```java
public javax.swing.JComboBox jComboBoxCharSet;
```

#### `public javax.swing.JLabel jLabel3;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabel3;
```

#### `public javax.swing.JLabel jLabel4;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabel4;
```

#### `public javax.swing.JLabel jLabel5;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabel5;
```

#### `public javax.swing.JLabel jLabel6;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabel6;
```

#### `public javax.swing.JLabel jLabel7;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabel7;
```

#### `public javax.swing.JLabel jLabel8;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabel8;
```

#### `public javax.swing.JLabel jLabel9;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabel9;
```

#### `public javax.swing.JLabel jLabelFuzzTitle;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelFuzzTitle;
```

#### `public javax.swing.JLabel jLabelList;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelList;
```

#### `public javax.swing.JLabel jLabelPureBrute1;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelPureBrute1;
```

#### `public javax.swing.JLabel jLabelPureBrute2;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelPureBrute2;
```

#### `public javax.swing.JLabel jLabelPureBrute3;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelPureBrute3;
```

#### `public javax.swing.JLabel jLabelThreadsDisplay;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelThreadsDisplay;
```

#### `public javax.swing.JRadioButton jRadioButtonAuto`  <sub>(method)</sub>

```java
public javax.swing.JRadioButton jRadioButtonAuto;
```

#### `public javax.swing.JRadioButton jRadioButtonGET;`  <sub>(method)</sub>

```java
public javax.swing.JRadioButton jRadioButtonGET;
```

#### `public javax.swing.JRadioButton jRadioButtonList`  <sub>(method)</sub>

```java
public javax.swing.JRadioButton jRadioButtonListBased;
```

#### `public javax.swing.JRadioButton jRadioButtonPure`  <sub>(method)</sub>

```java
public javax.swing.JRadioButton jRadioButtonPureBrute;
```

#### `public javax.swing.JRadioButton jRadioButtonStan`  <sub>(method)</sub>

```java
public javax.swing.JRadioButton jRadioButtonStandardStart;
```

#### `public javax.swing.JRadioButton jRadioButtonURLF`  <sub>(method)</sub>

```java
public javax.swing.JRadioButton jRadioButtonURLFuzz;
```

#### `public javax.swing.JSeparator jSeparator1;`  <sub>(method)</sub>

```java
public javax.swing.JSeparator jSeparator1;
```

#### `public javax.swing.JSeparator jSeparator2;`  <sub>(method)</sub>

```java
public javax.swing.JSeparator jSeparator2;
```

#### `public javax.swing.JSeparator jSeparator3;`  <sub>(method)</sub>

```java
public javax.swing.JSeparator jSeparator3;
```

#### `public javax.swing.JSeparator jSeparator4;`  <sub>(method)</sub>

```java
public javax.swing.JSeparator jSeparator4;
```

#### `public javax.swing.JSlider jSliderThreads;`  <sub>(method)</sub>

```java
public javax.swing.JSlider jSliderThreads;
```

#### `public javax.swing.JTextField jTextFieldDirToSta`  <sub>(method)</sub>

```java
public javax.swing.JTextField jTextFieldDirToStart;
```

#### `public javax.swing.JTextField jTextFieldFile;`  <sub>(method)</sub>

```java
public javax.swing.JTextField jTextFieldFile;
```

#### `public javax.swing.JTextField jTextFieldFileExte`  <sub>(method)</sub>

```java
public javax.swing.JTextField jTextFieldFileExtention;
```

#### `public javax.swing.JTextField jTextFieldMaxLengt`  <sub>(method)</sub>

```java
public javax.swing.JTextField jTextFieldMaxLength;
```

#### `public javax.swing.JTextField jTextFieldMin;`  <sub>(method)</sub>

```java
public javax.swing.JTextField jTextFieldMin;
```

#### `public javax.swing.JTextField jTextFieldTarget;`  <sub>(method)</sub>

```java
public javax.swing.JTextField jTextFieldTarget;
```

#### `public javax.swing.JTextField jTextFieldURLFuzz;`  <sub>(method)</sub>

```java
public javax.swing.JTextField jTextFieldURLFuzz;
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/StartGUI.java`


#### `StartGUI`  <sub>(class)</sub>

```java
public class StartGUI extends javax.swing.JFrame
```

#### `StartGUI`  <sub>(method)</sub>

> Creates new form StartGUI

```java
    public StartGUI()
```

#### `initComponents`  <sub>(method)</sub>

```java
    private void initComponents()
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
            public void actionPerformed(java.awt.event.ActionEvent evt)
```

#### `main`  <sub>(method)</sub>

```java
    public static void main(String args[])
```

#### `run`  <sub>(method)</sub>

```java
            public void run()
```

#### `showSetup`  <sub>(method)</sub>

```java
    public void showSetup()
```

#### `showRunning`  <sub>(method)</sub>

```java
    public void showRunning()
```

#### `showReporting`  <sub>(method)</sub>

```java
    public void showReporting()
```

#### `updateTable`  <sub>(method)</sub>

```java
    public void updateTable(String finished, String started)
```

#### `updateProgress`  <sub>(method)</sub>

> public void updateTable(String finished, String started)  
> {  
> jPanelRunning.upDateResult(finished, started);  
> }

```java
    public void updateProgress(String current, String average, String total, int numberOfThreads, String timeLeft, String parseQueueLength)
```

#### `pause`  <sub>(method)</sub>

```java
    public void pause()
```

#### `unPause`  <sub>(method)</sub>

```java
    public void unPause()
```

#### `setStatus`  <sub>(method)</sub>

```java
    public void setStatus(String status)
```

#### `startBruteForceFile`  <sub>(method)</sub>

```java
    public void startBruteForceFile(String dirToStartWith,
                String fileToRead,
                String protocol,
                String host,
                int port,
                String fileExtention,
                int threadsNumber,
                boolean doDirs,
                boolean doFiles,
                boolean recursive,
                boolean auto,
                boolean useBlankExt,
                Vector extToUse)
```

#### `startBruteForcePure`  <sub>(method)</sub>

```java
    public void startBruteForcePure(String dirToStartWith, String[] charSet, int minLen, int maxLen, String protocol, String host, int port, String fileExtention, int threadsNumber, boolean doDirs, boolean doFiles, boolean recursive, boolean auto, boolean useBlankExt, Vector extToUse)
```

#### `startBruteForceFuzz`  <sub>(method)</sub>

> start bruteforce fuzzing

```java
    public void startBruteForceFuzz(String[] charSet, int minLen, int maxLen, String protocol, String host, int port, int threadsNumber, boolean auto, String urlFuzzStart, String urlFuzzEnd)
```

#### `setUpManager`  <sub>(method)</sub>

```java
    public void setUpManager(String inputFile,
        String protocol,
        String host,
        int port,
        StartGUI gui,
        int ThreadNumber,
        String urlFuzzStart,
        String urlFuzzEnd)
```

#### `startListBasedFuzz`  <sub>(method)</sub>

> public void setUpManager(String inputFile,  
> String protocol,  
> String host,  
> int port,  
> StartGUI gui,  
> int ThreadNumber,  
> String urlFuzzStart,  
> String urlFuzzEnd)

```java
    public void startListBasedFuzz(String inputFile,
                String protocol,
                String host,
                int port,
                int threadNumber,
                String urlFuzzStart,
                String urlFuzzEnd,
                boolean auto)
```

#### `finished`  <sub>(method)</sub>

```java
    public void finished()
```

#### `enableReport`  <sub>(method)</sub>

```java
    public void enableReport()
```

#### `loadHelp`  <sub>(method)</sub>

```java
    private void loadHelp()
```

#### `checkForUpdate`  <sub>(method)</sub>

> used to check for an update

```java
    private void checkForUpdate()
```

#### `setURL`  <sub>(method)</sub>

```java
    public void setURL(String URL)
```

#### `createLookAndFeelMenus`  <sub>(method)</sub>

```java
    private void createLookAndFeelMenus()
```

#### `ButtonGroup`  <sub>(method)</sub>

```java
final ButtonGroup group = new ButtonGroup();
```

#### `JRadioButtonMenuItem`  <sub>(method)</sub>

```java
final JRadioButtonMenuItem rb = new JRadioButtonMenuItem(lookAndFeels.elementAt(i).getName());
```

#### `itemStateChanged`  <sub>(method)</sub>

```java
               public void itemStateChanged(final ItemEvent ie)
```

#### `getSource`  <sub>(method)</sub>

```java
final JRadioButtonMenuItem rbi = (JRadioButtonMenuItem) ie.getSource();
```

#### `getClientProperty`  <sub>(method)</sub>

```java
final UIManager.LookAndFeelInfo info = (UIManager.LookAndFeelInfo) rbi.getClientProperty("Look and Feel Name");
```

#### `run`  <sub>(method)</sub>

```java
                           public void run()
```

#### `public JPanelSetup jPanelSetup;`  <sub>(method)</sub>

```java
public JPanelSetup jPanelSetup;
```

#### `public JPanelRunning jPanelRunning;`  <sub>(method)</sub>

```java
public JPanelRunning jPanelRunning;
```

#### `public JPanelReport jPanelReport;`  <sub>(method)</sub>

```java
public JPanelReport jPanelReport;
```

#### `public javax.swing.JCheckBoxMenuItem jCheckBoxMe`  <sub>(method)</sub>

```java
public javax.swing.JCheckBoxMenuItem jCheckBoxMenuDebug;
```

#### `public javax.swing.JCheckBoxMenuItem jCheckBoxMe`  <sub>(method)</sub>

```java
public javax.swing.JCheckBoxMenuItem jCheckBoxMenuFollowRedirets;
```

#### `public javax.swing.JCheckBoxMenuItem jCheckBoxMe`  <sub>(method)</sub>

```java
public javax.swing.JCheckBoxMenuItem jCheckBoxMenuItemCaseInsensativeMode;
```

#### `public javax.swing.JCheckBoxMenuItem jCheckBoxMe`  <sub>(method)</sub>

```java
public javax.swing.JCheckBoxMenuItem jCheckBoxMenuParseHTML;
```

#### `public javax.swing.JLabel jLabelCurrentWork;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelCurrentWork;
```

#### `public javax.swing.JLabel jLabelStatus;`  <sub>(method)</sub>

```java
public javax.swing.JLabel jLabelStatus;
```

#### `public javax.swing.JMenu jMenuAbout1;`  <sub>(method)</sub>

```java
public javax.swing.JMenu jMenuAbout1;
```

#### `public javax.swing.JMenuBar jMenuBar;`  <sub>(method)</sub>

```java
public javax.swing.JMenuBar jMenuBar;
```

#### `public javax.swing.JMenu jMenuFile1;`  <sub>(method)</sub>

```java
public javax.swing.JMenu jMenuFile1;
```

#### `public javax.swing.JMenu jMenuHelp1;`  <sub>(method)</sub>

```java
public javax.swing.JMenu jMenuHelp1;
```

#### `public javax.swing.JMenuItem jMenuItemAdvanced;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemAdvanced;
```

#### `public javax.swing.JMenuItem jMenuItemCheck;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemCheck;
```

#### `public javax.swing.JMenuItem jMenuItemExit;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemExit;
```

#### `public javax.swing.JMenuItem jMenuItemFAQ;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemFAQ;
```

#### `public javax.swing.JMenuItem jMenuItemHelp;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemHelp;
```

#### `public javax.swing.JMenuItem jMenuItemLicence;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemLicence;
```

#### `public javax.swing.JMenuItem jMenuItemNew;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemNew;
```

#### `public javax.swing.JMenuItem jMenuItemReport;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemReport;
```

#### `public javax.swing.JMenuItem jMenuItemVersion;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemVersion;
```

#### `public javax.swing.JMenuItem jMenuItemhome;`  <sub>(method)</sub>

```java
public javax.swing.JMenuItem jMenuItemhome;
```

#### `public javax.swing.JMenu jMenuLookAndFeel;`  <sub>(method)</sub>

```java
public javax.swing.JMenu jMenuLookAndFeel;
```

#### `public javax.swing.JMenu jMenuOptions;`  <sub>(method)</sub>

```java
public javax.swing.JMenu jMenuOptions;
```

#### `public javax.swing.JPanel jPanelStatus;`  <sub>(method)</sub>

```java
public javax.swing.JPanel jPanelStatus;
```

#### `public javax.swing.JSeparator jSeparator1;`  <sub>(method)</sub>

```java
public javax.swing.JSeparator jSeparator1;
```

#### `public javax.swing.JSeparator jSeparator2;`  <sub>(method)</sub>

```java
public javax.swing.JSeparator jSeparator2;
```

#### `public javax.swing.JSeparator jSeparator3;`  <sub>(method)</sub>

```java
public javax.swing.JSeparator jSeparator3;
```


## `src/com/sittinglittleduck/DirBuster/gui/JTableTree` <a id='pkg-src-com-sittinglittleduck-dirbuster-gui-jtabletree'></a>

*110 documented items*


### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JTableTree/AbstractCellEditor.java`


#### `AbstractCellEditor`  <sub>(class)</sub>

> A base class for CellEditors, providing default implementations for all  
> methods in the CellEditor interface and support for managing a series  
> of listeners.

```java
public class AbstractCellEditor implements CellEditor
```

#### `EventListenerList`  <sub>(method)</sub>

```java
protected EventListenerList listenerList = new EventListenerList();
```

#### `cancelCellEditing`  <sub>(method)</sub>

```java
    public void cancelCellEditing() {}
```

#### `addCellEditorListener`  <sub>(method)</sub>

```java
    public void addCellEditorListener(CellEditorListener l)
```

#### `removeCellEditorListener`  <sub>(method)</sub>

```java
    public void removeCellEditorListener(CellEditorListener l)
```

#### `fireEditingStopped`  <sub>(method)</sub>

> Notify all listeners that have registered interest for  
> notification on this event type.

```java
    protected void fireEditingStopped()
```

#### `fireEditingCanceled`  <sub>(method)</sub>

> Notify all listeners that have registered interest for  
> notification on this event type.

```java
    protected void fireEditingCanceled()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JTableTree/AbstractTreeTableModel.java`


#### `AbstractTreeTableModel`  <sub>(class)</sub>

> An abstract implementation of the TreeTableModel interface, handling  
> the list of listeners.

```java
public abstract class AbstractTreeTableModel implements TreeTableModel
```

#### `protected Object root;`  <sub>(method)</sub>

```java
protected Object root;
```

#### `EventListenerList`  <sub>(method)</sub>

```java
protected EventListenerList listenerList = new EventListenerList();
```

#### `AbstractTreeTableModel`  <sub>(method)</sub>

```java
    public AbstractTreeTableModel(Object root)
```

#### `getRoot`  <sub>(method)</sub>

```java
    public Object getRoot()
```

#### `isLeaf`  <sub>(method)</sub>

```java
    public boolean isLeaf(Object node)
```

#### `valueForPathChanged`  <sub>(method)</sub>

```java
    public void valueForPathChanged(TreePath path, Object newValue) {}
```

#### `getIndexOfChild`  <sub>(method)</sub>

```java
    public int getIndexOfChild(Object parent, Object child)
```

#### `addTreeModelListener`  <sub>(method)</sub>

```java
    public void addTreeModelListener(TreeModelListener l)
```

#### `removeTreeModelListener`  <sub>(method)</sub>

```java
    public void removeTreeModelListener(TreeModelListener l)
```

#### `fireTreeNodesChanged`  <sub>(method)</sub>

> Notify all listeners that have registered interest for  
> notification on this event type.  The event instance  
> is lazily created using the parameters passed into  
> the fire method.

```java
    protected void fireTreeNodesChanged(Object source, Object[] path,
                                            int[] childIndices,
                                            Object[] children)
```

#### `fireTreeNodesInserted`  <sub>(method)</sub>

> Notify all listeners that have registered interest for  
> notification on this event type.  The event instance  
> is lazily created using the parameters passed into  
> the fire method.

```java
    protected void fireTreeNodesInserted(Object source, Object[] path,
                                            int[] childIndices,
                                            Object[] children)
```

#### `fireTreeNodesRemoved`  <sub>(method)</sub>

> Notify all listeners that have registered interest for  
> notification on this event type.  The event instance  
> is lazily created using the parameters passed into  
> the fire method.

```java
    protected void fireTreeNodesRemoved(Object source, Object[] path,
                                            int[] childIndices,
                                            Object[] children)
```

#### `fireTreeStructureChanged`  <sub>(method)</sub>

> Notify all listeners that have registered interest for  
> notification on this event type.  The event instance  
> is lazily created using the parameters passed into  
> the fire method.

```java
    protected void fireTreeStructureChanged(Object source, Object[] path,
                                            int[] childIndices,
                                            Object[] children)
```

#### `isCellEditable`  <sub>(method)</sub>

> By default, make the column with the Tree in it the only editable one.  
> Making this column editable causes the JTable to forward mouse  
> and keyboard events in the Tree column to the underlying JTree.

```java
    public boolean isCellEditable(Object node, int column)
```

#### `setValueAt`  <sub>(method)</sub>

```java
    public void setValueAt(Object aValue, Object node, int column) {}
```

#### `getPathToRoot`  <sub>(method)</sub>

> public Object getChild(Object parent, int index)  
> public int getChildCount(Object parent)  
> public int getColumnCount()  
> public String getColumnName(Object node, int column)  
> public Object getValueAt(Object node, int column)

```java
    public TreeNode[] getPathToRoot(TreeNode aNode)
```

#### `getPathToRoot`  <sub>(method)</sub>

```java
    private TreeNode[] getPathToRoot(TreeNode aNode, int depth)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JTableTree/DynamicTreeTableModel.java`


#### `DynamicTreeTableModel`  <sub>(class)</sub>

> An implementation of TreeTableModel that uses reflection to answer  
> TableModel methods. This works off a handful  
> of values. A TreeNode is used to answer all the TreeModel related  
> methods (similiar to AbstractTreeTableModel and DefaultTreeModel).  
> The column names are specified in the constructor. The values for  
> the columns are dynamically obtained via reflection, you simply  
> provide the method names. The methods used to set a particular value are  
> also specified as an array of method names, a null method name, or  
> null array indicates the column isn't editable. And the class types,  
> used for the TableModel method getColumnClass are specified in the  
> constructor.

```java
public class DynamicTreeTableModel extends AbstractTreeTableModel
```

#### `private String[]        columnNames;`  <sub>(method)</sub>

```java
private String[]        columnNames;
```

#### `private String[]        methodNames;`  <sub>(method)</sub>

```java
private String[]        methodNames;
```

#### `private String[]        setterMethodNames;`  <sub>(method)</sub>

```java
private String[]        setterMethodNames;
```

#### `private Class[]         cTypes;`  <sub>(method)</sub>

```java
private Class[]         cTypes;
```

#### `DynamicTreeTableModel`  <sub>(method)</sub>

> Constructor for creating a DynamicTreeTableModel.

```java
    public DynamicTreeTableModel(TreeNode root, String[] columnNames,
    				 String[] getterMethodNames,
    				 String[] setterMethodNames,
    				 Class[] cTypes)
```

#### `getChildCount`  <sub>(method)</sub>

> TreeModel method to return the number of children of a particular  
> node. Since <code>node</code> is a TreeNode, this can be answered  
> via the TreeNode method <code>getChildCount</code>.

```java
    public int getChildCount(Object node)
```

#### `getChild`  <sub>(method)</sub>

> TreeModel method to locate a particular child of the specified  
> node. Since <code>node</code> is a TreeNode, this can be answered  
> via the TreeNode method <code>getChild</code>.

```java
    public Object getChild(Object node, int i)
```

#### `isLeaf`  <sub>(method)</sub>

> TreeModel method to determine if a node is a leaf.  
> Since <code>node</code> is a TreeNode, this can be answered  
> via the TreeNode method <code>isLeaf</code>.

```java
    public boolean isLeaf(Object node)
```

#### `getColumnCount`  <sub>(method)</sub>

> Returns the number of column names passed into the constructor.

```java
    public int getColumnCount()
```

#### `getColumnName`  <sub>(method)</sub>

> Returns the column name passed into the constructor.

```java
    public String getColumnName(int column)
```

#### `getColumnClass`  <sub>(method)</sub>

> Returns the column class for column <code>column</code>. This  
> is set in the constructor.

```java
    public Class getColumnClass(int column)
```

#### `getValueAt`  <sub>(method)</sub>

> Returns the value for the column <code>column</code> and object  
> <code>node</code>. The return value is determined by invoking  
> the method specified in constructor for the passed in column.

```java
    public Object getValueAt(Object node, int column)
```

#### `isCellEditable`  <sub>(method)</sub>

> Returns true if there is a setter method name for column  
> <code>column</code>. This is set in the constructor.

```java
    public boolean isCellEditable(Object node, int column)
```

#### `setValueAt`  <sub>(method)</sub>

```java
    public void setValueAt(Object aValue, Object node, int column)
```

#### `getParameterTypes`  <sub>(method)</sub>

```java
		       getParameterTypes().length == 1)
```

#### `getPathToRoot`  <sub>(method)</sub>

> Builds the parents of the node up to and including the root node,  
> where the original node is the last element in the returned array.  
> The length of the returned array gives the node's depth in the  
> tree.  
> specified node.

```java
    public TreeNode[] getPathToRoot(TreeNode aNode)
```

#### `getPathToRoot`  <sub>(method)</sub>

> Builds the parents of the node up to and including the root node,  
> where the original node is the last element in the returned array.  
> The length of the returned array gives the node's depth in the  
> tree.  
> the root (on recursive calls), used to size the returned array  
> specified node

```java
    private TreeNode[] getPathToRoot(TreeNode aNode, int depth)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JTableTree/JTreeTable.java`


#### `JTreeTable`  <sub>(class)</sub>

> This example shows how to create a simple JTreeTable component,  
> by using a JTree as a renderer (and editor) for the cells in a  
> particular column in the JTable.

```java
public class JTreeTable extends JTable
```

#### `protected TreeTableCellRenderer tree;`  <sub>(method)</sub>

```java
protected TreeTableCellRenderer tree;
```

#### `JTreeTable`  <sub>(method)</sub>

```java
    public JTreeTable(TreeTableModel treeTableModel)
```

#### `updateUI`  <sub>(method)</sub>

> Overridden to message super and forward the method to the tree.  
> Since the tree is not actually in the component hierarchy it will  
> never receive this unless we forward it in this manner.

```java
    public void updateUI()
```

#### `getEditingRow`  <sub>(method)</sub>

> Workaround for BasicTableUI anomaly. Make sure the UI never tries to  
> paint the editor. The UI currently uses different techniques to  
> paint the renderers and editors and overriding setBounds() below  
> is not the right thing to do for an editor. Returning -1 for the  
> editing row in this case, ensures the editor is never painted.

```java
    public int getEditingRow()
```

#### `setRowHeight`  <sub>(method)</sub>

> Overridden to pass the new rowHeight to the tree.

```java
    public void setRowHeight(int rowHeight)
```

#### `getTree`  <sub>(method)</sub>

> Returns the tree that is being shared between the model.

```java
    public JTree getTree()
```

#### `TreeTableCellRenderer`  <sub>(class)</sub>

> A TreeCellRenderer that displays a JTree.

```java
public class TreeTableCellRenderer extends JTree implements
```

#### `protected int visibleRow;`  <sub>(method)</sub>

```java
protected int visibleRow;
```

#### `TreeTableCellRenderer`  <sub>(method)</sub>

```java
        public TreeTableCellRenderer(TreeModel model)
```

#### `updateUI`  <sub>(method)</sub>

> updateUI is overridden to set the colors of the Tree's renderer  
> to match that of the table.

```java
        public void updateUI()
```

#### `setRowHeight`  <sub>(method)</sub>

> Sets the row height of the tree, and forwards the row height to  
> the table.

```java
        public void setRowHeight(int rowHeight)
```

#### `setBounds`  <sub>(method)</sub>

> This is overridden to set the height to match that of the JTable.

```java
        public void setBounds(int x, int y, int w, int h)
```

#### `paint`  <sub>(method)</sub>

> Sublcassed to translate the graphics such that the last visible  
> row will be drawn at 0,0.

```java
        public void paint(Graphics g)
```

#### `getTableCellRendererComponent`  <sub>(method)</sub>

> TreeCellRenderer method. Overridden to update the visible row.

```java
        public Component getTableCellRendererComponent(JTable table,
                                                           Object value,
                                                           boolean isSelected,
                                                           boolean hasFocus,
                                                           int row, int column)
```

#### `TreeTableCellEditor`  <sub>(class)</sub>

> TreeTableCellEditor implementation. Component returned is the  
> JTree.

```java
public class TreeTableCellEditor extends AbstractCellEditor implements
```

#### `getTableCellEditorComponent`  <sub>(method)</sub>

```java
        public Component getTableCellEditorComponent(JTable table,
                                                         Object value,
                                                         boolean isSelected,
                                                         int r, int c)
```

#### `isCellEditable`  <sub>(method)</sub>

> Overridden to return false, and if the event is a mouse event  
> it is forwarded to the tree.<p>  
> The behavior for this is debatable, and should really be offered  
> as a property. By returning false, all keyboard actions are  
> implemented in terms of the table. By returning true, the  
> tree would get a chance to do something with the keyboard  
> events. For the most part this is ok. But for certain keys,  
> such as left/right, the tree will expand/collapse where as  
> the table focus should really move to a different column. Page  
> up/down should also be implemented in terms of the table.  
> By returning false this also has the added benefit that clicking  
> outside of the bounds of the tree node, but still in the tree  
> column will select the row, whereas if this returned true  
> that wouldn't be the case.  
> <p>By returning false we are also enforcing the policy that  
> the tree will never be editable (at least by a key sequence).

```java
        public boolean isCellEditable(EventObject e)
```

#### `ListToTreeSelectionModelWrapper`  <sub>(class)</sub>

> ListToTreeSelectionModelWrapper extends DefaultTreeSelectionModel  
> to listen for changes in the ListSelectionModel it maintains. Once  
> a change in the ListSelectionModel happens, the paths are updated  
> in the DefaultTreeSelectionModel.

```java
class ListToTreeSelectionModelWrapper extends DefaultTreeSelectionModel
```

#### `protected boolean         updatingListSelectionM`  <sub>(method)</sub>

```java
protected boolean         updatingListSelectionModel;
```

#### `ListToTreeSelectionModelWrapper`  <sub>(method)</sub>

```java
        public ListToTreeSelectionModelWrapper()
```

#### `getListSelectionModel`  <sub>(method)</sub>

> Returns the list selection model. ListToTreeSelectionModelWrapper  
> listens for changes to this model and updates the selected paths  
> accordingly.

```java
        ListSelectionModel getListSelectionModel()
```

#### `resetRowSelection`  <sub>(method)</sub>

> This is overridden to set <code>updatingListSelectionModel</code>  
> and message super. This is the only place DefaultTreeSelectionModel  
> alters the ListSelectionModel.

```java
        public void resetRowSelection()
```

#### `createListSelectionListener`  <sub>(method)</sub>

> Creates and returns an instance of ListSelectionHandler.

```java
        protected ListSelectionListener createListSelectionListener()
```

#### `updateSelectedPathsFromSelectedRows`  <sub>(method)</sub>

> If <code>updatingListSelectionModel</code> is false, this will  
> reset the selected paths from the selected rows in the list  
> selection model.

```java
        protected void updateSelectedPathsFromSelectedRows()
```

#### `ListSelectionHandler`  <sub>(class)</sub>

> Class responsible for calling updateSelectedPathsFromSelectedRows  
> when the selection of the list changse.

```java
class ListSelectionHandler implements ListSelectionListener
```

#### `valueChanged`  <sub>(method)</sub>

```java
            public void valueChanged(ListSelectionEvent e)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JTableTree/MergeSort.java`


#### `MergeSort`  <sub>(class)</sub>

> An implementation of MergeSort, needs to be subclassed to provide a  
> comparator.

```java
public abstract class MergeSort extends Object
```

#### `sort`  <sub>(method)</sub>

```java
    public void sort(Object array[])
```

#### `mergeSort`  <sub>(method)</sub>

```java
    protected void mergeSort(int begin, int end)
```

#### `merge`  <sub>(method)</sub>

```java
    protected void merge(int begin, int middle, int end)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JTableTree/TreeTableModel.java`


#### `TreeTableModel`  <sub>(interface)</sub>

> TreeTableModel is the model used by a JTreeTable. It extends TreeModel  
> to add methods for getting inforamtion about the set of columns each  
> node in the TreeTableModel may have. Each column, like a column in  
> a TableModel, has a name and a type associated with it. Each node in  
> the TreeTableModel can return a value for each of the columns and  
> set that value if isCellEditable() returns true.

```java
public interface TreeTableModel extends TreeModel
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JTableTree/TreeTableModelAdapter.java`


#### `TreeTableModelAdapter`  <sub>(class)</sub>

> This is a wrapper class takes a TreeTableModel and implements  
> the table model interface. The implementation is trivial, with  
> all of the event dispatching support provided by the superclass:  
> the AbstractTableModel.

```java
public class TreeTableModelAdapter extends AbstractTableModel
```

#### `TreeTableModelAdapter`  <sub>(method)</sub>

```java
    public TreeTableModelAdapter(TreeTableModel treeTableModel, JTree tree)
```

#### `treeExpanded`  <sub>(method)</sub>

```java
	    public void treeExpanded(TreeExpansionEvent event)
```

#### `treeCollapsed`  <sub>(method)</sub>

```java
            public void treeCollapsed(TreeExpansionEvent event)
```

#### `treeNodesChanged`  <sub>(method)</sub>

```java
	    public void treeNodesChanged(TreeModelEvent e)
```

#### `treeNodesInserted`  <sub>(method)</sub>

```java
	    public void treeNodesInserted(TreeModelEvent e)
```

#### `treeNodesRemoved`  <sub>(method)</sub>

```java
	    public void treeNodesRemoved(TreeModelEvent e)
```

#### `treeStructureChanged`  <sub>(method)</sub>

```java
	    public void treeStructureChanged(TreeModelEvent e)
```

#### `getColumnCount`  <sub>(method)</sub>

```java
    public int getColumnCount()
```

#### `getColumnName`  <sub>(method)</sub>

```java
    public String getColumnName(int column)
```

#### `getColumnClass`  <sub>(method)</sub>

```java
    public Class getColumnClass(int column)
```

#### `getRowCount`  <sub>(method)</sub>

```java
    public int getRowCount()
```

#### `nodeForRow`  <sub>(method)</sub>

```java
    protected Object nodeForRow(int row)
```

#### `getValueAt`  <sub>(method)</sub>

```java
    public Object getValueAt(int row, int column)
```

#### `getRowNode`  <sub>(method)</sub>

```java
    public Object getRowNode(int row)
```

#### `isCellEditable`  <sub>(method)</sub>

```java
    public boolean isCellEditable(int row, int column)
```

#### `setValueAt`  <sub>(method)</sub>

```java
    public void setValueAt(Object value, int row, int column)
```

#### `delayedFireTableDataChanged`  <sub>(method)</sub>

> Invokes fireTableDataChanged after all the pending events have been  
> processed. SwingUtilities.invokeLater is used to handle this.

```java
    protected void delayedFireTableDataChanged()
```

#### `run`  <sub>(method)</sub>

```java
	    public void run()
```

#### `delayedFireTableCellUpdated`  <sub>(method)</sub>

> Invokes fireTableDataChanged after all the pending events have been  
> processed. SwingUtilities.invokeLater is used to handle this.

```java
    protected void delayedFireTableCellUpdated(final int row, final int column)
```

#### `run`  <sub>(method)</sub>

```java
	    public void run()
```

#### `delayedFireTableRowsUpdated`  <sub>(method)</sub>

> Invokes fireTableDataChanged after all the pending events have been  
> processed. SwingUtilities.invokeLater is used to handle this.

```java
    protected void delayedFireTableRowsUpdated(final int first, final int last)
```

#### `run`  <sub>(method)</sub>

```java
	    public void run()
```

#### `delayedFireTableStructureChanged`  <sub>(method)</sub>

> Invokes fireTableDataChanged after all the pending events have been  
> processed. SwingUtilities.invokeLater is used to handle this.

```java
    protected void delayedFireTableStructureChanged()
```

#### `run`  <sub>(method)</sub>

```java
	    public void run()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/JTableTree/TreeTableModelEvent.java`


#### `TreeTableModelEvent`  <sub>(class)</sub>

```java
public class TreeTableModelEvent extends TreeModelEvent
```

#### `protected int column;`  <sub>(method)</sub>

```java
protected int column;
```

#### `protected int type = 0;`  <sub>(method)</sub>

```java
protected int type = 0;
```

#### `public static final int INSERT =  1;`  <sub>(method)</sub>

```java
public static final int INSERT =  1;
```

#### `public static final int UPDATE =  0;`  <sub>(method)</sub>

```java
public static final int UPDATE =  0;
```

#### `public static final int DELETE = -1;`  <sub>(method)</sub>

```java
public static final int DELETE = -1;
```

#### `TreeTableModelEvent`  <sub>(method)</sub>

> Used to create an event when the node structure has changed in some way,  
> identifying the path to the root of the modified subtree as a TreePath  
> object. For more information on this event specification, see  
> <code>TreeModelEvent(Object,Object[])</code>.  
> the creator of the event object passes <code>this</code>  
> for its value)  
> change. In the DefaultTreeModel,  
> this object contains an array of user-data objects,  
> but a subclass of TreePath could use some totally  
> different mechanism -- for example, a node ID number

```java
    public TreeTableModelEvent(Object source, TreePath path, int column)
```

#### `TreeTableModelEvent`  <sub>(method)</sub>

> Used to create an event when the node structure has changed in some way,  
> identifying the path to the root of the modified subtree as a TreePath  
> object. For more information on this event specification, see  
> <code>TreeModelEvent(Object,Object[])</code>.  
> the creator of the event object passes <code>this</code>  
> for its value)  
> change. In the DefaultTreeModel,  
> this object contains an array of user-data objects,  
> but a subclass of TreePath could use some totally  
> different mechanism -- for example, a node ID number

```java
    public TreeTableModelEvent(Object source, TreePath path, int column, int type)
```

#### `getColumn`  <sub>(method)</sub>

```java
    public int getColumn()
```

#### `getType`  <sub>(method)</sub>

```java
    public int getType()
```


## `src/com/sittinglittleduck/DirBuster/gui/documentListeners` <a id='pkg-src-com-sittinglittleduck-dirbuster-gui-documentlisteners'></a>

*20 documented items*


### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/documentListeners/DirStartDocumentListener.java`


#### `DirStartDocumentListener`  <sub>(class)</sub>

```java
public class DirStartDocumentListener implements DocumentListener
```

#### `DirStartDocumentListener`  <sub>(method)</sub>

```java
    public DirStartDocumentListener(JPanelSetup setup)
```

#### `insertUpdate`  <sub>(method)</sub>

```java
    public void insertUpdate(DocumentEvent ev)
```

#### `removeUpdate`  <sub>(method)</sub>

```java
    public void removeUpdate(DocumentEvent ev)
```

#### `changedUpdate`  <sub>(method)</sub>

```java
    public void changedUpdate(DocumentEvent ev)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/documentListeners/FileExtDocumentListener.java`


#### `FileExtDocumentListener`  <sub>(class)</sub>

```java
public class FileExtDocumentListener implements DocumentListener
```

#### `FileExtDocumentListener`  <sub>(method)</sub>

```java
    public FileExtDocumentListener(JPanelSetup setup)
```

#### `insertUpdate`  <sub>(method)</sub>

```java
    public void insertUpdate(DocumentEvent ev)
```

#### `removeUpdate`  <sub>(method)</sub>

```java
    public void removeUpdate(DocumentEvent ev)
```

#### `changedUpdate`  <sub>(method)</sub>

```java
    public void changedUpdate(DocumentEvent ev)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/documentListeners/TargetDocumentListener.java`


#### `TargetDocumentListener`  <sub>(class)</sub>

```java
public class TargetDocumentListener implements DocumentListener
```

#### `TargetDocumentListener`  <sub>(method)</sub>

```java
    public TargetDocumentListener(JPanelSetup setup)
```

#### `insertUpdate`  <sub>(method)</sub>

```java
    public void insertUpdate(DocumentEvent ev)
```

#### `removeUpdate`  <sub>(method)</sub>

```java
    public void removeUpdate(DocumentEvent ev)
```

#### `changedUpdate`  <sub>(method)</sub>

```java
    public void changedUpdate(DocumentEvent ev)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/documentListeners/UrlFuzzDocumentListener.java`


#### `UrlFuzzDocumentListener`  <sub>(class)</sub>

```java
public class UrlFuzzDocumentListener implements DocumentListener
```

#### `UrlFuzzDocumentListener`  <sub>(method)</sub>

```java
    public UrlFuzzDocumentListener(JPanelSetup setup)
```

#### `insertUpdate`  <sub>(method)</sub>

```java
    public void insertUpdate(DocumentEvent ev)
```

#### `removeUpdate`  <sub>(method)</sub>

```java
    public void removeUpdate(DocumentEvent ev)
```

#### `changedUpdate`  <sub>(method)</sub>

```java
    public void changedUpdate(DocumentEvent ev)
```


## `src/com/sittinglittleduck/DirBuster/gui/tableModels` <a id='pkg-src-com-sittinglittleduck-dirbuster-gui-tablemodels'></a>

*105 documented items*


### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/tableModels/ButtonRenderer.java`


#### `ButtonRenderer`  <sub>(class)</sub>

```java
public class ButtonRenderer extends JButton implements TableCellRenderer
```

#### `ButtonRenderer`  <sub>(method)</sub>

```java
    public ButtonRenderer()
```

#### `getTableCellRendererComponent`  <sub>(method)</sub>

```java
    public Component getTableCellRendererComponent(JTable table, Object value, boolean isSelected, boolean hasFocus, int row, int column)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/tableModels/ErrorTableModel.java`


#### `ErrorTableModel`  <sub>(class)</sub>

```java
public class ErrorTableModel extends AbstractTableModel
```

#### `private String[] columnNames =`  <sub>(method)</sub>

```java
private String[] columnNames =
```

#### `String`  <sub>(method)</sub>

```java
        new String("Request"), new String("Error Message")
```

#### `private Vector<ErrorTableObject> data;`  <sub>(method)</sub>

```java
private Vector<ErrorTableObject> data;
```

#### `private JTable table;`  <sub>(method)</sub>

```java
private JTable table;
```

#### `ErrorTableModel`  <sub>(method)</sub>

```java
    public ErrorTableModel(JTable table)
```

#### `setColumnName`  <sub>(method)</sub>

> Function to set the table column names

```java
    public void setColumnName(int index, String name)
```

#### `getColumnName`  <sub>(method)</sub>

```java
    public String getColumnName(int col)
```

#### `isCellEditable`  <sub>(method)</sub>

```java
    public boolean isCellEditable(int row, int col)
```

#### `getValueAt`  <sub>(method)</sub>

```java
    public Object getValueAt(int row, int col)
```

#### `addRow`  <sub>(method)</sub>

```java
    public void addRow(ErrorTableObject object)
```

#### `getRowCount`  <sub>(method)</sub>

```java
    public int getRowCount()
```

#### `getColumnCount`  <sub>(method)</sub>

```java
    public int getColumnCount()
```

#### `getColumnClass`  <sub>(method)</sub>

```java
    public Class getColumnClass(int c)
```

#### `clearAllResults`  <sub>(method)</sub>

```java
    public void clearAllResults()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/tableModels/ErrorTableObject.java`


#### `ErrorTableObject`  <sub>(class)</sub>

```java
public class ErrorTableObject
```

#### `private URL url;`  <sub>(method)</sub>

```java
private URL url;
```

#### `private String reason;`  <sub>(method)</sub>

```java
private String reason;
```

#### `ErrorTableObject`  <sub>(method)</sub>

```java
    public ErrorTableObject(URL url, String reason)
```

#### `getUrl`  <sub>(method)</sub>

```java
    public URL getUrl()
```

#### `getReason`  <sub>(method)</sub>

```java
    public String getReason()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/tableModels/HTMLParseTableModel.java`


#### `HTMLParseTableModel`  <sub>(class)</sub>

```java
public class HTMLParseTableModel extends AbstractTableModel
```

#### `private Vector tableData;`  <sub>(method)</sub>

```java
private Vector tableData;
```

#### `HTMLParseTableModel`  <sub>(method)</sub>

> Creates a new instance of HTTPHeaderTableModel

```java
    public HTMLParseTableModel(Vector d)
```

#### `setColumnName`  <sub>(method)</sub>

```java
    public void setColumnName(int index, String name)
```

#### `isCellEditable`  <sub>(method)</sub>

```java
    public boolean isCellEditable(int row, int col)
```

#### `getRowCount`  <sub>(method)</sub>

```java
    public int getRowCount()
```

#### `getColumnCount`  <sub>(method)</sub>

```java
    public int getColumnCount()
```

#### `getColumnName`  <sub>(method)</sub>

```java
    public String getColumnName(int col)
```

#### `getValueAt`  <sub>(method)</sub>

```java
    public Object getValueAt(int row, int col)
```

#### `setValueAt`  <sub>(method)</sub>

```java
    public void setValueAt(Object value, int row, int col)
```

#### `clearData`  <sub>(method)</sub>

```java
    public void clearData()
```

#### `getVector`  <sub>(method)</sub>

```java
    public Vector getVector()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/tableModels/HTTPHeaderTableModel.java`


#### `HTTPHeaderTableModel`  <sub>(class)</sub>

```java
public class HTTPHeaderTableModel extends AbstractTableModel
```

#### `private Vector tableData;`  <sub>(method)</sub>

```java
private Vector tableData;
```

#### `HTTPHeaderTableModel`  <sub>(method)</sub>

> Creates a new instance of HTTPHeaderTableModel

```java
    public HTTPHeaderTableModel(Vector d)
```

#### `setColumnName`  <sub>(method)</sub>

```java
    public void setColumnName(int index, String name)
```

#### `isCellEditable`  <sub>(method)</sub>

```java
    public boolean isCellEditable(int row, int col)
```

#### `getRowCount`  <sub>(method)</sub>

```java
    public int getRowCount()
```

#### `getColumnCount`  <sub>(method)</sub>

```java
    public int getColumnCount()
```

#### `getColumnName`  <sub>(method)</sub>

```java
    public String getColumnName(int col)
```

#### `getValueAt`  <sub>(method)</sub>

```java
    public Object getValueAt(int row, int col)
```

#### `setValueAt`  <sub>(method)</sub>

```java
    public void setValueAt(Object value, int row, int col)
```

#### `clearData`  <sub>(method)</sub>

```java
    public void clearData()
```

#### `getVector`  <sub>(method)</sub>

```java
    public Vector getVector()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/tableModels/JButtonCellEditor.java`


#### `JButtonCellEditor`  <sub>(class)</sub>

```java
public class JButtonCellEditor extends AbstractCellEditor implements TableCellEditor, ActionListener
```

#### `JButton`  <sub>(method)</sub>

```java
private JButton b = new JButton("Stop");
```

#### `private Manager manager;`  <sub>(method)</sub>

```java
private Manager manager;
```

#### `private int action;`  <sub>(method)</sub>

```java
private int action;
```

#### `JButtonCellEditor`  <sub>(method)</sub>

```java
    public JButtonCellEditor()
```

#### `getTableCellEditorComponent`  <sub>(method)</sub>

```java
    public Component getTableCellEditorComponent(JTable table, Object value,
                       boolean isSelected, int row, int column)
```

#### `actionPerformed`  <sub>(method)</sub>

```java
    public void actionPerformed(ActionEvent e)
```

#### `getCellEditorValue`  <sub>(method)</sub>

```java
    public Object getCellEditorValue()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/tableModels/ProgressRenderer.java`


#### `ProgressRenderer`  <sub>(class)</sub>

```java
public class ProgressRenderer extends DefaultTableCellRenderer
```

#### `JProgressBar`  <sub>(method)</sub>

```java
private JProgressBar b = new JProgressBar();
```

#### `private Manager manager;`  <sub>(method)</sub>

```java
private Manager manager;
```

#### `ProgressRenderer`  <sub>(method)</sub>

```java
    public ProgressRenderer()
```

#### `getTableCellRendererComponent`  <sub>(method)</sub>

```java
    public Component getTableCellRendererComponent(JTable table, Object value,
                boolean isSelected, boolean hasFocus,
                int row, int column)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/tableModels/ResultsTableModel.java`


#### `ResultsTableModel`  <sub>(class)</sub>

```java
public class ResultsTableModel extends DefaultTableModel
```

#### `private int numberOfFiles = 0;`  <sub>(method)</sub>

```java
private int numberOfFiles = 0;
```

#### `private int numberOfDirs = 0;`  <sub>(method)</sub>

```java
private int numberOfDirs = 0;
```

#### `ResultsTableModel`  <sub>(method)</sub>

> Creates a new instance of ResultsTableModel

```java
    public ResultsTableModel()
```

#### `setColumnName`  <sub>(method)</sub>

```java
    public void setColumnName(int index, String name)
```

#### `isCellEditable`  <sub>(method)</sub>

```java
    public boolean isCellEditable(int row, int col)
```

#### `getColumnClass`  <sub>(method)</sub>

```java
    public Class getColumnClass(int c)
```

#### `addRow`  <sub>(method)</sub>

```java
    public void addRow()
```

#### `updateRow`  <sub>(method)</sub>

```java
    public void updateRow(String dirFinished, String dirStarted)
```

#### `removeRow`  <sub>(method)</sub>

```java
    public void removeRow(int index)
```

#### `getRowCount`  <sub>(method)</sub>

```java
    public int getRowCount()
```

#### `getColumnCount`  <sub>(method)</sub>

```java
    public int getColumnCount()
```

#### `getColumnName`  <sub>(method)</sub>

```java
    public String getColumnName(int col)
```

#### `getValueAt`  <sub>(method)</sub>

```java
    public Object getValueAt(int row, int col)
```

#### `getRowData`  <sub>(method)</sub>

```java
    public String getRowData(int row)
```

#### `getRowResponceCode`  <sub>(method)</sub>

```java
    public String getRowResponceCode(int row)
```

#### `getRowResponse`  <sub>(method)</sub>

```java
    public String getRowResponse(int row)
```

#### `getDirToCheck`  <sub>(method)</sub>

```java
    public DirToCheck getDirToCheck(int row)
```

#### `getRowRawResponse`  <sub>(method)</sub>

```java
    public String getRowRawResponse(int row)
```

#### `getBaseCase`  <sub>(method)</sub>

```java
    public String getBaseCase(int row)
```

#### `getBaseCaseObj`  <sub>(method)</sub>

```java
    public BaseCase getBaseCaseObj(int row)
```

#### `getSelectedURL`  <sub>(method)</sub>

```java
    public String getSelectedURL(int row)
```

#### `getSelectedStatus`  <sub>(method)</sub>

```java
    public String getSelectedStatus(int row)
```

#### `setValueAt`  <sub>(method)</sub>

> public String getRowData(int row)  
> {  
> if ( row < 0 || row >= data.size() ) return null;  
> ResultsTableObject temp = (ResultsTableObject) data.get(row);  
> return temp.getFieldType() + temp.getFieldFound();  
> }  
> public String getRowResponceCode(int row)  
> {  
> if ( row < 0 || row >= data.size() ) return null;  
> ResultsTableObject temp = (ResultsTableObject) data.get(row);  
> return temp.getFieldFound();  
> }  
> public String getRowResponse(int row)  
> {  
> if ( row < 0 || row >= data.size() ) return null;  
> ResultsTableObject temp = (ResultsTableObject) data.get(row);  
> //System.out.println("Getting Responce for row: " + row);  
> //System.out.println("Responce = " + temp.getResponce());  
> return temp.getResponce();  
> }  
> public DirToCheck getDirToCheck(int row)  
> {  
> if ( row < 0 || row >= data.size() ) return null;  
> ResultsTableObject temp = (ResultsTableObject) data.get(row);  
> //System.out.println("Getting Responce for row: " + row);  
> //System.out.println("Responce = " + temp.getResponce());  
> return temp.getDirToCheck();  
> }  
> public String getRowRawResponse(int row)  
> {  
> if ( row < 0 || row >= data.size() ) return null;  
> ResultsTableObject temp = (ResultsTableObject) data.get(row);  
> //System.out.println("Getting Responce for row: " + row);  
> //System.out.println("Responce = " + temp.getResponce());  
> return temp.getRawResponce();  
> }  
> public String getBaseCase(int row)  
> {  
> if ( row < 0 || row >= data.size() ) return null;  
> ResultsTableObject temp = (ResultsTableObject) data.get(row);  
> //System.out.println("Getting Responce for row: " + row);  
> //System.out.println("Responce = " + temp.getResponce());  
> return temp.getBaseCase();  
> }  
> public BaseCase getBaseCaseObj(int row)  
> {  
> if ( row < 0 || row >= data.size() ) return null;  
> ResultsTableObject temp = (ResultsTableObject) data.get(row);  
> return temp.getBaseCaseObj();  
> }  
> public String getSelectedURL(int row)  
> {  
> if ( row < 0 || row >= data.size() ) return null;  
> ResultsTableObject temp = (ResultsTableObject) data.get(row);  
> return temp.getFullURL();  
> }  
> public String getSelectedStatus(int row)  
> {  
> if ( row < 0 || row >= data.size() ) return null;  
> ResultsTableObject temp = (ResultsTableObject) data.get(row);  
> return temp.getFieldStatus();  
> }

```java
    public void setValueAt(Object value, int row, int col)
```

#### `getList`  <sub>(method)</sub>

```java
    public List getList()
```

#### `clearData`  <sub>(method)</sub>

```java
    public void clearData()
```

#### `setManager`  <sub>(method)</sub>

```java
    public void setManager(Manager manager)
```

#### `getNumberOfDirs`  <sub>(method)</sub>

```java
    public int getNumberOfDirs()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/tableModels/ScanInfoTableModel.java`


#### `ScanInfoTableModel`  <sub>(class)</sub>

```java
public class ScanInfoTableModel extends AbstractTableModel
```

#### `private String[] columnNames =`  <sub>(method)</sub>

```java
private String[] columnNames =
```

#### `String`  <sub>(method)</sub>

```java
        new String("Task"), new String("Progress"), new String("Control")
```

#### `private Vector<ErrorTableObject> data;`  <sub>(method)</sub>

```java
private Vector<ErrorTableObject> data;
```

#### `private JTable table;`  <sub>(method)</sub>

```java
private JTable table;
```

#### `ScanInfoTableModel`  <sub>(method)</sub>

```java
    public ScanInfoTableModel(JTable table)
```

#### `setColumnName`  <sub>(method)</sub>

> Function to set the table column names

```java
    public void setColumnName(int index, String name)
```

#### `getColumnName`  <sub>(method)</sub>

```java
    public String getColumnName(int col)
```

#### `isCellEditable`  <sub>(method)</sub>

```java
    public boolean isCellEditable(int row, int col)
```

#### `getValueAt`  <sub>(method)</sub>

```java
    public Object getValueAt(int row, int col)
```

#### `addRow`  <sub>(method)</sub>

```java
    public void addRow(ErrorTableObject object)
```

#### `getRowCount`  <sub>(method)</sub>

> public void addRow(ErrorTableObject object)  
> {  
> //check the item is not already in the table  
> if( ! data.contains(object))  
> {  
> data.addElement(object);  
> if(table.getRowSorter() != null)  
> {  
> int location = table.getRowSorter().convertRowIndexToView(data.size() - 1);  
> if(location > 0)  
> {  
> this.fireTableRowsInserted(location - 1, location - 1);  
> }  
> else  
> {  
> this.fireTableDataChanged();  
> }  
> }  
> else  
> {  
> this.fireTableDataChanged();  
> }  
> }  
> else  
> {  
> return;  
> }  
> }

```java
    public int getRowCount()
```

#### `getColumnCount`  <sub>(method)</sub>

```java
    public int getColumnCount()
```

#### `getColumnClass`  <sub>(method)</sub>

```java
    public Class getColumnClass(int c)
```

#### `clearAllResults`  <sub>(method)</sub>

```java
    public void clearAllResults()
```

#### `refreshTable`  <sub>(method)</sub>

```java
    public void refreshTable()
```


## `src/com/sittinglittleduck/DirBuster/gui/tree` <a id='pkg-src-com-sittinglittleduck-dirbuster-gui-tree'></a>

*33 documented items*


### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/tree/ResultsNode.java`


#### `ResultsNode`  <sub>(class)</sub>

```java
public class ResultsNode extends DefaultMutableTreeNode
```

#### `ResultsNode`  <sub>(method)</sub>

```java
    public ResultsNode(Result result)
```

#### `ResultsNode`  <sub>(method)</sub>

```java
    public ResultsNode(String name)
```

#### `addChild`  <sub>(method)</sub>

```java
    public void addChild(ResultsNode object)
```

#### `getChild`  <sub>(method)</sub>

```java
    public ResultsNode getChild(int index)
```

#### `getChildren`  <sub>(method)</sub>

```java
    public Object[] getChildren()
```

#### `isLeaf`  <sub>(method)</sub>

```java
    public boolean isLeaf()
```

#### `toString`  <sub>(method)</sub>

```java
    public String toString()
```

#### `covertData`  <sub>(method)</sub>

```java
    private String covertData(URL url)
```

#### `getChildrenVector`  <sub>(method)</sub>

```java
    public Vector<ResultsNode> getChildrenVector()
```

#### `clearData`  <sub>(method)</sub>

```java
    public void clearData()
```

#### `getResult`  <sub>(method)</sub>

```java
    public Result getResult()
```

#### `getNumberOfChildren`  <sub>(method)</sub>

```java
    public int getNumberOfChildren()
```

#### `isResultSet`  <sub>(method)</sub>

```java
    public boolean isResultSet()
```

#### `setResult`  <sub>(method)</sub>

```java
    public void setResult(Result result)
```

#### `findChildBasedOnString`  <sub>(method)</sub>

```java
    public ResultsNode findChildBasedOnString(String name)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/gui/tree/ResultsTableTreeModel.java`


#### `ResultsTableTreeModel`  <sub>(class)</sub>

```java
public class ResultsTableTreeModel extends AbstractTreeTableModel
```

#### `static final protected String[] cNames =`  <sub>(method)</sub>

```java
static final protected String[] cNames =
```

#### `String`  <sub>(method)</sub>

```java
        new String("Directory Stucture"), new String("Response Code"), new String("Response Size")
```

#### `static final protected Class[] cTypes =`  <sub>(method)</sub>

```java
static final protected Class[] cTypes =
```

#### `ResultsNode`  <sub>(method)</sub>

```java
private ResultsNode rootNode = new ResultsNode("/");
```

#### `ResultsTableTreeModel`  <sub>(method)</sub>

```java
    public ResultsTableTreeModel()
```

#### `getRoot`  <sub>(method)</sub>

```java
    public Object getRoot()
```

#### `cleartable`  <sub>(method)</sub>

```java
    public void cleartable()
```

#### `getChild`  <sub>(method)</sub>

```java
    public Object getChild(Object parent, int index)
```

#### `getChildCount`  <sub>(method)</sub>

```java
    public int getChildCount(Object parent)
```

#### `isLeaf`  <sub>(method)</sub>

```java
    public boolean isLeaf(Object node)
```

#### `getIndexOfChild`  <sub>(method)</sub>

```java
    public int getIndexOfChild(Object parent, Object child)
```

#### `getColumnCount`  <sub>(method)</sub>

```java
    public int getColumnCount()
```

#### `getColumnName`  <sub>(method)</sub>

```java
    public String getColumnName(int column)
```

#### `getValueAt`  <sub>(method)</sub>

```java
    public Object getValueAt(Object node, int col)
```

#### `getColumnClass`  <sub>(method)</sub>

```java
    public Class getColumnClass(int column)
```

#### `addRow`  <sub>(method)</sub>

```java
    public void addRow(Result result)
```


## `src/com/sittinglittleduck/DirBuster/headless` <a id='pkg-src-com-sittinglittleduck-dirbuster-headless'></a>

*2 documented items*


### `dirbuster/src/com/sittinglittleduck/DirBuster/headless/CatchExit.java`


#### `CatchExit`  <sub>(class)</sub>

```java
public class CatchExit implements Runnable
```

#### `run`  <sub>(method)</sub>

```java
    public void run()
```


## `src/com/sittinglittleduck/DirBuster/monitorThreads` <a id='pkg-src-com-sittinglittleduck-dirbuster-monitorthreads'></a>

*15 documented items*


### `dirbuster/src/com/sittinglittleduck/DirBuster/monitorThreads/ProcessChecker.java`


#### `ProcessChecker`  <sub>(class)</sub>

```java
public class ProcessChecker extends TimerTask
```

#### `private long timeStarted;`  <sub>(method)</sub>

```java
private long timeStarted;
```

#### `private long lastTotal = 0L;`  <sub>(method)</sub>

```java
private long lastTotal = 0L;
```

#### `Vector`  <sub>(method)</sub>

```java
private Vector lastTen = new Vector(10, 1);
```

#### `ProcessUpdate`  <sub>(interface)</sub>

> Creates a new instance of ProcessChecker

```java
public interface ProcessUpdate
```

#### `ProcessChecker`  <sub>(method)</sub>

```java
    public ProcessChecker()
```

#### `run`  <sub>(method)</sub>

```java
    public void run()
```

#### `convertSecsToTime`  <sub>(method)</sub>

```java
    private String convertSecsToTime(long secs)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/monitorThreads/ProcessEnd.java`


#### `ProcessEnd`  <sub>(class)</sub>

```java
public class ProcessEnd extends TimerTask
```

#### `ProcessUpdate`  <sub>(interface)</sub>

> Creates a new instance of ProcessChecker

```java
public interface ProcessUpdate
```

#### `ProcessEnd`  <sub>(method)</sub>

```java
    public ProcessEnd()
```

#### `run`  <sub>(method)</sub>

```java
    public void run()
```

#### `areAllWorkerFinished`  <sub>(method)</sub>

> tests if all the wrokers have finished

```java
    private boolean areAllWorkerFinished()
```

#### `areAllParsesFinished`  <sub>(method)</sub>

> tests if all the parsers are finished

```java
    private boolean areAllParsesFinished()
```

#### `areAllGenThreadsFinished`  <sub>(method)</sub>

> tests if all genthreads

```java
    private boolean areAllGenThreadsFinished()
```


## `src/com/sittinglittleduck/DirBuster/report` <a id='pkg-src-com-sittinglittleduck-dirbuster-report'></a>

*16 documented items*


### `dirbuster/src/com/sittinglittleduck/DirBuster/report/ReportWriter.java`


#### `ReportWriter`  <sub>(class)</sub>

```java
public class ReportWriter
```

#### `private String fileToWriteTo;`  <sub>(method)</sub>

```java
private String fileToWriteTo;
```

#### `private StartGUI gui;`  <sub>(method)</sub>

```java
private StartGUI gui;
```

#### `private Manager manager;`  <sub>(method)</sub>

```java
private Manager manager;
```

#### `private Vector<Result> data;`  <sub>(method)</sub>

```java
private Vector<Result> data;
```

#### `ReportWriter`  <sub>(method)</sub>

> Creates a new instance of ReportWriter

```java
    public ReportWriter(String fileToWriteTo, StartGUI gui)
```

#### `ReportWriter`  <sub>(method)</sub>

```java
    public ReportWriter(String fileToWriteTo)
```

#### `writeReport`  <sub>(method)</sub>

```java
    public void writeReport()
```

#### `writeReportHeader`  <sub>(method)</sub>

```java
    private void writeReportHeader(BufferedWriter out) throws IOException
```

#### `writeSimpleListDirsOnly`  <sub>(method)</sub>

```java
    public void writeSimpleListDirsOnly()
```

#### `writeSimpleListFilesOnly`  <sub>(method)</sub>

```java
    public void writeSimpleListFilesOnly()
```

#### `writeSimpleListFilesAndDirs`  <sub>(method)</sub>

```java
    public void writeSimpleListFilesAndDirs()
```

#### `writeXML`  <sub>(method)</sub>

```java
    public void writeXML()
```

#### `writeCSV`  <sub>(method)</sub>

```java
    public void writeCSV()
```

#### `writeSimpleListDirs`  <sub>(method)</sub>

```java
    private void writeSimpleListDirs(BufferedWriter out) throws IOException
```

#### `writeSimpleListFiles`  <sub>(method)</sub>

```java
    private void writeSimpleListFiles(BufferedWriter out) throws IOException
```


## `src/com/sittinglittleduck/DirBuster/utils` <a id='pkg-src-com-sittinglittleduck-dirbuster-utils'></a>

*5 documented items*


### `dirbuster/src/com/sittinglittleduck/DirBuster/utils/HeadRequestCheck.java`


#### `HeadRequestCheck`  <sub>(class)</sub>

```java
public class HeadRequestCheck
```

#### `test`  <sub>(method)</sub>

```java
    public static void test(String url)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/utils/Utils.java`


#### `Utils`  <sub>(class)</sub>

```java
public class Utils
```

#### `getNumberOfLineInAFile`  <sub>(method)</sub>

```java
    public static int getNumberOfLineInAFile(String file)
```

#### `makeItemsafe`  <sub>(method)</sub>

```java
    public static String makeItemsafe(String item)
```


## `src/com/sittinglittleduck/DirBuster/workGenerators` <a id='pkg-src-com-sittinglittleduck-dirbuster-workgenerators'></a>

*121 documented items*


### `dirbuster/src/com/sittinglittleduck/DirBuster/workGenerators/BruteForceURLFuzz.java`


#### `BruteForceURLFuzz`  <sub>(class)</sub>

```java
public class BruteForceURLFuzz implements Runnable
```

#### `private String[] list;`  <sub>(method)</sub>

```java
private String[] list;
```

#### `private int minLen;`  <sub>(method)</sub>

```java
private int minLen;
```

#### `private int maxLen;`  <sub>(method)</sub>

```java
private int maxLen;
```

#### `private Manager manager;`  <sub>(method)</sub>

```java
private Manager manager;
```

#### `private BlockingQueue<WorkUnit> workQueue;`  <sub>(method)</sub>

```java
private BlockingQueue<WorkUnit> workQueue;
```

#### `private String firstPart;`  <sub>(method)</sub>

```java
private String firstPart;
```

#### `private String finished;`  <sub>(method)</sub>

```java
private String finished;
```

#### `private String started;`  <sub>(method)</sub>

```java
private String started;
```

#### `private String urlFuzzStart;`  <sub>(method)</sub>

```java
private String urlFuzzStart;
```

#### `private String urlFuzzEnd;`  <sub>(method)</sub>

```java
private String urlFuzzEnd;
```

#### `private int counter = 0;`  <sub>(method)</sub>

```java
private int counter = 0;
```

#### `private boolean isWorking = true;`  <sub>(method)</sub>

```java
private boolean isWorking = true;
```

#### `BruteForceURLFuzz`  <sub>(method)</sub>

> Creates a new instance of BruteForceWorkGenerator

```java
    public BruteForceURLFuzz()
```

#### `run`  <sub>(method)</sub>

```java
    public void run()
```

#### `getCurrentDone`  <sub>(method)</sub>

```java
    public int getCurrentDone()
```

#### `isWorking`  <sub>(method)</sub>

```java
    public boolean isWorking()
```

#### `getStartPoint`  <sub>(method)</sub>

```java
    public String getStartPoint()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/workGenerators/BruteForceWorkGenerator.java`


#### `BruteForceWorkGenerator`  <sub>(class)</sub>

```java
public class BruteForceWorkGenerator implements Runnable
```

#### `private String[] list = {"a", "b", "c", "d"};`  <sub>(method)</sub>

```java
private String[] list = {"a", "b", "c", "d"};
```

#### `private int[] listindex;`  <sub>(method)</sub>

```java
private int[] listindex;
```

#### `private int minLen;`  <sub>(method)</sub>

```java
private int minLen;
```

#### `private int maxLen;`  <sub>(method)</sub>

```java
private int maxLen;
```

#### `private Manager manager;`  <sub>(method)</sub>

```java
private Manager manager;
```

#### `private BlockingQueue<WorkUnit> workQueue;`  <sub>(method)</sub>

```java
private BlockingQueue<WorkUnit> workQueue;
```

#### `private BlockingQueue<DirToCheck> dirQueue;`  <sub>(method)</sub>

```java
private BlockingQueue<DirToCheck> dirQueue;
```

#### `private String firstPart;`  <sub>(method)</sub>

```java
private String firstPart;
```

#### `private String fileExtention;`  <sub>(method)</sub>

```java
private String fileExtention;
```

#### `private String finished;`  <sub>(method)</sub>

```java
private String finished;
```

#### `private String started;`  <sub>(method)</sub>

```java
private String started;
```

#### `private String currentDir = "/";`  <sub>(method)</sub>

```java
private String currentDir = "/";
```

#### `private int failcode = 404;`  <sub>(method)</sub>

```java
private int failcode = 404;
```

#### `private boolean doingDirs = true;`  <sub>(method)</sub>

```java
private boolean doingDirs = true;
```

#### `BruteForceWorkGenerator`  <sub>(method)</sub>

> Creates a new instance of BruteForceWorkGenerator

```java
    public BruteForceWorkGenerator()
```

#### `run`  <sub>(method)</sub>

```java
    public void run()
```

#### `makeList`  <sub>(method)</sub>

```java
    private void makeList(int minLen, int maxLen, String baseCase, BaseCase baseCaseObj)
```

#### `showString`  <sub>(method)</sub>

```java
    private void showString(int len, String baseCase, BaseCase baseCaseObj)
```

#### `incrementCounter`  <sub>(method)</sub>

```java
    private void incrementCounter(int len)
```

#### `initIndex`  <sub>(method)</sub>

```java
    private void initIndex()
```

#### `calcTotalPerPass`  <sub>(method)</sub>

```java
    private void calcTotalPerPass(int listLength, int minLen, int maxLen)
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/workGenerators/BruteForceWorkerGeneratorMultiThreaded.java`


#### `BruteForceWorkerGeneratorMultiThreaded`  <sub>(class)</sub>

> Produces the work to be done, when we are reading from a list

```java
public class BruteForceWorkerGeneratorMultiThreaded extends MultiThreadedGenerator
```

#### `private int minLen;`  <sub>(method)</sub>

```java
private int minLen;
```

#### `private int maxLen;`  <sub>(method)</sub>

```java
private int maxLen;
```

#### `private String[] list;`  <sub>(method)</sub>

```java
private String[] list;
```

#### `public static final int doDIR = 0;`  <sub>(method)</sub>

```java
public static final int doDIR = 0;
```

#### `public static final int doFile = 1;`  <sub>(method)</sub>

```java
public static final int doFile = 1;
```

#### `getInstance`  <sub>(method)</sub>

```java
private Manager manager = Manager.getInstance();
```

#### `private BlockingQueue<WorkUnit> workQueue;`  <sub>(method)</sub>

```java
private BlockingQueue<WorkUnit> workQueue;
```

#### `private String firstPart;`  <sub>(method)</sub>

```java
private String firstPart;
```

#### `private boolean stopMe = false;`  <sub>(method)</sub>

```java
private boolean stopMe = false;
```

#### `private boolean isWorking = true;`  <sub>(method)</sub>

```java
private boolean isWorking = true;
```

#### `private boolean first = false;`  <sub>(method)</sub>

```java
private boolean first = false;
```

#### `private int counter = 0;`  <sub>(method)</sub>

```java
private int counter = 0;
```

#### `private boolean pleasewait = false;`  <sub>(method)</sub>

```java
private boolean pleasewait = false;
```

#### `BruteForceWorkerGeneratorMultiThreaded`  <sub>(method)</sub>

> Creates a new instance of WorkerGenerator

```java
    public BruteForceWorkerGeneratorMultiThreaded(String startpoint, String fileExt, int type, boolean first)
```

#### `run`  <sub>(method)</sub>

> Thread run method

```java
    public void run()
```

#### `stopMe`  <sub>(method)</sub>

> Method to stop the manager while it is working

```java
    public void stopMe()
```

#### `isWorking`  <sub>(method)</sub>

```java
    public boolean isWorking()
```

#### `getCurrentPoint`  <sub>(method)</sub>

```java
    public int getCurrentPoint()
```

#### `getFileExt`  <sub>(method)</sub>

```java
    public String getFileExt()
```

#### `getStartpoint`  <sub>(method)</sub>

```java
    public String getStartpoint()
```

#### `getType`  <sub>(method)</sub>

```java
    public int getType()
```

#### `pause`  <sub>(method)</sub>

```java
    public void pause()
```

#### `unPause`  <sub>(method)</sub>

```java
    public void unPause()
```

#### `isPaused`  <sub>(method)</sub>

```java
    public boolean isPaused()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/workGenerators/MultiThreadedGenerator.java`


#### `MultiThreadedGenerator`  <sub>(class)</sub>

```java
public abstract class MultiThreadedGenerator implements Runnable
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/workGenerators/WorkerGenerator.java`


#### `WorkerGenerator`  <sub>(class)</sub>

> Produces the work to be done, when we are reading from a list

```java
public class WorkerGenerator implements Runnable
```

#### `private Manager manager;`  <sub>(method)</sub>

```java
private Manager manager;
```

#### `private BlockingQueue<WorkUnit> workQueue;`  <sub>(method)</sub>

```java
private BlockingQueue<WorkUnit> workQueue;
```

#### `private BlockingQueue<DirToCheck> dirQueue;`  <sub>(method)</sub>

```java
private BlockingQueue<DirToCheck> dirQueue;
```

#### `private String inputFile;`  <sub>(method)</sub>

```java
private String inputFile;
```

#### `private String firstPart;`  <sub>(method)</sub>

```java
private String firstPart;
```

#### `private String fileExtention;`  <sub>(method)</sub>

```java
private String fileExtention;
```

#### `private String finished;`  <sub>(method)</sub>

```java
private String finished;
```

#### `private String started;`  <sub>(method)</sub>

```java
private String started;
```

#### `private boolean stopMe = false;`  <sub>(method)</sub>

```java
private boolean stopMe = false;
```

#### `private boolean skipCurrent = false;`  <sub>(method)</sub>

```java
private boolean skipCurrent = false;
```

#### `WorkerGenerator`  <sub>(method)</sub>

> Creates a new instance of WorkerGenerator

```java
    public WorkerGenerator()
```

#### `run`  <sub>(method)</sub>

> Thread run method

```java
    public void run()
```

#### `makeItemsafe`  <sub>(method)</sub>

```java
    private String makeItemsafe(String item)
```

#### `stopMe`  <sub>(method)</sub>

> Method to stop the manager while it is working

```java
    public void stopMe()
```

#### `skipCurrent`  <sub>(method)</sub>

```java
    public void skipCurrent()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/workGenerators/WorkerGeneratorMultiThreaded.java`


#### `WorkerGeneratorMultiThreaded`  <sub>(class)</sub>

> Produces the work to be done, when we are reading from a list

```java
public class WorkerGeneratorMultiThreaded extends MultiThreadedGenerator
```

#### `public static final int doDIR = 0;`  <sub>(method)</sub>

```java
public static final int doDIR = 0;
```

#### `public static final int doFile = 1;`  <sub>(method)</sub>

```java
public static final int doFile = 1;
```

#### `private Manager manager;`  <sub>(method)</sub>

```java
private Manager manager;
```

#### `private BlockingQueue<WorkUnit> workQueue;`  <sub>(method)</sub>

```java
private BlockingQueue<WorkUnit> workQueue;
```

#### `private String inputFile;`  <sub>(method)</sub>

```java
private String inputFile;
```

#### `private String firstPart;`  <sub>(method)</sub>

```java
private String firstPart;
```

#### `private boolean stopMe = false;`  <sub>(method)</sub>

```java
private boolean stopMe = false;
```

#### `private boolean isWorking = true;`  <sub>(method)</sub>

```java
private boolean isWorking = true;
```

#### `private boolean first = false;`  <sub>(method)</sub>

```java
private boolean first = false;
```

#### `private int counter = 0;`  <sub>(method)</sub>

```java
private int counter = 0;
```

#### `private boolean pleasewait = false;`  <sub>(method)</sub>

```java
private boolean pleasewait = false;
```

#### `WorkerGeneratorMultiThreaded`  <sub>(method)</sub>

> Creates a new instance of WorkerGenerator

```java
    public WorkerGeneratorMultiThreaded(String startpoint, String fileExt, int type, boolean first)
```

#### `run`  <sub>(method)</sub>

> Thread run method

```java
    public void run()
```

#### `stopMe`  <sub>(method)</sub>

> Method to stop the manager while it is working

```java
    public void stopMe()
```

#### `isWorking`  <sub>(method)</sub>

```java
    public boolean isWorking()
```

#### `getCurrentPoint`  <sub>(method)</sub>

```java
    public int getCurrentPoint()
```

#### `getFileExt`  <sub>(method)</sub>

```java
    public String getFileExt()
```

#### `getStartpoint`  <sub>(method)</sub>

```java
    public String getStartpoint()
```

#### `getType`  <sub>(method)</sub>

```java
    public int getType()
```

#### `pause`  <sub>(method)</sub>

```java
    public void pause()
```

#### `unPause`  <sub>(method)</sub>

```java
    public void unPause()
```

#### `isPaused`  <sub>(method)</sub>

```java
    public boolean isPaused()
```

### `dirbuster/src/com/sittinglittleduck/DirBuster/workGenerators/WorkerGeneratorURLFuzz.java`


#### `WorkerGeneratorURLFuzz`  <sub>(class)</sub>

> Produces the work to be done, when we are reading from a list

```java
public class WorkerGeneratorURLFuzz implements Runnable
```

#### `private Manager manager;`  <sub>(method)</sub>

```java
private Manager manager;
```

#### `private BlockingQueue<WorkUnit> workQueue;`  <sub>(method)</sub>

```java
private BlockingQueue<WorkUnit> workQueue;
```

#### `private String inputFile;`  <sub>(method)</sub>

```java
private String inputFile;
```

#### `private String firstPart;`  <sub>(method)</sub>

```java
private String firstPart;
```

#### `private boolean stopMe = false;`  <sub>(method)</sub>

```java
private boolean stopMe = false;
```

#### `private boolean isWorking = true;`  <sub>(method)</sub>

```java
private boolean isWorking = true;
```

#### `private String urlFuzzStart;`  <sub>(method)</sub>

```java
private String urlFuzzStart;
```

#### `private String urlFuzzEnd;`  <sub>(method)</sub>

```java
private String urlFuzzEnd;
```

#### `private int counter = 0;`  <sub>(method)</sub>

```java
private int counter = 0;
```

#### `WorkerGeneratorURLFuzz`  <sub>(method)</sub>

> Creates a new instance of WorkerGenerator

```java
    public WorkerGeneratorURLFuzz()
```

#### `run`  <sub>(method)</sub>

> Thread run method

```java
    public void run()
```

#### `stopMe`  <sub>(method)</sub>

> Method to stop the manager while it is working

```java
    public void stopMe()
```

#### `getStartPoint`  <sub>(method)</sub>

```java
    public String getStartPoint()
```

#### `getCurrentCount`  <sub>(method)</sub>

```java
    public int getCurrentCount()
```

#### `isWorking`  <sub>(method)</sub>

```java
    public boolean isWorking()
```
