import { open } from "@tauri-apps/api/dialog";
import { useState } from "react";
import "./App.css";

function App() {
	const [filePath, setFilePath] = useState("");

	function openLoadDialog() {
		open({
			directory: false,
			multiple: false,
			// filters: [{ name: 'csv', extensions: ['csv'] }],
		}).then((res) => {
			// if res is null, the user closed the dialog without selecting a file
			if (res === null) {
				return;
			}
			// if res is an array, occurs error
			// can't select multiple files
			if (Array.isArray(res)) {
				console.error("Error: open dialog return an array");
				return;
			}
			setFilePath(res);
		});
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

			<p>{filePath}</p>
		</div>
	);
}

export default App;
