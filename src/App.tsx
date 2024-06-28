import { open } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api/tauri";
import type {Response} from "@tauri-apps/api/http";

interface LoadHistory {
	id: number;
	path: string;
	count: number;
	loaded_at: string;
}

interface ExistenceInfo {
	exists: boolean;
	count: number;
	last_loaded_at: string;
	histories: LoadHistory[];
}

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
			const existence_info: Response<ExistenceInfo> = await invoke("check_data_existence");
			// parse the json object
			if (existence_info.data.exists) {
				setFilePath(existence_info.data.last_loaded_at);
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
		<div>
			{isLoaded ? (
				<div>
					<p>File path: {filePath}</p>
				</div>
			) : (
				<div>
					<h1>Data is not loaded</h1>
					{/* biome-ignore lint/a11y/useButtonType: <explanation> */}
					<button onClick={openLoadDialog}>Open dialog</button>
				</div>
			)}
			{/* the dialog to load the file */}
			{openConfirmDialog && (
				<div>
					<p>Are you sure you want to load the file?</p>
					{/* biome-ignore lint/a11y/useButtonType: <explanation> */}
					<button onClick={loadFile}>Yes</button>
					{/* biome-ignore lint/a11y/useButtonType: <explanation> */}
					<button onClick={() => setOpenConfirmDialog(false)}>No</button>
				</div>
			)}
			{error && <p>Error: {error}</p>}
		</div>
	);
}

export default App;
