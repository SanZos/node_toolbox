# Tools for nodejs written in rust with napi-rs
❗ Only Linux binary is supplied for now ❗

Install from github release:
* `npm install SanZos/node_toolbox#v1.0.0`

## watch
Start a watcher to a directory it's recursive by default

If you don't want it to be recursive, use { path: 'dir_path', recurse: false }

### Watcher with recurse
```js
const { setWatcher } = require('node_toolbox');
function listener(err, message) {
    if (err) {
        // log the error and exit the listener
        console.error(err);
        return;
    }
    console.log(data);
    // if a new file named index.ts is created, the data will be 'index.ts Created'
    // if a file named index.ts is modified, the data will be 'index.ts Changed'
    // if a file named index.ts is deleted, the data will be 'index.ts Deleted'
}
let handler = setWatcher('path', listener);
if(!handler) {
    // things to do if no watcher
}
```

### Watcher without recurse
```js
const { setWatcher } = require('node_toolbox');
function listener(err, message) {
    if (err) {
        // log the error and exit the listener
        console.error(err);
        return;
    }
    console.log(data);
    // if a new file named index.ts is created, the data will be 'index.ts Created'
    // if a file named index.ts is modified, the data will be 'index.ts Changed'
    // if a file named index.ts is deleted, the data will be 'index.ts Deleted'
}
let handler = setWatcher({ path: 'path', recurse: false }, listener);
if(!handler) {
    // things to do if no watcher
}
```

### Watcher for a file
```js
const { setWatcher } = require('node_toolbox');
function listener(err, message) {
    if (err) {
        // log the error and exit the listener
        console.error(err);
        return;
    }
    console.log(data);
    // if a file named index.ts is modified, the data will be 'index.ts Changed'
    // if a file named index.ts is deleted, the data will be 'index.ts Deleted'
}
let handler = setWatcher({ path: 'path.file', recurse: false }, listener);
if(!handler) {
    // things to do if no watcher
}
```

### Cleanup when you don't need the watcher anymore
```js
const { setWatcher, clearWatcher } = require('node_toolbox');
function listener(err, message) {
    // do your things
}
let handler = setWatcher('dir/or/file', listener);
if(!handler) {
    // things to do if no watcher
}

// Stop the watcher
clearWatcher(handler);
```