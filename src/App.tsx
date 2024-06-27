import {open} from "@tauri-apps/api/dialog";
import {useEffect, useState} from "react";
import "./App.css";
import {invoke} from "@tauri-apps/api/tauri";

function App() {
	const [isLoaded, setIsLoaded] = useState(false);
	const [filePath, setFilePath] = useState("");
	const [openConfirmDialog, setOpenConfirmDialog] = useState(false);
	const [error, setError] = useState("");

	// Check the data has already existed in the sqlite database
	// If the data is already existed, load the data from the database
	// If the data is not existed, display button to open dialog of file selection
	useEffect(() => {
		(async () => {
			// result is json object
			const result: string = await invoke("check_data_exists");
			// parse the json object
			const existence_info = JSON.parse(result);
			if (existence_info.exists) {
				setFilePath(existence_info.path);
				setIsLoaded(true);
			} else {
				setIsLoaded(false);
			}
		})();
	}, []);

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
			setOpenConfirmDialog(true);
		});
	}

	async function loadCSV() {
		// load csv file
		await invoke("load_csv", { path: filePath });
	}

	function loadFile() {
		// load file
		loadCSV()
			.then(() => {
				setIsLoaded(true);
				setOpenConfirmDialog(false);
			})
			.catch((err) => {
				console.error(err);
				setError(err);
				setIsLoaded(false);
				setOpenConfirmDialog(false);
			});
	}

	return (
		<div className="m-0 flex justify-center text-center pt-6 px-2 flex-col">
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
				{error && (
					<div className="w-full">
						<p className="text-red-500">{error}</p>
					</div>
				)}
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

			{openConfirmDialog && (
				<div className="fixed top-0 left-0 w-full h-full bg-black bg-opacity-50 flex justify-center items-center">
					<div className="bg-zinc-800 p-4 rounded-md border border-zinc-600 gap-y-6 flex flex-col">
						<div className="flex flex-col w-fit">
							<p>Do you want to load this file?</p>
							<p className="text-xs">{filePath}</p>
						</div>
						<div className="flex justify-center gap-2">
							<button
								type="button"
								className="py-2 px-4 text-xs hover:outline-neutral-100 rounded-md border-2 border-zinc-700 bg-zinc-800"
								onClick={loadFile}
							>
								Yes
							</button>
							<button
								type="button"
								className="py-2 px-4 text-xs hover:outline-neutral-100 rounded-md border-2 border-zinc-700 bg-zinc-800"
								onClick={() => {
									setOpenConfirmDialog(false);
									setIsLoaded(false);
								}}
							>
								No
							</button>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}

export default App;
