/*
** $Id: loadlib.c $
** Dynamic library loader for Lua
** See Copyright Notice in lua.h
**
** This module contains an implementation of loadlib for Unix systems
** that have dlfcn, an implementation for Windows, and a stub for other
** systems.
*/

#define loadlib_c
#define LUA_LIB

#include "lprefix.h"


#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "lua.h"

#include "lauxlib.h"
#include "lualib.h"


/*
** LUA_IGMARK is a mark to ignore all before it when building the
** luaopen_ function name.
*/
#if !defined (LUA_IGMARK)
#define LUA_IGMARK		"-"
#endif


/*
** LUA_CSUBSEP is the character that replaces dots in submodule names
** when searching for a C loader.
** LUA_LSUBSEP is the character that replaces dots in submodule names
** when searching for a Lua loader.
*/
#if !defined(LUA_CSUBSEP)
#define LUA_CSUBSEP		LUA_DIRSEP
#endif

#if !defined(LUA_LSUBSEP)
#define LUA_LSUBSEP		LUA_DIRSEP
#endif


/* prefix for open functions in C libraries */
#define LUA_POF		"luaopen_"

/* separator for open functions in C libraries */
#define LUA_OFSEP	"_"


#define LIB_FAIL	"open"


/*
** Special type equivalent to '(void*)' for functions in gcc
** (to suppress warnings when converting function pointers)
*/
typedef void (*voidf)(void);


/*
** {======================================================
** 'require' function
** =======================================================
*/


static int readable (const char *filename) {
  FILE *f = fopen(filename, "r");  /* try to open file */
  if (f == NULL) return 0;  /* open failed */
  fclose(f);
  return 1;
}


/*
** Get the next name in '*path' = 'name1;name2;name3;...', changing
** the ending ';' to '\0' to create a zero-terminated string. Return
** NULL when list ends.
*/
static const char *getnextfilename (char **path, char *end) {
  char *sep;
  char *name = *path;
  if (name == end)
    return NULL;  /* no more names */
  else if (*name == '\0') {  /* from previous iteration? */
    *name = *LUA_PATH_SEP;  /* restore separator */
    name++;  /* skip it */
  }
  sep = strchr(name, *LUA_PATH_SEP);  /* find next separator */
  if (sep == NULL)  /* separator not found? */
    sep = end;  /* name goes until the end */
  *sep = '\0';  /* finish file name */
  *path = sep;  /* will start next search from here */
  return name;
}


/*
** Given a path such as ";blabla.so;blublu.so", pushes the string
**
** no file 'blabla.so'
**	no file 'blublu.so'
*/
static void pusherrornotfound (lua_State *L, const char *path) {
  luaL_Buffer b;
  luaL_buffinit(L, &b);
  luaL_addstring(&b, "no file '");
  luaL_addgsub(&b, path, LUA_PATH_SEP, "'\n\tno file '");
  luaL_addstring(&b, "'");
  luaL_pushresult(&b);
}


static const char *searchpath (lua_State *L, const char *name,
                                             const char *path,
                                             const char *sep,
                                             const char *dirsep) {
  luaL_Buffer buff;
  char *pathname;  /* path with name inserted */
  char *endpathname;  /* its end */
  const char *filename;
  /* separator is non-empty and appears in 'name'? */
  if (*sep != '\0' && strchr(name, *sep) != NULL)
    name = luaL_gsub(L, name, sep, dirsep);  /* replace it by 'dirsep' */
  luaL_buffinit(L, &buff);
  /* add path to the buffer, replacing marks ('?') with the file name */
  luaL_addgsub(&buff, path, LUA_PATH_MARK, name);
  luaL_addchar(&buff, '\0');
  pathname = luaL_buffaddr(&buff);  /* writable list of file names */
  endpathname = pathname + luaL_bufflen(&buff) - 1;
  while ((filename = getnextfilename(&pathname, endpathname)) != NULL) {
    if (readable(filename))  /* does file exist and is readable? */
      return lua_pushstring(L, filename);  /* save and return name */
  }
  luaL_pushresult(&buff);  /* push path to create error message */
  pusherrornotfound(L, lua_tostring(L, -1));  /* create error message */
  return NULL;  /* not found */
}


static int ll_searchpath (lua_State *L) {
  const char *f = searchpath(L, luaL_checkstring(L, 1),
                                luaL_checkstring(L, 2),
                                luaL_optstring(L, 3, "."),
                                luaL_optstring(L, 4, LUA_DIRSEP));
  if (f != NULL) return 1;
  else {  /* error message is on top of the stack */
    luaL_pushfail(L);
    lua_insert(L, -2);
    return 2;  /* return fail + error message */
  }
}


static const char *findfile (lua_State *L, const char *name,
                                           const char *dirsep) {
#if defined(_WIN32)
  const char *path = ".\\?.lua;.\\?\\init.lua";
#else
  const char *path = "./?.lua;./?/init.lua";
#endif

  return searchpath(L, name, path, ".", dirsep);
}


static int checkload (lua_State *L, int stat, const char *filename) {
  if (l_likely(stat)) {  /* module loaded successfully? */
    lua_pushstring(L, filename);  /* will be 2nd argument to module */
    return 2;  /* return open function and file name */
  }
  else
    return luaL_error(L, "error loading module '%s' from file '%s':\n\t%s",
                          lua_tostring(L, 1), filename, lua_tostring(L, -1));
}


static int searcher_Lua (lua_State *L) {
  const char *filename;
  const char *name = luaL_checkstring(L, 1);
  filename = findfile(L, name, LUA_LSUBSEP);
  if (filename == NULL) return 1;  /* module not found in this path */
  return checkload(L, (luaL_loadfilex(L, filename, "t") == LUA_OK), filename);
}


static int searcher_preload (lua_State *L) {
  const char *name = luaL_checkstring(L, 1);
  lua_getfield(L, LUA_REGISTRYINDEX, LUA_PRELOAD_TABLE);
  if (lua_getfield(L, -1, name) == LUA_TNIL) {  /* not found? */
    lua_pushfstring(L, "no field package.preload['%s']", name);
    return 1;
  }
  else {
    lua_pushliteral(L, ":preload:");
    return 2;
  }
}


static void findloader (lua_State *L, const char *name) {
  const lua_CFunction c_searchers[] = {
      searcher_preload,
      searcher_Lua,
      NULL,
  };

  int i;
  luaL_Buffer msg;  /* to build error message */
  luaL_buffinit(L, &msg);

  /*  iterate over available searchers to find a loader */
  for (i = 0; ; i++) {
    luaL_addstring(&msg, "\n\t");  /* error-message prefix */
    if (c_searchers[i] == NULL) {  /* no more searchers? */
      lua_pop(L, 1);  /* remove nil */
      luaL_buffsub(&msg, 2);  /* remove prefix */
      luaL_pushresult(&msg);  /* create error message */
      luaL_error(L, "module '%s' not found:%s", name, lua_tostring(L, -1));
    }
    lua_pushcclosure(L, c_searchers[i], 1);
    lua_pushstring(L, name);
    lua_call(L, 1, 2);  /* call it */
    if (lua_isfunction(L, -2))  /* did it find a loader? */
      return;  /* module loader found */
    else if (lua_isstring(L, -2)) {  /* searcher returned error message? */
      lua_pop(L, 1);  /* remove extra return */
      luaL_addvalue(&msg);  /* concatenate error message */
    }
    else {  /* no error message */
      lua_pop(L, 2);  /* remove both returns */
      luaL_buffsub(&msg, 2);  /* remove prefix */
    }
  }
}


static int ll_require (lua_State *L) {
  const char *name = luaL_checkstring(L, 1);
  lua_settop(L, 1);  /* LOADED table will be at index 2 */
  lua_getfield(L, LUA_REGISTRYINDEX, LUA_LOADED_TABLE);
  lua_getfield(L, 2, name);  /* LOADED[name] */
  if (lua_toboolean(L, -1))  /* is it there? */
    return 1;  /* package is already loaded */
  /* else must load package */
  lua_pop(L, 1);  /* remove 'getfield' result */
  findloader(L, name);
  lua_rotate(L, -2, 1);  /* function <-> loader data */
  lua_pushvalue(L, 1);  /* name is 1st argument to module loader */
  lua_pushvalue(L, -3);  /* loader data is 2nd argument */
  /* stack: ...; loader data; loader function; mod. name; loader data */
  lua_call(L, 2, 1);  /* run loader to load module */
  /* stack: ...; loader data; result from loader */
  if (!lua_isnil(L, -1))  /* non-nil return? */
    lua_setfield(L, 2, name);  /* LOADED[name] = returned value */
  else
    lua_pop(L, 1);  /* pop nil */
  if (lua_getfield(L, 2, name) == LUA_TNIL) {   /* module set no value? */
    lua_pushboolean(L, 1);  /* use true as result */
    lua_copy(L, -1, -2);  /* replace loader result */
    lua_setfield(L, 2, name);  /* LOADED[name] = true */
  }
  lua_rotate(L, -2, 1);  /* loader data <-> module result  */
  return 2;  /* return module result and loader data */
}

/* }====================================================== */



static const luaL_Reg pk_funcs[] = {
  {"searchpath", ll_searchpath},
  /* placeholders */
  {"preload", NULL},
  {"loaded", NULL},
  {NULL, NULL}
};


static const luaL_Reg ll_funcs[] = {
  {"require", ll_require},
  {NULL, NULL}
};


LUAMOD_API int luaopen_package (lua_State *L) {
  luaL_newlib(L, pk_funcs);  /* create 'package' table */
  /* set field 'loaded' */
  luaL_getsubtable(L, LUA_REGISTRYINDEX, LUA_LOADED_TABLE);
  lua_setfield(L, -2, "loaded");
  /* set field 'preload' */
  luaL_getsubtable(L, LUA_REGISTRYINDEX, LUA_PRELOAD_TABLE);
  lua_setfield(L, -2, "preload");
  lua_pushglobaltable(L);
  lua_pushvalue(L, -2);  /* set 'package' as upvalue for next lib */
  luaL_setfuncs(L, ll_funcs, 1);  /* open lib into global table */
  lua_pop(L, 1);  /* pop global table */
  return 1;  /* return 'package' table */
}
