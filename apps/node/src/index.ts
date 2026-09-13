import { fib as nativeFib } from "@packages/api/node"

function fib(n: number): number {
  if (n === 0) {
    return 0;
  }

  if (n === 1) {
    return 1;
  }

  return fib(n - 1) + fib(n - 2)
}

const startNative = performance.now()
console.log(`nativeFib(45) = ${nativeFib(45)}`)
const endNative = performance.now()

console.log(`fib(45) = ${fib(45)}`)
const endFib = performance.now()

console.log(`nativeFib took ${endNative - startNative}ms, fib took ${endFib - startNative}ms`)
