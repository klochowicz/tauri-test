import { useState } from 'react'
import logo from './logo.svg'
import './App.css'
import {invoke} from "@tauri-apps/api/tauri";

function App() {
  const [count, setCount] = useState(0)

  // Invocation from JS

  invoke('my_custom_command', {
    number: 42,
  })
      .then((res) =>
          console.log(`Message: ${res.message}, Other Val: ${res.other_val}`)
      )
      .catch((e) => console.error(e))

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Hello Vite + React!</p>
        <p>
          <button type="button" onClick={() => setCount((count) => count + 1)}>
            count is: {count}
          </button>
        </p>
        <p>
          This is bullshit
        </p>
        <p>
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
          </a>
          {' | '}
          <a
            className="App-link"
            href="https://vitejs.dev/guide/features.html"
            target="_blank"
            rel="noopener noreferrer"
          >
            Vite Docs
          </a>
        </p>
      </header>
    </div>
  )
}

export default App
