# dirb — Complete API Reference

Exhaustive per-item reference for **every** type, function, method, constant, and macro in the dirb source (`gitlab.com/kalilinux/packages/dirb` @ `2db1375`, v2.22). Language: **C**. Generated from a full source read; test files are excluded. Each item shows its full signature and doc comment; composite types list their fields/members.

**91 documented items** across 17 source files.


---

## Modules / packages

- [`gendict_src`](#pkg-gendict-src) — 3 items
- [`src`](#pkg-src) — 87 items
- [`web2dic`](#pkg-web2dic) — 1 items

---


## `gendict_src` <a id='pkg-gendict-src'></a>

*3 documented items*


### `dirb/gendict_src/gendict.c`


#### `main`  <sub>(function)</sub>

```c
int main(int argc, char **argv)
```

#### `gen`  <sub>(function)</sub>

```c
void gen(char *old_prefix, char current, char *pattern, char *charset, char wildcard)
```

#### `usage`  <sub>(function)</sub>

```c
void usage (char *progname)
```


## `src` <a id='pkg-src'></a>

*87 documented items*


### `dirb/src/calculanec.c`


#### `get_necs`  <sub>(function)</sub>

> GET_NECS: Obtienes los NECs correspondientes a cada extension

```c
int get_necs(char *direccion)
```

#### `calcula_nec`  <sub>(struct)</sub>

> CALCULA_NEC: Obtiene el codigo de pagina no existente

```c
struct result *calcula_nec(char *direccion)
```

### `dirb/src/crea_wordlist.c`


#### `crea_wordlist`  <sub>(struct)</sub>

> CREA_WORDLIST: Crea la lista de palabras a probar a partir de un fichero

```c
struct words *crea_wordlist(char *ficheros)
```

#### `crea_wordlist_fich`  <sub>(struct)</sub>

> crea_wordlist_fich: Crea una lista de palabras a partir de un fichero

```c
struct words *crea_wordlist_fich(char *fichero)
```

#### `crea_extslist`  <sub>(struct)</sub>

> CREA_EXTSLIST: Crea la lista de extensiones

```c
struct words *crea_extslist(char *lista)
```

#### `count_words`  <sub>(function)</sub>

> COUNT_WORDS: Cuenta las palabras de una wordlist

```c
int count_words(struct words *list)
```

### `dirb/src/dirb.c`


#### `main`  <sub>(function)</sub>

> MAIN: Nucleo del programa

```c
int main(int argc, char **argv)
```

#### `banner`  <sub>(function)</sub>

> BANNER: Muestra el banner de presentacion del programa

```c
void banner(void)
```

#### `ayuda`  <sub>(function)</sub>

> OPCIONES: Muestra el menu de opciones disponibles

```c
void ayuda(void)
```

### `dirb/src/dirb.h`


#### `IMPRIME`  <sub>(macro)</sub>

> MACROs

```c
#define   IMPRIME(f, s...)  printf(f, ## s); if(options.saveoutput) fprintf(outfile, f, ## s);
```

### `dirb/src/estructuras.h`


#### `result`  <sub>(struct)</sub>

> Resultado de una peticion

```c
struct result
```

**Fields:**
```c
char url[STRING_SIZE];
CURLcode estado;
int codigo_http;
int head_size;
int head_lines;
int body_size;
int body_words;
int body_lines;
char location[STRING_SIZE];
char server[STRING_SIZE];
```


#### `code`  <sub>(struct)</sub>

> Codigo HTTP

```c
struct code
```

**Fields:**
```c
int codenum;
char desc[STRING_SIZE];
```


#### `words`  <sub>(struct)</sub>

> Lista de palabras

```c
struct words
```

**Fields:**
```c
void *anterior;
char word[STRING_SIZE];
void *siguiente;
```


#### `opciones`  <sub>(struct)</sub>

> Opciones de linea de comandos

```c
struct opciones
```

**Fields:**
```c
int use_vhost;
int use_proxy;
int use_proxypass;
int use_pass;
int use_cookie;
int verify_ssl;
int use_agent;
int ignore_nec;
int default_nec;
int debuging;
int exitonwarn;
int save_found;
int mutations_file;
int mutations_list;
int extensions_file;
int extensions_list;
int saveoutput;
int print_location;
int finetunning;
int interactive;
int nothide;
int insensitive;
int lasting_bar;
int dont_recurse;
int add_header;
int silent_mode;
int speed;
char mutation_file[STRING_SIZE];
char mutation_list[STRING_SIZE];
char exts_file[STRING_SIZE];
char exts_list[STRING_SIZE];
char proxy[STRING_SIZE];
char pass_string[STRING_SIZE];
char proxypass_string[STRING_SIZE];
char cookie[STRING_SIZE];
char url_inicial[STRING_SIZE];
char mfile[STRING_SIZE];
char agente[STRING_SIZE];
char vhost[STRING_SIZE];
char savefile[STRING_SIZE];
char header_string[STRING_SIZE];
int recursion_level;
char current_word[STRING_SIZE];
char current_dir[STRING_SIZE];
```


### `dirb/src/get_url.c`


#### `get_url`  <sub>(struct)</sub>

> GET_URL: Descarga una URL y devuelve una estrcutra con el resultado

```c
struct result get_url(char *resp_url)
```

#### `get_header`  <sub>(function)</sub>

> GET_HEADER: Funcion de manejo de las cabeceras

```c
size_t get_header(void *ptr, size_t size, size_t nmemb, void *stream)
```

#### `get_body`  <sub>(function)</sub>

> GET_BODY: Funcion de manejo del cuerpo HTML

```c
size_t get_body(void *ptr, size_t size, size_t nmemb, void *stream)
```

### `dirb/src/global.h`


#### `MAX_FAILS`  <sub>(macro)</sub>

> Constantes

```c
#define MAX_FAILS   	3
```

#### `TIMEOUT`  <sub>(macro)</sub>

```c
#define TIMEOUT     	50
```

#### `BIGWORDLIST`  <sub>(macro)</sub>

```c
#define BIGWORDLIST   	100000
```

#### `AGENT_STRING`  <sub>(macro)</sub>

```c
#define AGENT_STRING  	"Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1)"
```

#### `MAX_ALERT`  <sub>(macro)</sub>

```c
#define MAX_ALERT   	100
```

#### `STRING_SIZE`  <sub>(macro)</sub>

```c
#define STRING_SIZE   	256
```

#### `DUMP_TIMEOUT`  <sub>(macro)</sub>

```c
#define DUMP_TIMEOUT  	100
```

#### `DUMP_DIR`  <sub>(macro)</sub>

```c
#define DUMP_DIR    	".cache/dirb/resume"
```

#### `OPTIONS_DUMP`  <sub>(macro)</sub>

```c
#define OPTIONS_DUMP  	".cache/dirb/resume/options.dump"
```

#### `WORDLIST_DUMP`  <sub>(macro)</sub>

```c
#define WORDLIST_DUMP 	".cache/dirb/resume/wordlist.dump"
```

#### `DIRLIST_DUMP`  <sub>(macro)</sub>

```c
#define DIRLIST_DUMP  	".cache/dirb/resume/dirlist.dump"
```

### `dirb/src/http_codes.h`


#### `HTTP_CONTINUE`  <sub>(macro)</sub>

> HTTP Code

```c
#define HTTP_CONTINUE                      100
```

#### `HTTP_SWITCHING_PROTOCOLS`  <sub>(macro)</sub>

```c
#define HTTP_SWITCHING_PROTOCOLS           101
```

#### `HTTP_OK`  <sub>(macro)</sub>

```c
#define HTTP_OK                            200
```

#### `HTTP_CREATED`  <sub>(macro)</sub>

```c
#define HTTP_CREATED                       201
```

#### `HTTP_ACCEPTED`  <sub>(macro)</sub>

```c
#define HTTP_ACCEPTED                      202
```

#### `HTTP_NON_AUTHORITATIVE`  <sub>(macro)</sub>

```c
#define HTTP_NON_AUTHORITATIVE             203
```

#### `HTTP_NO_CONTENT`  <sub>(macro)</sub>

```c
#define HTTP_NO_CONTENT                    204
```

#### `HTTP_RESET_CONTENT`  <sub>(macro)</sub>

```c
#define HTTP_RESET_CONTENT                 205
```

#### `HTTP_PARTIAL_CONTENT`  <sub>(macro)</sub>

```c
#define HTTP_PARTIAL_CONTENT               206
```

#### `HTTP_MULTIPLE_CHOICES`  <sub>(macro)</sub>

```c
#define HTTP_MULTIPLE_CHOICES              300
```

#### `HTTP_MOVED_PERMANENTLY`  <sub>(macro)</sub>

```c
#define HTTP_MOVED_PERMANENTLY             301
```

#### `HTTP_MOVED_TEMPORARILY`  <sub>(macro)</sub>

```c
#define HTTP_MOVED_TEMPORARILY             302
```

#### `HTTP_SEE_OTHER`  <sub>(macro)</sub>

```c
#define HTTP_SEE_OTHER                     303
```

#### `HTTP_NOT_MODIFIED`  <sub>(macro)</sub>

```c
#define HTTP_NOT_MODIFIED                  304
```

#### `HTTP_USE_PROXY`  <sub>(macro)</sub>

```c
#define HTTP_USE_PROXY                     305
```

#### `HTTP_BAD_REQUEST`  <sub>(macro)</sub>

```c
#define HTTP_BAD_REQUEST                   400
```

#### `HTTP_UNAUTHORIZED`  <sub>(macro)</sub>

```c
#define HTTP_UNAUTHORIZED                  401
```

#### `HTTP_PAYMENT_REQUIRED`  <sub>(macro)</sub>

```c
#define HTTP_PAYMENT_REQUIRED              402
```

#### `HTTP_FORBIDDEN`  <sub>(macro)</sub>

```c
#define HTTP_FORBIDDEN                     403
```

#### `HTTP_NOT_FOUND`  <sub>(macro)</sub>

```c
#define HTTP_NOT_FOUND                     404
```

#### `HTTP_METHOD_NOT_ALLOWED`  <sub>(macro)</sub>

```c
#define HTTP_METHOD_NOT_ALLOWED            405
```

#### `HTTP_NOT_ACCEPTABLE`  <sub>(macro)</sub>

```c
#define HTTP_NOT_ACCEPTABLE                406
```

#### `HTTP_PROXY_AUTHENTICATION_REQUIRED`  <sub>(macro)</sub>

```c
#define HTTP_PROXY_AUTHENTICATION_REQUIRED 407
```

#### `HTTP_REQUEST_TIME_OUT`  <sub>(macro)</sub>

```c
#define HTTP_REQUEST_TIME_OUT              408
```

#### `HTTP_CONFLICT`  <sub>(macro)</sub>

```c
#define HTTP_CONFLICT                      409
```

#### `HTTP_GONE`  <sub>(macro)</sub>

```c
#define HTTP_GONE                          410
```

#### `HTTP_LENGTH_REQUIRED`  <sub>(macro)</sub>

```c
#define HTTP_LENGTH_REQUIRED               411
```

#### `HTTP_PRECONDITION_FAILED`  <sub>(macro)</sub>

```c
#define HTTP_PRECONDITION_FAILED           412
```

#### `HTTP_REQUEST_ENTITY_TOO_LARGE`  <sub>(macro)</sub>

```c
#define HTTP_REQUEST_ENTITY_TOO_LARGE      413
```

#### `HTTP_REQUEST_URI_TOO_LARGE`  <sub>(macro)</sub>

```c
#define HTTP_REQUEST_URI_TOO_LARGE         414
```

#### `HTTP_UNSUPPORTED_MEDIA_TYPE`  <sub>(macro)</sub>

```c
#define HTTP_UNSUPPORTED_MEDIA_TYPE        415
```

#### `HTTP_INTERNAL_SERVER_ERROR`  <sub>(macro)</sub>

```c
#define HTTP_INTERNAL_SERVER_ERROR         500
```

#### `HTTP_NOT_IMPLEMENTED`  <sub>(macro)</sub>

```c
#define HTTP_NOT_IMPLEMENTED               501
```

#### `HTTP_BAD_GATEWAY`  <sub>(macro)</sub>

```c
#define HTTP_BAD_GATEWAY                   502
```

#### `HTTP_SERVICE_UNAVAILABLE`  <sub>(macro)</sub>

```c
#define HTTP_SERVICE_UNAVAILABLE           503
```

#### `HTTP_GATEWAY_TIME_OUT`  <sub>(macro)</sub>

```c
#define HTTP_GATEWAY_TIME_OUT              504
```

#### `HTTP_VERSION_NOT_SUPPORTED`  <sub>(macro)</sub>

```c
#define HTTP_VERSION_NOT_SUPPORTED         505
```

#### `HTTP_VARIANT_ALSO_VARIES`  <sub>(macro)</sub>

```c
#define HTTP_VARIANT_ALSO_VARIES           506
```

### `dirb/src/lanza_ataque.c`


#### `lanza_ataque`  <sub>(function)</sub>

> LANZA_ATAQUE: Lanza el bucle de peticiones y muestra los resultados

```c
void lanza_ataque(char *inicio, struct words *wordlist)
```

### `dirb/src/options.c`


#### `get_options`  <sub>(function)</sub>

> OPTIONS: Maneja las opciones de usuario

```c
void get_options(void)
```

### `dirb/src/resume.c`


#### `_GNU_SOURCE`  <sub>(macro)</sub>

> RESUME  
> resume.c - Dump y Resume de sesiones de escaneo

```c
#define _GNU_SOURCE
```

#### `dump`  <sub>(function)</sub>

> DUMP: Vuelca a fichero el estado de una sesion de escaneo

```c
void dump(void)
```

#### `resume`  <sub>(function)</sub>

> RESUME: Restaura el estado de una sesion de escaneo

```c
void resume(void)
```

#### `mkpath`  <sub>(function)</sub>

> MKPATH: Crea el el directorio de volcado recursivamente (pcatalina)

```c
int mkpath(const char *s, mode_t mode)
```

### `dirb/src/utils.c`


#### `limpia_url`  <sub>(function)</sub>

> LIMPIA_URL: Corta la cadena si contiene los caracteres (\r), (\n) o ( )

```c
void limpia_url(char *limpia)
```

#### `barra`  <sub>(function)</sub>

> BARRA: A�ade una barra (/) al final de una cadena si no la tiene

```c
void barra(char *barr)
```

#### `guardadir`  <sub>(function)</sub>

> GUARDADIR: Guarda un directorio en la lista de directorios

```c
void guardadir(char *direccion)
```

#### `elimina_dupwords`  <sub>(function)</sub>

> ELIMINA_DUPWORDS: Elimina elementos duplicados en una lista de palabras

```c
void elimina_dupwords(struct words *puntero)
```

#### `abrir_file`  <sub>(function)</sub>

> ABRIR_FILE: Abre un fichero de output

```c
FILE *abrir_file(char *file)
```

#### `location_cmp`  <sub>(function)</sub>

> LOCATION_CMP: Compara 2 cabeceras Location

```c
int location_cmp(char *A, char *B)
```

#### `location_clean`  <sub>(function)</sub>

> LOCATION_CLEAN: Limpia la cacebera location

```c
void location_clean(char *cleaned, char *toelim)
```

#### `check_url`  <sub>(function)</sub>

> CHECK_URL: Comprueba que la URL inicial tiene el formato correcto

```c
void check_url(char *url)
```

#### `islistable`  <sub>(function)</sub>

> ISLISTABLE: Comprueba si un directorio es listable o no

```c
int islistable(char *direccion)
```

#### `kbhit`  <sub>(function)</sub>

> KBHIT: Comprueba si alguna tecla ha sido pulsada (no bloqueante)

```c
char kbhit(void)
```

#### `cierre`  <sub>(function)</sub>

> CIERRE: Codigo de finalizacion

```c
void cierre(void)
```

#### `code2string`  <sub>(function)</sub>

> CODE2STRING: Convierte un codigo en su cadena equivalente

```c
char *code2string(struct code *a, u_int v)
```

#### `init_exts`  <sub>(function)</sub>

> INIT_EXTS: Inicializa el array de extensiones

```c
void init_exts(void)
```

#### `uri_decode`  <sub>(function)</sub>

> URI_DECODE: Decodifica una cadena con caracteres uri-encoded

```c
char *uri_decode(char *uri)
```

#### `compare_str`  <sub>(function)</sub>

> COMPARE_STR: Devuelve la posicion del primer caracater diferente

```c
int compare_str(char *A, char *B)
```


## `web2dic` <a id='pkg-web2dic'></a>

*1 documented items*


### `dirb/web2dic/html2dic.c`


#### `main`  <sub>(function)</sub>

```c
int main(int argc, char **argv)
```
