import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import "./App.css";

function App() {
	const [greetMsg, setGreetMsg] = useState("");
	const [name, setName] = useState("");

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
		setGreetMsg(await invoke("greet", { name }));
	}

	return (
		<div className="container">
			<div className="w-full flex justify-end gap-2">
				<button
					type="button"
					className="py-2 px-4 text-xs hover:outline-neutral-100 rounded-md border-2 border-zinc-700 bg-zinc-800"
					onClick={openLoadDialog}
				>
					Load
				</button>
				<button
					type="button"
					className="py-2 px-4 text-xs hover:outline-neutral-100 rounded-md border-2 border-zinc-700 bg-zinc-800"
				>
					Export
				</button>
			</div>
			<h1 className="text-xl">Welcome to Tauri!</h1>

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
