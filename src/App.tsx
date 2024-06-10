import { open } from "@tauri-apps/api/dialog";
import { useState } from "react";
import "./App.css";

function App() {
	const [filePath, setFilePath] = useState("");
	const [isLoaded, setIsLoaded] = useState(false);

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
			setIsLoaded(true);
		});
	}

	return (
		<div className="m-0 flex justify-center text-center pt-6 flex-col">
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
			<div>
				{isLoaded ? (
					<div className="w-full">
						<p>{filePath}</p>
					</div>
				) : (
					<div className="w-full h-96">
						<div className="flex justify-center items-center h-full">
							<p className="text-lg">No data</p>
						</div>
					</div>
				)}
			</div>
		</div>
	);
}

export default App;
