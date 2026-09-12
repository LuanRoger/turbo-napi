import { add, divide, multiply, subtract } from "@packages/api/web";

const left = 12;
const right = 4;

function App() {
  return (
    <main>
      <h1>Rust math in WebAssembly</h1>
      <p>
        Operations using {left} and {right}:
      </p>
      <ul>
        <li>Add: {add(left, right)}</li>
        <li>Subtract: {subtract(left, right)}</li>
        <li>Multiply: {multiply(left, right)}</li>
        <li>Divide: {divide(left, right)}</li>
      </ul>
    </main>
  );
}

export default App;
