const js = import('@ginlon/hello-wasm')

js.then(js => {
  const n = 20;

  console.time('rust-wasm-fibonacci')
  console.log(`fibonacci(${n}) = ${js.fibonacci(n)}`)
  console.timeEnd('rust-wasm-fibonacci')

  console.time('fibonacci')
  console.log(`fibonacci(${n}) = ${fibonacci(n)}`)
  console.timeEnd('fibonacci')
  
})

function fibonacci(n){
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}