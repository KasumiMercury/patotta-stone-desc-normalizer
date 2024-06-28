import { open } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api/tauri";
import * as dayjs from "dayjs";

type ISODateString = string & { __brand: "ISODateString" };

function isISODateString(value: string): value is ISODateString {
	return dayjs(value).isValid();
}

function toISODateString(value: string): ISODateString {
	if (!isISODateString(value)) {
		throw new Error("Invalid ISO date string");
	}
	return value;
}

interface LoadHistory {
	id: number;
	path: string;
	count: number;
	loaded_at: ISODateString;
}

interface ExistenceInfo {
	exists: boolean;
	count: number;
	last_loaded_at: ISODateString;
	histories: LoadHistory[];
}

class LoadHistoryImpl implements LoadHistory {
	constructor(
		public id: number,
		public path: string,
		public count: number,
		public loaded_at: ISODateString,
	) {}
}

class ExistenceInfoImpl implements ExistenceInfo {
	constructor(
		public exists: boolean,
		public count: number,
		public last_loaded_at: ISODateString,
		public histories: LoadHistory[],
	) {}
}

async function fetchExistenceInfo(): Promise<ExistenceInfo> {
	try {
		const result = await invoke("check_data_existence");
		const data = JSON.parse(result as string);
		return new ExistenceInfoImpl(
			data.exists,
			data.count,
			toISODateString(data.last_loaded_at),
			data.histories.map(
				(history: LoadHistory) =>
					new LoadHistoryImpl(
						history.id,
						history.path,
						history.count,
						toISODateString(history.loaded_at),
					),
			),
		);
	} catch (error) {
		console.error("Failed to fetch existence info", error);
		throw error;
	}
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
			const existenceInfo = await fetchExistenceInfo();

			if (existenceInfo.exists) {
				setIsLoaded(true);
				setFilePath(existenceInfo.histories[0].path);
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
