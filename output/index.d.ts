/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export declare class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export declare function alignedFree(mem: bigint): void
export declare function asciiDigitValue(c: number): number
export declare function asciiTolower(c: number): number
export declare function asciiToupper(c: number): number
export declare function asciiXdigitValue(c: number): number
export declare function atomicIntAdd(atomic: ExternalObject<number>, val: number): number
export declare function atomicIntAnd(atomic: ExternalObject<number>, val: number): number
export declare function atomicIntCompareAndExchange(atomic: ExternalObject<number>, oldval: number, newval: number): boolean
export declare function atomicIntDecAndTest(atomic: ExternalObject<number>): boolean
export declare function atomicIntExchange(atomic: ExternalObject<number>, newval: number): number
export declare function atomicIntGet(atomic: ExternalObject<number>): number
export declare function atomicIntInc(atomic: ExternalObject<number>): void
export declare function atomicIntOr(atomic: ExternalObject<number>, val: number): number
export declare function atomicIntSet(atomic: ExternalObject<number>, newval: number): void
export declare function atomicIntXor(atomic: ExternalObject<number>, val: number): number
export declare function atomicPointerAdd(atomic: bigint, val: number): number
export declare function atomicPointerCompareAndExchange(atomic: bigint, oldval: bigint, newval: bigint): boolean
export declare function atomicPointerSet(atomic: bigint, newval: bigint): void
export declare function atomicRcBoxRelease(memBlock: bigint): void
export declare function atomicRefCountCompare(arc: ExternalObject<number>, val: number): boolean
export declare function atomicRefCountDec(arc: ExternalObject<number>): boolean
export declare function atomicRefCountInc(arc: ExternalObject<number>): void
export declare function atomicRefCountInit(arc: ExternalObject<number>): void
export declare function bitLock(address: ExternalObject<number>, lockBit: number): void
export declare function bitTrylock(address: ExternalObject<number>, lockBit: number): boolean
export declare function bitUnlock(address: ExternalObject<number>, lockBit: number): void
export declare function blowChunks(): void
export declare function bookmarkFileErrorQuark(): number
export declare function checkVersion(requiredMajor: number, requiredMinor: number, requiredMicro: number): string
export declare function clearError(): void
export declare function close(fd: number): boolean
export declare function closefrom(lowfd: number): number
export declare function convertErrorQuark(): number
export declare function datasetDestroy(datasetLocation: bigint): void
export declare function dateGetMondayWeeksInYear(year: number): number
export declare function dateGetSundayWeeksInYear(year: number): number
export declare function dateIsLeapYear(year: number): boolean
export declare function dateValidDay(day: number): boolean
export declare function dateValidJulian(julianDate: number): boolean
export declare function dateValidYear(year: number): boolean
export declare function directEqual(v1: bigint, v2: bigint): boolean
export declare function directHash(v: bigint): number
export declare function doubleEqual(v1: bigint, v2: bigint): boolean
export declare function doubleHash(v: bigint): number
export declare function fdwalkSetCloexec(lowfd: number): number
export declare function fileErrorQuark(): number
export declare function free(mem: bigint): void
export declare function fsync(fd: number): number
export declare function getApplicationName(): string
export declare function getCodeset(): string
export declare function getCurrentDir(): string
export declare function getHomeDir(): string
export declare function getHostName(): string
export declare function getMonotonicTime(): number
export declare function getNumProcessors(): number
export declare function getPrgname(): string
export declare function getRealName(): string
export declare function getRealTime(): number
export declare function getTmpDir(): string
export declare function getUserCacheDir(): string
export declare function getUserConfigDir(): string
export declare function getUserDataDir(): string
export declare function getUserName(): string
export declare function getUserRuntimeDir(): string
export declare function getUserStateDir(): string
export declare function idleRemoveByData(data: bigint): boolean
export declare function int64Equal(v1: bigint, v2: bigint): boolean
export declare function int64Hash(v: bigint): number
export declare function intEqual(v1: bigint, v2: bigint): boolean
export declare function intHash(v: bigint): number
export declare function ioChannelErrorQuark(): number
export declare function keyFileErrorQuark(): number
export declare function listPopAllocator(): void
export declare function logGetDebugEnabled(): boolean
export declare function logSetDebugEnabled(enabled: boolean): void
export declare function logWriterDefaultSetUseStderr(useStderr: boolean): void
export declare function logWriterIsJournald(outputFd: number): boolean
export declare function logWriterSupportsColor(outputFd: number): boolean
export declare function mainDepth(): number
export declare function markupErrorQuark(): number
export declare function memChunkInfo(): void
export declare function nodePopAllocator(): void
export declare function nullifyPointer(nullifyLocation: bigint): void
export declare function numberParserErrorQuark(): number
export declare function onceInitEnterPointer(location: bigint): boolean
export declare function onceInitLeavePointer(location: bigint, result: bigint): void
export declare function optionErrorQuark(): number
export declare function pathBufEqual(v1: bigint, v2: bigint): boolean
export declare function pointerBitLock(address: bigint, lockBit: number): void
export declare function pointerBitTrylock(address: bigint, lockBit: number): boolean
export declare function pointerBitUnlock(address: bigint, lockBit: number): void
export declare function quarkToString(quark: number): string
export declare function randomInt(): number
export declare function randomIntRange(begin: number, end: number): number
export declare function randomSetSeed(seed: number): void
export declare function rcBoxRelease(memBlock: bigint): void
export declare function refCountCompare(rc: ExternalObject<number>, val: number): boolean
export declare function refCountDec(rc: ExternalObject<number>): boolean
export declare function refCountInc(rc: ExternalObject<number>): void
export declare function refCountInit(rc: ExternalObject<number>): void
export declare function regexErrorQuark(): number
export declare function reloadUserSpecialDirsCache(): void
export declare function shellErrorQuark(): number
export declare function slistPopAllocator(): void
export declare function sourceRemove(tag: number): boolean
export declare function sourceRemoveByUserData(userData: bigint): boolean
export declare function spacedPrimesClosest(num: number): number
export declare function spawnCheckWaitStatus(waitStatus: number): boolean
export declare function spawnClosePid(pid: number): void
export declare function spawnErrorQuark(): number
export declare function spawnExitErrorQuark(): number
export declare function strEqual(v1: bigint, v2: bigint): boolean
export declare function strHash(v: bigint): number
export declare function strerror(errnum: number): string
export declare function strsignal(signum: number): string
export declare function testDisableCrashReporting(): void
export declare function testFail(): void
export declare function testFailed(): boolean
export declare function testGetPath(): string
export declare function testQueueFree(gfreePointer: bigint): void
export declare function testRandInt(): number
export declare function testRandIntRange(begin: number, end: number): number
export declare function testRun(): number
export declare function testSetNonfatalAssertions(): void
export declare function testSubprocess(): boolean
export declare function testTimerStart(): void
export declare function testTrapHasPassed(): boolean
export declare function testTrapReachedTimeout(): boolean
export declare function threadErrorQuark(): number
export declare function threadExit(retval: bigint): void
export declare function threadPoolGetMaxIdleTime(): number
export declare function threadPoolGetMaxUnusedThreads(): number
export declare function threadPoolGetNumUnusedThreads(): number
export declare function threadPoolSetMaxIdleTime(interval: number): void
export declare function threadPoolSetMaxUnusedThreads(maxThreads: number): void
export declare function threadPoolStopUnusedThreads(): void
export declare function threadYield(): void
export declare function unixErrorQuark(): number
export declare function unixSetFdNonblocking(fd: number, nonblock: boolean): boolean
export declare function uriErrorQuark(): number
export declare function uuidStringRandom(): string
export declare function variantParseErrorQuark(): number
export declare class VoidPointer { }
export declare class Allocator { }
export declare class Array { }
export declare class AsyncQueue { }
export declare class BookmarkFile { }
export declare class ByteArray { }
export declare class Bytes { }
export declare class Cache { }
export declare class Checksum { }
export declare class Completion { }
export declare class Cond { }
export declare class Data { }
export declare class Date { }
export declare class DateTime { }
export declare class DebugKey { }
export declare class Dir { }
export declare class Error { }
export declare class HashTable { }
export declare class HashTableIter { }
export declare class Hmac { }
export declare class Hook { }
export declare class HookList { }
export type IOChannel = IoChannel
export declare class IoChannel { }
export type IOFuncs = IoFuncs
export declare class IoFuncs { }
export declare class KeyFile { }
export declare class List { }
export declare class LogField { }
export declare class MainContext { }
export declare class MainLoop { }
export declare class MappedFile { }
export declare class MarkupParseContext { }
export declare class MarkupParser { }
export declare class MatchInfo { }
export declare class MemChunk { }
export declare class MemVTable { }
export declare class Node { }
export declare class Once { }
export declare class OptionContext { }
export declare class OptionEntry { }
export declare class OptionGroup { }
export declare class PathBuf { }
export declare class PatternSpec { }
export type PollFD = PollFd
export declare class PollFd { }
export declare class Private { }
export declare class PtrArray { }
export declare class Queue { }
export type RWLock = RwLock
export declare class RwLock { }
export declare class Rand { }
export declare class RecMutex { }
export declare class Regex { }
export declare class Relation { }
export declare class SList { }
export declare class Scanner { }
export declare class ScannerConfig { }
export declare class Sequence { }
export declare class SequenceIter { }
export declare class Source { }
export declare class SourceCallbackFuncs { }
export declare class SourceFuncs { }
export declare class SourcePrivate { }
export declare class StatBuf { }
export declare class String { }
export declare class StringChunk { }
export declare class StrvBuilder { }
export declare class TestCase { }
export declare class TestConfig { }
export declare class TestLogBuffer { }
export declare class TestLogMsg { }
export declare class TestSuite { }
export declare class Thread { }
export declare class ThreadPool { }
export declare class TimeVal { }
export declare class TimeZone { }
export declare class Timer { }
export declare class TrashStack { }
export declare class Tree { }
export declare class TreeNode { }
export declare class Tuples { }
export declare class UnixPipe { }
export declare class Uri { }
export declare class UriParamsIter { }
export declare class Variant { }
export declare class VariantBuilder { }
export declare class VariantDict { }
export declare class VariantType { }
