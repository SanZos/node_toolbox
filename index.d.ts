/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export declare class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export interface DirInfo {
  path: string
  recursive: boolean
}
/**
 * Add a watcher to a directory it's recursive by default
 *
 * If you don't want it to be recursive, use { path: 'dir_path', recurse: false }
 *
 * Every time a file is modified or added or deleted the callback is called with the message : "[dir_path] [change_type]"
 *
 * enum change_type { Created, Changed, Deleted }
 *
 * @returns a handler to terminate the watcher if the watcher has been started else false with an error message in the callback
 *
 * ```js
 * const { setWatcher } = require('node_toolbox');
 * function listener(err, message) {
 *   if (err) {
 *     // log the error and exit the listener
 *     console.error(err);
 *     return;
 *   }
 *
 *   console.log(data);
 *   // if a new file named index.ts is created, the data will be 'index.ts Created'
 *   // if a file named index.ts is modified, the data will be 'index.ts Changed'
 *   // if a file named index.ts is deleted, the data will be 'index.ts Deleted'
 * }
 * const handler = setWatcher('path', listener);
 * if(!handler) {
 *   // things to do if no watcher
 * }
 * ```
 */
export declare function setWatcher(config: string | DirInfo, callback: (err: null | Error, result: string) => void): ExternalObject<ThreadHandler> | boolean
/**
 * Clear a previously instantiate watcher
 * ```js
 * const handler = setWatcher('path', () => {});
 * if(!handler) {
 *   // things to do if no watcher
 * }
 *
 * // wait some time and cleanup the watcher
 * clearWatcher(handler);
 * ```
 */
export declare function clearWatcher(handle: ExternalObject<ThreadHandler>): boolean