import { useState } from 'react';
import reactLogo from './assets/react.svg';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import './App.css';

function App() {
  const [greetMsg, setGreetMsg] = useState('');
  const [name, setName] = useState('');

  function openLoadDialog() {
    open({
      directory: false,
      // filters: [{ name: 'csv', extensions: ['csv'] }],
    }).then((res) => {
      console.log(res);
    });
  }

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke('greet', { name }));
  }

  return (
    <div className="container">
      <div className="w-full flex justify-end gap-2">
        <button
          className="py-2 px-4 text-xs hover:outline-neutral-100 rounded-md border-2 border-zinc-700 bg-zinc-800"
          onClick={openLoadDialog}
        >
          Load
        </button>
        <button className="py-2 px-4 text-xs hover:outline-neutral-100 rounded-md border-2 border-zinc-700 bg-zinc-800">
          Export
        </button>
      </div>
      <h1 className="text-xl">Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
