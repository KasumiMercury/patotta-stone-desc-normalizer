import { open } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api/tauri";

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
		<div>
		</div>
	);
}

export default App;
