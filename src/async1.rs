// Future == Promise.
//
// When rust sees an async block, it complies it as an anonymous data type block which uses
// Futures.
//
// When rust sees an async func, it compiles the func as a non - async func with a async block
// which uses Futures.
//
// So 
//
// Async func === Non async func return Futures.
//
// With the threads, its OS's func, to check how long a func needs to run, at what order etc.
//
// But with async func, it runtime's job, to check it. When you run 2 async func, we get the same 
// order everytime we run it, it is because runtime, check and doesnt let a func go ahead by
// leaving another func behind.
