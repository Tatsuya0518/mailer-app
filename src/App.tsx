/*
import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

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
    </main>
  );
}

export default App;
*/

// 例: Reactコンポーネント (src/App.tsx など)
import React, { useState } from 'react';
//import { invoke } from '@tauri-apps/api/tauri';
import { invoke } from '@tauri-apps/api/core';

function App() {
  const [to, setTo] = useState('');
  const [subject, setSubject] = useState('');
  const [body, setBody] = useState('');
  const [status, setStatus] = useState('');

  const handleSendEmail = async (event: React.FormEvent) => {
    event.preventDefault(); // フォームのデフォルト送信を防ぐ
    setStatus('Sending...');

    const payload = { to, subject, body };

    try {
      // バックエンドの 'send_email' コマンドを呼び出す
      await invoke('send_email', { payload: payload }); // 第2引数のキー 'payload' はRust側の引数名と一致させる
      setStatus('Email sent successfully!');
      // 成功したらフォームをクリアするなど
      // setTo('');
      // setSubject('');
      // setBody('');
    } catch (error) {
      console.error('Failed to send email:', error);
      setStatus(`Failed to send email: ${error}`);
    }
  };

  return (
    <div className="container">
      <h1>Send Email</h1>
      <form onSubmit={handleSendEmail}>
        <div>
          <label htmlFor="to">To:</label>
          <input
            id="to"
            type="email"
            value={to}
            onChange={(e) => setTo(e.target.value)}
            required
          />
        </div>
        <div>
          <label htmlFor="subject">Subject:</label>
          <input
            id="subject"
            type="text"
            value={subject}
            onChange={(e) => setSubject(e.target.value)}
            required
          />
        </div>
        <div>
          <label htmlFor="body">Body:</label>
          <textarea
            id="body"
            value={body}
            onChange={(e) => setBody(e.target.value)}
            required
          />
        </div>
        <button type="submit">Send Email</button>
      </form>
      {status && <p>{status}</p>}
    </div>
  );
}

export default App;